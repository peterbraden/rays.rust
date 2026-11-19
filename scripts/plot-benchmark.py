#!/usr/bin/env python3
"""Graph demo/benchmarks/history.csv (from scripts/benchmark.py) as an SVG: render and
wall time, rays cast and the image difference to the previous row, one panel each,
by commit. No dependencies beyond the standard library.

  scripts/plot-benchmark.py [history.csv] [-o history.svg]
"""
import argparse
import csv
import html
import os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def num(v):
    try:
        return float(v)
    except (TypeError, ValueError):
        return None


def panel(rows, series, title, y0, width, height, log=False):
    """One chart: a list of (key, colour, label) lines over the rows."""
    import math
    left, right, top, bottom = 70, 20, 28, 26
    w, h = width - left - right, height - top - bottom
    values = [num(r[k]) for r in rows for k, _, _ in series]
    values = [v for v in values if v is not None and (v > 0 or not log)]
    out = [f'<g transform="translate(0,{y0})">',
           f'<text x="{left}" y="18" class="title">{html.escape(title)}</text>']
    if not values:
        return out + ["</g>"]
    lo, hi = (min(values), max(values)) if log else (0, max(values))
    if log:
        lo, hi = math.log10(lo), math.log10(hi)
    if hi <= lo:
        hi = lo + 1
    f = (lambda v: math.log10(v)) if log else (lambda v: v)
    x = lambda i: left + (w * i / max(1, len(rows) - 1))
    y = lambda v: top + h - h * (f(v) - lo) / (hi - lo)
    for t in range(5):
        fv = lo + (hi - lo) * t / 4
        v = 10 ** fv if log else fv
        yy = top + h - h * t / 4
        out.append(f'<line x1="{left}" x2="{left + w}" y1="{yy:.1f}" y2="{yy:.1f}" class="grid"/>')
        out.append(f'<text x="{left - 6}" y="{yy + 4:.1f}" class="tick" text-anchor="end">{v:.3g}</text>')
    for key, colour, label in series:
        pts = [(x(i), y(num(r[key]))) for i, r in enumerate(rows)
               if num(r[key]) is not None and (num(r[key]) > 0 or not log)]
        if pts:
            d = " ".join(f"{a:.1f},{b:.1f}" for a, b in pts)
            out.append(f'<polyline points="{d}" fill="none" stroke="{colour}" stroke-width="2"/>')
            out += [f'<circle cx="{a:.1f}" cy="{b:.1f}" r="3" fill="{colour}"/>' for a, b in pts]
    lx = left + w
    for key, colour, label in reversed(series):
        out.append(f'<text x="{lx}" y="18" class="legend" fill="{colour}" text-anchor="end">{html.escape(label)}</text>')
        lx -= 9 * len(label) + 16
    for i, r in enumerate(rows):
        out.append(f'<text x="{x(i):.1f}" y="{top + h + 16}" class="tick" text-anchor="middle">'
                   f'{html.escape(r["commit"][:7])}<title>{html.escape(r["commit_date"][:10] + " " + r["subject"])}</title></text>')
    return out + ["</g>"]


def main():
    p = argparse.ArgumentParser(description=__doc__,
                                formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("csv", nargs="?", default=os.path.join(ROOT, "demo/benchmarks/history.csv"))
    p.add_argument("-o", "--output", help="SVG to write (default: beside the CSV)")
    a = p.parse_args()
    with open(a.csv, newline="") as f:
        rows = [r for r in csv.DictReader(f) if r["status"] == "ok"]
    width = max(720, 60 * len(rows) + 90)
    ph = 220
    panels = [
        ([("render_mean_s", "#2a6fdb", "render s"), ("wall_mean_s", "#e07b39", "wall s")],
         "Time (mean of runs, log scale)", True),
        ([("rays", "#3a9d5d", "rays")], "Rays cast", False),
        ([("diff_mean", "#b03a8c", "mean abs"), ("diff_rmse", "#7a5cc4", "rmse")],
         "Image difference to previous commit (8-bit levels)", False),
    ]
    body = []
    for i, (series, title, log) in enumerate(panels):
        body += panel(rows, series, title, i * ph, width, ph, log)
    svg = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{ph * len(panels)}" '
           f'font-family="sans-serif">',
           "<style>.title{font-size:14px;font-weight:600}.tick{font-size:10px;fill:#555}"
           ".legend{font-size:12px}.grid{stroke:#ddd}</style>",
           f'<rect width="100%" height="100%" fill="white"/>', *body, "</svg>"]
    out = a.output or os.path.splitext(a.csv)[0] + ".svg"
    with open(out, "w") as f:
        f.write("\n".join(svg))
    print(out)


if __name__ == "__main__":
    main()
