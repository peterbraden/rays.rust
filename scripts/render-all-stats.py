#!/usr/bin/env python3
"""Statistics of scripts/render-all.sh's runs, kept so that re-runs can be compared.

render-all.sh calls `record` after each scene and `report` at the end. A row per scene per
run is appended to demo/benchmarks/render-all.csv (full-settings runs) or
target/render-all/previews.csv (runs with --samples), with:

  run, commit, dirty   the run's start (UTC), HEAD, and whether src/, shaders/ or Cargo
                       files differed from HEAD when it started
  cpu, threads, gpu    the machine; gpu is the adapter, when the GPU rendered the scene
  scene, scene_hash    the scene, and a hash of its file: times only compare between rows
                       of the same hash (files the scene names are not hashed)
  status               ok or failed (note: the log's last line)
  wall_s               the whole process; build_s, render_s the renderer's own figures
  integrator           gpu, wavefront or path-tracer; note gives why the GPU was refused
  width, height, samples, rays, mrays_per_s, peak_build_mib, primitives
  diff_mean, diff_rmse the new image against HEAD's, in 8-bit levels (needs Pillow and
                       numpy; blank otherwise). Sampling noise alone gives a few levels

  # A run in progress, or one made before render-all.sh recorded: import its logs
  scripts/render-all-stats.py import
  # Compare the latest run with the previous one on this machine
  scripts/render-all-stats.py report
"""
import argparse
import csv
import datetime
import hashlib
import io
import os
import platform
import re
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUT = os.path.join(ROOT, "target/render-all")
FULL_CSV = os.path.join(ROOT, "demo/benchmarks/render-all.csv")
PREVIEW_CSV = os.path.join(OUT, "previews.csv")
FIELDS = [
    "run", "commit", "dirty", "cpu", "threads", "gpu", "preview_samples", "scene", "scene_hash",
    "status", "wall_s", "build_s", "render_s", "integrator", "width", "height", "samples",
    "rays", "mrays_per_s", "peak_build_mib", "primitives", "diff_mean", "diff_rmse", "note",
]
ANSI = re.compile(r"\x1b\[[0-9;]*m")


def git(*args):
    return subprocess.run(["git", *args], cwd=ROOT, capture_output=True, text=True).stdout.strip()


def cpu_name():
    try:
        with open("/proc/cpuinfo") as f:
            for line in f:
                if line.startswith("model name"):
                    return line.split(":", 1)[1].strip()
    except OSError:
        pass
    if sys.platform == "darwin":
        name = subprocess.run(["sysctl", "-n", "machdep.cpu.brand_string"],
                              capture_output=True, text=True).stdout.strip()
        if name:
            return name
    return platform.processor() or platform.machine()


def duration(text):
    """Rust's Debug format for a Duration: 1.5s, 759.29ms, 12.3µs."""
    m = re.match(r"([0-9.]+)\s*(s|ms|µs|us|ns)$", text.strip())
    if not m:
        return None
    return float(m.group(1)) * {"s": 1, "ms": 1e-3, "µs": 1e-6, "us": 1e-6, "ns": 1e-9}[m.group(2)]


def parse_log(path):
    """What a render's log says: its size, samples, rays, times and integrator."""
    try:
        with open(path, encoding="utf-8", errors="replace") as f:
            text = f.read()
    except OSError:
        return {}
    # The terminal preview is coloured blocks; progress bars rewrite lines with \r.
    lines = [ANSI.sub("", l).strip() for l in re.split(r"[\r\n]", text)]
    lines = [l for l in lines if l and "█" not in l]
    row = {}
    gpu_refused = wavefront_refused = None
    for l in lines:
        if m := re.match(r"- Output: (\d+)x(\d+) @ (?:\d+ to )?(\d+) samples", l):
            row["width"], row["height"], row["samples"] = m.groups()
        elif m := re.match(r"- Scene Objects: \d+, Primitives: (\d+)", l):
            row["primitives"] = m.group(1)
        elif m := re.match(r"- Built in ([0-9.]+) s(?:, peak memory (\d+) MiB)?", l):
            row["build_s"] = m.group(1)
            row["peak_build_mib"] = m.group(2) or ""
        elif m := re.match(r"- Integrator: (.+)", l):
            row["integrator"] = ("gpu" if "GPU" in m.group(1) else "wavefront"
                                 if "wavefront" in m.group(1) else "path-tracer")
        elif m := re.match(r"- GPU: (.+)", l):
            row["gpu"] = m.group(1)
        elif m := re.match(r"- The GPU can't render this scene \((.*)\); rendering with", l):
            gpu_refused = m.group(1)
        elif m := re.match(r"- The wavefront integrator (?:can't render this scene \((.*)\)|doesn't sample adaptively); rendering with the path tracer", l):
            wavefront_refused = m.group(1) or "adaptive sampling"
        elif m := re.match(r"\| Rays Cast: (\d+)", l):
            row["rays"] = m.group(1)
        elif m := re.match(r"\| Elapsed Time: (.+)", l):
            seconds = duration(m.group(1))
            row["render_s"] = "" if seconds is None else f"{seconds:.3f}"
        elif m := re.match(r"\| Rays per sec: ([0-9.]+)", l):
            row["mrays_per_s"] = f"{float(m.group(1)) / 1e6:.3f}"
    if "integrator" in row:
        pass
    elif "gpu" in row:
        # Logs before rays named its integrator.
        row["integrator"] = "gpu"
    elif any(l.startswith("- Wavefront integrator: ") for l in lines) and wavefront_refused is None:
        row["integrator"] = "wavefront"
    elif "render_s" in row:
        row["integrator"] = "path-tracer"
    row["note"] = "; ".join(r for r in [gpu_refused, wavefront_refused] if r)
    row["last_line"] = lines[-1] if lines else ""
    return row


def png_pixels(data):
    from PIL import Image
    import numpy as np
    return np.asarray(Image.open(io.BytesIO(data)).convert("RGB"), dtype=np.float64)


def image_diff(image):
    """Mean and RMS difference, in 8-bit levels, from HEAD's copy of the image."""
    rel = os.path.relpath(os.path.abspath(image), ROOT)
    old = subprocess.run(["git", "show", f"HEAD:{rel}"], cwd=ROOT, capture_output=True).stdout
    if not old or not os.path.exists(image):
        return "", ""
    try:
        import numpy as np
        with open(image, "rb") as f:
            a, b = png_pixels(f.read()), png_pixels(old)
    except ImportError:
        return "", ""
    if a.shape != b.shape:
        return "size", "size"
    d = a - b
    return f"{np.abs(d).mean():.3f}", f"{np.sqrt((d * d).mean()):.3f}"


def scene_hash(scene):
    with open(scene, "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()[:12]


def read_rows(path):
    if not os.path.exists(path):
        return []
    with open(path, newline="") as f:
        return list(csv.DictReader(f))


def append_row(path, row):
    new = not os.path.exists(path)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "a", newline="") as f:
        w = csv.DictWriter(f, fieldnames=FIELDS, extrasaction="ignore")
        if new:
            w.writeheader()
        w.writerow(row)


def make_row(run, commit, dirty, scene, status, wall, preview, log, image):
    parsed = parse_log(log)
    row = {k: "" for k in FIELDS}
    row.update(parsed)
    row.update({
        "run": run, "commit": commit, "dirty": dirty, "cpu": cpu_name(),
        "threads": os.cpu_count(), "preview_samples": preview or "",
        "scene": os.path.splitext(os.path.basename(scene))[0], "scene_hash": scene_hash(scene),
        "status": status, "wall_s": wall,
    })
    if status != "ok":
        row["note"] = parsed.get("last_line", "")
    elif not preview:
        row["diff_mean"], row["diff_rmse"] = image_diff(image)
    return row


def record(args):
    row = make_row(args.run, args.commit or git("rev-parse", "HEAD"), args.dirty, args.scene,
                   args.status, args.wall, args.preview, args.log, args.image)
    append_row(PREVIEW_CSV if args.preview else FULL_CSV, row)


def find_scene(name):
    for f in [f"demo/scenes/{name}.json", f"demo/scenes/{name}.nff", f"demo/{name}.json"]:
        if os.path.exists(os.path.join(ROOT, f)):
            return os.path.join(ROOT, f)
    return None


def import_run(args):
    """The latest run in summary.txt, from its logs (which later runs overwrite)."""
    with open(os.path.join(OUT, "summary.txt")) as f:
        lines = f.read().splitlines()
    start = max(i for i, l in enumerate(lines) if l.startswith("# "))
    m = re.match(r"# (\d{4}-\d\d-\d\d \d\d:\d\d) (\S+)\s*(?:\((\d+) spp preview\))?", lines[start])
    when, short, preview = m.groups()
    run = datetime.datetime.strptime(when, "%Y-%m-%d %H:%M").astimezone(
        datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    commit = git("rev-parse", short) or short
    path = PREVIEW_CSV if preview else FULL_CSV
    have = {(r["run"], r["scene"]) for r in read_rows(path)}
    count = 0
    for l in lines[start + 1:]:
        name, status, wall = l.split()[:3]
        scene = find_scene(name)
        if scene is None or (run, name) in have:
            continue
        image = os.path.join(OUT, f"{name}.png") if preview else os.path.splitext(scene)[0] + ".png"
        row = make_row(run, commit, "", scene, "ok" if status == "ok" else "failed",
                       wall.rstrip("s"), preview, os.path.join(OUT, f"{name}.log"), image)
        append_row(path, row)
        count += 1
    print(f"Imported {count} scenes of the run of {when} into {os.path.relpath(path, ROOT)}")


def num(row, key):
    try:
        return float(row[key])
    except (KeyError, ValueError):
        return None


def hours(s):
    return f"{int(s // 3600)}h {int(s % 3600 // 60):02d}m" if s >= 3600 else f"{s:.0f}s" if s < 120 else f"{s / 60:.0f}m"


def report(args):
    path = PREVIEW_CSV if args.preview else FULL_CSV
    rows = read_rows(path)
    if not rows:
        print(f"No runs recorded in {os.path.relpath(path, ROOT)}")
        return
    run = args.run or rows[-1]["run"]
    this = [r for r in rows if r["run"] == run]
    if not this:
        print(f"No run {run} in {os.path.relpath(path, ROOT)}")
        return
    cpu = this[0]["cpu"]
    # The previous run on this machine at the same settings.
    earlier = sorted({r["run"] for r in rows if r["run"] < run and r["cpu"] == cpu
                      and r["preview_samples"] == this[0]["preview_samples"]})
    prev_run = args.against or (earlier[-1] if earlier else None)
    prev = {r["scene"]: r for r in rows if r["run"] == prev_run}

    ok = [r for r in this if r["status"] == "ok"]
    print(f"Run {run} at {this[0]['commit'][:9]}{' (dirty)' if this[0]['dirty'] == '1' else ''} on {cpu}: "
          f"{len(ok)} of {len(this)} scenes rendered, {hours(sum(num(r, 'wall_s') or 0 for r in this))} in all")
    print(f"\n  {'integrator':<12} {'scenes':>6} {'wall':>9} {'build':>9} {'render':>9} {'Mrays/s':>8}")
    for kind in ["gpu", "wavefront", "path-tracer"]:
        rs = [r for r in ok if r["integrator"] == kind]
        if not rs:
            continue
        render = sum(num(r, "render_s") or 0 for r in rs)
        rays = sum(num(r, "rays") or 0 for r in rs)
        print(f"  {kind:<12} {len(rs):>6} {hours(sum(num(r, 'wall_s') or 0 for r in rs)):>9} "
              f"{hours(sum(num(r, 'build_s') or 0 for r in rs)):>9} {hours(render):>9} "
              f"{rays / render / 1e6 if render else 0:>8.2f}")
    print("\n  Slowest:")
    for r in sorted(ok, key=lambda r: -(num(r, "wall_s") or 0))[:args.top]:
        print(f"    {r['scene']:<28} {hours(num(r, 'wall_s') or 0):>8}  {r['integrator']}")
    for r in this:
        if r["status"] != "ok":
            print(f"  Failed: {r['scene']}: {r['note']}")

    if not prev:
        print("\nNo earlier run on this machine at these settings to compare with.")
        return
    print(f"\nAgainst the run of {prev_run} at {next(iter(prev.values()))['commit'][:9]}:")
    same, changed = [], []
    for r in ok:
        p = prev.get(r["scene"])
        if p is None or p["status"] != "ok":
            continue
        (same if p["scene_hash"] == r["scene_hash"] else changed).append((r, p))
    for key, label in [("render_s", "render"), ("build_s", "build"), ("wall_s", "wall")]:
        a = sum(num(p, key) or 0 for _, p in same)
        b = sum(num(r, key) or 0 for r, _ in same)
        if a:
            print(f"  {label:<7} {hours(a):>8} -> {hours(b):>8} ({(b / a - 1) * 100:+.0f}%) over {len(same)} unchanged scenes")
    moves = []
    for r, p in same:
        a, b = num(p, "render_s"), num(r, "render_s")
        if a and b and max(a, b) >= args.min_seconds and abs(b / a - 1) >= args.threshold:
            moves.append((b / a, r, a, b))
    if moves:
        print(f"\n  Render time changed by {args.threshold:.0%} or more:")
        for ratio, r, a, b in sorted(moves, key=lambda m: m[0]):
            print(f"    {r['scene']:<28} {hours(a):>8} -> {hours(b):>8}  x{ratio:.2f}")
    switched = [(r, p) for r, p in same + changed if r["integrator"] != p["integrator"]]
    if switched:
        print("\n  Rendered differently:")
        for r, p in switched:
            print(f"    {r['scene']:<28} {p['integrator']} -> {r['integrator']}")
    if changed:
        print(f"\n  Scene file changed, not compared: {', '.join(r['scene'] for r, _ in changed)}")
    newly_failed = [r["scene"] for r in this if r["status"] != "ok" and prev.get(r["scene"], {}).get("status") == "ok"]
    if newly_failed:
        print(f"\n  Failed this time only: {', '.join(newly_failed)}")
    differ = [r for r in ok if (num(r, "diff_rmse") or 0) >= args.diff]
    if differ:
        print(f"\n  Pictures differing from HEAD's by {args.diff} levels RMS or more:")
        for r in differ:
            print(f"    {r['scene']:<28} {r['diff_rmse']}")


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    sub = ap.add_subparsers(dest="command", required=True)
    r = sub.add_parser("record", help="append one scene's row (called by render-all.sh)")
    r.add_argument("--run", required=True)
    r.add_argument("--commit", default="")
    r.add_argument("--dirty", default="")
    r.add_argument("--scene", required=True)
    r.add_argument("--status", required=True)
    r.add_argument("--wall", required=True)
    r.add_argument("--preview", default="", help="the run's --samples, for a preview")
    r.add_argument("--log", required=True)
    r.add_argument("--image", required=True)
    i = sub.add_parser("import", help="import the latest run in summary.txt from its logs")
    p = sub.add_parser("report", help="summarise a run and compare it with the previous one")
    p.add_argument("--run", help="the run to report (default: the latest)")
    p.add_argument("--against", help="the run to compare with (default: the previous one on this machine)")
    p.add_argument("--preview", action="store_true", help="report preview runs")
    p.add_argument("--top", type=int, default=10, help="how many of the slowest scenes to list")
    p.add_argument("--threshold", type=float, default=0.1, help="the change in render time worth listing")
    p.add_argument("--min-seconds", type=float, default=30, help="ignore scenes quicker than this")
    p.add_argument("--diff", type=float, default=8, help="RMS difference in levels worth listing")
    args = ap.parse_args()
    {"record": record, "import": import_run, "report": report}[args.command](args)


if __name__ == "__main__":
    main()
