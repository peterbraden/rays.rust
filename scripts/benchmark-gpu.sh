#!/usr/bin/env bash
# Benchmark the path tracer (trace), the CPU wavefront integrator and the GPU on the demo scenes
# the GPU renders, at one size. For each scene and integrator: wall time at LOW and HIGH samples
# per pixel (their difference gives the cost of a sample without setup), rays per second, and
# mean FLIP at both against a `trace` reference at REF samples (rendered once, kept in OUT).
# Writes CSV to stdout; runs that fell back to another integrator are marked `fallback`.
#
#   cargo build --release --features gpu
#   scripts/benchmark-gpu.sh 640 360 8 64 256 > target/benchmark-gpu.csv
#   MODES=gpu scripts/benchmark-gpu.sh ...    # only the GPU's rows, eg. after changing it
#
# Scenes default to those the GPU renders (found by trying each at 1 sample); name others after
# the five numbers. OUT (default target/benchmark-gpu) holds images, logs and references.
set -u
cd "$(dirname "$0")/.."
W=$1; H=$2; LOW=$3; HIGH=$4; REF=$5; shift 5
RAYS=target/release/rays
OUT=${OUT:-target/benchmark-gpu}
mkdir -p "$OUT"
now() { python3 -I -c 'import time; print(time.time())'; }
seconds() { # "1.5s", "250ms" or "80µs", as printed by `rays`, in seconds
    python3 -I -c '
import re, sys
m = re.match(r"([\d.]+)(ms|µs|s)", sys.argv[1])
unit = {"ms": 1e-3, "µs": 1e-6, "s": 1}
print("" if not m else "%.3f" % (float(m.group(1)) * unit[m.group(2)]))' "$1"
}

scenes=("$@")
if [ ${#scenes[@]} -eq 0 ]; then
    for f in demo/*.json demo/scenes/*.json; do
        if timeout 120 "$RAYS" "$f" --fresh --gpu --samples 1 --width 64 --height 64 -o "$OUT/probe.png" 2>&1 | grep -q '^- GPU:'; then
            scenes+=("$f")
        fi
    done
fi

echo "scene,integrator,spp,wall_s,elapsed_s,rays,rays_per_s,flip"
for f in "${scenes[@]}"; do
    n=$(basename "$f" .json)
    ref="$OUT/$n-ref.exr"
    [ -f "$ref" ] || "$RAYS" "$f" --fresh --width "$W" --height "$H" --samples "$REF" -o "$ref" > "$OUT/$n-ref.log" 2>&1
    for mode in ${MODES:-trace wavefront gpu}; do
        case $mode in trace) flag="";; wavefront) flag="--wavefront";; gpu) flag="--gpu";; esac
        for spp in $LOW $HIGH; do
            log="$OUT/$n-$mode-$spp.log"
            t0=$(now)
            "$RAYS" "$f" --fresh $flag --width "$W" --height "$H" --samples "$spp" -o "$OUT/$n-$mode-$spp.png" --compare "$ref" > "$log" 2>&1
            t1=$(now)
            wall=$(python3 -I -c "print(f'{$t1 - $t0:.3f}')")
            elapsed=$(seconds "$(sed -n 's/^| Elapsed Time: \(.*\)/\1/p' "$log")")
            rays=$(sed -n 's/^| Rays Cast: \(.*\)/\1/p' "$log")
            rps=$(sed -n 's/^| Rays per sec: \(.*\)/\1/p' "$log")
            flip=$(sed -n 's/^- Mean FLIP error: \([0-9.]*\).*/\1/p' "$log")
            if [ "$mode" = gpu ] && ! grep -q '^- GPU:' "$log"; then flip=fallback; fi
            if [ "$mode" = wavefront ] && ! grep -q '^- Wavefront integrator:' "$log"; then flip=fallback; fi
            echo "$n,$mode,$spp,$wall,$elapsed,$rays,$rps,$flip"
        done
    done
done
