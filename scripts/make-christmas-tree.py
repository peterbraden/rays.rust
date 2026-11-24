#!/usr/bin/env python3
"""Writes demo/scenes/christmas-tree.json: a decorated Christmas tree in a dark, snowy forest.

A stress test of many things at once: a spruce dressed with a few hundred fairy lights (small
emitters chosen by the light BVH) on a wire of curves, tinsel garlands of fur, baubles of
glass, metal and lacquer, glass icicles and a gold star; presents in checked paper under it;
a forest of snow-laden conifers, thinning in round a wide clearing; and fog thickening towards
the ground, seen through a thin lens.

The placements are random but seeded, so rerunning writes the same file:

    python3 scripts/make-christmas-tree.py
    python3 scripts/make-christmas-tree.py --preview /tmp/preview.json  # cheap, for framing
    cargo run --release -- demo/scenes/christmas-tree.json
"""

import json
import math
import random
import sys

rng = random.Random(25)


# Rotations are nalgebra's Euler angles, in degrees: roll about x, pitch about y (the vertical),
# yaw about z.


def r3(v):
    return [round(x, 4) for x in v]


# The tree, and the cone its branch tips reach: radius `CROWN_R` at `CROWN_BASE` above the
# ground, narrowing to nothing at `TREE_H`.
TREE_H = 3.4
CROWN_BASE = 0.35
CROWN_R = 1.25


def crown(h):
    """The crown's radius at height h."""
    return CROWN_R * max(0.0, (TREE_H - h) / (TREE_H - CROWN_BASE))


def on_crown(h, angle, inset):
    """A point `inset` of the way in from the crown's surface at height h and angle."""
    r = crown(h) * (1 - inset)
    return [r * math.sin(angle), h, -r * math.cos(angle)]


materials = {
    "SNOW": {"type": "snow", "preset": "fresh", "sparkle": 0.08, "seed": 1},
    "SPRUCE_NEEDLE_1": {"type": "leaf", "reflectance": [0.03, 0.08, 0.04], "transmittance": [0.01, 0.04, 0.015], "roughness": 0.2},
    "SPRUCE_NEEDLE_2": {"type": "leaf", "reflectance": [0.04, 0.1, 0.05], "transmittance": [0.015, 0.05, 0.02], "roughness": 0.2},
    "PINE_NEEDLE": {"type": "leaf", "reflectance": [0.06, 0.11, 0.05], "transmittance": [0.02, 0.06, 0.015], "roughness": 0.2},
    "BARK_BASE": {"type": "lambertian", "albedo": [0.22, 0.17, 0.13]},
    "BARK": {"type": "noise", "base_material": "BARK_BASE", "noise_type": "fbm", "octaves": 4, "scale": 2.5, "color": [0.36, 0.34, 0.3], "blend_factor": 0.6},
    # The tree in the clearing carries a dusting; the forest's trees are laden.
    "DUSTED_NEEDLE_1": {"type": "snow_cover", "material": "SPRUCE_NEEDLE_1", "cover": 0.3, "drift": 0.04, "snow": {"preset": "fresh"}, "seed": 1},
    "DUSTED_NEEDLE_2": {"type": "snow_cover", "material": "SPRUCE_NEEDLE_2", "cover": 0.25, "drift": 0.04, "snow": {"preset": "fresh"}, "seed": 2},
    "LADEN_NEEDLE_1": {"type": "snow_cover", "material": "SPRUCE_NEEDLE_1", "cover": 0.5, "drift": 0.3, "snow": {"preset": "fresh"}, "seed": 3},
    "LADEN_NEEDLE_2": {"type": "snow_cover", "material": "SPRUCE_NEEDLE_2", "cover": 0.45, "drift": 0.3, "snow": {"preset": "fresh"}, "seed": 4},
    "LADEN_PINE_NEEDLE": {"type": "snow_cover", "material": "PINE_NEEDLE", "cover": 0.5, "drift": 0.3, "snow": {"preset": "fresh"}, "seed": 5},
    "SNOWY_BARK": {"type": "snow_cover", "material": "BARK", "cover": 0.4, "drift": 0.2, "snow": {"preset": "settled"}, "seed": 6},
    "GRANITE": {"type": "rock", "preset": "granite", "lichen": 0.3, "seed": 1},
    "SNOWY_GRANITE": {"type": "snow_cover", "material": "GRANITE", "cover": 0.55, "drift": 0.15, "snow": {"preset": "fresh"}, "seed": 7},
    # Fairy lights: warm white, most of them dim, some brighter, a few coloured, on a dark green
    # wire; and a sparser string of big old-fashioned bulbs in colours, brighter still, so the
    # lights have a hierarchy rather than an even sparkle.
    "FAIRY_DIM": {"type": "diffuse-light", "temperature": 2300, "intensity": 15},
    "FAIRY": {"type": "diffuse-light", "temperature": 2400, "intensity": 35},
    "FAIRY_BRIGHT": {"type": "diffuse-light", "temperature": 2600, "intensity": 80},
    "FAIRY_RED": {"type": "diffuse-light", "color": [1, 0.12, 0.08], "intensity": 30},
    "FAIRY_BLUE": {"type": "diffuse-light", "color": [0.2, 0.35, 1], "intensity": 35},
    "BIG_WARM": {"type": "diffuse-light", "temperature": 2200, "intensity": 60},
    "BIG_RED": {"type": "diffuse-light", "color": [1, 0.1, 0.06], "intensity": 60},
    "BIG_GREEN": {"type": "diffuse-light", "color": [0.15, 1, 0.25], "intensity": 45},
    "BIG_BLUE": {"type": "diffuse-light", "color": [0.2, 0.35, 1], "intensity": 75},
    "BIG_AMBER": {"type": "diffuse-light", "color": [1, 0.5, 0.1], "intensity": 60},
    "WIRE": {"type": "plastic", "albedo": [0.02, 0.06, 0.03], "roughness": 0.2},
    "TINSEL_GOLD": {"type": "conductor", "metal": "Au", "roughness": 0.08},
    "TINSEL_SILVER": {"type": "conductor", "metal": "Ag", "roughness": 0.06},
    "TINSEL_RED": {"type": "conductor", "reflectance": [0.85, 0.08, 0.06], "roughness": 0.1},
    "BAUBLE_RED": {"type": "plastic", "albedo": [0.6, 0.02, 0.02], "roughness": 0.02},
    "BAUBLE_GREEN": {"type": "plastic", "albedo": [0.03, 0.25, 0.08], "roughness": 0.02},
    "BAUBLE_GOLD": {"type": "conductor", "metal": "Au", "roughness": 0.04},
    "BAUBLE_SATIN_GOLD": {"type": "conductor", "metal": "Au", "roughness": 0.3},
    "BAUBLE_SILVER": {"type": "conductor", "metal": "Ag", "roughness": 0.01},
    "BAUBLE_COPPER": {"type": "coated_conductor", "metal": "Cu", "conductor_roughness": 0.25, "roughness": 0},
    "BAUBLE_BLUE": {"type": "coated_conductor", "reflectance": [0.1, 0.2, 0.85], "conductor_roughness": 0.2, "roughness": 0},
    "BAUBLE_GLASS": {"type": "dielectric", "refractive_index": 1.5, "attenuate": [1, 1, 1]},
    "BAUBLE_FROSTED": {"type": "rough_dielectric", "refractive_index": "BK7", "roughness": 0.3},
    "CAP": {"type": "conductor", "metal": "CuZn", "roughness": 0.25},
    "ICICLE": {"type": "dielectric", "refractive_index": 1.31, "attenuate": [1, 1, 1]},
    "STAR": {"type": "diffuse-light", "color": [1, 0.7, 0.25], "intensity": 1.5},
    # Presents: wrapping paper, checked and plain, and satin ribbon.
    "WRAP_RED": {"type": "checker", "colors": [[0.55, 0.03, 0.03], [0.7, 0.62, 0.5]], "checks": 0.08},
    "WRAP_GREEN": {"type": "coated_diffuse", "albedo": [0.04, 0.22, 0.08], "roughness": 0.1},
    "WRAP_BLUE": {"type": "checker", "colors": [[0.05, 0.1, 0.4], [0.75, 0.75, 0.8]], "checks": 0.05},
    "WRAP_GOLD": {"type": "coated_conductor", "metal": "Au", "conductor_roughness": 0.3, "roughness": 0.05},
    "WRAP_WHITE": {"type": "coated_diffuse", "albedo": [0.75, 0.73, 0.7], "roughness": 0.15},
    "RIBBON_RED": {"type": "plastic", "albedo": [0.5, 0.02, 0.03], "roughness": 0.25},
    "RIBBON_GOLD": {"type": "conductor", "metal": "Au", "roughness": 0.2},
    "RIBBON_SILVER": {"type": "conductor", "metal": "Ag", "roughness": 0.2},
}

objects = []


def add(o):
    objects.append(o)


def sphere(c, r, mat):
    return {"type": "sphere", "location": r3(c), "radius": round(r, 4), "material": mat}


def cylinder(a, b, r, mat):
    return {"type": "cylinder", "base": r3(a), "cap": r3(b), "radius": round(r, 4), "material": mat}


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

# The Christmas tree.
add({"type": "tree", "species": "norway-spruce", "seed": 7, "location": [0, 0, 0],
     "bark": "BARK", "leaves": ["DUSTED_NEEDLE_1", "DUSTED_NEEDLE_2"],
     "params": {"scale": TREE_H, "scale_v": 0, "base_size": 0.1, "leaves": 60, "leaf_scale": 0.12, "branch_loss": 0}})

# The fairy lights: a string spiralling up the tree, bulbs every few centimetres, each on a
# short socket, the wire between them sagging.
turns = 14
bulbs = 650
bulb_kinds = ["FAIRY_DIM"] * 9 + ["FAIRY"] * 7 + ["FAIRY_BRIGHT"] * 2 + ["FAIRY_RED", "FAIRY_BLUE"]
string = []
for i in range(bulbs):
    t = i / (bulbs - 1)
    # Strung by hand: the pace round the tree varies, and each loop dips and rises rather than
    # running level, so the string doesn't read as even rings.
    angle = t * turns * 2 * math.pi + 0.8 * math.sin(t * 23) + rng.uniform(-0.04, 0.04)
    h = CROWN_BASE + 0.08 + t * (TREE_H - CROWN_BASE - 0.3) + 0.07 * math.sin(angle * 1.5 + 2 * math.sin(t * 9))
    h = min(max(h, CROWN_BASE + 0.05), TREE_H - 0.15)
    # Some on the tips, some deep in the branches, glowing through them.
    string.append(on_crown(h, angle, rng.uniform(0.0, 0.1) if rng.random() < 0.7 else rng.uniform(0.1, 0.3)))
wire = []
for a, b in zip(string, string[1:]):
    sag = 0.25 * math.dist(a, b)
    p1 = [a[0] + (b[0] - a[0]) / 3, a[1] + (b[1] - a[1]) / 3 - sag, a[2] + (b[2] - a[2]) / 3]
    p2 = [a[0] + 2 * (b[0] - a[0]) / 3, a[1] + 2 * (b[1] - a[1]) / 3 - sag, a[2] + 2 * (b[2] - a[2]) / 3]
    wire.append({"points": [r3(a), r3(p1), r3(p2), r3(b)], "width": 0.004})
add({"type": "curves", "kind": "cylinder", "curves": wire, "material": "WIRE"})
for p in string:
    r = rng.uniform(0.007, 0.012)
    add(sphere([p[0], p[1] - 0.012, p[2]], 0.009, "WIRE"))
    add(sphere([p[0], p[1] - 0.014 - r, p[2]], r, rng.choice(bulb_kinds)))

# More about the top, where the crown narrows and the main string thins out under the star: a
# short string wound tightly over the top metre, brighter on the whole.
top_kinds = ["FAIRY_DIM"] * 3 + ["FAIRY"] * 4 + ["FAIRY_BRIGHT"] * 3 + ["FAIRY_RED", "FAIRY_BLUE"]
top = []
for i in range(90):
    t = i / 89
    angle = t * 7 * 2 * math.pi + 0.5 * math.sin(t * 17) + 2.1
    h = TREE_H - 1.0 + t * 0.88 + 0.04 * math.sin(angle * 1.3)
    # Out on the tips: up here the branches reach well past the cone the rest are placed on.
    h = min(h, TREE_H - 0.1)
    r = crown(h) + rng.uniform(0.08, 0.18)
    top.append([r * math.sin(angle), h, -r * math.cos(angle)])
add({"type": "curves", "kind": "cylinder", "material": "WIRE", "curves": [
    {"points": [r3(a), r3([a[i] + (b[i] - a[i]) / 3 - (0.2 * math.dist(a, b) if i == 1 else 0) for i in range(3)]),
                r3([a[i] + 2 * (b[i] - a[i]) / 3 - (0.2 * math.dist(a, b) if i == 1 else 0) for i in range(3)]), r3(b)], "width": 0.004}
    for a, b in zip(top, top[1:])]})
for p in top:
    r = rng.uniform(0.007, 0.012)
    add(sphere([p[0], p[1] - 0.012, p[2]], 0.009, "WIRE"))
    add(sphere([p[0], p[1] - 0.014 - r, p[2]], r, rng.choice(top_kinds)))

# The big bulbs: a few turns, on the branch tips, each on a socket, colours in turn.
big_kinds = ["BIG_WARM", "BIG_RED", "BIG_GREEN", "BIG_AMBER", "BIG_BLUE"]
big = []
for i in range(60):
    t = i / 59
    h = CROWN_BASE + 0.15 + t * (TREE_H - CROWN_BASE - 0.45)
    big.append(on_crown(h, t * 5 * 2 * math.pi + 0.9 + rng.uniform(-0.1, 0.1), rng.uniform(-0.02, 0.04)))
big_wire = []
for a, b in zip(big, big[1:]):
    sag = 0.2 * math.dist(a, b)
    p1 = [a[0] + (b[0] - a[0]) / 3, a[1] + (b[1] - a[1]) / 3 - sag, a[2] + (b[2] - a[2]) / 3]
    p2 = [a[0] + 2 * (b[0] - a[0]) / 3, a[1] + 2 * (b[1] - a[1]) / 3 - sag, a[2] + 2 * (b[2] - a[2]) / 3]
    big_wire.append({"points": [r3(a), r3(p1), r3(p2), r3(b)], "width": 0.006})
add({"type": "curves", "kind": "cylinder", "curves": big_wire, "material": "WIRE"})
for k, p in enumerate(big):
    add(cylinder([p[0], p[1] - 0.025, p[2]], [p[0], p[1], p[2]], 0.009, "WIRE"))
    add({"type": "transform", "item": {"type": "sphere", "location": [0, 0, 0], "radius": 1},
         "scale": [0.016, 0.026, 0.016], "translate": r3([p[0], p[1] - 0.045, p[2]]), "material": big_kinds[k % len(big_kinds)]})

# Tinsel: garlands of fur draped in swags from one branch tip to the next, round the tree.
for g, (mat, phase, low, high) in enumerate([("TINSEL_GOLD", 0.0, 0.55, 2.6), ("TINSEL_SILVER", 2.1, 0.75, 2.85), ("TINSEL_RED", 4.2, 0.95, 3.0)]):
    swags = 18
    for s in range(swags):
        h0 = low + (high - low) * s / swags
        h1 = low + (high - low) * (s + 1) / swags
        a0 = phase + s * 2 * math.pi / 5
        a1 = phase + (s + 1) * 2 * math.pi / 5
        start, end = on_crown(h0, a0, 0.02), on_crown(h1, a1, 0.02)
        length = math.dist(start, end)
        steps = max(4, int(length / 0.022))
        for k in range(steps):
            u = (k + 0.5) / steps
            # Along the chord between the tips, bowed in towards the crown and hanging below.
            h = h0 + (h1 - h0) * u - 0.12 * math.sin(math.pi * u) * length
            a = a0 + (a1 - a0) * u
            p = on_crown(h, a, 0.06 * math.sin(math.pi * u))
            add({"type": "fur", "center": r3(p), "radius": 0.006, "count": 70, "length": 0.03,
                 "width": [0.003, 0.0025], "droop": 0.15, "curl": 0.35, "kind": "flat",
                 "seed": g * 10000 + s * 100 + k, "material": mat})

# Baubles hung from the branches, more where the crown is wider; each with a cap and a hook.
bauble_kinds = ["BAUBLE_RED"] * 3 + ["BAUBLE_GOLD"] * 2 + ["BAUBLE_SILVER"] * 2 + [
    "BAUBLE_GREEN", "BAUBLE_SATIN_GOLD", "BAUBLE_COPPER", "BAUBLE_BLUE", "BAUBLE_GLASS", "BAUBLE_FROSTED"]
placed = []
while len(placed) < 75:
    h = CROWN_BASE + 0.1 + (TREE_H - CROWN_BASE - 0.45) * (1 - math.sqrt(rng.random()))
    a = rng.uniform(0, 2 * math.pi)
    r = 0.035 + 0.035 * rng.random() * crown(h) / CROWN_R
    p = on_crown(h, a, rng.uniform(0.04, 0.14))
    if any(math.dist(p, q) < r + s + 0.03 for q, s in placed):
        continue
    placed.append((p, r))
    mat = rng.choice(bauble_kinds)
    add(sphere(p, r, mat))
    top = [p[0], p[1] + r * 0.92, p[2]]
    add(cylinder([top[0], top[1] - 0.004, top[2]], [top[0], top[1] + 0.012, top[2]], r * 0.28, "CAP"))
    add(cylinder([top[0], top[1] + 0.012, top[2]], [top[0], top[1] + 0.05, top[2]], 0.0012, "CAP"))

# Glass icicles hanging from the lower branch tips.
for i in range(16):
    h = CROWN_BASE + 0.15 + rng.random() * 1.6
    p = on_crown(h, rng.uniform(0, 2 * math.pi), 0.03)
    length = rng.uniform(0.08, 0.16)
    add({"type": "cone", "base": r3(p), "base_radius": 0.009, "cap": r3([p[0], p[1] - length, p[2]]),
         "cap_radius": 0.0005, "material": "ICICLE"})

# The star on top: a glowing five-pointed prism standing upright, and a warm light just in
# front of it, cheap to sample, lighting the top of the tree that the string barely reaches.
points = []
for k in range(10):
    rad = 0.21 if k % 2 == 0 else 0.085
    ang = k * math.pi / 5
    points.append([round(rad * math.sin(ang), 4), round(rad * math.cos(ang), 4)])
points.append(points[0])
star_y = TREE_H + 0.12
add({"type": "transform", "item": {"type": "prism", "height1": -0.015, "height2": 0.015, "points": points},
     "rotate": {"roll": -90}, "translate": [0, star_y, 0], "material": "STAR"})
add(cylinder([0, TREE_H - 0.15, 0], [0, star_y - 0.1, 0], 0.008, "CAP"))

# Presents under the tree, in paper and ribbon, a few with snow on their lids.
present_kinds = [("WRAP_RED", "RIBBON_GOLD"), ("WRAP_GREEN", "RIBBON_RED"), ("WRAP_BLUE", "RIBBON_SILVER"),
                 ("WRAP_GOLD", "RIBBON_RED"), ("WRAP_WHITE", "RIBBON_GOLD")]
presents = []
while len(presents) < 9:
    a = rng.uniform(-2.2, 2.2)  # mostly on the side facing the camera
    d = rng.uniform(0.45, 1.25)
    w, dp, ht = rng.uniform(0.18, 0.42), rng.uniform(0.18, 0.42), rng.uniform(0.12, 0.35)
    c = [d * math.sin(a), 0, -d * math.cos(a)]
    rad = 0.5 * math.hypot(w, dp)
    if any(math.dist(c, q) < rad + s + 0.02 for q, s in presents):
        continue
    presents.append((c, rad))
    paper, ribbon = present_kinds[len(presents) % len(present_kinds)]
    yaw = round(rng.uniform(0, 90), 1)
    sunk = -0.02
    rb = 0.018
    parts = [
        {"type": "box", "min": r3([-w / 2, 0, -dp / 2]), "max": r3([w / 2, ht, dp / 2]), "material": paper},
        {"type": "box", "min": r3([-rb, -0.001, -dp / 2 - 0.002]), "max": r3([rb, ht + 0.002, dp / 2 + 0.002]), "material": ribbon},
        {"type": "box", "min": r3([-w / 2 - 0.002, -0.001, -rb]), "max": r3([w / 2 + 0.002, ht + 0.002, rb]), "material": ribbon},
        {"type": "transform", "item": {"type": "torus", "major": 0.035, "minor": 0.009}, "rotate": {"roll": 90, "yaw": 35},
         "translate": r3([0.03, ht + 0.03, 0]), "material": ribbon},
        {"type": "transform", "item": {"type": "torus", "major": 0.035, "minor": 0.009}, "rotate": {"roll": 90, "yaw": -35},
         "translate": r3([-0.03, ht + 0.03, 0]), "material": ribbon},
    ]
    for part in parts:
        add({"type": "transform", "item": part, "rotate": {"pitch": yaw}, "translate": r3([c[0], sunk, c[2]])})

# Young spruces at the clearing's edge, framing the tree.
for i, (x, z, h) in enumerate([(-3.4, -2.6, 1.6), (-6.5, 4.5, 2.4), (5.2, 1.5, 1.9), (7.5, 7.5, 3.0)]):
    add({"type": "tree", "species": "norway-spruce", "seed": 20 + i, "location": [x, 0, z],
         "bark": "SNOWY_BARK", "leaves": ["LADEN_NEEDLE_1", "LADEN_NEEDLE_2"],
         "params": {"scale": h, "scale_v": 0, "base_size": 0.08, "leaf_scale": 0.12}})

# The dark forest round the clearing, its trees laden with snow.
add({"type": "forest", "seed": 12, "lod": {"pixels": 1},
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
    "output": {"exposure": 6.0, "tonemap": "filmic"},
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
with open(out, "w") as f:
    # One object a line, so the file stays readable and diffs stay small.
    f.write("{\n")
    keys = list(scene)
    for i, k in enumerate(keys):
        sep = "," if i < len(keys) - 1 else ""
        if k == "materials":
            f.write('  "materials": {\n')
            items = list(scene[k].items())
            for j, (name, m) in enumerate(items):
                f.write("    %s: %s%s\n" % (json.dumps(name), json.dumps(m), "," if j < len(items) - 1 else ""))
            f.write("  }%s\n" % sep)
        elif k == "objects":
            f.write('  "objects": [\n')
            for j, o in enumerate(scene[k]):
                f.write("    %s%s\n" % (json.dumps(o), "," if j < len(scene[k]) - 1 else ""))
            f.write("  ]%s\n" % sep)
        else:
            f.write("  %s: %s%s\n" % (json.dumps(k), json.dumps(scene[k]), sep))
    f.write("}\n")
print("wrote %s: %d objects" % (out, len(objects)))
