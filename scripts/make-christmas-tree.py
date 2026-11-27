#!/usr/bin/env python3
"""Writes demo/scenes/christmas-tree.json: a decorated Christmas tree in a dark, snowy forest.

A stress test of many things at once: a spruce dressed with a few hundred fairy lights (small
emitters chosen by the light BVH) on a wire of curves, tinsel garlands of fur, baubles of
glass, metal and lacquer, glass icicles and a glowing star; presents in checked paper under it;
a forest of snow-laden conifers, thinning in round a wide clearing; and fog thickening towards
the ground, seen through a thin lens.

The placements are random but seeded, so rerunning writes the same file:

    python3 scripts/make-christmas-tree.py
    python3 scripts/make-christmas-tree.py --preview /tmp/preview.json  # cheap, for framing
    cargo run --release -- demo/scenes/christmas-tree.json
"""

import json
import math
import os
import random
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from christmas_tree import MATERIALS as materials, decorate, r3, write_scene  # noqa: E402

rng = random.Random(25)

objects = []


def add(o):
    objects.append(o)


# The ground: snow lying in drifts round the clearing (behind the tree and to the sides, out
# of the camera's way), and a few snow-covered boulders.
add({"type": "plane", "y": 0, "material": "SNOW", "normal": {"pattern": "bumps", "amount": 0.25, "scale": 1.5}})
for _ in range(16):
    a = rng.uniform(-2.0, 2.0)
    d = rng.uniform(5, 15)
    w = rng.uniform(2, 5)
    add({"type": "transform", "item": {"type": "sphere", "location": [0, 0, 0], "radius": 1},
         "scale": r3([w, rng.uniform(0.06, 0.16), w * rng.uniform(0.5, 1.2)]),
         "rotate": {"pitch": round(rng.uniform(0, 180), 1)},
         "translate": r3([d * math.sin(a), -0.02, d * math.cos(a) + 2]), "material": "SNOW"})
for i, (x, z, r) in enumerate([(-4.2, 3.5, 0.55), (-3.6, 4.4, 0.3), (5.5, 6.5, 0.45), (2.9, -1.6, 0.22)]):
    add({"type": "boulder", "location": [x, -0.15 * r, z], "radius": r, "angularity": 0.6, "seed": 3 + i, "material": "SNOWY_GRANITE"})

# The Christmas tree, dressed, and its presents.
star_y = decorate(rng, add)

# Young spruces at the clearing's edge, framing the tree.
for i, (x, z, h) in enumerate([(-3.4, -2.6, 1.6), (-6.5, 4.5, 2.4), (5.2, 1.5, 1.9), (7.5, 7.5, 3.0)]):
    add({"type": "tree", "species": "norway-spruce", "seed": 20 + i, "location": [x, 0, z],
         "bark": "SNOWY_BARK", "leaves": ["LADEN_NEEDLE_1", "LADEN_NEEDLE_2"],
         "params": {"scale": h, "scale_v": 0, "base_size": 0.08, "leaf_scale": 0.12}})

# The dark forest round the clearing, its trees laden with snow. Its trees and their levels of
# detail are kept in a cache (not in git), so later renders skip growing them.
add({"type": "forest", "seed": 12, "lod": {"pixels": 1}, "cache": "demo/scenes/.forest-cache",
     "area": {"min": [-80, -30], "max": [80, 110]}, "spacing": 4.5, "clearings": 0.05,
     "patch_size": 30, "patchiness": 0.3, "variants": 8,
     "where": "min(1, max(0, (sqrt(x * x + 0.6 * (z - 10) * (z - 10)) - 24) / 20)) * min(1, max(0, (sqrt(x * x + (z + 10) * (z + 10)) - 7) / 2))",
     "species": [
         {"species": "norway-spruce", "weight": 4, "spacing": 0.8, "bark": "SNOWY_BARK", "leaves": ["LADEN_NEEDLE_1", "LADEN_NEEDLE_2"]},
         {"species": "balsam-fir", "weight": 3, "spacing": 0.7, "bark": "SNOWY_BARK", "leaves": ["LADEN_NEEDLE_1", "LADEN_NEEDLE_2"]},
         {"species": "scots-pine", "weight": 1, "bark": "SNOWY_BARK", "leaves": "LADEN_PINE_NEEDLE"},
     ]})

# Fog, thicker near the ground and in drifting banks.
add({"type": "volume", "min": [-80, -0.5, -80], "max": [80, 25, 100],
     "density": {"node": "expr", "expr": "0.025 * exp(-max(y, 0) / 5) * (0.7 + 0.3 * noise3d(x / 9, y / 4, z / 9))", "range": [0, 0.025]},
     "albedo": [0.98, 0.98, 0.98], "g": 0.75,
     # The bulbs' glow sampled along each camera ray too: far less speckle about them.
     "equiangular": True})

# A portrait of the tree, close, the forest behind it softened by the lens.
camera = {"location": [0.15, 1.15, -6.2], "lookat": [0, 1.85, 0], "up": [0, 1, 0], "angle": 0.7,
          "aperture": 0.06}
WIDTH, HEIGHT = 640, 960

scene = {
    "width": WIDTH,
    "height": HEIGHT,
    "supersamples": 512,
    "chunk_size": 32,
    "samples_per_chunk": 4,
    "max_depth": 6,
    "path_termination": "russian_roulette",
    "spectral": True,
    "light_sampler": "bvh",
    "filter": "mitchell",
    "adaptive": {"threshold": 0.012, "min_samples": 32},
    # Bulbs are white-hot at any exposure that shows the tree; the camera's glare gives each a
    # halo of its own colour, as in a photograph.
    "output": {"exposure": 3.5, "tonemap": "filmic", "glare": True},
    "background": [0.006, 0.007, 0.012],
    "camera": camera,
    "materials": materials,
    "objects": objects,
    "lights": [{"location": [0, star_y - 0.35, -0.3], "intensity": 1.2, "temperature": 2600}],
}

# A preview for working on the composition and the lights: the same objects, without the fog,
# in RGB, with shallow paths and a few samples; the bulbs still glow, and a faint key light
# high over the clearing shows the forest's silhouettes.
args = [a for a in sys.argv[1:] if a != "--preview"]
if "--preview" in sys.argv[1:]:
    objects = [o for o in objects if o["type"] != "volume"]
    scene.update({"objects": objects, "spectral": False, "max_depth": 2,
                  "supersamples": 8, "adaptive": {"threshold": 0},
                  "lights": scene["lights"] + [{"location": [2, 30, -4], "intensity": 120}]})
out = args[0] if args else "demo/scenes/christmas-tree.json"
write_scene(scene, out)
