#!/usr/bin/env python3
"""Forests' levels of detail against full detail, under many conditions (docs/research/learned-lod/suite.md).

    scripts/lod-suite.py [--cases aerial,low,...] [--pixels 2] [--all] [--out target/lod-suite]
                         [--width 480 --height 280 --samples 32] [--binary target/release/rays]
                         [--refresh-references] [--list]

Each case is a demo scene changed one way (the camera, the sun, the species, their spacing or
season). It is rendered twice or more: at full detail, without the forests' `"lod"`, kept as the
reference and reused until `--refresh-references` (its file is named by a hash of its scene, so a
changed case makes a new one); and with `"lod": {"pixels": p}` for each `--pixels`, its trees kept in `<out>/tree-cache` (which changes
no render, only how long it takes to build). For each
LOD render it records:

- mean FLIP against the reference (the renderer's `--compare`);
- mean linear luminance against the reference, over the image and over its near and far halves
  by the reference's depth (`scripts/compare-linear.py`'s clamping, against fireflies);
- the forest's built and placed primitives, build seconds, peak memory and render seconds;

into `<out>/results.csv` (a row a case and setting, replacing any earlier one) and a table in
`<out>/summary.md`, with a picture of each (`<out>/<case>-p<p>.png`: the reference, the LOD
render, and where it is brighter or darker). Cases marked heavy (large full-detail references)
run only with `--all` or by name. Needs numpy, OpenEXR and Pillow.
"""
import argparse
import copy
import csv
import hashlib
import importlib.util
import json
import os
import re
import subprocess
import sys
import time

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
spec = importlib.util.spec_from_file_location("compare_linear", os.path.join(HERE, "compare-linear.py"))
compare_linear = importlib.util.module_from_spec(spec)
spec.loader.exec_module(compare_linear)

BROADLEAVES = {"quaking-aspen", "black-tupelo", "california-black-oak", "weeping-willow", "lombardy-poplar",
               "hazel", "hawthorn", "shrub", "juniper"}
CONIFERS = {"scots-pine", "stone-pine", "norway-spruce", "balsam-fir", "cedar-of-lebanon", "italian-cypress"}


def forests(doc):
    return [o for o in doc.get("objects", []) if o.get("type") == "forest"]


def camera(location, lookat, up=(0, 1, 0)):
    def change(doc):
        doc["camera"].update({"location": list(location), "lookat": list(lookat), "up": list(up)})
    return change


def sun(direction):
    def change(doc):
        for o in doc["objects"]:
            if o.get("type") == "skysphere":
                o["sun_direction"] = list(direction)
    return change


def species(keep, autumn=None):
    def change(doc):
        for f in forests(doc):
            f["species"] = [s for s in f["species"] if s.get("species") in keep]
            if autumn is not None:
                f["autumn"] = autumn
                for s in f["species"]:
                    s.get("params", {}).pop("autumn", None)
        doc["objects"] = [o for o in doc["objects"] if o.get("type") != "forest" or o["species"]]
    return change


def spacing(factor, clearings=None):
    def change(doc):
        for f in forests(doc):
            f["spacing"] = f.get("spacing", 8) * factor
            if clearings is not None:
                f["clearings"] = clearings
    return change


def both(*changes):
    def change(doc):
        for c in changes:
            c(doc)
    return change


# name: (base scene, what it tests, change, heavy)
CASES = {
    "aerial": ("forest-aerial", "the forest from the air, 15-30 degrees down (the research's own view)", None, False),
    "low": ("forest-aerial", "from 90 m up, looking along the forest", camera((0, 90, -900), (0, 40, 600)), False),
    "nadir": ("forest-aerial", "straight down from 1200 m", camera((0, 1200, 1000), (0, 0, 1000.5), (0, 0, 1)), False),
    "low-sun": ("forest-aerial", "a sun 9 degrees up: long shadows, light through the crowns' sides",
                sun((-0.9, 0.16, 0.4)), False),
    "backlit": ("forest-aerial", "the sun ahead of the camera: light through the leaves", sun((0.15, 0.42, 0.9)), False),
    "broadleaf-autumn": ("forest-aerial", "broadleaves only, in autumn colour", species(BROADLEAVES, autumn=0.7), False),
    "conifers": ("forest-aerial", "conifers only", species(CONIFERS), False),
    "sparse": ("forest-aerial", "trees twice as far apart, with more clearings: more ground and shadow",
               spacing(2, clearings=0.4), False),
    "ground": ("forest", "at eye height in a wood: trees from near to far", None, False),
    "dense": ("forest-dense", "a closed canopy with an understory, from the air", None, True),
    "sunlit": ("forest-sunlit", "forests out to 14 km, under a low side light", None, True),
}


def scene_for(case, lod, cache=None):
    base, _, change, _ = CASES[case]
    doc = json.load(open(os.path.join(ROOT, "demo", "scenes", base + ".json")))
    if change:
        change(doc)
    doc = copy.deepcopy(doc)
    for f in forests(doc):
        f.pop("cache", None)
        if lod is None:
            f.pop("lod", None)
        else:
            f["lod"] = {**(f.get("lod") if isinstance(f.get("lod"), dict) else {}), "pixels": lod}
            # Trees' levels and placements kept between cases and settings: renders are the same
            # with or without (`procedural::tree_cache`).
            if cache:
                f["cache"] = cache
    return doc


def digest(doc, a):
    text = json.dumps(doc, sort_keys=True) + f"{a.width}x{a.height}x{a.samples}"
    return hashlib.sha1(text.encode()).hexdigest()[:12]


def seconds(m):
    """A Rust `Duration` as printed (`1.5s`, `71.6ms`), in seconds, from a match of its number and unit."""
    if not m:
        return ""
    return f"{float(m.group(1)) * {'ns': 1e-9, 'µs': 1e-6, 'us': 1e-6, 'ms': 1e-3, 's': 1.}[m.group(2)]:.3f}"


def run(a, scene_path, out, extra):
    cmd = [a.binary, scene_path, "--width", str(a.width), "--height", str(a.height), "--samples", str(a.samples),
           "--fresh", "-o", out] + extra
    started = time.time()
    log = subprocess.run(cmd, capture_output=True, text=True)
    text = log.stdout + log.stderr
    open(out + ".log", "w").write(text)
    if log.returncode not in (0, 2):
        sys.exit(f"{' '.join(cmd)} failed:\n{text[-2000:]}")
    found = lambda pattern: (m.group(1) if (m := re.search(pattern, text)) else "")
    return {
        "built_primitives": sum(int(x) for x in re.findall(r"(\d+) primitives built", text)),
        "placed": sum(int(x) for x in re.findall(r"(\d+) placed", text)),
        "build_s": found(r"Built in ([0-9.]+) s"),
        "peak_mib": found(r"peak memory (\d+) MiB"),
        "render_s": seconds(re.search(r"Elapsed Time: ([0-9.]+)(ns|µs|us|ms|s)", text)),
        "flip": found(r"Mean FLIP error: ([0-9.]+)"),
        "wall_s": f"{time.time() - started:.1f}",
    }


def brightness(test, reference, depth, clamp=0.999):
    t, r = compare_linear.luminance(compare_linear.read(test)), compare_linear.luminance(compare_linear.read(reference))
    top = np.quantile(r, clamp)
    t, r = np.minimum(t, top), np.minimum(r, top)
    d = compare_linear.read(depth)
    d = d[..., 0] if d.ndim == 3 else d
    surface = np.isfinite(d) & (d < 1e6)
    split = np.median(d[surface]) if surface.any() else np.inf
    near, far = surface & (d <= split), surface & (d > split)
    ratio = lambda m: t[m].mean() / r[m].mean() if m.any() and r[m].mean() > 0 else float("nan")
    return {"bright_all": f"{t.mean() / r.mean():.4f}", "bright_near": f"{ratio(near):.4f}", "bright_far": f"{ratio(far):.4f}"}


def picture(test, reference, path, label):
    from PIL import Image, ImageDraw
    t, r = compare_linear.read(test)[..., :3], compare_linear.read(reference)[..., :3]
    scale = 0.18 / max(np.median(compare_linear.luminance(r)), 1e-6)
    show = lambda x: Image.fromarray((np.clip(np.where(x * scale <= 0.0031308, 12.92 * x * scale,
                                                      1.055 * np.clip(x * scale, 0, None) ** (1 / 2.4) - 0.055), 0, 1) * 255).astype(np.uint8))
    lt, lr = compare_linear.luminance(t), compare_linear.luminance(r)
    top = np.quantile(lr, 0.999)
    k = 4
    blur = lambda a: np.lib.stride_tricks.sliding_window_view(np.pad(np.minimum(a, top), k, mode="edge"), (2 * k + 1, 2 * k + 1)).mean(axis=(-1, -2))
    q = np.clip((blur(lt) / np.maximum(blur(lr), 1e-9) - 1) / 0.15, -1, 1)
    m = np.stack([np.where(q > 0, 1, 1 + q), 1 - np.abs(q), np.where(q < 0, 1, 1 - q)], axis=-1)
    tiles = [show(r), show(t), Image.fromarray((m * 255).astype(np.uint8))]
    w, h = tiles[0].size
    sheet = Image.new("RGB", (3 * w + 8, h + 20), (255, 255, 255))
    for i, tile in enumerate(tiles):
        sheet.paste(tile, (i * (w + 4), 20))
    ImageDraw.Draw(sheet).text((4, 4), f"{label}   full detail | LOD | brighter red, darker blue (+-15%)", fill=(0, 0, 0))
    sheet.save(path)


FIELDS = ["case", "pixels", "description", "flip", "bright_all", "bright_near", "bright_far", "built_primitives",
          "placed", "build_s", "peak_mib", "render_s", "full_build_s", "full_peak_mib", "full_render_s", "reference"]


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--cases", help="comma-separated names (default: all but the heavy ones)")
    ap.add_argument("--all", action="store_true", help="the heavy cases too")
    ap.add_argument("--pixels", default="2", help="comma-separated LOD settings (default 2)")
    ap.add_argument("--out", default=os.path.join(ROOT, "target", "lod-suite"))
    ap.add_argument("--binary", default=os.path.join(ROOT, "target", "release", "rays"))
    ap.add_argument("--width", type=int, default=480)
    ap.add_argument("--height", type=int, default=280)
    ap.add_argument("--samples", type=int, default=32)
    ap.add_argument("--refresh-references", action="store_true")
    ap.add_argument("--list", action="store_true", help="list the cases and stop")
    a = ap.parse_args()
    if a.list:
        for name, (base, what, _, heavy) in CASES.items():
            print(f"{name:18} {base:15} {'(heavy) ' if heavy else ''}{what}")
        return
    names = a.cases.split(",") if a.cases else [n for n, c in CASES.items() if a.all or not c[3]]
    unknown = [n for n in names if n not in CASES]
    if unknown:
        sys.exit(f"unknown cases {unknown}; --list lists them")
    if not os.path.exists(a.binary):
        sys.exit(f"no renderer at {a.binary}: cargo build --release")
    os.makedirs(a.out, exist_ok=True)
    results = os.path.join(a.out, "results.csv")
    rows = {}
    if os.path.exists(results):
        for row in csv.DictReader(open(results)):
            rows[(row["case"], row["pixels"])] = row

    for case in names:
        full = scene_for(case, None)
        key = digest(full, a)
        full_scene = os.path.join(a.out, f"{case}-full-{key}.json")
        reference, depth = full_scene[:-5] + ".exr", full_scene[:-5] + "-depth.exr"
        json.dump(full, open(full_scene, "w"), indent=1)
        if a.refresh_references or not os.path.exists(reference):
            print(f"{case}: full detail...", flush=True)
            full_stats = run(a, full_scene, reference, ["--depth", depth])
            json.dump(full_stats, open(reference + ".json", "w"))
        full_stats = json.load(open(reference + ".json"))
        for p in a.pixels.split(","):
            scene = scene_for(case, float(p), os.path.join(a.out, "tree-cache"))
            path = os.path.join(a.out, f"{case}-p{p}.json")
            json.dump(scene, open(path, "w"), indent=1)
            print(f"{case}: pixels {p}...", flush=True)
            out = path[:-5] + ".exr"
            stats = run(a, path, out, ["--compare", reference])
            row = {"case": case, "pixels": p, "description": CASES[case][1], **stats,
                   **brightness(out, reference, depth), "full_build_s": full_stats["build_s"],
                   "full_peak_mib": full_stats["peak_mib"], "full_render_s": full_stats["render_s"],
                   "reference": os.path.basename(reference)}
            rows[(case, p)] = row
            picture(out, reference, path[:-5] + ".png", f"{case}, pixels {p}: FLIP {row['flip']}, brightness {row['bright_all']}")
            print(f"  FLIP {row['flip']}, brightness all/near/far {row['bright_all']}/{row['bright_near']}/{row['bright_far']}, "
                  f"peak {row['peak_mib']} MiB (full {row['full_peak_mib']}), render {row['render_s']} s (full {row['full_render_s']})", flush=True)
            with open(results, "w", newline="") as f:
                w = csv.DictWriter(f, fieldnames=FIELDS, extrasaction="ignore")
                w.writeheader()
                for r in rows.values():
                    w.writerow(r)

    num = lambda v: f"{float(v):.1f}" if v else "—"
    pct = lambda v: f"{(float(v) - 1) * 100:+.1f}%" if v not in ("", "nan") else "—"
    with open(os.path.join(a.out, "summary.md"), "w") as f:
        f.write(f"| Case | `pixels` | FLIP | Brightness | Near | Far | Peak MiB (full) | Render s (full) |\n|---|---|---|---|---|---|---|---|\n")
        for r in rows.values():
            f.write(f"| {r['case']} | {r['pixels']} | {r['flip']} | {pct(r['bright_all'])} | {pct(r['bright_near'])} | {pct(r['bright_far'])} "
                    f"| {r['peak_mib']} ({r['full_peak_mib']}) | {num(r['render_s'])} ({num(r['full_render_s'])}) |\n")
    print(open(os.path.join(a.out, "summary.md")).read())


if __name__ == "__main__":
    main()
