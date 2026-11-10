"""Writes the island heightmap used by demo/scenes/lighthouse-island.json: a 16-bit greyscale
image, an elongated island shaped by fractal value noise, with a flat top for the lighthouse.

    python3 scripts/gen-island-heightmap.py demo/heightmaps/island.png
"""
import numpy as np, sys
from PIL import Image
N = 512
rng = np.random.default_rng(11)
def value_noise(n, cells):
    g = rng.random((cells + 1, cells + 1))
    x = np.linspace(0, cells, n, endpoint=False)
    i = x.astype(int); f = x - i; f = f * f * (3 - 2 * f)
    a = g[np.ix_(i, i)]; b = g[np.ix_(i + 1, i)]; c = g[np.ix_(i, i + 1)]; d = g[np.ix_(i + 1, i + 1)]
    fx = f[:, None]; fy = f[None, :]
    return (a * (1 - fx) + b * fx) * (1 - fy) + (c * (1 - fx) + d * fx) * fy
fbm = sum(value_noise(N, 4 * 2**k) * 0.5**k for k in range(6))
fbm = (fbm - fbm.min()) / (fbm.max() - fbm.min())
y, x = np.mgrid[0:N, 0:N] / (N - 1) * 2 - 1
# Elongated island, its peak off-centre (toward +x, -z in the image's rows).
cx, cy = 0.15, 0.05
r = np.sqrt(((x - cx) / 0.75) ** 2 + ((y - cy) / 0.55) ** 2)
r = r * (0.8 + 0.45 * fbm)
h = np.clip(1 - r, 0, None) ** 1.1
h = h * (0.6 + 0.7 * fbm)
fine = sum(value_noise(N, 32 * 2**k) * 0.5**k for k in range(4))
fine = (fine - fine.min()) / (fine.max() - fine.min())
h = np.where(h > 0, h + 0.1 * (fine - 0.5) * np.clip(h * 6, 0, 1), 0)
plateau = 0.66
# Flat top near the peak for the lighthouse.
d = np.sqrt((x - cx) ** 2 + (y - cy) ** 2)
h = np.where(d < 0.045, plateau, np.minimum(h, plateau))
h = np.clip(h / plateau, 0, 1)
Image.fromarray((h * 65535).astype(np.uint16)).save(sys.argv[1])
