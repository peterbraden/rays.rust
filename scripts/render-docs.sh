#!/usr/bin/env bash
# Render the documentation scenes at their full settings, in order, committing each image as
# it is done. Scenes whose image is newer than the scene file are skipped, so an interrupted
# run picks up where it stopped.
#
#   scripts/render-docs.sh [--push] scene ...
set -u
cd "$(dirname "$0")/.."
push=0
if [ "${1:-}" = "--push" ]; then push=1; shift; fi
for name in "$@"; do
    scene="demo/scenes/$name.json"
    png="demo/scenes/$name.png"
    if [ -f "$png" ] && [ "$png" -nt "$scene" ] && git diff --quiet -- "$png" 2>/dev/null && [ -z "$(git status --porcelain -- "$png")" ]; then
        echo "$name: up to date"
        continue
    fi
    start=$(date +%s)
    if ! ./target/release/rays "$scene" > /dev/null 2>&1; then
        echo "$name: render failed"
        continue
    fi
    echo "$name: $(( $(date +%s) - start ))s"
    git add "$png"
    git commit -q -m "Render the $name scene

Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: https://claude.ai/code/session_01U8ZZinwhHe5p11D21Sa8SU" -- "$png"
    if [ $push = 1 ]; then git push -q || echo "$name: push failed"; fi
done
