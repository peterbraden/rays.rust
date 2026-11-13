#!/usr/bin/env bash
# Measure the noise of a fixed set of scenes, to compare sampling techniques: for each, the
# relative mean squared error of one render and the efficiency 1 / (relMSE x time), from
# several renders differing only in their seed (`rays --variance`). Run before and after a
# change, at the same settings; ratios of efficiency are what matter.
#
# Usage: scripts/variance.sh [output directory] [renders per scene]
set -euo pipefail
out=${1:-target/variance}
runs=${2:-4}
mkdir -p "$out"
cargo build --release --quiet 2> /dev/null

# scene, width, height, samples per pixel
scenes=(
  "demo/reference/cornell-box.json 256 256 64"
  "demo/scenes/spheres.json 320 240 64"
  "demo/scenes/glossy-sun.json 480 240 64"
  "demo/scenes/forest.json 400 200 8"
  "demo/scenes/orbit.json 160 80 4"
)

printf '%-16s %10s %10s %10s %12s\n' scene seconds relMSE median efficiency
for entry in "${scenes[@]}"; do
  read -r scene width height samples <<< "$entry"
  name=$(basename "$scene" .json)
  log="$out/$name.log"
  ./target/release/rays "$scene" --width "$width" --height "$height" --samples "$samples" \
    --variance "$runs" -o "$out/$name.png" > "$log" 2>&1
  seconds=$(sed -n 's/.*per pixel, \([0-9.]*\) s each/\1/p' "$log")
  mean=$(sed -n 's/.*relMSE of one render: mean \([0-9.e-]*\),.*/\1/p' "$log")
  median=$(sed -n 's/.*median \([0-9.e-]*\)$/\1/p' "$log")
  efficiency=$(sed -n 's/.*time): \([0-9.e-]*\)/\1/p' "$log")
  printf '%-16s %10s %10s %10s %12s\n' "$name" "$seconds" "$mean" "$median" "$efficiency"
done
