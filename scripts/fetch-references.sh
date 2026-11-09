#!/bin/sh
# Fetch reference renders from other renderers, for comparison with ours.
#
# Each archive is checked against a known SHA-256 before anything is extracted, and lands in
# references/<scene>/ (not committed). Sources and licences are recorded in
# demo/reference/README.md.
#
# Usage: scripts/fetch-references.sh
set -eu

cd "$(dirname "$0")/.."
DEST=references

# fetch NAME URL SHA256 FILE...  -- download NAME's archive, verify it, extract FILEs
fetch() {
    name=$1; url=$2; sha=$3; shift 3
    dir="$DEST/$name"
    mkdir -p "$dir"
    archive="$dir/archive.zip"
    if [ ! -f "$archive" ]; then
        echo "Downloading $name from $url"
        curl -fsSL -o "$archive.part" "$url"
        mv "$archive.part" "$archive"
    fi
    actual=$(shasum -a 256 "$archive" | cut -d' ' -f1)
    if [ "$actual" != "$sha" ]; then
        echo "Checksum mismatch for $name: expected $sha, got $actual" >&2
        exit 1
    fi
    for f in "$@"; do
        unzip -o -j -q "$archive" "$f" -d "$dir"
    done
    echo "$name: $(ls "$dir" | grep -v archive.zip | tr '\n' ' ')"
}

# Benedikt Bitterli, "Cornell Box" (CC0). The pbrt-v4 archive carries the Tungsten reference
# render (linear half-float EXR, 1024x1024) and the pbrt scene our meshes were converted from.
fetch cornell-box \
    https://benedikt-bitterli.me/resources/pbrt-v4/cornell-box.zip \
    3ac9dcd287f7079b23dbc9eaefe19e64c942c0ec3740b328e09d44c2163b835d \
    cornell-box/TungstenRender.exr cornell-box/scene-v4.pbrt cornell-box/LICENSE.txt
