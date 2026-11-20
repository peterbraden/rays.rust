#!/usr/bin/env bash
# Render every scene the cloud atlas (demo/cloud-atlas.md) shows, at full settings, each image
# written beside its scene file. The list is read from the atlas itself, so it stays in step.
#
#   scripts/render-atlas.sh                 # all of them, afresh
#   scripts/render-atlas.sh --list          # just list them
#   scripts/render-atlas.sh --samples 16    # a quick preview into target/render-all/
#   scripts/render-atlas.sh --missing       # only scenes with no image yet
#   scripts/render-atlas.sh --commit        # commit each image as it is done
#
# Any other options are render-all.sh's (see scripts/render-all.sh --help). A long run is best
# left in the background:
#
#   nohup scripts/render-atlas.sh > render-atlas.out 2>&1 &
#   tail -f render-atlas.out
set -u
cd "$(dirname "$0")/.."
scenes=$(grep -o '`[a-z0-9-]*\.json`' demo/cloud-atlas.md | tr -d '`' | sed 's/\.json$//' | awk '!seen[$0]++')
# shellcheck disable=SC2086
exec scripts/render-all.sh "$@" $scenes
