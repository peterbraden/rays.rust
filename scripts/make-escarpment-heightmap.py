# A heightmap for demo/scenes/displaced-terrain.json: a valley floor, a scree slope and an
# escarpment rising to a plateau, cut by gullies. 16-bit PNG, 0..1 over `HEIGHT` metres.
import numpy as np, sys
from PIL import Image
W, H = 201, 268            # 1.5 m pixels over x [-150, 150], z [-30, 370]
X0, Z0, CELL, HEIGHT = -150., -30., 1.5, 40.
rng = np.random.default_rng(3)
x = X0 + np.arange(W) * CELL
z = Z0 + np.arange(H) * CELL
X, Z = np.meshgrid(x, z)

def smooth_noise(scale, octaves=4, seed=0):
    r = np.random.default_rng(seed)
    out = np.zeros_like(X)
    amp, total = 1., 0.
    for o in range(octaves):
        s = scale / 2 ** o
        nx, nz = int(300 / s) + 3, int(400 / s) + 3
        g = r.standard_normal((nz, nx))
        u, v = (X - X0) / s, (Z - Z0) / s
        i, j = np.floor(u).astype(int), np.floor(v).astype(int)
        fu, fv = u - i, v - j
        fu, fv = fu * fu * (3 - 2 * fu), fv * fv * (3 - 2 * fv)
        val = (g[j, i] * (1 - fu) + g[j, i + 1] * fu) * (1 - fv) + (g[j + 1, i] * (1 - fu) + g[j + 1, i + 1] * fu) * fv
        out += amp * val; total += amp; amp *= 0.5
    return out / total

# The escarpment's foot wanders in and out (spurs and bays).
foot = 38 + 10 * smooth_noise(60, 3, 1) + 4 * np.sin(X / 23.)
d = Z - foot                                 # metres beyond the cliff's foot
cliff_h = 26 + 4 * smooth_noise(80, 2, 2)
# Cliff: steep band 14 m deep; scree apron below it, concave, about 32 degrees at its top.
t = np.clip(d / 14., 0, 1)
cliff = cliff_h * (0.35 + 0.65 * (t * t * (3 - 2 * t)))
apron_h = 0.35 * cliff_h
apron = apron_h * np.exp(np.minimum(d, 0) / 16.)
h = np.where(d > 0, cliff, apron)
# Gullies down the face.
gully = np.clip(1 - np.abs(np.sin(X / 17. + 0.6 * smooth_noise(40, 2, 4))) * 3, 0, 1)
h -= 5 * gully * np.clip((d + 10) / 25., 0, 1) * np.clip((60 - d) / 30, 0, 1)
# Plateau and floor undulation.
h += 1.2 * smooth_noise(25, 4, 5) + 0.5 * smooth_noise(6, 3, 6)
h = h - h.min() + 0.5
print("range", h.min(), h.max(), "at camera", h[int((0 - Z0) / CELL), int((0 - X0) / CELL)])
img = np.clip(h / HEIGHT, 0, 1) * 65535
Image.fromarray(img.astype(np.uint16)).save(sys.argv[1])
