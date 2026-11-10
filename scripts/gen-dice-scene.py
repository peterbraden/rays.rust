"""Writes demo/scenes/dice.json: one CSG die (a rounded cube less its pips) defined once under
"geometries" and instanced many times, plus extruded text. Run from the repository root:

    python3 scripts/gen-dice-scene.py demo/scenes/dice.json
"""
import json, random, sys
random.seed(4)
s = 0.55  # pip offset from face centre
def pips(n):
    return {1: [(0, 0)], 2: [(-s, -s), (s, s)], 3: [(-s, -s), (0, 0), (s, s)],
            4: [(-s, -s), (-s, s), (s, -s), (s, s)], 5: [(-s, -s), (-s, s), (0, 0), (s, -s), (s, s)],
            6: [(-s, -s), (-s, 0), (-s, s), (s, -s), (s, 0), (s, s)]}[n]
holes = []
# Opposite faces sum to seven.
faces = {1: (1, 0, 0), 6: (-1, 0, 0), 2: (0, 1, 0), 5: (0, -1, 0), 3: (0, 0, 1), 4: (0, 0, -1)}
for n, (fx, fy, fz) in faces.items():
    for a, b in pips(n):
        if fx: c = [fx * 1.06, a, b]
        elif fy: c = [a, fy * 1.06, b]
        else: c = [a, b, fz * 1.06]
        holes.append({"type": "sphere", "location": c, "radius": 0.25})
die = {"type": "difference", "items": [
    {"type": "intersection", "items": [
        {"type": "box", "min": [-1, -1, -1], "max": [1, 1, 1]},
        {"type": "sphere", "location": [0, 0, 0], "radius": 1.38}]},
    *holes]}

mats = ["IVORY"] * 5 + ["RED"] * 4 + ["GLASS"] * 3 + ["BLACK"] * 2 + ["GOLD"]
objects = [
    {"type": "box", "min": [-400, -1, -400], "max": [400, 0, 400], "material": "FELT", "normal": {"pattern": "bumps", "amount": 0.25, "scale": 0.05}},
    {"type": "sphere", "location": [-14, 26, -10], "radius": 6, "material": "LIGHT"},
    {"type": "sphere", "location": [18, 18, -16], "radius": 3, "material": "FILL"},
    {"type": "transform", "item": {"type": "text", "font": "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
                                   "string": "rays.rust", "thickness": 0.5},
     "steps": [{"scale": 5}, {"rotate": [0, 180, 0]}, {"translate": [13.5, 0, 19]}],
     "material": "GOLD", "normal": {"pattern": "dents", "amount": 0.3, "scale": 0.08}},
]
placed = []
tries = 0
while len(placed) < 26 and tries < 5000:
    tries += 1
    x, z = random.uniform(-10, 10), random.uniform(-3, 13)
    sc = random.uniform(0.45, 0.7)
    if any((x - px) ** 2 + (z - pz) ** 2 < (sc + ps + 0.35) ** 2 * 2 for px, pz, ps in placed):
        continue
    placed.append((x, z, sc))
    # Resting on a face: a quarter turn about x or z, then any turn about y.
    up = random.choice([[0, 0, 0], [90, 0, 0], [-90, 0, 0], [180, 0, 0], [0, 0, 90], [0, 0, -90]])
    objects.append({"type": "transform", "item": "die",
                    "steps": [{"rotate": up}, {"rotate": [0, random.uniform(0, 360), 0]},
                              {"scale": sc}, {"translate": [x, sc, z]}],
                    "material": mats[len(placed) % len(mats)]})
# A tower of three, each turned a little.
for i, (sc, yaw) in enumerate([(0.9, 10), (0.75, 35), (0.6, 70)]):
    y = sum(2 * s2 for s2, _ in [(0.9, 0), (0.75, 0), (0.6, 0)][:i]) + sc
    objects.append({"type": "transform", "item": "die",
                    "steps": [{"rotate": [0, yaw, 0]}, {"scale": sc}, {"translate": [-1.5, y, 2]}],
                    "material": ["RED", "IVORY", "GLASS"][i]})

scene = {
    "width": 960, "height": 540, "supersamples": 256, "chunk_size": 64, "samples_per_chunk": 4,
    "shadow_bias": 0.0001, "max_depth": 8,
    "background": {"type": "gradient", "bottom": [0.05, 0.05, 0.06], "top": [0.04, 0.045, 0.06]},
    "geometries": {"die": die},
    "materials": {
        "FELT": {"type": "lambertian", "albedo": [0.06, 0.28, 0.16]},
        "IVORY": {"type": "plastic", "albedo": [0.9, 0.87, 0.78], "roughness": 0.08},
        "RED": {"type": "plastic", "albedo": [0.75, 0.06, 0.05], "roughness": 0.05},
        "BLACK": {"type": "plastic", "albedo": [0.03, 0.03, 0.03], "roughness": 0.05},
        "GLASS": {"type": "dielectric", "refractive_index": 1.5, "attenuate": [0.85, 0.93, 1.0]},
        "GOLD": {"type": "metal", "reflective": [1, 0.78, 0.4], "roughness": 0.04},
        "LIGHT": {"type": "diffuse-light", "color": [1, 0.95, 0.88], "intensity": 7},
        "FILL": {"type": "diffuse-light", "color": [0.7, 0.8, 1], "intensity": 4},
    },
    "media": {}, "variables": {}, "lights": [],
    "camera": {"location": [0, 6.5, -13], "lookat": [0, 1.2, 5], "up": [0, 1, 0], "angle": 0.62, "aperture": 0.12},
    "objects": objects,
}
json.dump(scene, open(sys.argv[1], "w"), indent=2)
