#!/usr/bin/env python3
"""Render the material library (src/material/library.json) as a labelled contact sheet.

Each preset is rendered on its own: a rounded block and a sphere of it on a grey floor in
sunlight, about half a metre across, so its patterns are seen at the size they were made for.

    scripts/material-gallery.py [--size 320] [--samples 64] [--only name,...] [-o docs/material-library.png]
"""
import argparse, json, os, subprocess, sys, tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

def scene(name, size, samples):
    return {
        "width": size, "height": size, "supersamples": samples, "chunk_size": 32, "samples_per_chunk": 4,
        "max_depth": 8, "path_termination": "russian_roulette", "filter": "mitchell",
        "output": {"exposure": 0.55, "tonemap": "filmic"},
        "sun": {"intensity": 5},
        "camera": {"location": [0.3, 0.8, -1.4], "lookat": [0.03, 0.15, 0.06], "up": [0, 1, 0], "angle": 0.36, "aperture": 0.002},
        "materials": {"FLOOR": {"type": "lambertian", "albedo": [0.3, 0.3, 0.3]},
                      "M": {"type": "library", "name": name}},
        "objects": [
            {"type": "skysphere", "sun_direction": [-0.55, 0.6, -0.4]},
            {"type": "plane", "y": 0, "material": "FLOOR"},
            {"type": "superellipsoid", "e": 0.2, "n": 0.2, "material": "M",
             "transform": [{"scale": 0.2}, {"rotate": [0, 32, 0]}, {"translate": [-0.12, 0.2, 0.15]}]},
            {"type": "sphere", "location": [0.28, 0.13, -0.08], "radius": 0.13, "material": "M"},
        ],
    }

def main():
    a = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    a.add_argument("--size", type=int, default=320)
    a.add_argument("--samples", type=int, default=64)
    a.add_argument("--columns", type=int, default=7)
    a.add_argument("--only", default="")
    a.add_argument("--rays", default=os.path.join(ROOT, "target/release/rays"))
    a.add_argument("-o", "--output", default=os.path.join(ROOT, "docs/material-library.png"))
    args = a.parse_args()
    from PIL import Image, ImageDraw
    names = list(json.load(open(os.path.join(ROOT, "src/material/library.json"))).keys())
    if args.only:
        names = [n for n in names if n in args.only.split(",")]
    tiles = []
    with tempfile.TemporaryDirectory() as tmp:
        for n in names:
            path = os.path.join(tmp, n + ".json")
            json.dump(scene(n, args.size, args.samples), open(path, "w"))
            out = os.path.join(tmp, n + ".png")
            subprocess.run([args.rays, path, "--fresh", "-o", out], check=True, cwd=ROOT, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            tile = Image.open(out).convert("RGB")
            d = ImageDraw.Draw(tile)
            d.rectangle([0, args.size - 18, args.size, args.size], fill=(0, 0, 0))
            d.text((6, args.size - 15), n, fill=(255, 255, 255))
            tiles.append(tile)
            print(n, file=sys.stderr)
    cols = min(args.columns, len(tiles))
    rows = (len(tiles) + cols - 1) // cols
    sheet = Image.new("RGB", (cols * args.size, rows * args.size), (0, 0, 0))
    for k, t in enumerate(tiles):
        sheet.paste(t, ((k % cols) * args.size, (k // cols) * args.size))
    sheet.save(args.output)

if __name__ == "__main__":
    main()
