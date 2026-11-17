#!/usr/bin/env python3
"""Write demo/scenes/ascent.json: a camera climbing from the sea surface into orbit.

    scripts/make-ascent.py [--duration 30] [--out demo/scenes/ascent.json]

A climb looks smooth when it gains the same ratio of altitude every second, so the camera's
altitude rises exponentially, from 3 m to 400 km. Its speed in log altitude ramps up over the
first `--ease` seconds and down into orbit over the last (a trapezoid), and is slowed where
there is something to see (`LINGER`): through the cumulus layer, the thin cirrus (crossed in a
sixth of a second at a steady rate, the sky would switch from cloud to black in a frame), and
the stratosphere, where the sky darkens. The rate is solved for so the camera reaches orbit at
`--duration`. It flies forward as it climbs, and pitches down to keep the horizon, which dips
as it rises, a little below the centre of the picture. The path is written as keys every
quarter of a second (`"keys"`, see docs/animation.md), which the renderer interpolates
smoothly between.
"""
import argparse
import json
import math

R = 6.36e6          # the planet's radius, m
LOW, HIGH = 3., 4e5  # altitudes at the start and the end, m
# Layers to linger in: (bottom, top, how many times slower), m. The scene's cumulus is 1.5 to
# 4 km, its cirrus 8 to 10 km; the stratosphere darkens the sky from about 12 to 50 km.
LINGER = [(1500., 4000., 3.), (8000., 10000., 6.), (12000., 50000., 1.5)]


def ramp(t, duration, ease):
    """A trapezoid: 0 at the ends, 1 between `ease` from either."""
    return min(1., t / ease, (duration - t) / ease) if ease > 0. else 1.


def slowness(h):
    """How many times slower the climb is at altitude h: smoothly into and out of each layer
    (over a fifth of an e-fold of altitude either side)."""
    x = math.log(h)
    w = 1.
    for lo, hi, k in LINGER:
        edge = 0.2
        inside = min(1., max(0., (x - math.log(lo) + edge) / (2. * edge)), max(0., (math.log(hi) + edge - x) / (2. * edge)))
        inside = inside * inside * (3. - 2. * inside)
        w = max(w, 1. + (k - 1.) * inside)
    return w


def climb(rate, duration, ease, dt=1e-3):
    """Altitude through time at top speed `rate` (e-folds per second): [(t, h)], every dt."""
    out, x, t = [], math.log(LOW), 0.
    while t <= duration + 1e-9:
        out.append((t, math.exp(x)))
        x += rate * ramp(t + dt / 2., duration, ease) / slowness(math.exp(x)) * dt
        t += dt
    return out


def path(duration, ease):
    """The climb reaching HIGH at `duration`: the top speed found by bisection."""
    lo, hi = 0.01, 50.
    for _ in range(60):
        mid = (lo + hi) / 2.
        lo, hi = (mid, hi) if climb(mid, duration, ease, 0.01)[-1][1] < HIGH else (lo, mid)
    samples = climb(hi, duration, ease)
    return lambda t: min(HIGH, samples[min(len(samples) - 1, round(t / 1e-3))][1])


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--duration", type=float, default=30., help="seconds of video")
    ap.add_argument("--ease", type=float, default=3., help="seconds of easing in and out")
    ap.add_argument("--fps", type=int, default=24)
    ap.add_argument("--out", default="demo/scenes/ascent.json")
    a = ap.parse_args()

    altitude = path(a.duration, a.ease)
    location, lookat = [], []
    steps = int(a.duration * 4)
    for i in range(steps + 1):
        t = a.duration * i / steps
        h = altitude(t)
        # Gliding forward over the water at first, then ahead about as far as it climbs.
        z = 4. * t + 0.7 * h
        dip = math.acos(R / (R + h))
        pitch = -dip + math.radians(4)
        d = 1000.
        location.append([round(t, 3), [0, round(h, 3), round(z, 3)]])
        lookat.append([round(t, 3), [0, round(h + d * math.sin(pitch), 3), round(z + d * math.cos(pitch), 3)]])

    scene = {
        "width": 480, "height": 270, "supersamples": 32, "samples_per_chunk": 2, "max_depth": 32,
        "shadow_bias": 1, "path_termination": "russian_roulette",
        "animation": {"fps": a.fps, "frames": int(a.duration * a.fps), "shutter": 0.5},
        "background": {"type": "space"},
        "output": {"flare": {"blades": 6, "blade_rotation": 20, "f_number": 11, "sources": 2, "ghosts": 20, "starburst": 2}},
        "camera": {"location": {"keys": location}, "lookat": {"keys": lookat}, "up": [0, 1, 0], "angle": 0.9, "aperture": 0},
        "sun": {"intensity": 6},
        "objects": [
            {"type": "ocean", "wind": [9, 3]},
            {"type": "skysphere", "sun_direction": [0.15, 0.06, 1]},
            {"type": "nubis_clouds", "mapping": "global", "base": 1500, "top": 4000, "coverage": 0.5, "density": 0.03,
             "albedo": [0.99, 0.99, 0.99], "g": 0.85, "similarity_after": 2, "seed": 66, "map_size": 512,
             "wind": [15, 0]},
            {"type": "hogan_kew_cirrus", "mapping": "global", "genus": "cirrus", "seed": 3},
        ],
    }
    with open(a.out, "w") as f:
        json.dump(scene, f, separators=(",", ":"))
        f.write("\n")
    print("wrote", a.out, "with", len(location), "keys over", a.duration, "s")


if __name__ == "__main__":
    main()
