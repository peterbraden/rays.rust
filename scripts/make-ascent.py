#!/usr/bin/env python3
"""Write demo/scenes/ascent.json: a camera climbing from the sea surface into orbit.

    scripts/make-ascent.py [--duration 20] [--out demo/scenes/ascent.json]

A climb looks smooth when it gains the same ratio of altitude every second, so the camera's
altitude rises exponentially, from 3 m to 400 km: in log altitude its speed ramps up over the
first `--ease` seconds, holds steady, and ramps down into orbit over the last (a trapezoid), so
through the middle of the climb it doubles every second (over 20 s). It flies forward as it
climbs, and pitches down to keep the horizon, which dips as it rises, a little below the centre
of the picture. The path is written
as keys every quarter of a second (`"keys"`, see docs/animation.md), which the renderer
interpolates smoothly between.
"""
import argparse
import json
import math

R = 6.36e6          # the planet's radius, m
LOW, HIGH = 3., 4e5  # altitudes at the start and the end, m


def progress(t, duration, ease):
    """How far through the climb (0 to 1) at time t: the integral of a trapezoidal speed that
    ramps up over `ease` seconds, holds, and ramps down over the last `ease`."""
    t = min(max(t, 0.), duration)
    whole = duration - ease  # the area under the trapezoid, at unit top speed
    if t < ease:
        done = t * t / (2. * ease)
    elif t <= duration - ease:
        done = ease / 2. + (t - ease)
    else:
        left = duration - t
        done = whole - left * left / (2. * ease)
    return done / whole


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--duration", type=float, default=20., help="seconds of video")
    ap.add_argument("--ease", type=float, default=3., help="seconds of easing in and out")
    ap.add_argument("--fps", type=int, default=24)
    ap.add_argument("--out", default="demo/scenes/ascent.json")
    a = ap.parse_args()

    location, lookat = [], []
    steps = int(a.duration * 4)
    for i in range(steps + 1):
        t = a.duration * i / steps
        h = LOW * (HIGH / LOW) ** progress(t, a.duration, a.ease)
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
             "albedo": [0.99, 0.99, 0.99], "g": 0.85, "similarity_after": 2, "seed": 4, "map_size": 512,
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
