#!/usr/bin/env python3
"""Render a scene's animation frame by frame, then join the frames into a video.

    scripts/animate.py demo/scenes/harris-cumulus.json --samples 8 -- --width 480 --height 300

The scene says how its frames map onto time, and each object's clock how fast it changes
(see docs/animation.md):

    "animation": { "fps": 24, "frames": 72 },
    "clocks": { "weather": "40 min/s" },
    "objects": [ { "type": "harris_cumulus", "time": 3600, "clock": "weather" }, ... ]

Each frame is rendered on its own (`rays scene.json --frame N`) and depends only on the scene
and its frame number, so frames may be rendered in any order or on other machines; they are
rendered in order here, so a `harris_cumulus` or `landscape` with a `cache` simulates only from
one frame to the next. Frames already rendered are kept, so an interrupted run picks up where it
stopped; `--first`/`--last` render part of the animation (eg. to share it between machines).
Extra arguments after `--` go to the renderer. Writes `<scene>.mp4` with ffmpeg.
"""
import argparse
import json
import os
import subprocess
import sys


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("scene")
    ap.add_argument("--frames", type=int, help="how many frames (default: the scene's animation's)")
    ap.add_argument("--first", type=int, default=0, help="first frame to render")
    ap.add_argument("--last", type=int, help="last frame to render (default: the last)")
    ap.add_argument("--samples", type=int, help="samples per pixel for each frame (default: the scene's)")
    ap.add_argument("--out", help="directory for the frames (default: <scene>-frames)")
    ap.add_argument("--video", help="output video (default: <scene>.mp4)")
    ap.add_argument("--no-video", action="store_true", help="render the frames only")
    ap.add_argument("renderer_args", nargs="*")
    a = ap.parse_args()

    base = os.path.splitext(a.scene)[0]
    out = a.out or base + "-frames"
    video = a.video or base + ".mp4"
    os.makedirs(out, exist_ok=True)
    with open(a.scene) as f:
        animation = json.load(f).get("animation", {})
    frames = a.frames or animation.get("frames")
    if not frames:
        sys.exit("How many frames? Give the scene \"animation\": {\"frames\": ...}, or --frames")
    fps = animation.get("fps", 24)
    last = frames - 1 if a.last is None else min(a.last, frames - 1)
    binary = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "target", "release", "rays")
    pattern = os.path.join(out, "%04d.png")
    extra = (["--samples", str(a.samples)] if a.samples else []) + a.renderer_args

    for n in range(a.first, last + 1):
        png = pattern % n
        if os.path.exists(png) and not os.path.exists(png + ".accum"):
            continue
        print("frame %d of %d, %.3f s" % (n + 1, frames, animation.get("start", 0) + n / fps), flush=True)
        subprocess.run([binary, a.scene, "--frame", str(n), "-o", pattern] + extra, check=True,
                       stdout=subprocess.DEVNULL)
        # The frame is done: its resumable state is no longer needed.
        if os.path.exists(png + ".accum"):
            os.remove(png + ".accum")

    if a.no_video:
        return
    subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", str(fps),
                    "-i", pattern, "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "18", video], check=True)
    print("wrote", video)


if __name__ == "__main__":
    main()
