#!/usr/bin/env python3
"""Fit a library material's parameters to a scanned reference, by rendering and comparing.

The reference is a scanned surface's albedo map (eg. an ambientCG or Poly Haven "Color" map,
which are delit) and its physical size in metres. The candidate is the preset rendered the same
way: a plane seen straight down by an orthographic camera covering the same size, under a
uniform white sky with no sun, so a lambertian surface renders as its albedo exactly (and a
bump changes nothing). Each candidate is written to a library file the renderer reads at run
time (a scene's "library"), so no rebuild is needed.

The loss compares the distributions of small patches of the two images at several scales,
not their pixels (procedural noise never lines up with a photograph): a sliced Wasserstein
distance between the sets of 5x5 patches of each level of a Gaussian pyramid, in sRGB-encoded
values so darks count as the eye counts them. After Heitz, Vanhoey, Chambon and Belcour, "A
sliced Wasserstein loss for neural texture synthesis" (CVPR 2021), and Elnekave and Weiss,
"Generating natural images with direct patch distributions matching" (ECCV 2022), on raw
patches rather than a network's features. The projections are fixed, and each render is
deterministic, so the objective is too.

The mean colour is compared besides, along the opponent axes, the hues weighed most (`mean_colour`). Colour is compared besides, and more strictly: the patches' random projections mostly see
lightness, so an optimiser left to them alone will happily trade grey stones for sub-pixel
confetti of saturated colours averaging grey. Each level's pixels are also compared as colour
distributions along fixed opponent axes (lightness, red-green, blue-yellow) and random
directions in that space. And candidates are rendered at twice the resolution and averaged
down in linear light, as the reference was, so detail finer than a pixel averages out rather
than showing as sampling noise.

That loss sees colour, and little else: scripts/loss-benchmark.py, which tests a loss on
pairs of scan crops whose answer is known, found it scored a scan's other half no nearer than
a different scan coloured like it (a third of the time), barely told a scan from its own
phase-randomised blobs, and was least at the right scale for an eighth of the scans. So
structure is compared besides (`StructureLoss`, on lightness normalised to mean 0 and
deviation 1): the joint distribution of an oriented filter bank's responses with the next
level's (after Portilla and Simoncelli), which tells shapes from blobs, and the radially
averaged power spectrum, which tells sizes, and so holds the scale.

Its colour term had blind spots too: a third less contrast, or the right colours in the wrong
places (brown on the stones and grey on the leaves, every histogram the same), scored like a
quadrant of the same scan. So colour is compared in Oklab (`ColourLoss`), where distances are
about as different as they look: the mean colours, and the colours' distribution about their
mean, each axis over the reference's deviation along it so chroma counts as much as lightness,
jointly with each pixel's neighbourhood's colour, which sees where colours are. `FitLoss` is
colour and structure together, each weighed to be about 1 between two different scans;
`--no-structure` falls back to the patch loss alone, as fits before these were.

And a fit sees the scan twice: whole, at --resolution pixels, and in detail, a quarter of it
across at the same pixels (four times as fine), compared with four tiles of the scan, one in
each quadrant (`DETAIL_TILES`). At 128 pixels across a two-metre scan a pebble is a few
pixels and a leaf one or two, too few for any loss to see their shapes; in detail they're
there (the benchmark's structure and identity scores rise threefold at that sampling). The
detail view doubles the renders; `--no-detail` leaves it out.

The parameters are found by CMA-ES (Hansen, "The CMA evolution strategy: a tutorial", 2016),
which needs no gradients: the renderer isn't differentiable.

A fit is described by a reference in the preset's own entry in src/material/library.json,
which the renderer ignores (it reads only a preset's `material`, `variables`, `seed_direction`
and `overlay`):

    "scree": { ..., "references": [
       {"source": "ambientcg:Gravel043", "licence": "CC0", "size": 1.6, "usage": {"scale": 0.2},
        "params": [...],
        "fit": {"loss": {"start": ..., "fitted": ...}, "values": {...}, "applied": false, ...}} ] }

A reference's `"view": "wall"` renders it on a vertical plane facing the camera, for scans of
cliffs and walls (taken facing them, their up the scene's up); by default it is ground, seen
from above.

Fit it with `scripts/fit-material.py scree` (its first reference; `scree:1` the second): the
scan is fetched into references/materials/ if it isn't there (fetch-material-reference.py), and
`--record` writes the result into the reference's `fit`: the losses, the settings, and the
fitted values by parameter (a field's name, or the path joined with dots); `applied` says
whether they were copied into the preset, which is left to a person who has looked at
compare.png. Or give a spec file instead of a preset's name:

    {"preset": "scree", "name": "gravel043",
     "reference": "path/to/Color.jpg", "size": 1.6,
     "params": [
        {"field": "scale", "min": 0.03, "max": 0.6, "log": true},
        {"path": ["variables", "palette", 0, 1], "kind": "color"},
        {"path": ["material", "albedo", "color_map", 1, 0], "min": 0, "max": 0.05}
     ]}

`field` is a field of the library material that uses the preset (its scale, or a variable
of the preset by name); `path` a place in the preset's JSON, a number or (`"kind": "color"`)
an [r, g, b]. Each is searched between `min` and `max`, on a log scale if `log`. A colour is
searched as a lightness (the mean of its channels) and a tint along the opponent axes,
red-green and blue-yellow, each within `chroma` (default 0.12, relative to the lightness) of
the preset's own tint. Its lightness stays between `min` and `max`, by default half and 1.6
times the preset's own. So a colour keeps the part its author gave it (the pale stones stay
pale and cream): left freer, an optimiser swaps colours' roles and wanders into pink pebbles
and green gaps that match the statistics with a picture that looks nothing like the scan. A
fit refines a preset's colours; it doesn't choose them.

Better still, as a search's cost grows with its dimensions (CMA-ES wants of the order of a
hundred evaluations per dimension), a preset's colours can be fitted together:
`{"kind": "colours", "paths": [...]}` searches four numbers for all of them, as for one colour:
a gain on their lightness (`gain`, default 0.5 to 2), shifts of their tints along red-green and
blue-yellow (within `chroma`, default 0.12, of the author's), and a contrast about their mean
lightness (`contrast`, default 0.5 to 2), keeping the colours' relations as their author gave
them. (Gains on red, green and blue apiece let a group's hue wander anywhere: grey lichen
turned blue, ochre turned pink.) Give colours that cover little of the surface a smaller
`chroma`: their hue hardly moves the loss, so it drifts (trail_dirt's stones, a tenth of its
area, turned mint green at 0.12).

    scripts/fit-material.py scree|spec.json [--iterations 30] [--resolution 160] [--samples 8] [--record] [-o out_dir]

Writes to out_dir: fitted.json (a library file with the fitted preset under `name`, and the
library material using it), compare.png (reference, start, fitted; a second row for the
detail view, against the scan's first tile), and log.csv.
"""
import argparse, copy, csv, json, math, os, subprocess, sys, tempfile, time

import numpy as np
from PIL import Image

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def srgb_to_linear(c):
    return np.where(c <= 0.04045, c / 12.92, ((c + 0.055) / 1.055) ** 2.4)


def load_reference(path, resolution, crop=None):
    """The reference albedo, sRGB-encoded in [0, 1], resampled to resolution square (averaged
    in linear light, as the render's pixels average it); or a crop of it, (x, y, width) as
    fractions of its width."""
    im = np.asarray(Image.open(path).convert("RGB"), dtype=np.float64) / 255.
    lin = srgb_to_linear(im)
    if crop:
        n = min(lin.shape[:2])
        x, y, w = (int(round(c * n)) for c in crop)
        lin = lin[y:y + w, x:x + w]
    channels = [Image.fromarray(lin[:, :, k].astype(np.float32), mode="F").resize((resolution, resolution), Image.BOX)
                for k in range(3)]
    lin = np.stack([np.asarray(c, dtype=np.float64) for c in channels], axis=2)
    return linear_to_srgb(lin)


# The detail view: a quarter of the scan across, compared with these tiles of it (x, y and
# width as fractions of the scan's width), one in each quadrant, so no one tile's lighting or
# odd stone decides the fit.
DETAIL = 0.25
DETAIL_TILES = [(0.125, 0.125, DETAIL), (0.625, 0.125, DETAIL), (0.125, 0.625, DETAIL), (0.625, 0.625, DETAIL)]


def linear_to_srgb(c):
    c = np.clip(c, 0, None)
    return np.where(c <= 0.0031308, c * 12.92, 1.055 * c ** (1 / 2.4) - 0.055)


def oklab(srgb):
    """sRGB-encoded colours to Oklab (Ottosson, 2020): lightness L and the opponent axes a
    (green-red) and b (blue-yellow), scaled so equal distances look about equally different;
    0.02 is about as small a difference as one notices."""
    lin = srgb_to_linear(np.clip(srgb, 0, 1))
    lms = lin @ np.array([[0.4122214708, 0.5363325363, 0.0514459929],
                          [0.2119034982, 0.6806995451, 0.1073969566],
                          [0.0883024619, 0.2817188376, 0.6299787005]]).T
    lms = np.cbrt(lms)
    return lms @ np.array([[0.2104542553, 0.7936177850, -0.0040720468],
                           [1.9779984951, -2.4285922050, 0.4505937099],
                           [0.0259040371, 0.7827717662, -0.8086757660]]).T


def from_oklab(lab):
    lms = lab @ np.array([[1., 0.3963377774, 0.2158037573],
                          [1., -0.1055613458, -0.0638541728],
                          [1., -0.0894841775, -1.2914855480]]).T
    lin = (lms ** 3) @ np.array([[4.0767416621, -3.3077115913, 0.2309699292],
                                 [-1.2684380046, 2.6097574011, -0.3413193965],
                                 [-0.0041960863, -0.7034186147, 1.7076147010]]).T
    return linear_to_srgb(lin)


# Opponent colour axes: lightness, red-green, blue-yellow.
OPPONENT = np.array([[1, 1, 1], [1, -1, 0], [0.5, 0.5, -1]], dtype=np.float64).T / np.array([3 ** 0.5, 2 ** 0.5, 1.5 ** 0.5])


class PatchLoss:
    """Multi-scale sliced Wasserstein distance between 5x5 patch distributions, and between
    colour distributions along opponent axes."""

    def __init__(self, levels=4, patch=5, projections=64, samples=4096, seed=1, colour_weight=4.):
        self.levels, self.patch, self.samples = levels, patch, samples
        rng = np.random.default_rng(seed)
        d = patch * patch * 3
        p = rng.normal(size=(d, projections))
        self.projections = p / np.linalg.norm(p, axis=0)
        self.rng_seed = seed
        self.colour_weight = colour_weight
        c = rng.normal(size=(3, 29))
        self.colour_axes = np.concatenate([OPPONENT, c / np.linalg.norm(c, axis=0)], axis=1)

    def colours(self, a, b, rng):
        pa, pb = a.reshape(-1, 3), b.reshape(-1, 3)
        n = min(len(pa), len(pb), 4 * self.samples)
        pa, pb = pa[rng.permutation(len(pa))[:n]], pb[rng.permutation(len(pb))[:n]]
        qa, qb = np.sort(pa @ self.colour_axes, axis=0), np.sort(pb @ self.colour_axes, axis=0)
        # The chromatic axes weigh as much as all the random ones.
        w = np.concatenate([[1., 4., 4.], np.ones(self.colour_axes.shape[1] - 3) * 0.2])
        return float(np.mean(((qa - qb) ** 2) * w))

    @staticmethod
    def pyramid(img, levels):
        out = [img]
        for _ in range(levels - 1):
            h, w = (out[-1].shape[0] // 2) * 2, (out[-1].shape[1] // 2) * 2
            a = out[-1][:h, :w]
            out.append(0.25 * (a[0::2, 0::2] + a[1::2, 0::2] + a[0::2, 1::2] + a[1::2, 1::2]))
        return out

    def patches(self, img, rng):
        p = self.patch
        h, w = img.shape[0] - p + 1, img.shape[1] - p + 1
        n = min(self.samples, h * w)
        ys, xs = rng.integers(0, h, n), rng.integers(0, w, n)
        idx = np.stack([img[ys + dy, xs + dx] for dy in range(p) for dx in range(p)], axis=1)
        return idx.reshape(n, -1)

    def mean_colour(self, a, b):
        """The difference of the two images' mean colours along the opponent axes, the hues
        weighed most: the eye judges a surface's hue by its mean before anything else, and the
        distributions above barely see a small shift of it (tan becomes pink at 0.03)."""
        d = (a.reshape(-1, 3).mean(0) - b.reshape(-1, 3).mean(0)) @ OPPONENT
        return float(np.sum(np.array([5., 30., 30.]) * d * d))

    def __call__(self, a, b):
        total = self.levels * self.mean_colour(a, b)
        for k, (la, lb) in enumerate(zip(self.pyramid(a, self.levels), self.pyramid(b, self.levels))):
            rng = np.random.default_rng(self.rng_seed + k)
            pa, pb = self.patches(la, rng), self.patches(lb, rng)
            n = min(len(pa), len(pb))
            qa = np.sort(pa[:n] @ self.projections, axis=0)
            qb = np.sort(pb[:n] @ self.projections, axis=0)
            total += float(np.mean((qa - qb) ** 2)) + self.colour_weight * self.colours(la, lb, rng)
        return total / self.levels


def luminance(img):
    """Lightness along the first opponent axis, normalised to mean 0 and deviation 1: what's
    left is structure, its contrast and colour being the colour terms' business."""
    l = img @ OPPONENT[:, 0]
    return (l - l.mean()) / (l.std() + 1e-6)


class StructureLoss:
    """What things look like, not their colour: the shapes, sizes and orientations of the
    pattern, compared on lightness normalised to mean 0 and deviation 1 (so neither a scan's
    uneven lighting nor a candidate's contrast passes for structure). Three parts:

    - `patch`: the sliced Wasserstein distance between contrast-normalised 7x7 patches (each
      patch less its own mean, over the image's deviation) at each level of a pyramid: edges,
      corners and textures in a neighbourhood, wherever they are, whatever their brightness.
    - `bank`: an oriented filter bank's responses, jointly with the coarser level's: at each
      level, a pixel's complex responses to four orientations of a log-Gabor filter an octave
      wide about a quarter of the sampling rate (even and odd parts: lines and edges), with the
      magnitudes of the next level's at the same place. The joint distribution of these is a
      texture's signature in Portilla and Simoncelli's sense ("A parametric texture model based
      on joint statistics of complex wavelet coefficients", IJCV 2000): which orientations
      occur together, and how detail sits within larger forms, tell a leaf from a blob of the
      same size; compared by sliced Wasserstein distance, as Heitz et al. compare a network's
      features, without the network.
    - `spectrum`: the radially averaged power spectrum in half-octave bands, as log power: the
      sizes things come in. It moves along the frequency axis when the material is scaled, so
      it is what holds a fit's scale.
    """

    # Each part's weight makes it 1 for two different scans (the median over the scans that
    # the library's presets name, at 256 pixels a quadrant; scripts/loss-benchmark.py).
    WEIGHTS = {"patch": 50., "bank": 360., "spectrum": 2.4}

    def __init__(self, parts=("bank", "spectrum"), levels=4, samples=4096, projections=64, seed=3):
        self.parts, self.levels, self.samples, self.seed = parts, levels, samples, seed
        self.weights = self.WEIGHTS
        rng = np.random.default_rng(seed)
        self.proj = {}
        for name, d in (("patch", 49), ("bank", 12)):
            p = rng.normal(size=(d, projections))
            self.proj[name] = p / np.linalg.norm(p, axis=0)

    @staticmethod
    def swd(fa, fb, proj):
        n = min(len(fa), len(fb))
        qa, qb = np.sort(fa[:n] @ proj, axis=0), np.sort(fb[:n] @ proj, axis=0)
        return float(np.mean((qa - qb) ** 2))

    def patches(self, l, rng):
        p = 7
        h, w = l.shape[0] - p + 1, l.shape[1] - p + 1
        n = min(self.samples, h * w)
        ys, xs = rng.integers(0, h, n), rng.integers(0, w, n)
        x = np.stack([l[ys + dy, xs + dx] for dy in range(p) for dx in range(p)], axis=1)
        return x - x.mean(axis=1, keepdims=True)

    @staticmethod
    def bank(l):
        """Complex responses to four orientations of a log-Gabor filter, (h, w, 4)."""
        h, w = l.shape
        fy, fx = np.meshgrid(np.fft.fftfreq(h), np.fft.fftfreq(w), indexing="ij")
        r = np.hypot(fx, fy) + 1e-9
        radial = np.exp(-np.log(r / 0.25) ** 2 / (2 * (0.55 * np.log(2)) ** 2))
        radial[0, 0] = 0.
        phi = np.arctan2(fy, fx)
        f = np.fft.fft2(l)
        out = []
        for k in range(4):
            d = np.cos(phi - k * np.pi / 4)
            angular = np.where(d > 0, d ** 4, 0.) * 2.
            out.append(np.fft.ifft2(f * radial * angular))
        return np.stack(out, axis=2)

    def bank_features(self, levels, rng):
        feats = []
        for k in range(len(levels) - 1):
            fine, coarse = self.bank(levels[k]), np.abs(self.bank(levels[k + 1]))
            h, w = fine.shape[:2]
            coarse = np.repeat(np.repeat(coarse, 2, axis=0), 2, axis=1)[:h, :w]
            n = min(self.samples, h * w)
            idx = rng.integers(0, h * w, n)
            f = np.concatenate([fine.real.reshape(-1, 4), fine.imag.reshape(-1, 4), coarse.reshape(-1, 4)], axis=1)[idx]
            feats.append(f)
        return feats

    @staticmethod
    def spectrum(l):
        h, w = l.shape
        p = np.abs(np.fft.fft2(l)) ** 2 / (h * w)
        fy, fx = np.meshgrid(np.fft.fftfreq(h), np.fft.fftfreq(w), indexing="ij")
        r = np.hypot(fx, fy)
        # Half-octave bands from four cycles across the image to the sampling limit.
        edges = 0.5 * 2. ** -np.arange(0., np.log2(min(h, w) / 8.) + 0.25, 0.5)[::-1]
        bands = [p[(r >= lo) & (r < hi)].mean() for lo, hi in zip(edges[:-1], edges[1:])]
        return np.log(np.array(bands) + 1e-8)

    def parts_of(self, a, b):
        la, lb = luminance(a), luminance(b)
        pa, pb = PatchLoss.pyramid(la, self.levels), PatchLoss.pyramid(lb, self.levels)
        out = {}
        if "patch" in self.parts:
            t = 0.
            for k, (x, y) in enumerate(zip(pa, pb)):
                rng = np.random.default_rng(self.seed + k)
                t += self.swd(self.patches(x, rng), self.patches(y, rng), self.proj["patch"])
            out["patch"] = t / self.levels
        if "bank" in self.parts:
            fa, fb = self.bank_features(pa, np.random.default_rng(self.seed)), self.bank_features(pb, np.random.default_rng(self.seed))
            out["bank"] = sum(self.swd(x, y, self.proj["bank"]) for x, y in zip(fa, fb)) / len(fa)
        if "spectrum" in self.parts:
            out["spectrum"] = float(np.mean((self.spectrum(la) - self.spectrum(lb)) ** 2))
        return out

    def __call__(self, a, b):
        return sum(self.weights[k] * v for k, v in self.parts_of(a, b).items())


class ColourLoss:
    """Colour compared in Oklab, where distances are about as different as they look, in
    three parts, each of which the patch loss's colour term was nearly blind to one way
    (scripts/loss-benchmark.py):

    - `mean`: the squared distance between the images' mean colours.
    - `spread`: the colours' distribution about its mean, at each pyramid level: the sliced
      Wasserstein distance between the centred colours, each axis over the reference's own
      deviation along it, so a material's chroma counts as much as its lightness and a
      contrast flattened or boosted shows (it didn't: a third less contrast scored as a
      quadrant of the same scan).
    - `binding`: which colours go where: each pixel's colour with its neighbourhood's (the
      next pyramid level's, at the same place), centred and scaled as above, compared jointly.
      Brown leaves on grey stones and grey leaves on brown stones have the same histograms;
      a pixel's colour beside its surroundings' tells them apart.
    """

    WEIGHTS = {"mean": 50., "spread": 3., "binding": 1.}

    def __init__(self, parts=("mean", "spread", "binding"), levels=4, samples=8192, projections=48, seed=5):
        self.parts, self.levels, self.samples, self.seed = parts, levels, samples, seed
        rng = np.random.default_rng(seed)
        self.proj = {}
        for name, d in (("spread", 3), ("binding", 6)):
            p = rng.normal(size=(d, projections))
            self.proj[name] = p / np.linalg.norm(p, axis=0)

    def parts_of(self, a, b):
        la, lb = oklab(a), oklab(b)
        out = {}
        if "mean" in self.parts:
            out["mean"] = float(np.sum((la.reshape(-1, 3).mean(0) - lb.reshape(-1, 3).mean(0)) ** 2))
        pa, pb = PatchLoss.pyramid(la, self.levels), PatchLoss.pyramid(lb, self.levels)
        spread = binding = 0.
        for k in range(self.levels):
            x, y = pa[k], pb[k]
            mx, my = x.reshape(-1, 3).mean(0), y.reshape(-1, 3).mean(0)
            scale = x.reshape(-1, 3).std(0) + 0.02
            rng = np.random.default_rng(self.seed + k)
            fx, fy = ((x - mx) / scale).reshape(-1, 3), ((y - my) / scale).reshape(-1, 3)
            n = min(self.samples, len(fx))
            ix, iy = rng.permutation(len(fx))[:n], rng.permutation(len(fy))[:n]
            if "spread" in self.parts:
                spread += StructureLoss.swd(fx[ix], fy[iy], self.proj["spread"])
            if "binding" in self.parts and k + 1 < self.levels:
                def joint(img, m, f):
                    c = np.repeat(np.repeat(img, 2, axis=0), 2, axis=1)
                    h, w = (img.shape[0] * 2, img.shape[1] * 2)
                    c = ((c - m) / scale).reshape(-1, 3)
                    return np.concatenate([f.reshape(h, w, 3)[:h, :w].reshape(-1, 3), c], axis=1)
                h2, w2 = pa[k + 1].shape[0] * 2, pa[k + 1].shape[1] * 2
                gx = joint(pa[k + 1], mx, fx.reshape(x.shape)[:h2, :w2])
                gy = joint(pb[k + 1], my, fy.reshape(y.shape)[:h2, :w2])
                n = min(self.samples, len(gx))
                binding += StructureLoss.swd(gx[rng.permutation(len(gx))[:n]], gy[rng.permutation(len(gy))[:n]],
                                             self.proj["binding"])
        if "spread" in self.parts:
            out["spread"] = spread / self.levels
        if "binding" in self.parts:
            out["binding"] = binding / (self.levels - 1)
        return out

    def __call__(self, a, b):
        return sum(self.WEIGHTS[k] * v for k, v in self.parts_of(a, b).items())


class FitLoss:
    """What a fit minimises: colour (`ColourLoss`) and structure (`StructureLoss`), and
    optionally the older `PatchLoss` (`patch`), each weighed to be about 1 between two
    different scans (the median over the scans the presets name, a quadrant each at 128
    pixels), so each counts about as much as the others."""

    WEIGHTS = {"colour": 1., "structure": 0.5, "patch": 3.6}

    def __init__(self, parts=("colour", "structure")):
        self.losses = {k: {"colour": ColourLoss, "structure": StructureLoss, "patch": PatchLoss}[k]() for k in parts}

    def parts_of(self, a, b):
        return {k: self.WEIGHTS[k] * loss(a, b) for k, loss in self.losses.items()}

    def __call__(self, a, b):
        return sum(self.parts_of(a, b).values())


def get(obj, path):
    for k in path:
        obj = obj[k]
    return obj


def put(obj, path, value):
    for k in path[:-1]:
        obj = obj[k]
    obj[path[-1]] = value


class Problem:
    """The spec's parameters as a vector in [0, 1]^n, and the material they make."""

    def __init__(self, spec, preset):
        self.spec, self.preset = spec, preset
        self.slots = []  # (param, component or None)
        for p in spec["params"]:
            if p.get("kind") == "color":
                self.slots += [(p, c) for c in range(3)]
            elif p.get("kind") == "colours":
                self.slots += [(p, ("group", c)) for c in range(4)]
            else:
                self.slots.append((p, None))

    def group_bounds(self, p, c):
        """The group's numbers: a gain on lightness, two tint shifts, a contrast."""
        if c in (1, 2):
            chroma = p.get("chroma", 0.12)
            return -chroma, chroma
        return p.get("gain", [0.5, 2.]) if c == 0 else p.get("contrast", [0.5, 2.])

    def group_value(self, p, c, u):
        lo, hi = self.group_bounds(p, c)
        u = min(max(u, 0.), 1.)
        if c in (1, 2):
            return lo + u * (hi - lo)
        return math.exp(math.log(lo) + u * (math.log(hi) - math.log(lo)))

    def apply_group(self, preset, p, values):
        """The gain, tint shifts and contrast applied to every colour of the group, from the
        preset's own."""
        gain, shift_a, shift_b, contrast = values
        own = [get(self.preset, path) for path in p["paths"]]
        mean = sum(sum(c) / 3. for c in own) / len(own)
        for path, colour in zip(p["paths"], own):
            light, a, b = self.tints(colour)
            light = gain * (mean + contrast * (light - mean))
            a, b = a + shift_a, b + shift_b
            rgb = [light * (1. + a + 0.5 * b), light * (1. - a + 0.5 * b), light * (1. - b)]
            put(preset, path, [round(max(v, 0.), 4) for v in rgb])

    def bounds(self, p):
        if p.get("kind") == "color":
            # By default near the preset's own lightness, so the colour keeps its role.
            own = sum(get(self.preset, p["path"])) / 3.
            return (p.get("min", 0.5 * own), p.get("max", min(1., 1.6 * own + 0.02)))
        return (p["min"], p["max"])

    @staticmethod
    def tints(rgb):
        """A colour's lightness, and its tints along red-green and blue-yellow, relative to it."""
        light = sum(rgb) / 3.
        if light <= 0:
            return 0., 0., 0.
        t = [c / light - 1. for c in rgb]
        return light, (t[0] - t[1]) / 2., (t[0] + t[1] - 2. * t[2]) / 3.

    def colour_to_unit(self, p, rgb, lo, hi):
        """A colour as lightness and two tints in the unit cube (`colour_from_unit` reversed)."""
        _, a0, b0 = self.tints(get(self.preset, p["path"]))
        light, a, b = self.tints(rgb)
        chroma = p.get("chroma", 0.12)
        unit = lambda v, v0: ((v - v0) / chroma + 1.) / 2.
        return [(light - lo) / max(hi - lo, 1e-9), unit(a, a0), unit(b, b0)]

    def colour_from_unit(self, p, u, lo, hi):
        """Lightness, then tints along red-green and blue-yellow about the preset's own (both
        keeping the mean)."""
        _, a0, b0 = self.tints(get(self.preset, p["path"]))
        light = lo + min(max(u[0], 0.), 1.) * (hi - lo)
        chroma = p.get("chroma", 0.12)
        a, b = [v0 + chroma * (2. * min(max(t, 0.), 1.) - 1.) for t, v0 in zip(u[1:], (a0, b0))]
        return [max(light * (1. + a + 0.5 * b), 0.), max(light * (1. - a + 0.5 * b), 0.), max(light * (1. - b), 0.)]

    def to_unit(self, p, v):
        lo, hi = self.bounds(p)
        if p.get("log"):
            return (math.log(v) - math.log(lo)) / (math.log(hi) - math.log(lo))
        return (v - lo) / (hi - lo)

    def from_unit(self, p, u):
        lo, hi = self.bounds(p)
        u = min(max(u, 0.), 1.)
        if p.get("log"):
            return math.exp(math.log(lo) + u * (math.log(hi) - math.log(lo)))
        return lo + u * (hi - lo)

    def start(self, usage):
        x = []
        for p, c in self.slots:
            if isinstance(c, tuple):
                lo, hi = self.group_bounds(p, c[1])
                x.append(0.5 if c[1] in (1, 2) else (0. - math.log(lo)) / (math.log(hi) - math.log(lo)))
                continue
            if "field" in p:
                v = usage.get(p["field"], p.get("default", 1.))
            elif c is None:
                try:
                    v = get(self.preset, p["path"])
                except (KeyError, IndexError):
                    v = p["default"]
            else:
                if c == 0:
                    lo, hi = self.bounds(p)
                    colour_units = self.colour_to_unit(p, get(self.preset, p["path"]), lo, hi)
                x.append(colour_units[c])
                continue
            x.append(self.to_unit(p, v))
        return np.clip(np.array(x), 0., 1.)

    def make(self, x, usage):
        """The preset and the library material using it, at x."""
        preset, usage = copy.deepcopy(self.preset), dict(usage)
        k = 0
        while k < len(self.slots):
            p, c = self.slots[k]
            if isinstance(c, tuple):
                self.apply_group(preset, p, [self.group_value(p, j, float(x[k + j])) for j in range(4)])
                k += 4
                continue
            if c is not None:
                lo, hi = self.bounds(p)
                put(preset, p["path"], [round(v, 4) for v in self.colour_from_unit(p, [float(u) for u in x[k:k + 3]], lo, hi)])
                k += 3
                continue
            v = self.from_unit(p, float(x[k]))
            if "field" in p:
                usage[p["field"]] = v
            else:
                put(preset, p["path"], v)
            k += 1
        return preset, usage


def scene(usage, library_path, size, resolution, samples, seed, view="ground"):
    # A scan of a cliff was taken facing the cliff, its up the scene's up: seen as a wall (the
    # plane z = 0, from in front), so a preset's beds and streaks along y line up with it.
    if view == "wall":
        camera = {"type": "orthographic", "location": [0, 0, -5], "lookat": [0, 0, 0], "up": [0, 1, 0], "height": size}
        ground = {"type": "plane", "normal": [0, 0, -1], "distance": 0, "material": "M"}
    else:
        camera = {"type": "orthographic", "location": [0, 5, 0], "lookat": [0, 0, 0], "up": [0, 0, 1], "height": size}
        ground = {"type": "plane", "y": 0, "material": "M"}
    return {
        "width": 2 * resolution, "height": 2 * resolution, "supersamples": samples, "max_depth": 2, "seed": seed,
        "background": [1, 1, 1], "output": {"tonemap": "none", "exposure": 1.0},
        "library": library_path,
        "camera": camera,
        "materials": {"M": usage},
        "objects": [ground],
    }


def render(preset, usage, name, args, size, tmp, tag):
    library_path = os.path.join(tmp, "library-%s.json" % tag)
    json.dump({name: preset}, open(library_path, "w"))
    path = os.path.join(tmp, "scene-%s.json" % tag)
    json.dump(scene(usage, library_path, size, args.resolution, args.samples, args.seed, args.view), open(path, "w"))
    out = os.path.join(tmp, "render-%s.png" % tag)
    subprocess.run([args.rays, path, "--fresh", "-o", out], check=True, cwd=ROOT,
                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    img = srgb_to_linear(np.asarray(Image.open(out).convert("RGB"), dtype=np.float64) / 255.)
    # Rendered at twice the size and averaged down in linear light, as the reference was.
    img = 0.25 * (img[0::2, 0::2] + img[1::2, 0::2] + img[0::2, 1::2] + img[1::2, 1::2])
    return linear_to_srgb(img)


def main():
    a = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    a.add_argument("spec", help="a preset with references in the library (name or name:index), or a spec file")
    a.add_argument("--record", action="store_true", help="write the fit into the preset's reference in library.json")
    a.add_argument("--iterations", type=int, default=30)
    a.add_argument("--resolution", type=int, default=160, help="pixels across (at least 64, for the loss's four levels)")
    a.add_argument("--samples", type=int, default=8)
    a.add_argument("--seed", type=int, default=1)
    a.add_argument("--sigma", type=float, default=0.2, help="CMA-ES's initial step, in the unit cube")
    a.add_argument("--rays", default=os.path.join(ROOT, "target/release/rays"))
    a.add_argument("-o", "--output", default="fit")
    a.add_argument("--no-structure", dest="structure", action="store_false",
                   help="the colour loss alone (PatchLoss), as fits before the structure term were")
    a.add_argument("--no-detail", dest="detail", action="store_false",
                   help="the whole scan only, without the detail view (half the renders)")
    args = a.parse_args()
    import cma

    library_path = os.path.join(ROOT, "src/material/library.json")
    library = json.load(open(library_path))
    reference_at = None
    if os.path.exists(args.spec):
        spec = json.load(open(args.spec))
        base = os.path.dirname(os.path.abspath(args.spec))
    else:
        preset_name, _, index = args.spec.partition(":")
        index = int(index or 0)
        refs = library.get(preset_name, {}).get("references") or sys.exit("no preset %s with references in the library" % preset_name)
        reference_at = (preset_name, index)
        spec = dict(refs[index], preset=preset_name)
        spec.setdefault("name", preset_name + "_fit")
        asset = spec["source"].partition(":")[2]
        base = os.path.join(ROOT, "references", "materials", asset)
        if not os.path.exists(os.path.join(base, "albedo.jpg")):
            subprocess.run([sys.executable, os.path.join(ROOT, "scripts", "fetch-material-reference.py"), spec["source"],
                            "--preset", preset_name] + (["--size", str(spec["size"])] if spec.get("size") else []), check=True)
        spec["reference"] = "albedo.jpg"
    if spec.get("library"):
        library.update(json.load(open(os.path.join(base, spec["library"]))))
    preset = library[spec["preset"]]
    name = spec.get("name", spec["preset"] + "_fit")
    usage = {"type": "library", "name": name, **spec.get("usage", {})}
    size = float(spec["size"])
    args.view = spec.get("view", "ground")
    reference = load_reference(os.path.join(base, spec["reference"]), args.resolution)
    details = [load_reference(os.path.join(base, spec["reference"]), args.resolution, t) for t in DETAIL_TILES] \
        if args.detail else []
    loss = FitLoss() if args.structure else PatchLoss()
    problem = Problem(spec, preset)
    os.makedirs(args.output, exist_ok=True)

    with tempfile.TemporaryDirectory() as tmp:
        def evaluate(x, tag):
            p, u = problem.make(x, usage)
            img = render(p, u, name, args, size, tmp, tag)
            total = loss(reference, img)
            if not details:
                return total, [img]
            # The detail view: as much again, its loss the mean over the scan's tiles.
            detail = render(p, u, name, args, size * DETAIL, tmp, tag + "d")
            return total + float(np.mean([loss(t, detail) for t in details])), [img, detail]

        x0 = problem.start(usage)
        start_loss, start_img = evaluate(x0, "start")
        print("start: loss %.5f" % start_loss, file=sys.stderr)
        es = cma.CMAEvolutionStrategy(list(x0), args.sigma, {"bounds": [0, 1], "seed": args.seed, "verbose": -9})
        best = (start_loss, x0, start_img)
        log = [("iteration", "evaluations", "best", "generation_best", "seconds")]
        t0 = time.time()
        for it in range(args.iterations):
            xs = es.ask()
            results = [evaluate(np.array(x), "c%d" % k) for k, x in enumerate(xs)]
            es.tell(xs, [r[0] for r in results])
            k = int(np.argmin([r[0] for r in results]))
            if results[k][0] < best[0]:
                best = (results[k][0], np.array(xs[k]), results[k][1])
            log.append((it + 1, es.countevals, best[0], results[k][0], round(time.time() - t0, 1)))
            print("iteration %d: best %.5f (this generation %.5f)" % (it + 1, best[0], results[k][0]), file=sys.stderr)

        fitted, fitted_usage = problem.make(best[1], usage)
        json.dump({"library": {name: fitted}, "material": fitted_usage, "loss": {"start": start_loss, "fitted": best[0]}},
                  open(os.path.join(args.output, "fitted.json"), "w"), indent=1)
        with open(os.path.join(args.output, "log.csv"), "w", newline="") as f:
            csv.writer(f).writerows(log)
        # A row a view: the scan (its first tile, for the detail), the start and the fit.
        r = args.resolution
        sheet = Image.new("RGB", (3 * r, r * len(start_img)))
        for row, ref in enumerate([reference] + details[:1]):
            for k, img in enumerate([ref, start_img[row], best[2][row]]):
                sheet.paste(Image.fromarray((np.clip(img, 0, 1) * 255).astype(np.uint8)), (k * r, row * r))
        sheet.save(os.path.join(args.output, "compare.png"))
        print("loss %.5f -> %.5f; wrote %s" % (start_loss, best[0], args.output), file=sys.stderr)
        if args.record:
            if reference_at is None:
                sys.exit("--record needs a preset's reference, not a spec file")
            values = {}
            for p in spec["params"]:
                if p.get("kind") == "colours":
                    for path in p["paths"]:
                        values[".".join(map(str, path))] = get(fitted, path)
                    continue
                if "field" in p:
                    values[p["field"]] = round(fitted_usage[p["field"]], 4)
                else:
                    v = get(fitted, p["path"])
                    values[".".join(map(str, p["path"]))] = [round(c, 4) for c in v] if isinstance(v, list) else round(v, 4)
            library = json.load(open(library_path))
            name, index = reference_at
            library[name]["references"][index]["fit"] = {
                "loss": {"start": round(start_loss, 6), "fitted": round(best[0], 6)},
                "iterations": args.iterations, "resolution": args.resolution, "samples": args.samples,
                "loss_kind": ("structure" if args.structure else "colour") + ("+detail" if args.detail else ""),
                "date": time.strftime("%Y-%m-%d"), "applied": False, "values": values}
            with open(library_path, "w") as f:
                json.dump(library, f, indent=2)
            print("recorded in %s's reference %d" % (name, index), file=sys.stderr)


if __name__ == "__main__":
    main()
