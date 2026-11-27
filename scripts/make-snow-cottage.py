#!/usr/bin/env python3
"""Writes demo/scenes/snow-cottage.json: a stone cottage in a snowy forest clearing at blue
hour, its windows lit, and beside it the Christmas tree of `christmas-tree.json`, dressed in
its lights, tinsel and baubles, presents heaped under it.

The cottage is the renderer's (`"cottage"`, `src/procedural/building.rs`); the tree and its
decorations come from `christmas_tree.py`, shared with `make-christmas-tree.py`. The
placements are random but seeded, so rerunning writes the same file:

    python3 scripts/make-snow-cottage.py
    python3 scripts/make-snow-cottage.py --preview /tmp/preview.json  # cheap, for framing
    cargo run --release -- demo/scenes/snow-cottage.json
"""

import math
import os
import random
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from christmas_tree import MATERIALS, decorate, turned, write_scene  # noqa: E402

rng = random.Random(25)

# The camera, looking past the tree at the cottage, the sky's afterglow low on the left.
camera = {"location": [-6.0, 1.5, 11.5], "lookat": [1.3, 2.0, 1.4], "up": [0, 1, 0], "angle": 0.7, "aperture": 0.02}

# The tree, out in front of the cottage's right-hand end, turned so that its presents (heaped
# on its -z side) face the camera.
TREE_AT = [3.2, 0, 5.0]
towards = [camera["location"][0] - TREE_AT[0], camera["location"][2] - TREE_AT[2]]
TURN = round(math.degrees(math.atan2(-towards[0], -towards[1])), 1)

materials = dict(MATERIALS)
materials.update({
    "SNOWY_GRANITE": {"type": "snow_cover", "material": "GRANITE", "cover": 0.6, "drift": 0.15, "snow": {"preset": "fresh"}, "seed": 7},
})

LADEN = ["LADEN_NEEDLE_1", "LADEN_NEEDLE_2"]
objects = [
    # Blue hour: the sun just below the horizon to the front left, so its afterglow lights the
    # front of the cottage and the roof takes the blue of the sky.
    {"type": "skysphere", "sun_direction": [-0.85, -0.008, 0.5]},
    {"type": "plane", "y": 0, "material": "SNOW", "normal": {"pattern": "bumps", "amount": 0.25, "scale": 1.5}},
    {"type": "cottage", "style": "stone", "location": [0, 0, 0], "yaw": -18, "snow": 0.22, "seed": 3},
    # Trees framing the view, and boulders under the snow.
    {"type": "tree", "species": "norway-spruce", "seed": 31, "location": [-9.5, 0, 7.5], "bark": "SNOWY_BARK", "leaves": LADEN,
     "params": {"scale": 4.2, "scale_v": 0, "base_size": 0.08, "leaf_scale": 0.12}},
    {"type": "tree", "species": "balsam-fir", "seed": 32, "location": [7.5, 0, 2.5], "bark": "SNOWY_BARK", "leaves": LADEN,
     "params": {"scale": 5.5, "scale_v": 0, "base_size": 0.08, "leaf_scale": 0.12}},
    {"type": "tree", "species": "norway-spruce", "seed": 33, "location": [6.2, 0, 9.8], "bark": "SNOWY_BARK", "leaves": LADEN,
     "params": {"scale": 2.2, "scale_v": 0, "base_size": 0.08, "leaf_scale": 0.12}},
    {"type": "boulder", "location": [-5.2, -0.25, 5.2], "radius": 0.55, "angularity": 0.4, "seed": 4, "material": "SNOWY_GRANITE"},
    {"type": "boulder", "location": [6.3, -0.3, 6.6], "radius": 0.4, "angularity": 0.5, "seed": 5, "material": "SNOWY_GRANITE"},
    # The forest round the clearing, open in a widening gap along the view towards the sky.
    {"type": "forest", "seed": 12, "lod": {"pixels": 1}, "cache": "demo/scenes/.forest-cache",
     "area": {"min": [-90, -90], "max": [90, 40]}, "spacing": 4.5, "clearings": 0.05, "patch_size": 30, "patchiness": 0.3,
     "variants": 8,
     "where": "min(1, max(0, (sqrt(x * x + 1.6 * (z - 6) * (z - 6)) - 16) / 8)) * min(1, max(0, (sqrt(((x + 7.5) * 0.874 + (z - 13.5) * 0.486) * ((x + 7.5) * 0.874 + (z - 13.5) * 0.486)) - 7 - 0.5 * max(0, ((x + 7.5) * 0.486 - (z - 13.5) * 0.874) - 14)) / 4))",
     "species": [
         {"species": "norway-spruce", "weight": 4, "spacing": 0.8, "bark": "SNOWY_BARK", "leaves": LADEN},
         {"species": "balsam-fir", "weight": 3, "spacing": 0.7, "bark": "SNOWY_BARK", "leaves": LADEN},
         {"species": "scots-pine", "weight": 1, "bark": "SNOWY_BARK", "leaves": "LADEN_PINE_NEEDLE"},
     ]},
]

# The Christmas tree, dressed, with its presents; a warm lamp just in front of its star.
star_y = decorate(rng, objects.append, TREE_AT, TURN)
lamp = [round(q + a, 4) for q, a in zip(turned([0, star_y - 0.35, -0.3], TURN), TREE_AT)]

scene = {
    "width": 960,
    "height": 600,
    "supersamples": 256,
    "chunk_size": 32,
    "samples_per_chunk": 4,
    "max_depth": 6,
    "path_termination": "russian_roulette",
    "light_sampler": "bvh",
    "filter": "mitchell",
    "adaptive": {"threshold": 0.015, "min_samples": 32},
    # The bulbs are white-hot at an exposure that shows the cottage; the camera's glare gives
    # each a halo of its own colour, as in a photograph.
    "output": {"exposure": 6, "tonemap": "filmic", "glare": True},
    "camera": camera,
    "materials": materials,
    "objects": objects,
    "lights": [{"location": lamp, "intensity": 1.2, "temperature": 2600}],
}

args = [a for a in sys.argv[1:] if a != "--preview"]
if "--preview" in sys.argv[1:]:
    scene.update({"max_depth": 3, "supersamples": 8, "adaptive": {"threshold": 0}})
write_scene(scene, args[0] if args else "demo/scenes/snow-cottage.json")
