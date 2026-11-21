#!/usr/bin/env python3
"""Compare two linear renders (.exr) of the same scene by mean luminance, overall and binned by
the depth image of the first, eg. a level-of-detail render against full detail.

    scripts/compare-linear.py test.exr reference.exr depth.exr [--bins 6] [--mask-below 0]

Prints, per depth bin, the pixels in it and the ratio of the mean luminances (test / reference),
with a standard error by the jackknife over rows, and the ratio of the medians. Both images are
clamped at a high quantile of the reference first, so a few fireflies don't decide the means.
Needs numpy and OpenEXR.
"""
import argparse
import numpy as np
import OpenEXR


def read(path):
    f = OpenEXR.File(path)
    channels = f.channels()
    # One layer of R, G, B (or a single depth channel).
    if "RGB" in channels:
        return np.asarray(channels["RGB"].pixels, dtype=np.float64)
    names = list(channels)
    return np.stack([np.asarray(channels[n].pixels, dtype=np.float64) for n in names], axis=-1)


def luminance(img):
    return 0.2126 * img[..., 0] + 0.7152 * img[..., 1] + 0.0722 * img[..., 2]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("test")
    ap.add_argument("reference")
    ap.add_argument("depth")
    ap.add_argument("--bins", type=int, default=6)
    ap.add_argument("--clamp", type=float, default=0.999,
                    help="clamp both images at this quantile of the reference, against fireflies (1 for none)")
    a = ap.parse_args()
    t, r = luminance(read(a.test)), luminance(read(a.reference))
    if a.clamp < 1:
        top = np.quantile(r, a.clamp)
        print(f"clamped at {top:.4g} (reference quantile {a.clamp}): {(t > top).sum()} test and {(r > top).sum()} reference pixels")
        t, r = np.minimum(t, top), np.minimum(r, top)
    d = read(a.depth)
    d = d[..., 0] if d.ndim == 3 else d
    ok = np.isfinite(d) & (d > 0) & (d < 1e6)
    print(f"all pixels: test/reference = {t.mean() / r.mean():.4f} (medians {np.median(t) / np.median(r):.4f})")
    edges = np.quantile(d[ok], np.linspace(0, 1, a.bins + 1))
    print(f"{'depth from':>10} {'to':>8} {'pixels':>7} {'ratio':>7} {'±':>6} {'medians':>8}")
    for lo, hi in zip(edges[:-1], edges[1:]):
        m = ok & (d >= lo) & (d <= hi)
        rows = [(t[i][m[i]].sum(), r[i][m[i]].sum()) for i in range(t.shape[0]) if m[i].any()]
        ts, rs = np.array([x[0] for x in rows]), np.array([x[1] for x in rows])
        ratio = ts.sum() / rs.sum()
        # Spread by the jackknife over rows.
        jk = np.array([(ts.sum() - ts[i]) / (rs.sum() - rs[i]) for i in range(len(ts))]) if len(ts) > 1 else np.array([ratio])
        se = np.sqrt((len(jk) - 1) / len(jk) * ((jk - jk.mean()) ** 2).sum()) if len(jk) > 1 else 0.
        print(f"{lo:10.0f} {hi:8.0f} {m.sum():7d} {ratio:7.4f} {se:6.4f} {np.median(t[m]) / np.median(r[m]):8.4f}")


if __name__ == "__main__":
    main()
