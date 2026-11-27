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
camera = {"location": [-6.0, 1.5, 11.5], "lookat": [1.4, 3.4, 1.8], "up": [0, 1, 0], "angle": 0.8, "aperture": 0.02}
WIDTH, HEIGHT = 960, 600
# Where the moon is in the picture: (-1, -1) bottom left to (1, 1) top right.
MOON_ON_SCREEN = (-0.05, 1.3)

# The cottage, turned to be seen three-quarter on (its front about 45 degrees from the
# camera): its front, and its right-hand gable with its chimney. `local` places a point given in the cottage's own frame (x along the ridge, z
# out of the front).
YAW = -72


def local(x, z):
    c, s = math.cos(math.radians(YAW)), math.sin(math.radians(YAW))
    return [round(x * c + z * s, 3), 0, round(-x * s + z * c, 3)]

def from_camera(ahead, right):
    """A point on the ground `ahead` metres along the camera's view and `right` to its right."""
    eye, at = camera["location"], camera["lookat"]
    f = [at[0] - eye[0], at[2] - eye[2]]
    n = math.hypot(*f)
    f = [f[0] / n, f[1] / n]
    return [round(eye[0] + f[0] * ahead - f[1] * right, 3), 0, round(eye[2] + f[1] * ahead + f[0] * right, 3)]


# The tree, in the foreground to the right, before the cottage's gable, turned so that its
# presents (heaped on its -z side) face the camera.
TREE_AHEAD, TREE_RIGHT = (float(x) for x in os.environ.get("TREE", "9.5,3.0").split(","))
TREE_AT = from_camera(TREE_AHEAD, TREE_RIGHT)
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
    {"type": "plane", "y": -0.004, "material": "SNOW", "normal": {"pattern": "bumps", "amount": 0.25, "scale": 1.5}},
    {"type": "cottage", "style": "stone", "location": [0, 0, 0], "yaw": YAW, "snow": 0.22, "seed": 3,
     # A stack on the far gable only; the near one keeps its window.
     "chimney": "left"},
    # Trees framing the view, and boulders under the snow.
    {"type": "tree", "species": "norway-spruce", "seed": 31, "location": [-9.5, 0, 7.5], "bark": "SNOWY_BARK", "leaves": LADEN,
     "params": {"scale": 4.2, "scale_v": 0, "base_size": 0.08, "leaf_scale": 0.12}},
    {"type": "tree", "species": "balsam-fir", "seed": 32, "location": [7.5, 0, 2.5], "bark": "SNOWY_BARK", "leaves": LADEN,
     "params": {"scale": 5.5, "scale_v": 0, "base_size": 0.08, "leaf_scale": 0.12}},
    {"type": "tree", "species": "norway-spruce", "seed": 33, "location": [6.2, 0, 9.8], "bark": "SNOWY_BARK", "leaves": LADEN,
     "params": {"scale": 2.2, "scale_v": 0, "base_size": 0.08, "leaf_scale": 0.12}},
    {"type": "boulder", "location": [-5.2, -0.25, 5.2], "radius": 0.55, "angularity": 0.4, "seed": 4, "material": "SNOWY_GRANITE"},
    {"type": "boulder", "location": [6.3, -0.3, 6.6], "radius": 0.4, "angularity": 0.5, "seed": 5, "material": "SNOWY_GRANITE"},
    # The forest round the clearing, but for a gap behind the cottage, widening away from the
    # camera, where the sky shows over the trees.
    {"type": "forest", "seed": 12, "lod": {"pixels": 1}, "cache": "demo/scenes/.forest-cache",
     "area": {"min": [-90, -90], "max": [90, 40]}, "spacing": 4.5, "clearings": 0.05, "patch_size": 30, "patchiness": 0.3,
     "variants": 8,
     "where": "min(1, max(0, (sqrt(x * x + 1.2 * (z - 2) * (z - 2)) - 18) / 6)) * min(1, max(0, (sqrt(((x + 6) * 0.819 + (z - 11.5) * 0.574) * ((x + 6) * 0.819 + (z - 11.5) * 0.574)) - 5 - 0.3 * max(0, ((x + 6) * 0.574 - (z - 11.5) * 0.819) - 14)) / 4))",
     "species": [
         {"species": "norway-spruce", "weight": 4, "spacing": 0.8, "bark": "SNOWY_BARK", "leaves": LADEN},
         {"species": "balsam-fir", "weight": 3, "spacing": 0.7, "bark": "SNOWY_BARK", "leaves": LADEN},
         {"species": "scots-pine", "weight": 1, "bark": "SNOWY_BARK", "leaves": "LADEN_PINE_NEEDLE"},
     ]},
]

# Trees standing about the cottage, behind it and to either side, younger and lower than the
# forest's: spruces and firs, laden with snow (in the cottage's frame: x, z, height).
for i, (x, z, h, species) in enumerate([
        (-6.5, -6.0, 6.0, "norway-spruce"), (-1.5, -8.0, 7.0, "balsam-fir"), (3.5, -6.5, 5.0, "norway-spruce"),
        (8.0, -4.0, 6.5, "balsam-fir"), (-9.0, -2.0, 5.5, "norway-spruce"), (-10.5, 2.5, 4.5, "balsam-fir"),
        (11.0, 1.0, 5.5, "norway-spruce"), (6.0, -9.5, 7.5, "norway-spruce"), (-4.0, -11.0, 6.5, "balsam-fir"),
        (1.5, -12.5, 5.5, "norway-spruce")]):
    objects.append({"type": "tree", "species": species, "seed": 40 + i, "location": local(x, z), "bark": "SNOWY_BARK",
                    "leaves": LADEN, "params": {"scale": h, "scale_v": 0, "base_size": 0.08, "leaf_scale": 0.12}})

def on_screen(sx, sy):
    """The direction from the camera to the point (sx, sy) of its picture, each from -1 (left,
    bottom) to 1 (right, top); the camera's `angle` is its horizontal field of view."""
    eye, at = camera["location"], camera["lookat"]
    f = [at[i] - eye[i] for i in range(3)]
    n = math.sqrt(sum(c * c for c in f))
    f = [c / n for c in f]
    right = [-f[2], 0, f[0]]
    n = math.hypot(right[0], right[2])
    right = [c / n for c in right]
    up = [right[1] * f[2] - right[2] * f[1], right[2] * f[0] - right[0] * f[2], right[0] * f[1] - right[1] * f[0]]
    half = math.tan(camera["angle"] / 2)
    aspect = HEIGHT / WIDTH
    d = [f[i] + right[i] * sx * half + up[i] * sy * half * aspect for i in range(3)]
    n = math.sqrt(sum(c * c for c in d))
    return [round(c / n, 4) for c in d]


# The moon, rising nearly full opposite the set sun, in the sky above the cottage: three
# times its true size, as a longer lens would show it against the cottage.
objects.append({"type": "moon", "seed": 1, "radius": 3 * 1737400, "direction": on_screen(*MOON_ON_SCREEN),
                "from": camera["location"]})

# Smoke from the far stack's pot: a plume rising, widening and thinning as it goes, carried
# off on a light air and broken into puffs. Where the pot is, from the cottage's dimensions
# (`Cottage::stone`: walls 2.4 m, 5 m deep between centres, 0.55 thick; pitch 40 degrees;
# slates 0.15; the stack 0.75 over the crest, its cap 0.1, a pot about 0.4).
pitch = math.radians(40)
crest = 2.4 + (2.5 + 0.275) * math.tan(pitch) + 0.15 / math.cos(pitch)
pot = local(-(3.75 + 0.275) + 0.3, 0.2)
SMOKE = [pot[0], crest + 0.75 + 0.1 + 0.38, pot[2]]
wind = [0.3, 0.12]  # metres across per metre risen: drifting right, across the sky
h = "max(y - %.3f, 0)" % SMOKE[1]
dx = "(x - %.3f - %.2f * %s)" % (SMOKE[0], wind[0], h)
dz = "(z - %.3f - %.2f * %s)" % (SMOKE[2], wind[1], h)
r = "(0.12 + 0.24 * %s)" % h
smoke = ("12 * min(1, max(0, (y - %.3f) * 25)) * exp(-(%s * %s + %s * %s) / (%s * %s)) * (0.12 / %s) * exp(-%s / 6)"
         " * (0.25 + 1.5 * max(0, noise3d(x * 2.5, y * 2.5 - 7, z * 2.5) + 0.25))") % (SMOKE[1], dx, dx, dz, dz, r, r, r, h)
objects.append({"type": "volume", "min": [round(SMOKE[0] - 4, 2), round(SMOKE[1] - 0.05, 2), round(SMOKE[2] - 4, 2)],
                "max": [round(SMOKE[0] + 4, 2), round(SMOKE[1] + 8, 2), round(SMOKE[2] + 4, 2)],
                "density": {"node": "expr", "expr": smoke, "range": [0, 24]},
                "albedo": [0.9, 0.9, 0.91], "g": 0.5})

# The Christmas tree, dressed, with its presents; a warm lamp just in front of its star.
star_y = decorate(rng, objects.append, TREE_AT, TURN)
lamp = [round(q + a, 4) for q, a in zip(turned([0, star_y - 0.35, -0.3], TURN), TREE_AT)]

# Footprints: someone has walked out of the door, across to the tree with the presents and
# back off towards the camera's left. Each print a dent pressed into the snow, heel and toe,
# its rim pushed up a little, laid left and right of a wandering path at a stride's spacing;
# they are a displacement of a terrain patch over the clearing (the plane lies just below).
door = local(-0.1, 2.5 + 0.275 + 0.5)
out = local(-0.1, 2.5 + 0.275 + 2.0)
near_tree = [TREE_AT[0] - 1.2 * math.sin(math.radians(TURN)), 0, TREE_AT[2] - 1.2 * math.cos(math.radians(TURN))]
away = from_camera(4.0, -2.5)
path = [door, out, near_tree, from_camera(6.5, 0.5), away]


def along(points, step):
    """Points every `step` metres along a smooth curve through `points` (Catmull-Rom), with the
    heading there."""
    pts = [points[0]] + points + [points[-1]]
    dense = []
    for i in range(1, len(pts) - 2):
        p0, p1, p2, p3 = pts[i - 1], pts[i], pts[i + 1], pts[i + 2]
        for k in range(40):
            t = k / 40
            dense.append([0.5 * ((2 * p1[j]) + (-p0[j] + p2[j]) * t + (2 * p0[j] - 5 * p1[j] + 4 * p2[j] - p3[j]) * t * t
                                 + (-p0[j] + 3 * p1[j] - 3 * p2[j] + p3[j]) * t ** 3) for j in (0, 2)])
    out, gone = [], step
    for a, b in zip(dense, dense[1:]):
        gone += math.dist(a, b)
        if gone >= step:
            gone -= step
            out.append((b, math.atan2(b[0] - a[0], b[1] - a[1])))
    return out


prints = []
for k, ((x, z), heading) in enumerate(along(path, 0.36)):
    side = 0.11 if k % 2 == 0 else -0.11
    jitter = rng.uniform(-0.03, 0.03)
    px = x + math.cos(heading) * (side + jitter)
    pz = z - math.sin(heading) * (side + jitter)
    turn = heading + (0.12 if side > 0 else -0.12) + rng.uniform(-0.08, 0.08)
    prints.append((px, pz, math.sin(turn), math.cos(turn), rng.uniform(0.14, 0.2)))
terms = []
for px, pz, sn, cs, deep in prints:
    u = "((x - %.3f) * %.4f + (z - %.3f) * %.4f)" % (px, sn, pz, cs)   # along the foot
    v = "((x - %.3f) * %.4f - (z - %.3f) * %.4f)" % (px, cs, pz, sn)   # across it
    # A boot's outline: longer than wide, the waist narrower than heel and toe.
    w = "(0.07 - 0.012 * exp(-sqr(%s / 0.035)))" % u
    d = "(sqr(%s / 0.17) + sqr(%s / %s))" % (u, v, w)
    terms.append("%.3f * (0.35 * exp(-sqr((sqrt(%s) - 1.4) / 0.3)) - exp(-sqr(sqr(%s))))" % (deep, d, d))
# And the snow between them trodden and kicked about: a shallow trough along the way.
for (x, z), heading in along(path, 0.18):
    terms.append("-0.05 * exp(-(sqr(x - %.3f) + sqr(z - %.3f)) / 0.035)" % (x, z))
FOOTPRINTS = {"node": "expr", "expr": " + ".join(terms), "range": [-0.3, 0.08]}
materials["TRODDEN_SNOW"] = {"type": "snow", "preset": "settled", "grain_size": 1.5, "soot": 150, "dust": 40, "sparkle": 0.02, "seed": 2}
xs = [p[0] for p in prints]
zs = [p[1] for p in prints]
objects.append({"type": "terrain", "heightmap": {"type": "flat", "min": [round(min(xs) - 1, 2), round(min(zs) - 1, 2)],
                                                  "max": [round(max(xs) + 1, 2), round(max(zs) + 1, 2)], "cell_size": 0.25},
                "displacement": FOOTPRINTS,
                # Trodden snow is packed: its grains bonded coarser, a little greyer.
                "material": {"type": "mix", "materials": ["SNOW", "TRODDEN_SNOW"],
                             "amount": {"pattern": FOOTPRINTS, "color_map": [[-0.1, [1, 1, 1]], [-0.01, [0, 0, 0]]]}}, "normal": {"pattern": "bumps", "amount": 0.25, "scale": 1.5}})

scene = {
    "width": WIDTH,
    "height": HEIGHT,
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
    # The sun, below the horizon: no light of its own here, but the moon reflects it. Its
    # intensity sets the sky's brightness: this one leaves it at a skysphere's default (20).
    "sun": {"intensity": 7.663},
    "materials": materials,
    "objects": objects,
    "lights": [{"location": lamp, "intensity": 1.2, "temperature": 2600}],
}

args = [a for a in sys.argv[1:] if a != "--preview"]
if "--preview" in sys.argv[1:]:
    scene.update({"max_depth": 3, "supersamples": 8, "adaptive": {"threshold": 0}})
write_scene(scene, args[0] if args else "demo/scenes/snow-cottage.json")
