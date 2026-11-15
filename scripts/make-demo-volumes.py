#!/usr/bin/env python3
# Generate the demo voxel grids in demo/volumes/ as Mitsuba .vol files (uint8; see
# src/volume/grid.rs): a smoke plume rising from a fire and bending in the wind (plume.vol, its
# density) and the fire's temperature (fire.vol: 255 is 2000 K). Needs numpy.
# Usage: scripts/make-demo-volumes.py
import os
import struct
import numpy as np

rng = np.random.default_rng(11)
NX, NY, NZ = 64, 112, 64
# The box the grids span: x and z from -1 to 1, y from 0 to 3.5.
LO, HI = (-1.0, 0.0, -1.0), (1.0, 3.5, 1.0)
out = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'demo', 'volumes') + '/'
os.makedirs(out, exist_ok=True)

z, y, x = np.meshgrid(*(np.linspace(l, h, n, endpoint=False) + (h - l) / n / 2 for l, h, n in
                        zip(LO[::-1], HI[::-1], (NZ, NY, NX))), indexing='ij')

def value_noise(px, py, pz, cells, seed):
    """Smooth lattice noise in [0, 1] with `cells` lattice cells per unit."""
    g = np.random.default_rng(seed).random((cells * 4 + 2,) * 3)
    def at(i, j, k):
        n = g.shape[0]
        return g[i % n, j % n, k % n]
    sx, sy, sz = (px + 1.5) * cells, py * cells, (pz + 1.5) * cells
    ix, iy, iz = np.floor(sx).astype(int), np.floor(sy).astype(int), np.floor(sz).astype(int)
    fx, fy, fz = [(f * f * (3 - 2 * f)) for f in (sx - ix, sy - iy, sz - iz)]
    c = 0
    for dx in (0, 1):
        for dy in (0, 1):
            for dz in (0, 1):
                w = (fx if dx else 1 - fx) * (fy if dy else 1 - fy) * (fz if dz else 1 - fz)
                c = c + w * at(ix + dx, iy + dy, iz + dz)
    return c

def fbm(px, py, pz, base, octaves, seed):
    total, amp, norm = 0, 1.0, 0
    for o in range(octaves):
        total = total + amp * value_noise(px, py, pz, base * 2 ** o, seed + o)
        norm += amp
        amp *= 0.5
    return total / norm

# Rising air carries the noise upwards: sample it lower down at the top, and swirl it.
warp = 0.35
wx = (fbm(x, y * 0.7, z, 2, 3, 100) - 0.5) * warp * (0.3 + y)
wz = (fbm(x, y * 0.7, z, 2, 3, 200) - 0.5) * warp * (0.3 + y)
# The plume's axis bends downwind (+x) as it rises; it widens linearly, as a turbulent plume does.
cx = 0.12 * y ** 1.6 - 0.45
r = 0.12 + 0.17 * y
dist = np.sqrt((x + wx - cx) ** 2 + (z + wz) ** 2)
billow = fbm(x + wx, y - 0.3 * y ** 1.2, z + wz, 3, 4, 300)
edge = np.clip((r * (0.55 + 0.9 * billow) - dist) / (0.35 * r), 0, 1)
# Thinning as it spreads, and fading out near the top of the box.
density = edge * (0.4 + 0.8 * billow) / (1 + 0.6 * y) * np.clip((3.4 - y) / 0.6, 0, 1)
density *= np.clip(y / 0.15, 0, 1)
# The fire: hot at the base of the axis, cooling as it rises and mixes.
flame = fbm(x, y * 1.6 - 0.2, z, 4, 3, 400)
core = np.exp(-(((x + wx * 0.5 - cx) ** 2 + (z + wz * 0.5) ** 2) / (0.13 + 0.05 * y) ** 2))
temperature = 2000 * core * np.exp(-y / 0.55) * (0.6 + 0.6 * flame)
temperature = np.clip(temperature, 0, 2000)
# The fire burns in its own gases: density where it is hot too.
density = np.maximum(density, 0.6 * core * np.exp(-y / 0.5))

def write(name, values, scale):
    v = np.clip(values / scale * 255 + 0.5, 0, 255).astype(np.uint8)
    with open(out + name, 'wb') as f:
        f.write(b'VOL' + bytes([3]))
        f.write(struct.pack('<5i', 3, NX, NY, NZ, 1))
        f.write(struct.pack('<6f', *LO, *HI))
        f.write(v.tobytes(order='C'))   # indexed [z][y][x]: x fastest

write('plume.vol', density, density.max())
write('fire.vol', temperature, 2000)
print('plume: max density', density.max(), 'filled', (density > 0.01).mean())
print('fire: max temperature', temperature.max())
