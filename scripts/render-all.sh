#!/usr/bin/env bash
# Render every demo scene at its full settings, one after another, each image written beside
# its scene file (demo/scenes/<name>.png). Meant to run in the background:
#
#   nohup scripts/render-all.sh > render-all.out 2>&1 &
#   tail -f render-all.out
#
# Options (before any scene names):
#   --missing    only scenes that have no image yet
#   --samples N  a quick preview at N samples per pixel, written to target/render-all/ instead,
#                leaving the checked-in images alone
#   --commit     commit each image as it is done (not with --samples)
#   --list       list the scenes and stop
# Scene names (eg. `orbit spheres`) limit the run to those; by default, every scene in demo/
# and demo/scenes/.
#
# Renders always start afresh (--fresh): a saved render state is keyed by the scene file, not
# the renderer, so after renderer changes resuming one would mix old samples with new.
# A failed scene is reported and the run carries on. Each scene's output goes to
# target/render-all/<name>.log, and a line per scene to target/render-all/summary.txt.
set -u
cd "$(dirname "$0")/.."

missing=0; samples=""; commit=0; list=0
while [ $# -gt 0 ]; do
    case "$1" in
        --missing) missing=1 ;;
        --samples) samples="$2"; shift ;;
        --commit) commit=1 ;;
        --list) list=1 ;;
        -h|--help) sed -n '2,22p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
        -*) echo "Unknown option $1" >&2; exit 1 ;;
        *) break ;;
    esac
    shift
done
if [ -n "$samples" ] && [ $commit = 1 ]; then
    echo "--commit renders at full settings; it can't be used with --samples" >&2; exit 1
fi

# Every scene file, or those named.
scenes=()
if [ $# -gt 0 ]; then
    for name in "$@"; do
        found=""
        for f in "demo/scenes/$name.json" "demo/scenes/$name.nff" "demo/$name.json"; do
            [ -f "$f" ] && found="$f" && break
        done
        [ -n "$found" ] || { echo "No scene called $name" >&2; exit 1; }
        scenes+=("$found")
    done
else
    for f in demo/*.json demo/scenes/*.json demo/scenes/*.nff; do scenes+=("$f"); done
fi

if [ $list = 1 ]; then printf '%s\n' "${scenes[@]}"; exit 0; fi

out=target/render-all
mkdir -p "$out"
summary="$out/summary.txt"
echo "# $(date '+%Y-%m-%d %H:%M') $(git rev-parse --short HEAD) ${samples:+($samples spp preview)}" >> "$summary"

echo "Building..."
cargo build --release --quiet 2> "$out/build.log" || { echo "Build failed: see $out/build.log"; exit 1; }

failed=0; done_count=0; total=${#scenes[@]}; run_start=$(date +%s)
for scene in "${scenes[@]}"; do
    name=$(basename "${scene%.*}")
    image="${scene%.*}.png"
    done_count=$((done_count + 1))
    if [ $missing = 1 ] && [ -f "$image" ]; then
        echo "[$done_count/$total] $name: has an image, skipped"
        continue
    fi
    args=(--fresh)
    if [ -n "$samples" ]; then
        image="$out/$name.png"
        args+=(--samples "$samples" -o "$image")
    fi
    echo "[$done_count/$total] $(date '+%H:%M:%S') $name..."
    start=$(date +%s)
    if ./target/release/rays "$scene" "${args[@]}" > "$out/$name.log" 2>&1; then
        took=$(( $(date +%s) - start ))
        echo "    done in ${took}s -> $image"
        echo "$name ok ${took}s" >> "$summary"
        if [ $commit = 1 ]; then
            git add "$image" && git commit -q -m "Render the $name scene" -- "$image" \
                || echo "    commit failed"
        fi
    else
        took=$(( $(date +%s) - start ))
        failed=$((failed + 1))
        echo "    FAILED after ${took}s: $(tail -1 "$out/$name.log")"
        echo "$name FAILED ${took}s" >> "$summary"
    fi
done

elapsed=$(( $(date +%s) - run_start ))
echo "Finished in $((elapsed / 3600))h $((elapsed % 3600 / 60))m: $failed failed. Summary: $summary"
[ $failed = 0 ]
