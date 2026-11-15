#!/usr/bin/env python3
# Generate the demo textures in demo/textures/: bricks (albedo, sRGB), their tangent-space normal
# map (from the bricks height field), a dirt mask and a fine checkerboard. Needs numpy and Pillow.
# Usage: scripts/make-demo-textures.py
import numpy as np
from PIL import Image
rng = np.random.default_rng(7)
N = 256
import os
out = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'demo', 'textures') + '/'
# Bricks: 8 rows of 4 bricks, every other row offset by half a brick; mortar 3% of a row.
y, x = np.mgrid[0:N, 0:N] / N
row = np.floor(y * 8)
xs = x * 4 + 0.5 * (row % 2)
col = np.floor(xs)
fx, fy = xs - col, y * 8 - row
mortar_w = 0.06
edge = np.minimum.reduce([fx / (mortar_w * 2), (1 - fx) / (mortar_w * 2), fy / mortar_w, (1 - fy) / mortar_w])
height = np.clip(edge, 0, 1)                       # 0 in the mortar, rising to 1 on the brick face
height = height ** 0.5
# Per-brick colour, and fine noise.
ids = (row * 16 + col).astype(int)
tint = rng.uniform(0.75, 1.15, size=ids.max() + 1)[ids]
noise = rng.normal(0, 0.04, size=(N, N))
brick = np.stack([0.55 * tint, 0.22 * tint, 0.14 * tint], -1) + noise[..., None]
mortar = np.array([0.55, 0.53, 0.5]) + noise[..., None] * 0.5
albedo = np.where((height > 0.02)[..., None], brick, mortar)
albedo = np.clip(albedo, 0, 1)
srgb = np.where(albedo <= 0.0031308, 12.92 * albedo, 1.055 * albedo ** (1 / 2.4) - 0.055)
Image.fromarray((srgb * 255 + 0.5).astype(np.uint8)).save(out + 'bricks.png')
# Normal map from the height field (with a little surface roughness): x along u (right), y along v
# (up the image, against the rows).
h = height * 0.6 + rng.normal(0, 0.02, size=(N, N))
dhdu = np.gradient(h, axis=1) * N / 20
dhdv = -np.gradient(h, axis=0) * N / 20
n = np.stack([-dhdu, -dhdv, np.ones_like(h)], -1)
n /= np.linalg.norm(n, axis=-1, keepdims=True)
Image.fromarray(((n * 0.5 + 0.5) * 255 + 0.5).astype(np.uint8)).save(out + 'bricks-normal.png')
# A dirt mask: smooth blotches.
m = rng.normal(size=(64, 64))
mask = np.array(Image.fromarray(((m - m.min()) / (m.max() - m.min()) * 255).astype(np.uint8)).resize((N, N), Image.BICUBIC)) / 255
mask = np.clip((mask - 0.45) * 4, 0, 1)
Image.fromarray((mask * 255).astype(np.uint8)).convert('RGB').save(out + 'dirt.png')
# A fine checkerboard, 64 x 64 squares.
c = ((np.floor(x * 64) + np.floor(y * 64)) % 2)
c = c[..., None].repeat(3, -1)
Image.fromarray((c * 255).astype(np.uint8)).save(out + 'checker.png')
# For the lights demo: a slide for the projector and the light panel (a stained-glass rose of
# coloured cells in lead), and a goniometric diagram of a lamp under a shade (equirectangular,
# top row up: a bright cone downwards, a soft glow through the shade, dark above it).
S = 256
yy, xx = np.mgrid[0:S, 0:S] / S - 0.5
r = np.hypot(xx, yy)
a = np.arctan2(yy, xx)
petal = np.cos(a * 8) * 0.5 + 0.5
ring = np.floor(r * 10)
cell = (ring * 7 + np.floor((a + np.pi) / (2 * np.pi) * 16)).astype(int)
palette = np.array([[0.9, 0.15, 0.1], [0.1, 0.3, 0.9], [0.95, 0.75, 0.1], [0.1, 0.7, 0.3], [0.6, 0.1, 0.7], [0.9, 0.45, 0.1]])
glass = palette[cell % len(palette)] * (0.55 + 0.45 * petal[..., None])
lead = (np.abs((r * 10) % 1 - 0.5) > 0.44) | (np.abs(((a + np.pi) / (2 * np.pi) * 16) % 1 - 0.5) > 0.46)
glass[lead] = 0.02
glass[r > 0.48] = 0
srgb = np.where(glass <= 0.0031308, 12.92 * glass, 1.055 * glass ** (1 / 2.4) - 0.055)
Image.fromarray((np.clip(srgb, 0, 1) * 255 + 0.5).astype(np.uint8)).save(out + 'rose-window.png')
W, H = 128, 64
theta = (np.arange(H) + 0.5) / H * np.pi          # from up
phi = (np.arange(W) + 0.5) / W * 2 * np.pi
T, P = np.meshgrid(theta, phi, indexing='ij')
down = np.clip((T - np.radians(125)) / np.radians(20), 0, 1) ** 2          # the open bottom
shade = 0.12 * np.exp(-((T - np.radians(95)) / np.radians(20)) ** 2) * (1 + 0.3 * np.cos(P * 6))
gonio = np.clip(down + shade, 0, 1)
g8 = np.where(gonio <= 0.0031308, 12.92 * gonio, 1.055 * gonio ** (1 / 2.4) - 0.055)
Image.fromarray((g8 * 255 + 0.5).astype(np.uint8)).convert('RGB').save(out + 'lampshade.png')
