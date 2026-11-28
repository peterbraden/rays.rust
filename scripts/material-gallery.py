#!/usr/bin/env python3
"""Render the material library (src/material/library.json) as a labelled contact sheet.

Each preset is rendered on its own: a rounded block and a sphere of it on a grey floor in
sunlight, about half a metre across, so its patterns are seen at the size they were made for.
Overlays (dust, grime, moss...) are shown over a plain grey stone, worn_edges over red paint.

A preset may say how it is best seen, by "show" in the library: "landscape" (ground and rock:
two views side by side, a patch of rough ground with a half-buried boulder from eye height, and
hills a few hundred metres across from above, so it is seen from a metre to the horizon) or
"bark" (a trunk). Overlays that are natural are shown over "over_for_landscape" in the library
(default granite_outcrop; "plain" for a plain grey rock) in those views.

    scripts/material-gallery.py [--size 320] [--samples 64] [--only name,...] [--library FILE] [--objects] [-o docs/material-library.jpg]

--objects shows every preset on the block and sphere, ground and bark included.

--library adds a file of presets (library.json's form) over the built-in ones, as a scene's
"library" does: presets are read at run time, so one can be tuned there and re-rendered
without rebuilding the renderer.
"""
import argparse, json, os, subprocess, sys, tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

BASES = {
    "worn_edges": {"type": "plastic", "albedo": [0.5, 0.06, 0.04], "roughness": 0.25},
}
PALE = {"type": "plastic", "albedo": [0.28, 0.28, 0.3], "roughness": 0.5}
# Plain grey rock with some relief, for natural overlays in the landscape views.
PLAIN_ROCK = {"type": "lambertian", "albedo": [0.3, 0.29, 0.27],
              "normal": {"graph": {"node": "fractal", "style": "billow", "scale": 3, "smallest": 0.004, "amplitude": 0.05}, "amount": 1}}

def common(size, samples, width=None):
    return {"width": width or size, "height": size, "supersamples": samples, "chunk_size": 32, "samples_per_chunk": 4,
            "max_depth": 6, "path_termination": "russian_roulette", "filter": "mitchell",
            "output": {"exposure": 0.55, "tonemap": "filmic"}, "sun": {"intensity": 5}}

def near_ground(m, size, samples):
    """A patch of rough ground with a boulder half sunk in it, from eye height."""
    s = common(size, samples)
    s.update({
        "camera": {"location": [0, 1.6, -3], "lookat": [0, 0.5, 3], "up": [0, 1, 0], "angle": 0.6, "aperture": 0.0},
        "materials": {"M": m},
        "objects": [
            {"type": "skysphere", "sun_direction": [-0.75, 0.4, -0.25]},
            {"type": "terrain", "material": "M", "heightmap": {"type": "noise", "min": [-20, -4], "max": [20, 40],
                "cell_size": 0.06, "amplitude": 0.5, "scale": 4, "octaves": 5, "seed": 3}},
            {"type": "plane", "y": -0.6, "material": "M"},
            {"type": "sphere", "location": [0.9, -0.15, 1.8], "radius": 0.6, "material": "M",
             "transform": [{"scale": [1, 0.7, 1.2]}, {"rotate": [10, 30, 5]}, {"translate": [0.9, 0.05, 1.8]}]},
        ],
    })
    s["objects"][3]["location"] = [0, 0, 0]
    return s

def far_ground(m, size, samples):
    """Hills a few hundred metres across, from above."""
    s = common(size, samples)
    s.update({
        "camera": {"location": [0, 75, -60], "lookat": [0, 0, 140], "up": [0, 1, 0], "angle": 0.55, "aperture": 0.0},
        "materials": {"M": m},
        "objects": [
            {"type": "skysphere", "sun_direction": [-0.75, 0.4, -0.25]},
            {"type": "terrain", "material": "M", "heightmap": {"type": "noise", "min": [-300, -80], "max": [300, 500],
                "cell_size": 1.0, "amplitude": 55, "scale": 160, "octaves": 6, "ridged": True, "seed": 5}},
        ],
    })
    return s

def trunk(m, size, samples):
    s = common(size, samples)
    s.update({
        "camera": {"location": [0.2, 1.4, -1.6], "lookat": [0, 1.1, 0], "up": [0, 1, 0], "angle": 0.45, "aperture": 0.0},
        "materials": {"M": m, "FLOOR": {"type": "soil", "preset": "forest_floor"}},
        "objects": [
            {"type": "skysphere", "sun_direction": [-0.6, 0.5, -0.4]},
            {"type": "plane", "y": 0, "material": "FLOOR"},
            {"type": "cylinder", "base": [0, -0.1, 0], "cap": [0, 4, 0], "radius": 0.3, "material": "M"},
        ],
    })
    return s

def scene(name, size, samples, overlay):
    m = {"type": "library", "name": name}
    if overlay:
        m["over"] = BASES.get(name, PALE)
    return {
        "width": size, "height": size, "supersamples": samples, "chunk_size": 32, "samples_per_chunk": 4,
        "max_depth": 8, "path_termination": "russian_roulette", "filter": "mitchell",
        "output": {"exposure": 0.55, "tonemap": "filmic"},
        "sun": {"intensity": 5},
        "camera": {"location": [0.3, 0.8, -1.4], "lookat": [0.03, 0.15, 0.06], "up": [0, 1, 0], "angle": 0.36, "aperture": 0.002},
        "materials": {"FLOOR": {"type": "lambertian", "albedo": [0.3, 0.3, 0.3]},
                      "M": m},
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
    a.add_argument("--library", default="", help="a file of presets over the built-in ones")
    a.add_argument("--objects", action="store_true", help="every preset on the block and sphere, whatever its \"show\"")
    a.add_argument("--rays", default=os.path.join(ROOT, "target/release/rays"))
    a.add_argument("-o", "--output", default=os.path.join(ROOT, "docs/material-library.jpg"))
    args = a.parse_args()
    from PIL import Image, ImageDraw
    library = json.load(open(os.path.join(ROOT, "src/material/library.json")))
    extra = os.path.abspath(args.library) if args.library else None
    if extra:
        library.update(json.load(open(extra)))
    names = list(library.keys())
    if args.only:
        names = [n for n in names if n in args.only.split(",")]
    tiles = []
    with tempfile.TemporaryDirectory() as tmp:
        for n in names:
            path = os.path.join(tmp, n + ".json")
            p = library[n]
            show = "object" if args.objects else p.get("show", "object")
            m = {"type": "library", "name": n}
            if p.get("overlay"):
                m["over"] = BASES.get(n, PALE) if show == "object" else (PLAIN_ROCK if p.get("over_for_landscape") == "plain" else {"type": "library", "name": p.get("over_for_landscape", "granite_outcrop")})
            views = {"object": lambda: [scene(n, args.size, args.samples, p.get("overlay", False))],
                     "landscape": lambda: [near_ground(m, args.size, args.samples), far_ground(m, args.size, args.samples)],
                     "bark": lambda: [trunk(m, args.size, args.samples)]}[show]()
            parts = []
            for k, v in enumerate(views):
                if extra:
                    v["library"] = extra
                json.dump(v, open(path, "w"))
                out = os.path.join(tmp, "%s-%d.png" % (n, k))
                subprocess.run([args.rays, path, "--fresh", "-o", out], check=True, cwd=ROOT, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
                parts.append(Image.open(out).convert("RGB"))
            # Views side by side, a cell each.
            tile = Image.new("RGB", (args.size * len(parts), args.size))
            for k, part in enumerate(parts):
                tile.paste(part, (k * args.size, 0))
            d = ImageDraw.Draw(tile)
            d.rectangle([0, args.size - 18, args.size * len(parts), args.size], fill=(0, 0, 0))
            d.text((6, args.size - 15), n, fill=(255, 255, 255))
            tiles.append(tile)
            print(n, file=sys.stderr)
    # Tiles laid out in rows of `columns` cells, a wide tile taking as many as it needs.
    cols = max(args.columns, max(t.size[0] // args.size for t in tiles))
    places, row, col = [], 0, 0
    for t in tiles:
        w = t.size[0] // args.size
        if col + w > cols:
            row, col = row + 1, 0
        places.append((col, row))
        col += w
    used = max(c + t.size[0] // args.size for (c, _), t in zip(places, tiles))
    sheet = Image.new("RGB", (min(cols, used) * args.size, (row + 1) * args.size), (0, 0, 0))
    for (c, r), t in zip(places, tiles):
        sheet.paste(t, (c * args.size, r * args.size))
    sheet.save(args.output, **({"quality": 88, "optimize": True} if args.output.lower().endswith((".jpg", ".jpeg")) else {}))

if __name__ == "__main__":
    main()
