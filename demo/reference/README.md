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
- Not matched: the reference uses adaptive sampling and a tent reconstruction filter; each of
  our pixels averages samples over its own area.

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
RMSE now barely falls with more samples: what remains is a systematic difference of about
0.017, which the FLIP map places at the light's front and back edges, where a radiance of 17
makes pixels sensitive to the reconstruction filter.

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
