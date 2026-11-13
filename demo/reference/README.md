# Reference scenes

Scenes from other renderers, converted to our format, with published reference images to
compare against. The references themselves are not committed: `scripts/fetch-references.sh`
downloads them into `references/`, checking each archive's SHA-256.

Tungsten scenes (Bitterli's archives under `resources/tungsten/`) load directly, without
converting: `cargo run --release -- <dir>/scene.json -o out.exr --compare <dir>/TungstenRender.exr`
(see `src/tungsten.rs` for what is approximated). `--dump-scene` writes the translation, to keep
or edit as one of the scenes here.

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
- Reconstruction filter: the reference uses a tent of radius one pixel at 1024x1024. Compared at
  full resolution, use `"filter": "tent"`. Compared at a lower resolution, the harness averages
  the reference down, which acts as a box filter at that resolution, so keep the default box.
- Not matched: the reference uses adaptive sampling.

## Comparing

```
# Fetch the references, then render and compare (the reference is averaged down to our size)
scripts/fetch-references.sh
cargo run --release -- demo/reference/cornell-box.json -o cornell.exr --compare references/cornell-box/TungstenRender.exr
```

At 256x256, with area-light sampling (#73):

| Samples per pixel | Exposure fit | RMSE (fitted) | Mean FLIP |
|---|---|---|---|
| 256 | 0.9919 | 0.0184 (0.0171) | 0.026 |
| 1024 | 0.9922 | 0.0172 (0.0159) | 0.018 |

Without area-light sampling (#72), 1024 samples per pixel gave RMSE 0.0280 and FLIP 0.078. The
RMSE barely falls with more samples: what remains is a systematic difference of about 0.017,
which the FLIP map places at the light's front and back edges.

That residual is the reconstruction filter. At the reference's own resolution (1024x1024, 64
samples per pixel, as the reference):

| Filter | Exposure fit | RMSE (fitted) | Mean FLIP |
|---|---|---|---|
| Box | 0.9924 | 0.0360 (0.0354) | 0.038 |
| Tent | 0.9983 | 0.0142 (0.0141) | 0.037 |

With the tent filter the outline around the light disappears from the FLIP map, RMSE falls by
2.5x and the exposure fit comes within 0.2%. FLIP at 64 samples per pixel is dominated by the
noise in both images.


## Ray Tracing in One Weekend: "view-distant"

- Source: Peter Shirley, Trevor David Black, Steve Hollasch, "Ray Tracing in One Weekend",
  v4.0.2, 2025, <https://raytracing.github.io/books/RayTracingInOneWeekend.html>, listing
  "Scene with alternate viewpoint" and image `img-1.20-view-distant.png` (400x225, 100 samples
  per pixel, gamma-2 encoded 8-bit PNG). CC0.
- Scene (`rtiow-view-distant.json`), as in the listing: a ground sphere, a diffuse sphere, a
  glass sphere (index 1.5) containing an air bubble (index 1/1.5, relative to the glass), and a
  fuzzy metal sphere (fuzz 1.0); camera at (-2, 2, 1) looking at (0, 0, -1), vertical field of
  view 90 degrees, so `angle` = atan(2 tan(45 degrees)) = atan(2); the book's white to
  (0.5, 0.7, 1) gradient background; gamma-2 output.
- Differences in the models, not the scene:
  - Metal fuzz: the book adds a point on the unit sphere to the normalised reflection; we add
    a point within the unit ball.
  - Fresnel: the book's Schlick term uses the incident cosine on both sides; ours uses the
    cosine on the less dense side (#39).
  - Quantisation: the book truncates, int(256 clamp(v, 0, 0.999)); we round, round(255 v).

```
cargo run --release -- demo/reference/rtiow-view-distant.json -o rtiow.png --compare references/rtiow-view-distant/img-1.20-view-distant.png
```

Results (8-bit references are decoded to linear with the scene's transfer function):

| Comparison | Exposure fit | RMSE | Mean FLIP |
|---|---|---|---|
| Ours against the book | 1.0018 | 0.0080 | 0.032 |
| Ours with the book's metal fuzz and Fresnel term | 1.0021 | 0.0074 | 0.030 |
| Ours against ours with another seed (noise floor) | 0.9999 | 0.0072 | 0.017 |

RMSE is close to the noise floor of two 100-sample renders. The FLIP map shows the rest in the
sky as banding, consistent with truncation against rounding, and on the glass and metal spheres,
where the model differences above make up about 0.0015 of it.


## Stanford dragon

- Model: Stanford 3D Scanning Repository, "Dragon",
  <https://graphics.stanford.edu/data/3Dscanrep/>. Dragon model courtesy of the Stanford Computer
  Graphics Laboratory. The repository permits research use, free redistribution, and publishing
  images made with the models with credit to the laboratory; not commercial use, or use in a
  product for sale, without permission. It also asks that the dragon not be animated, morphed,
  broken or subjected to Boolean operations.
- Reconstruction: Brian Curless and Marc Levoy, "A Volumetric Method for Building Complex Models
  from Range Images", SIGGRAPH 1996, pp. 303-312, doi:10.1145/237170.237269
  (<https://graphics.stanford.edu/papers/volrange/>): about 70 range scans merged as a
  cumulative weighted signed distance function, from which the surface is extracted. We use the
  full-resolution `dragon_vrip.ply` (ASCII PLY, 437,645 vertices, 871,414 triangles; the web
  page's 1,132,830 refers to an earlier reconstruction), loaded by `src/shapes/ply.rs`.
- Reference: Stanford publishes no reference image, and pbrt-v4-scenes' dragon uses subsurface
  scattering, which we lack. The reference is rendered with Mitsuba 3 (version 3.9.1, `path`
  integrator, 1024 samples per pixel) by `scripts/mitsuba_dragon_reference.py`, from the same PLY
  and ground OBJ.
- Scene (`stanford-dragon.json`): a white clay dragon (Lambertian, albedo 0.8) on a grey
  ground square (0.5), lit only by a uniform white sky (the background, radiance 1). This
  isolates geometry: with no lights or glossy materials to sample, any difference comes from
  intersection, self-shadowing and visibility on 871k small triangles. The dragon is scaled 50x by
  a `transform` and set on the ground; flat shading, as the PLY has no normals (Mitsuba's
  `face_normals`), and two-sided diffuse, as our Lambertian is.

```
# Fetch the model, render the reference (pip install mitsuba==3.9.1), then render and compare
scripts/fetch-references.sh
python scripts/mitsuba_dragon_reference.py references/stanford-dragon/dragon_vrip.ply demo/reference/stanford-dragon/ground.obj references/stanford-dragon/MitsubaRender.exr
cargo run --release -- demo/reference/stanford-dragon.json -o dragon.png --compare references/stanford-dragon/MitsubaRender.exr
```

Results, at 640x480 against Mitsuba at 1024 samples per pixel:

| Comparison | Exposure fit | RMSE | Mean FLIP |
|---|---|---|---|
| Ours, 256 samples per pixel | 1.0000 | 0.0079 | 0.0097 |
| Ours, 1024 samples per pixel | 1.0000 | 0.0054 | 0.0080 |
| Ours against ours with another seed, 1024 (noise floor) | 1.0000 | 0.0047 | 0.0061 |

The difference is at the noise floor: the FLIP map shows only noise, with no structure at the
dragon's edges or in its crevices. Before #82 the dragon did not appear at all under the
transform (see `src/shapes/triangle.rs`).
