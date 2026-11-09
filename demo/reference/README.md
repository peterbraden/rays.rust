# Reference scenes

Scenes from other renderers, converted to our format, with published reference images to
compare against. The references themselves are not committed: `scripts/fetch-references.sh`
downloads them into `references/`, checking each archive's SHA-256.

## Cornell box

- Source: Benedikt Bitterli, "Rendering Resources", <https://benedikt-bitterli.me/resources/>,
  scene "Cornell Box". Released under CC0 (public domain); see `LICENSE.txt` in the archive.
- Archive: <https://benedikt-bitterli.me/resources/pbrt-v4/cornell-box.zip>, which carries the
  pbrt-v4 scene and the Tungsten reference render (`TungstenRender.exr`: 1024x1024, linear,
  half-float, PIZ-compressed; Tungsten path tracer, 64 samples per pixel with adaptive sampling,
  up to 64 bounces).
- The box descends from Goral, Torrance, Greenberg and Battaile, "Modeling the Interaction of
  Light Between Diffuse Surfaces", SIGGRAPH 1984, doi:10.1145/964965.808601; this two-block,
  ceiling-light configuration is a later Cornell version.

Conversion (`cornell-box.json`, `cornell-box/*.obj`):

- Geometry: every surface in the pbrt-v4 scene is an explicit triangle mesh;
  `scripts/pbrt_trianglemesh_to_obj.py` copies them into one OBJ per material, keeping vertex
  order, so the light's winding (and so the side it emits from) matches pbrt.
- Materials: Lambertian, with the scene's RGB reflectances. The light is a one-sided
  `diffuse-light` of radiance (17, 12, 4).
- Camera: at (0, 1, 6.8) looking at (0, 1, 0), field of view 19.5 degrees (square image, so
  horizontal and vertical agree). Our camera's `angle` is defined so that tan(angle) is the
  image's full height at unit distance, so `angle` = atan(2 tan(19.5/2 degrees)) = 0.331018.
- Not matched: the reference uses light sampling and adaptive sampling; we have no area-light
  sampling, so our render converges more slowly on this small light.
