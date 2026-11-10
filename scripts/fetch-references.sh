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

# fetch_file NAME URL SHA256 FILENAME  -- download one file, verify it
fetch_file() {
    name=$1; url=$2; sha=$3; file=$4
    dir="$DEST/$name"
    mkdir -p "$dir"
    if [ ! -f "$dir/$file" ]; then
        echo "Downloading $name from $url"
        curl -fsSL -o "$dir/$file.part" "$url"
        mv "$dir/$file.part" "$dir/$file"
    fi
    actual=$(shasum -a 256 "$dir/$file" | cut -d' ' -f1)
    if [ "$actual" != "$sha" ]; then
        echo "Checksum mismatch for $name: expected $sha, got $actual" >&2
        exit 1
    fi
    echo "$name: $file"
}

# fetch_tar NAME URL SHA256 FILE...  -- as fetch, for a .tar.gz archive
fetch_tar() {
    name=$1; url=$2; sha=$3; shift 3
    dir="$DEST/$name"
    mkdir -p "$dir"
    archive="$dir/archive.tar.gz"
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
        tar -xzf "$archive" -O "$f" > "$dir/$(basename "$f")"
    done
    echo "$name: $(ls "$dir" | grep -v archive.tar.gz | tr '\n' ' ')"
}

# Benedikt Bitterli, "Cornell Box" (CC0). The pbrt-v4 archive carries the Tungsten reference
# render (linear half-float EXR, 1024x1024) and the pbrt scene our meshes were converted from.
fetch cornell-box \
    https://benedikt-bitterli.me/resources/pbrt-v4/cornell-box.zip \
    3ac9dcd287f7079b23dbc9eaefe19e64c942c0ec3740b328e09d44c2163b835d \
    cornell-box/TungstenRender.exr cornell-box/scene-v4.pbrt cornell-box/LICENSE.txt

# Peter Shirley, Trevor David Black, Steve Hollasch, "Ray Tracing in One Weekend" v4.0.2 (CC0):
# the "view-distant" image, 400x225, gamma-2 encoded 8-bit PNG.
fetch_file rtiow-view-distant \
    https://raw.githubusercontent.com/RayTracing/raytracing.github.io/v4.0.2/images/img-1.20-view-distant.png \
    039acbc2eabf254af24543aeaffd8cda6d504ab1e9c160eae2e2c2aa515e5017 \
    img-1.20-view-distant.png

# Stanford 3D Scanning Repository, "Dragon": the full-resolution vrip reconstruction (ASCII PLY,
# 437,645 vertices, 871,414 triangles). Free to redistribute and to publish images of, with
# credit to the Stanford Computer Graphics Laboratory, but not for commercial use; see
# demo/reference/README.md. Stanford publishes no reference image: ours is rendered with Mitsuba 3
# by scripts/mitsuba_dragon_reference.py.
fetch_tar stanford-dragon \
    http://graphics.stanford.edu/pub/3Dscanrep/dragon/dragon_recon.tar.gz \
    74ac1d90989c9b1732edee82d57e9ce71452144cf4355f108d8c9c616d28d02f \
    dragon_recon/dragon_vrip.ply dragon_recon/README
