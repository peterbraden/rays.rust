#!/usr/bin/env python3
"""Render speed over history: build master commits and time a fixed scene on each.

Each commit is checked out into a scratch worktree (target/bench/src), built with
its own Cargo.lock into a shared target directory (target/bench/target), and renders
the benchmark scene from *this* checkout (demo/benchmarks/teapot-sky.json: the Utah
teapot as a mesh, a checkered plane and a Rayleigh/Mie skysphere; written in the
oldest scene format so every commit reads it) `--runs` times. A row is appended to
the CSV per commit, oldest first, with:

  wall time      the whole process (scene load, sky tables, BVH build and render)
  render time    the renderer's own "Elapsed Time"
  rays           the renderer's "Rays Cast" (mean over runs)
  diff           mean absolute and RMS difference, in 8-bit levels, between this
                 commit's image and the previous row's (path-tracing noise alone
                 gives a few levels; larger changes mean the picture changed)

Commits already in the CSV are skipped, so it can be rerun as master moves on:

  scripts/benchmark.py                        # master's head, if not measured yet
  scripts/benchmark.py --history 12 --every 5 # backfill: 12 commits, every 5th merge
  scripts/benchmark.py abc123 def456          # these commits

Images are kept in target/bench/images/<sha>.png; when the previous row's image is
missing (a fresh clone), its commit is rebuilt and rendered once to compare against.
Times only compare between rows measured on the same machine (`cpu`, `threads`).
"""
import argparse
import csv
import datetime
import math
import os
import platform
import re
import shutil
import statistics
import subprocess
import sys
import time

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SCENE = os.path.join(ROOT, "demo/benchmarks/teapot-sky.json")
CSV = os.path.join(ROOT, "demo/benchmarks/history.csv")
WORK = os.path.join(ROOT, "target/bench")
FIELDS = [
    "commit", "commit_date", "subject", "measured_at", "cpu", "threads", "runs",
    "wall_mean_s", "wall_std_s", "render_mean_s", "render_std_s", "rays",
    "mrays_per_s", "diff_to", "diff_mean", "diff_rmse", "status",
]


def git(*args, cwd=ROOT):
    return subprocess.run(["git", *args], cwd=cwd, check=True, capture_output=True,
                          text=True).stdout.strip()


def cpu_name():
    try:
        with open("/proc/cpuinfo") as f:
            for line in f:
                if line.startswith("model name"):
                    return line.split(":", 1)[1].strip()
    except OSError:
        pass
    return platform.processor() or platform.machine()


def duration(text):
    """Rust's Debug format for a Duration: 1.5s, 759.29ms, 12.3µs."""
    m = re.match(r"([0-9.]+)\s*(s|ms|µs|us|ns)$", text.strip())
    if not m:
        return float("nan")
    scale = {"s": 1, "ms": 1e-3, "µs": 1e-6, "us": 1e-6, "ns": 1e-9}[m.group(2)]
    return float(m.group(1)) * scale


def read_rows(path):
    if not os.path.exists(path):
        return []
    with open(path, newline="") as f:
        return list(csv.DictReader(f))


def append_row(path, row):
    new = not os.path.exists(path)
    with open(path, "a", newline="") as f:
        w = csv.DictWriter(f, fieldnames=FIELDS)
        if new:
            w.writeheader()
        w.writerow(row)


def master_ref():
    for ref in ("origin/master", "master"):
        try:
            git("rev-parse", "--verify", "--quiet", ref)
            return ref
        except subprocess.CalledProcessError:
            continue
    sys.exit("no master branch")


def worktree():
    src = os.path.join(WORK, "src")
    if not os.path.exists(os.path.join(src, ".git")):
        os.makedirs(WORK, exist_ok=True)
        git("worktree", "add", "--detach", "--force", src, "HEAD")
    return src


def build(sha, log):
    src = worktree()
    git("checkout", "--detach", "--force", sha, cwd=src)
    git("clean", "-fdq", cwd=src)
    env = dict(os.environ, CARGO_TARGET_DIR=os.path.join(WORK, "target"))
    with open(log, "w") as f:
        r = subprocess.run(["cargo", "build", "--release"], cwd=src, env=env,
                           stdout=f, stderr=subprocess.STDOUT)
    return r.returncode == 0


def render_once(log):
    """One render of the scene from the worktree's root (old commits resolve the
    mesh path from the working directory, and write the image beside the scene)."""
    src = os.path.join(WORK, "src")
    scene = os.path.join(src, "bench-scene.json")
    shutil.copy(SCENE, scene)
    os.makedirs(os.path.join(src, "demo"), exist_ok=True)
    shutil.copy(os.path.join(ROOT, "demo/teapot.obj"), os.path.join(src, "demo/teapot.obj"))
    image = os.path.join(src, "bench-scene.png")
    for stale in (image, image + ".accum"):
        if os.path.exists(stale):
            os.remove(stale)
    binary = os.path.join(WORK, "target/release/rays")
    start = time.perf_counter()
    with open(log, "w") as f:
        r = subprocess.run([binary, "bench-scene.json"], cwd=src, stdout=f,
                           stderr=subprocess.STDOUT)
    wall = time.perf_counter() - start
    with open(log, errors="replace") as f:
        out = f.read()
    rays = re.search(r"Rays Cast: (\d+)", out)
    elapsed = re.search(r"Elapsed Time: (\S+)", out)
    if r.returncode != 0 or not os.path.exists(image):
        return None
    return {
        "wall": wall,
        "render": duration(elapsed.group(1)) if elapsed else float("nan"),
        "rays": int(rays.group(1)) if rays else 0,
        "image": image,
    }


def image_diff(a, b):
    from PIL import Image
    import numpy as np
    x = np.asarray(Image.open(a).convert("RGB"), dtype=np.float64)
    y = np.asarray(Image.open(b).convert("RGB"), dtype=np.float64)
    if x.shape != y.shape:
        return float("nan"), float("nan")
    d = x - y
    return float(np.abs(d).mean()), float(np.sqrt((d * d).mean()))


def image_path(sha):
    return os.path.join(WORK, "images", sha + ".png")


def ensure_image(sha):
    """The image of an already measured commit, rendering it again if need be."""
    path = image_path(sha)
    if os.path.exists(path):
        return path
    print(f"  re-rendering {sha[:9]} to compare against", flush=True)
    logs = os.path.join(WORK, "logs")
    if not build(sha, os.path.join(logs, f"{sha}.build.log")):
        return None
    r = render_once(os.path.join(logs, f"{sha}.render.log"))
    if r is None:
        return None
    shutil.copy(r["image"], path)
    return path


def measure(sha, runs, previous):
    info = git("show", "-s", "--format=%cI%x00%s", sha).split("\x00")
    row = {
        "commit": sha, "commit_date": info[0], "subject": info[1],
        "measured_at": datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds"),
        "cpu": cpu_name(), "threads": os.cpu_count(), "runs": 0, "status": "ok",
    }
    logs = os.path.join(WORK, "logs")
    os.makedirs(logs, exist_ok=True)
    os.makedirs(os.path.join(WORK, "images"), exist_ok=True)
    print(f"{sha[:9]} {info[0][:10]} {info[1][:60]}", flush=True)
    if not build(sha, os.path.join(logs, f"{sha}.build.log")):
        print("  build failed", flush=True)
        row["status"] = "build_failed"
        return row
    results = []
    for i in range(runs):
        r = render_once(os.path.join(logs, f"{sha}.render{i}.log"))
        if r is None:
            print("  render failed", flush=True)
            row["status"] = "render_failed"
            return row
        print(f"  run {i + 1}: {r['wall']:.2f} s wall, {r['render']:.2f} s render, "
              f"{r['rays']} rays", flush=True)
        results.append(r)
    shutil.copy(results[-1]["image"], image_path(sha))

    def stats(key):
        v = [r[key] for r in results]
        return statistics.mean(v), (statistics.stdev(v) if len(v) > 1 else 0.0)

    wall, wall_sd = stats("wall")
    render, render_sd = stats("render")
    rays = round(statistics.mean(r["rays"] for r in results))
    row.update({
        "runs": runs, "wall_mean_s": f"{wall:.3f}", "wall_std_s": f"{wall_sd:.3f}",
        "render_mean_s": f"{render:.3f}", "render_std_s": f"{render_sd:.3f}",
        "rays": rays, "mrays_per_s": f"{rays / render / 1e6:.3f}" if render > 0 else "",
    })
    if previous:
        prev_image = ensure_image(previous)
        if prev_image:
            mean, rmse = image_diff(image_path(sha), prev_image)
            row.update({"diff_to": previous, "diff_mean": f"{mean:.3f}",
                        "diff_rmse": f"{rmse:.3f}"})
    return row


def main():
    p = argparse.ArgumentParser(description=__doc__,
                                formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("revs", nargs="*", help="commits to measure (default: master's head)")
    p.add_argument("--history", type=int, metavar="N",
                   help="measure N commits of master's first-parent history")
    p.add_argument("--every", type=int, default=1, metavar="K",
                   help="with --history, take every Kth commit (default 1)")
    p.add_argument("--runs", type=int, default=5, help="renders per commit (default 5)")
    p.add_argument("--csv", default=CSV, help=f"results file (default {os.path.relpath(CSV, ROOT)})")
    a = p.parse_args()

    master = master_ref()
    if a.history:
        line = git("rev-list", "--first-parent", master).split()
        revs = line[::a.every][:a.history]
    else:
        revs = a.revs or [master]
    shas = [git("rev-parse", "--verify", r + "^{commit}") for r in revs]

    # Oldest first, so each row compares with the one measured before it.
    order = {s: i for i, s in enumerate(git("rev-list", "--first-parent", "--topo-order",
                                            "--reverse", master).split())}
    shas = sorted(dict.fromkeys(shas), key=lambda s: (order.get(s, math.inf),
                                                      git("show", "-s", "--format=%ct", s)))
    rows = read_rows(a.csv)
    done = {r["commit"] for r in rows}
    todo = [s for s in shas if s not in done]
    if not todo:
        print("nothing new to measure")
        return
    previous = next((r["commit"] for r in reversed(rows) if r["status"] == "ok"), None)
    for sha in todo:
        row = measure(sha, a.runs, previous)
        append_row(a.csv, row)
        if row["status"] == "ok":
            previous = sha
            print(f"  mean {row['wall_mean_s']} s wall, {row['render_mean_s']} s render, "
                  f"{row['rays']} rays, diff {row.get('diff_mean', '-')}", flush=True)


if __name__ == "__main__":
    main()
