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
#   --resume     carry on from each scene's saved render state rather than starting afresh:
#                after raising a scene's samples, only the missing ones are rendered. Only
#                when the renderer hasn't changed since the state was saved
#   --cpu        build without the GPU, so scenes render on the CPU (by default rays renders
#                each scene with the first of the GPU, the wavefront integrator and the path
#                tracer that can, saying why it passed over the others)
#   --list       list the scenes and stop
# Scene names (eg. `orbit spheres`) limit the run to those; by default, every scene in demo/
# and demo/scenes/.
#
# Renders start afresh (--fresh) unless --resume: a saved render state is keyed by the scene
# file, not the renderer, so after renderer changes resuming one would mix old samples with new.
# A failed scene is reported and the run carries on. Each scene's output goes to
# target/render-all/<name>.log, and a line per scene, naming what rendered it, to
# target/render-all/summary.txt.
#
# Each run is also a benchmark: a row per scene (times, integrator, rays, peak memory and
# swapping by /usr/bin/time, the scene file's hash and the picture's difference from HEAD's)
# is appended to
# demo/benchmarks/render-all.csv (target/render-all/previews.csv with --samples), and the run
# ends with a report comparing it with the previous one on this machine
# (scripts/render-all-stats.py; `scripts/render-all-stats.py report` repeats it).
set -u
cd "$(dirname "$0")/.."

missing=0; samples=""; commit=0; list=0; resume=0; cpu=0
while [ $# -gt 0 ]; do
    case "$1" in
        --missing) missing=1 ;;
        --samples) samples="$2"; shift ;;
        --commit) commit=1 ;;
        --list) list=1 ;;
        --resume) resume=1 ;;
        --cpu) cpu=1 ;;
        -h|--help) sed -n '2,34p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
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
run=$(date -u "+%Y-%m-%dT%H:%M:%SZ"); head=$(git rev-parse HEAD)
# Whether the renderer differs from HEAD's (the run's own images don't count).
dirty=0; git diff --quiet HEAD -- src shaders Cargo.toml Cargo.lock || dirty=1
stats=(); [ -n "$samples" ] && stats=(--preview "$samples")
# A row of statistics for the scene; a failure here mustn't stop the run.
record() {
    python3 scripts/render-all-stats.py record --run "$run" --commit "$head" --dirty "$dirty" \
        --scene "$scene" --status "$1" --wall "$took" --log "$out/$name.log" --image "$image" \
        ${stats[@]+"${stats[@]}"} || echo "    (statistics not recorded)"
}

# Each render under /usr/bin/time, for its peak memory, page faults and CPU time.
timer=()
if [ -x /usr/bin/time ]; then
    [ "$(uname)" = Darwin ] && timer=(/usr/bin/time -l) || timer=(/usr/bin/time -v)
fi
# Stopping the run stops the render too: it runs under time, so isn't the script's own child.
child=""
stop() {
    if [ -n "$child" ]; then
        pkill -TERM -P "$child" 2> /dev/null; kill -TERM "$child" 2> /dev/null
    fi
    echo "Stopped"; exit 143
}
trap stop TERM INT

features=(); [ $cpu = 1 ] || features=(--features gpu)
echo "Building..."
cargo build --release --quiet "${features[@]}" 2> "$out/build.log" \
    || { echo "Build failed: see $out/build.log"; exit 1; }

failed=0; done_count=0; total=${#scenes[@]}; run_start=$(date +%s)
for scene in "${scenes[@]}"; do
    name=$(basename "${scene%.*}")
    image="${scene%.*}.png"
    done_count=$((done_count + 1))
    if [ $missing = 1 ] && [ -f "$image" ]; then
        echo "[$done_count/$total] $name: has an image, skipped"
        continue
    fi
    args=()
    [ $resume = 1 ] || args+=(--fresh)
    if [ -n "$samples" ]; then
        image="$out/$name.png"
        args+=(--samples "$samples" -o "$image")
    fi
    echo "[$done_count/$total] $(date '+%H:%M:%S') $name..."
    start=$(date +%s)
    # The render's output to its log, time's report to <name>.time.
    LOG="$out/$name.log" ${timer[@]+"${timer[@]}"} bash -c 'exec "$@" > "$LOG" 2>&1' _ \
        ./target/release/rays "$scene" "${args[@]}" 2> "$out/$name.time" &
    child=$!
    wait "$child"; status=$?; child=""
    if [ $status = 0 ]; then
        took=$(( $(date +%s) - start ))
        case "$(grep -m1 '^- Integrator: ' "$out/$name.log")" in
            *GPU*) by=gpu ;;
            *wavefront*) by=wavefront ;;
            *) by=path-tracer ;;
        esac
        echo "    done in ${took}s on the $by -> $image"
        echo "$name ok ${took}s $by" >> "$summary"
        record ok
        if [ $commit = 1 ]; then
            git add "$image" && git commit -q -m "Render the $name scene" -- "$image" \
                || echo "    commit failed"
        fi
    else
        took=$(( $(date +%s) - start ))
        failed=$((failed + 1))
        echo "    FAILED after ${took}s: $(tail -1 "$out/$name.log")"
        echo "$name FAILED ${took}s" >> "$summary"
        record failed
    fi
done

elapsed=$(( $(date +%s) - run_start ))
echo "Finished in $((elapsed / 3600))h $((elapsed % 3600 / 60))m: $failed failed. Summary: $summary"
echo
python3 scripts/render-all-stats.py report --run "$run" ${samples:+--preview} || true
if [ $commit = 1 ]; then
    git add demo/benchmarks/render-all.csv && git commit -q -m "Record the render-all run's statistics" \
        -- demo/benchmarks/render-all.csv || echo "Committing the statistics failed"
fi
[ $failed = 0 ]
