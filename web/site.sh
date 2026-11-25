#!/bin/sh
# Assemble the scene builder as a static site in the directory given (default _site): the page
# under web/, and the demo scenes and meshes under demo/, where the page fetches them from.
# Run scripts/build-web.sh first.
set -e
cd "$(dirname "$0")/.."
out="${1:-_site}"
test -f web/www/pkg/rays_web_bg.wasm || { echo "web/www/pkg is missing: run scripts/build-web.sh first" >&2; exit 1; }
rm -rf "$out"
mkdir -p "$out/web/www/pkg" "$out/demo/scenes"
cp web/index.html web/style.css web/app.js web/worker.js web/gpu-worker.js web/rays.js web/raycaster.js web/planet.js web/planet-worker.js "$out/web/"
cp web/www/pkg/rays_web.js web/www/pkg/rays_web_bg.wasm "$out/web/www/pkg/"
# Scenes and the files they name; not the rendered images, which the page doesn't use.
cp demo/*.json demo/*.obj "$out/demo/"
cp demo/scenes/*.json "$out/demo/scenes/"
# The site's front page is the builder.
cat > "$out/index.html" <<'HTML'
<!doctype html>
<meta charset="utf-8">
<title>Rays</title>
<meta http-equiv="refresh" content="0; url=web/">
<a href="web/">Rays scene builder</a>
HTML
# No Jekyll processing: serve the files as they are.
touch "$out/.nojekyll"
du -sh "$out" | cut -f1
