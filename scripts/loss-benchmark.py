#!/usr/bin/env python3
"""Measure what fit-material.py's loss can tell apart, from the scans alone (no renders).

A fit is only as good as its loss: an optimiser finds whatever the loss can't see (sub-pixel
confetti for colour, stones shrunk to imitate grain, blobs for leaves). This tests the loss
on pairs of images whose right answer is known, all cut from the scans in
references/materials/ (those the library's presets name, fetched by
fetch-material-reference.py):

- scale: one quadrant of a scan against another quadrant resampled as if the material were
  smaller or larger (by 0.6 to 1.67). The loss should be least at 1 and rise steeply either
  side. Reported as the mean log ratio of loss(s) to loss(1) at s = 0.75 and 1.33
  (`scale_contrast`, larger is better) and the share of scans whose least loss is at 1.
- structure: a quadrant against another quadrant after its phases are randomised, keeping its
  power spectrum, and its colours matched back to its histogram: the same colours and the
  same sizes, but blobs where there were shapes (the leaves were blobs). The log ratio of
  loss(phase-randomised) to loss(same scan), larger is better (`structure_contrast`).
- identity: a quadrant against another quadrant of the same scan, and against each other scan
  coloured like it (each channel's histogram matched), so only structure tells them apart. The
  share of scans whose own other quadrant is nearest (`identity`), and the mean log ratio of
  the nearest other scan's loss to the scan's own (`identity_margin`).

The quadrants are taken at the scale a fit renders at: a quarter of the scan, a scan's
size / 2 across, at --resolution pixels.

    scripts/loss-benchmark.py [--loss NAME] [--resolution 128]
"""
import argparse, importlib.util, json, os, sys
import numpy as np
from PIL import Image

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
spec = importlib.util.spec_from_file_location("fit", os.path.join(ROOT, "scripts", "fit-material.py"))
fit = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fit)


def scans():
    """The scans the library's presets name: (asset, view), once each."""
    library = json.load(open(os.path.join(ROOT, "src/material/library.json")))
    seen = {}
    for preset in library.values():
        for r in preset.get("references", []):
            asset = r["source"].partition(":")[2]
            path = os.path.join(ROOT, "references", "materials", asset, "albedo.jpg")
            if os.path.exists(path):
                seen.setdefault(asset, path)
    return sorted(seen.items())


def linear(path):
    im = np.asarray(Image.open(path).convert("RGB"), dtype=np.float64) / 255.
    return fit.srgb_to_linear(im)


def resample(lin, x0, y0, w, resolution):
    """A square crop w pixels across from (x0, y0), averaged in linear light to resolution,
    sRGB-encoded as the loss sees it."""
    crop = lin[int(y0):int(y0 + w), int(x0):int(x0 + w)]
    out = [np.asarray(Image.fromarray(crop[:, :, k].astype(np.float32), mode="F").resize((resolution, resolution), Image.BOX),
                      dtype=np.float64) for k in range(3)]
    return fit.linear_to_srgb(np.stack(out, axis=2))


def match_histograms(src, ref):
    """src with each channel's values replaced by ref's at the same rank."""
    out = np.empty_like(src)
    for k in range(3):
        s = src[:, :, k].ravel()
        order = np.argsort(s, kind="stable")
        r = np.sort(ref[:, :, k].ravel())
        v = np.empty_like(s)
        v[order] = r[np.linspace(0, len(r) - 1, len(s)).astype(int)]
        out[:, :, k] = v.reshape(src.shape[:2])
    return out


def phase_randomised(img, seed):
    """The same amplitude spectrum, one random phase for all channels (so colours stay
    correlated), then the colours matched back to the image's own histograms."""
    rng = np.random.default_rng(seed)
    phase = np.angle(np.fft.fft2(rng.normal(size=img.shape[:2])))
    out = np.stack([np.real(np.fft.ifft2(np.abs(np.fft.fft2(img[:, :, k])) * np.exp(1j * phase))) for k in range(3)], axis=2)
    return match_histograms(out, img)


def main():
    a = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    a.add_argument("--loss", default="PatchLoss", help="a loss class in fit-material.py")
    a.add_argument("--resolution", type=int, default=128)
    a.add_argument("-v", "--verbose", action="store_true")
    args = a.parse_args()
    # A class, or a class and the parts it compares: StructureLoss:bank,spectrum
    cls, _, parts = args.loss.partition(":")
    loss = getattr(fit, cls)(parts=tuple(parts.split(","))) if parts else getattr(fit, cls)()
    r = args.resolution
    factors = [0.6, 0.75, 1.0, 1.33, 1.67]
    data = []
    for asset, path in scans():
        lin = linear(path)
        n = min(lin.shape[:2])
        q = n // 2
        a_img = resample(lin, 0, 0, q, r)
        # Another quadrant, at each scale about the same centre: s < 1 shows more of the scan
        # (the material looks smaller), s > 1 less.
        b = {}
        for s in factors:
            w = q / s
            cx = cy = n - q / 2
            x0, y0 = min(max(cx - w / 2, 0), n - w), min(max(cy - w / 2, 0), n - w)
            b[s] = resample(lin, x0, y0, w, r)
        data.append((asset, a_img, b))
    rows, scale_c, scale_ok, struct_c = [], [], [], []
    for asset, a_img, b in data:
        ls = {s: loss(a_img, img) for s, img in b.items()}
        lp = loss(a_img, phase_randomised(b[1.0], 7))
        scale_c.append(0.5 * (np.log(ls[0.75] / ls[1.0]) + np.log(ls[1.33] / ls[1.0])))
        scale_ok.append(min(ls, key=ls.get) == 1.0)
        struct_c.append(np.log(lp / ls[1.0]))
        rows.append((asset, ls, lp))
    ident, margin = [], []
    for i, (asset, a_img, b) in enumerate(data):
        own = loss(a_img, b[1.0])
        others = [loss(a_img, match_histograms(o[2][1.0], b[1.0])) for j, o in enumerate(data) if j != i]
        ident.append(own < min(others))
        margin.append(np.log(min(others) / own))
        if args.verbose:
            print("%-26s scale %s  phase %.4f  own %.4f  nearest other %.4f" % (
                asset, " ".join("%.4f" % rows[i][1][s] for s in factors), rows[i][2], own, min(others)))
    print("%s: scale_contrast %.3f  scale_minimum %.2f  structure_contrast %.3f  identity %.2f  identity_margin %.3f" % (
        args.loss, np.mean(scale_c), np.mean(scale_ok), np.mean(struct_c), np.mean(ident), np.mean(margin)))


if __name__ == "__main__":
    main()
