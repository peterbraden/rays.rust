#!/usr/bin/env python3
"""Render an animation of a scene by stepping a time parameter, then join the frames into a video.

    scripts/animate.py demo/scenes/harris-cumulus.json --start 1800 --end 9000 --frames 72 --samples 8

Sets `"time"` on every object that has one (eg. `harris_cumulus`, `hogan_kew_cirrus`), renders
each frame with the release build, in order (a `harris_cumulus` with a `cache` then simulates
only from one frame to the next), and writes `<scene>.mp4` with ffmpeg. Frames already rendered
are kept, so an interrupted run picks up where it stopped. Extra arguments after `--` go to the
renderer (eg. `-- --width 480 --height 270`).
"""
import argparse
import json
import os
import subprocess
import sys


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("scene")
    ap.add_argument("--start", type=float, required=True, help="time of the first frame, s")
    ap.add_argument("--end", type=float, required=True, help="time of the last frame, s")
    ap.add_argument("--frames", type=int, default=60)
    ap.add_argument("--fps", type=int, default=24)
    ap.add_argument("--samples", type=int, help="samples per pixel for each frame (default: the scene's)")
    ap.add_argument("--out", help="directory for the frames (default: <scene>-frames)")
    ap.add_argument("--video", help="output video (default: <scene>.mp4)")
    ap.add_argument("renderer_args", nargs="*")
    a = ap.parse_args()

    base = os.path.splitext(a.scene)[0]
    out = a.out or base + "-frames"
    video = a.video or base + ".mp4"
    os.makedirs(out, exist_ok=True)
    with open(a.scene) as f:
        scene = json.load(f)
    if a.samples:
        scene["supersamples"] = a.samples
    timed = [o for o in scene.get("objects", []) if "time" in o]
    if not timed:
        sys.exit("No object in the scene has a \"time\" to animate")
    binary = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "target", "release", "rays")

    for n in range(a.frames):
        t = a.start + (a.end - a.start) * n / max(a.frames - 1, 1)
        png = os.path.join(out, "frame-%04d.png" % n)
        if os.path.exists(png):
            continue
        for o in timed:
            o["time"] = t
        frame_scene = os.path.join(out, "frame-%04d.json" % n)
        with open(frame_scene, "w") as f:
            json.dump(scene, f)
        print("frame %d/%d, t = %.0f s" % (n + 1, a.frames, t), flush=True)
        subprocess.run([binary, frame_scene, "-o", png] + a.renderer_args, check=True,
                       stdout=subprocess.DEVNULL)
        os.remove(frame_scene)

    subprocess.run(["ffmpeg", "-y", "-loglevel", "error", "-framerate", str(a.fps),
                    "-i", os.path.join(out, "frame-%04d.png"),
                    "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "18", video], check=True)
    print("wrote", video)


if __name__ == "__main__":
    main()
