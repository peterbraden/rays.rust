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

Colour is compared besides, and more strictly: the patches' random projections mostly see
lightness, so an optimiser left to them alone will happily trade grey stones for sub-pixel
confetti of saturated colours averaging grey. Each level's pixels are also compared as colour
distributions along fixed opponent axes (lightness, red-green, blue-yellow) and random
directions in that space. And candidates are rendered at twice the resolution and averaged
down in linear light, as the reference was, so detail finer than a pixel averages out rather
than showing as sampling noise.

The parameters are found by CMA-ES (Hansen, "The CMA evolution strategy: a tutorial", 2016),
which needs no gradients: the renderer isn't differentiable.

A fit is described by a reference in the preset's own entry in src/material/library.json,
which the renderer ignores (it reads only a preset's `material`, `variables`, `seed_direction`
and `overlay`):

    "scree": { ..., "references": [
       {"source": "ambientcg:Gravel043", "licence": "CC0", "size": 1.6, "usage": {"scale": 0.2},
        "params": [...],
        "fit": {"loss": {"start": ..., "fitted": ...}, "values": {...}, "applied": false, ...}} ] }

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
searched as a lightness (the mean of its channels, `min` to `max`, default 0 to 1) and a tint:
each channel within `chroma` (default 0.25) of the lightness, relatively, as natural materials
are mostly low in saturation.

    scripts/fit-material.py scree|spec.json [--iterations 30] [--resolution 160] [--samples 8] [--record] [-o out_dir]

Writes to out_dir: fitted.json (a library file with the fitted preset under `name`, and the
library material using it), compare.png (reference, start, fitted), and log.csv.
"""
import argparse, copy, csv, json, math, os, subprocess, sys, tempfile, time

import numpy as np
from PIL import Image

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def srgb_to_linear(c):
    return np.where(c <= 0.04045, c / 12.92, ((c + 0.055) / 1.055) ** 2.4)


def load_reference(path, resolution):
    """The reference albedo, sRGB-encoded in [0, 1], resampled to resolution square (averaged
    in linear light, as the render's pixels average it)."""
    im = np.asarray(Image.open(path).convert("RGB"), dtype=np.float64) / 255.
    lin = srgb_to_linear(im)
    channels = [Image.fromarray(lin[:, :, k].astype(np.float32), mode="F").resize((resolution, resolution), Image.BOX)
                for k in range(3)]
    lin = np.stack([np.asarray(c, dtype=np.float64) for c in channels], axis=2)
    return linear_to_srgb(lin)


def linear_to_srgb(c):
    c = np.clip(c, 0, None)
    return np.where(c <= 0.0031308, c * 12.92, 1.055 * c ** (1 / 2.4) - 0.055)


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

    def __call__(self, a, b):
        total = 0.
        for k, (la, lb) in enumerate(zip(self.pyramid(a, self.levels), self.pyramid(b, self.levels))):
            rng = np.random.default_rng(self.rng_seed + k)
            pa, pb = self.patches(la, rng), self.patches(lb, rng)
            n = min(len(pa), len(pb))
            qa = np.sort(pa[:n] @ self.projections, axis=0)
            qb = np.sort(pb[:n] @ self.projections, axis=0)
            total += float(np.mean((qa - qb) ** 2)) + self.colour_weight * self.colours(la, lb, rng)
        return total / self.levels


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
            else:
                self.slots.append((p, None))

    def bounds(self, p):
        if p.get("kind") == "color":
            return (p.get("min", 0.), p.get("max", 1.))
        return (p["min"], p["max"])

    @staticmethod
    def colour_to_unit(p, rgb, lo, hi):
        """A colour as lightness and tint in the unit cube (`colour_from_unit` reversed)."""
        light = sum(rgb) / 3.
        chroma = p.get("chroma", 0.25)
        tint = [((c / light - 1.) / chroma + 1.) / 2. if light > 0 else 0.5 for c in rgb[:2]]
        return [(light - lo) / (hi - lo)] + tint

    @staticmethod
    def colour_from_unit(p, u, lo, hi):
        light = lo + min(max(u[0], 0.), 1.) * (hi - lo)
        chroma = p.get("chroma", 0.25)
        r, g = [1. + chroma * (2. * min(max(t, 0.), 1.) - 1.) for t in u[1:]]
        # Blue makes the mean the lightness.
        return [light * r, light * g, max(light * (3. - r - g), 0.)]

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


def scene(usage, library_path, size, resolution, samples, seed):
    return {
        "width": 2 * resolution, "height": 2 * resolution, "supersamples": samples, "max_depth": 2, "seed": seed,
        "background": [1, 1, 1], "output": {"tonemap": "none", "exposure": 1.0},
        "library": library_path,
        "camera": {"type": "orthographic", "location": [0, 5, 0], "lookat": [0, 0, 0], "up": [0, 0, 1], "height": size},
        "materials": {"M": usage},
        "objects": [{"type": "plane", "y": 0, "material": "M"}],
    }


def render(preset, usage, name, args, size, tmp, tag):
    library_path = os.path.join(tmp, "library-%s.json" % tag)
    json.dump({name: preset}, open(library_path, "w"))
    path = os.path.join(tmp, "scene-%s.json" % tag)
    json.dump(scene(usage, library_path, size, args.resolution, args.samples, args.seed), open(path, "w"))
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
    reference = load_reference(os.path.join(base, spec["reference"]), args.resolution)
    loss = PatchLoss()
    problem = Problem(spec, preset)
    os.makedirs(args.output, exist_ok=True)

    with tempfile.TemporaryDirectory() as tmp:
        def evaluate(x, tag):
            p, u = problem.make(x, usage)
            img = render(p, u, name, args, size, tmp, tag)
            return loss(reference, img), img

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
        r = args.resolution
        sheet = Image.new("RGB", (3 * r, r))
        for k, img in enumerate([reference, start_img, best[2]]):
            sheet.paste(Image.fromarray((np.clip(img, 0, 1) * 255).astype(np.uint8)), (k * r, 0))
        sheet.save(os.path.join(args.output, "compare.png"))
        print("loss %.5f -> %.5f; wrote %s" % (start_loss, best[0], args.output), file=sys.stderr)
        if args.record:
            if reference_at is None:
                sys.exit("--record needs a preset's reference, not a spec file")
            values = {}
            for p in spec["params"]:
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
                "date": time.strftime("%Y-%m-%d"), "applied": False, "values": values}
            with open(library_path, "w") as f:
                json.dump(library, f, indent=2)
            print("recorded in %s's reference %d" % (name, index), file=sys.stderr)


if __name__ == "__main__":
    main()
