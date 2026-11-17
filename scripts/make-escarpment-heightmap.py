"""A heightmap for demo/scenes/displaced-terrain.json, after the sandstone escarpments of the
Colorado Plateau (a Wingate cliff above a sandy wash, as along Capitol Reef or Indian Creek):
a wash floor rising gently to a talus apron, a cliff about 30 m high whose foot steps in and
out in buttresses and alcoves, a ledgy rim and a slickrock bench above. The heightmap holds
only the big forms, at 1 m; ledges, joints, blocks and stones come from the scene's
displacement.

16-bit greyscale PNG, 0 to 1 over HEIGHT metres; row 0 at min z. Run from the repository root:
    python3 scripts/make-escarpment-heightmap.py demo/textures/escarpment-height.png
"""
import sys
import numpy as np
from PIL import Image

X0, Z0, CELL, HEIGHT = -150., -20., 1., 60.
W, H = 401, 321
x = X0 + np.arange(W) * CELL
z = Z0 + np.arange(H) * CELL
X, Z = np.meshgrid(x, z)


def noise(scale, octaves=4, seed=0):
    """Smooth value noise, roughly in [-1, 1]."""
    r = np.random.default_rng(seed)
    out, amp, total = np.zeros_like(X), 1., 0.
    for o in range(octaves):
        s = scale / 2 ** o
        g = r.standard_normal((int(H * CELL / s) + 3, int(W * CELL / s) + 3))
        u, v = (X - X0) / s, (Z - Z0) / s
        i, j = np.floor(u).astype(int), np.floor(v).astype(int)
        fu, fv = u - i, v - j
        fu, fv = fu * fu * (3 - 2 * fu), fv * fv * (3 - 2 * fv)
        out += amp * ((g[j, i] * (1 - fu) + g[j, i + 1] * fu) * (1 - fv)
                      + (g[j + 1, i] * (1 - fu) + g[j + 1, i + 1] * fu) * fv)
        total += amp
        amp *= 0.5
    return out / total


def smoothstep(a, b, t):
    t = np.clip((t - a) / (b - a), 0, 1)
    return t * t * (3 - 2 * t)


# The cliff's foot: buttresses and alcoves where joints let it retreat.
foot = 72 + 9 * noise(90, 3, 1) + 5 * np.abs(np.sin(X / 17. + 2.5 * noise(45, 2, 2))) ** 0.6 + 1.5 * noise(8, 2, 11)
d = Z - foot                               # metres in from the foot
cliff_h = 30 + 3 * noise(70, 2, 3)
apron_h = 10 + 3 * noise(40, 2, 4)

# Floor: rising 2% towards the cliff, gently rolling, with a shallow wash across it.
floor = 0.02 * (Z - Z0) + 0.4 * noise(30, 3, 5)
wash_z = 22 + 6 * np.sin(X / 37.) + 3 * noise(40, 2, 6)
floor -= 0.7 * np.exp(-((Z - wash_z) / 3.5) ** 2)
# Talus: a concave apron, steepest (about the angle of repose) where it meets the cliff.
# Debris cones where gullies spill off the cliff, and hummocks of fallen rock.
dn = np.minimum(d, 0)
cones = np.maximum(0, np.sin(X / 23. + 2 * noise(80, 2, 9))) ** 6
hummocks = 0.6 * np.maximum(noise(12, 3, 10), 0)
apron_h = apron_h * (1 + 0.35 * cones) + hummocks
apron = apron_h * np.exp(dn / (18. + 6 * noise(60, 2, 8)))
# The cliff: near vertical sandstone over 4 m of the heightmap.
rise = smoothstep(0, 4, d)
# A crisp rim, then the ledgy slope of the next formation up in two steps.
rim = 3 * smoothstep(9, 11, d + 2 * noise(20, 2, 12)) + 3 * smoothstep(17, 19.5, d + 2 * noise(20, 2, 13))
cliff = apron_h + (cliff_h - apron_h) * rise + rim
bench = 0.6 * noise(25, 3, 7) + 0.03 * np.maximum(d - 22, 0)
h = floor + np.where(d > 0, cliff + np.where(d > 22, bench, 0), apron)
h -= h.min() - 0.5
c = (int((0 - Z0) / CELL), int((0 - X0) / CELL))
print("heights %.1f to %.1f m; at the camera (0, 0): %.2f m" % (h.min(), h.max(), h[c]))
Image.fromarray((np.clip(h / HEIGHT, 0, 1) * 65535).astype(np.uint16)).save(sys.argv[1])
