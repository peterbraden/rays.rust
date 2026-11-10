# Scene formats of other renderers

A survey of the scene description formats used by other ray tracers, what each would take to
read, and which are worth supporting. Written to feed workstream 1 of `plan.md` (standard test
scenes and reference comparisons): every scene converted by hand so far (`demo/reference/`) came
from one of these formats.

Status: the NFF loader (`src/nff.rs`) and the Tungsten importer (`src/tungsten.rs`, with
`src/shapes/wo3.rs`) are implemented; the rest is research.


## What we can represent today

The importer's ceiling is our own scene model (`src/scenefile.rs`), so the survey measures each
format against it:

- Geometry: sphere, triangle, OBJ and PLY meshes (ASCII and binary PLY; flat or smooth
  normals), axis-aligned box, horizontal plane, CSG difference, and a `transform` wrapper
  (scale, rotate, translate) around any of these.
- Materials: Lambertian, rough metal (`metal`), smooth dielectric with attenuation, plastic
  (diffuse base under a dielectric coat), Whitted (pigment, reflection, Phong), flat colour,
  diffuse emitter (one- or two-sided).
- Lights: point lights (inverse-square), and any emitting surface with area becomes a sampled
  area light.
- Environment: solid or gradient background, Rayleigh/Mie sky sphere.
- Camera: look-at pinhole or thin lens; our `angle` satisfies tan(angle) = image height at unit
  distance, so a vertical field of view `fov` maps to `atan(2 tan(fov/2))`.
- Render settings: resolution, samples, max depth, Russian roulette, box or tent filter, seed.

Common gaps across formats: textures and UVs, microfacet BRDFs (GGX/Beckmann), rough
dielectrics, environment maps, spot/directional lights, quads/disks/cylinders as primitives,
instancing, spectral data, participating media in the formats' sense.


## Formats

Each was checked against its own source or example files (fetched during this survey), not from
memory, except where marked.

### NFF (Neutral File Format), Eric Haines

- Where it is used: the Standard Procedural Database (SPD: balls, gears, mount, rings, tetra,
  teapot, tree), the classic ray tracing efficiency benchmark (Haines, "A Proposal for Standard
  Graphics Environments", IEEE CG&A 7(11), 1987). The SPD generators are on GitHub
  (`erich666/StandardProceduralDatabases`), CC0 with an MIT fallback; the spec is `NFF.TXT` in
  that repository (version 3.9, 1996).
- Shape of the format: line-oriented text, one entity per line, no nesting, no transforms. Eight
  entities:
  - `v` view: `from`, `at`, `up`, `angle` (degrees, full field of view, centre of top row to
    centre of bottom row), `hither`, `resolution`. Aspect ratio is always 1.
  - `b` background RGB; `l x y z [r g b]` point light, intensity "of no particular value".
  - `f r g b Kd Ks Shine T ior`: the current surface, applied to following objects.
  - `s` sphere; `p n` polygon (convex, coplanar, counter-clockwise front); `pp n` polygon with
    per-vertex normals; `c` cone or cylinder (base/apex points and radii, no end caps).
- Mapping:
  - `v` -> camera, `angle` -> `atan(2 tan(angle/2))`; `b` -> background; `l` -> point light.
  - `f` -> our `whitted` material (pigment = rgb x Kd, reflection = Ks, phong = Shine) when
    T = 0, `dielectric` with `ior` when T > 0. This is the shading model NFF was written for.
  - `s` -> sphere; `p` -> triangle fan; `pp` -> fan with vertex normals (needs an in-memory
    smooth mesh, today `smoothmesh` loads only from files).
  - `c` -> no equivalent. Of the seven classic scenes only `rings` and `tree` emit cylinders
    or cones (checked in the generators' source; the extra `jacks` and `lattice` do too), so a
    cylinder/cone primitive is needed for those. `pp` is emitted by `teapot`.
  - The SPD repository is C source: the NFF files come from building and running its
    generators (eg. `balls 3 > balls.nff`), with a size factor controlling primitive count.
  - Negative radii (inside-only) are unused by the SPD.
- Light intensity is undefined, so absolute brightness cannot match any reference; SPD
  references (POV-Ray PNGs) are qualitative only (as `plan.md` already notes).
- Verdict: **fully supportable except cones**, and small: a few hundred lines with no new
  dependencies. Good for performance benchmarking (the SPD scales from hundreds to millions of
  primitives with a size parameter) rather than for image comparison.

### Tungsten JSON (Benedikt Bitterli)

- Where it is used: Tungsten, and Bitterli's Rendering Resources: about 30 scenes, most CC0 or
  CC-BY, each zip carrying `scene.json` and the linear EXR reference our harness compares against.
  The Cornell box we already use came from this collection (via its pbrt-v4 version).
- Shape of the format: JSON, very close to ours. Top-level `bsdfs` (named, array), `primitives`
  (array, each with `type`, `transform`, `bsdf` by name, optional `emission`/`power`), `camera`,
  `integrator`, `renderer`, `media`.
- Conventions (from Tungsten's source, `tunabrain/tungsten`):
  - `transform` is `{position, rotation, scale}` or `{position, look_at, up}`, composed as
    T x R x S with R = `Mat4f::rotYXZ(rotation)`, angles in degrees. Our `transform` composes in
    the same order but with roll/pitch/yaw Euler angles, so an importer should build the
    rotation matrix itself (our `Transform::from_parts` takes a `Rotation3`).
  - `quad`: the unit square in the XZ plane, centred on the origin, edges along local x and z.
    `cube`: the unit cube centred on the origin. Both are placed by the transform.
  - `camera.fov` is the **horizontal** field of view in degrees (`_planeDist = 1/tan(fov/2)`).
  - Meshes are `.wo3` (Tungsten's binary: a u64 vertex count, vertices of position, normal and
    uv as 8 f32s, a u64 triangle count, triangles of 3 u32 indices and an i32 material) or
    `.obj`. `.wo3` is trivial to read.
- Mapping: `lambert` -> lambertian; `mirror` -> metal with roughness 0; `conductor` -> metal
  (Tungsten takes a complex IOR by material name, we take a colour: needs a small table of
  normal-incidence reflectances); `dielectric` -> dielectric; `plastic` -> plastic; `null` with
  `emission` -> diffuse-light; `rough_conductor`, `rough_plastic`, `rough_dielectric` -> only
  approximately (our roughness is not a microfacet alpha). `sphere`, `cube` (box under a
  transform), `quad` (two triangles, or a new quad primitive), `mesh` all map;
  `infinite_sphere` with a constant emission maps to a solid background; textured skies do not.
  `integrator.max_bounces` -> `max_depth`, `renderer.spp` -> `supersamples`,
  `camera.reconstruction_filter` -> `filter`.
- Verdict: **best candidate**. It is the closest to our format, its scenes come with EXR
  references, and it removes the hand conversion step for each new reference scene. The Cornell
  box imports fully; scenes using microfacet materials or textures import with warnings (and
  will not match their references until those features exist).
- Done: `src/tungsten.rs`. Translating Tungsten's own Cornell box example reproduces every
  vertex and face normal of the pbrt-v4 conversion in `demo/reference/cornell-box/`, and
  rendered with the same camera and filter it matches `demo/reference/cornell-box.json` to the
  noise floor (256x256, 256 spp: RMSE 0.0087 against 0.0089 between two seeds, exposure fit
  0.9996). Cylinders, curves, instancing, textures and media are skipped or approximated with
  a warning.

### pbrt-v3 / pbrt-v4

- Where it is used: pbrt (Pharr, Jakob, Humphreys, *Physically Based Rendering*), the
  `pbrt-v4-scenes` repository (about 7 GB, per-scene licences), Bitterli's pbrt-v4 conversions,
  and exporters from Blender and others. The largest public library of test scenes.
- Shape of the format: tokenised text with a graphics state. Directives (`LookAt`, `Camera`,
  `Film`, `Sampler`, `WorldBegin`, `AttributeBegin/End`, `Translate`, `Rotate`, `Scale`,
  `Transform`, `ConcatTransform`, `Material`, `MakeNamedMaterial`, `NamedMaterial`,
  `AreaLightSource`, `LightSource`, `Shape`, `ObjectBegin/Instance`, `Include`, `Import`), each
  with a parameter list of `"type name" [values]`. Transforms accumulate onto a current
  transformation matrix pushed and popped by attribute blocks. Example checked:
  `killeroos/killeroo-simple.pbrt`.
- Mapping (subset): the camera (`LookAt` plus perspective `fov`, which applies to the shorter
  image axis), the transform stack, `sphere`, `trianglemesh`, `plymesh` (binary PLY, which we
  read; many scenes gzip them), `diffuse` -> lambertian, `conductor` -> metal, `dielectric`,
  `coateddiffuse` -> plastic (approximately), `diffuse` area lights, `point` lights,
  constant `infinite` light -> background. Not mappable: textures (most scenes), image infinite
  lights, `bilinearmesh`, `loopsubdiv`, `curve`, measured BSDFs, spectral parameters, media.
- `scripts/pbrt_trianglemesh_to_obj.py` already does the geometry part for one scene.
- Verdict: **worth a partial importer later**. The tokenizer and graphics state are moderate
  work (larger than NFF and Tungsten combined); the return is limited until we have textures
  and microfacet materials, since few pbrt-v4 scenes avoid both.

### Mitsuba 2 / 3 XML

- Where it is used: Mitsuba, Bitterli's Mitsuba conversions, `mitsuba-data` test scenes, and
  our own dragon reference (`scripts/mitsuba_dragon_reference.py`). Example checked:
  `mitsuba-data/scenes/cbox/cbox.xml`.
- Shape of the format: XML. `<scene>` holding `<default>` parameters (`$name` substitution,
  like our `variables`), `<integrator>`, `<sensor>` (with `<film>`, `<sampler>`, `<rfilter>`),
  `<bsdf id>` referenced by `<ref id>`, `<shape type>` (obj, ply, serialized, sphere,
  rectangle, cube, disk, cylinder) with nested `<emitter>` and `<transform>` (lookat, translate,
  rotate, scale, matrix), and `<emitter>` (point, area, constant, envmap).
- Mapping: much like pbrt. Diffuse, conductor, roughconductor (approximately), dielectric,
  plastic, `twosided` wrapper (ours is already two-sided) and area/point/constant emitters map.
  `fov` with `fov_axis` (x, y, smaller, larger, diagonal) maps exactly. Spectra given as
  wavelength:value lists (as in `cbox.xml`) need reducing to RGB. `serialized` meshes are a
  Mitsuba-specific zlib format.
- Needs an XML parser (eg. the `quick-xml` crate).
- Verdict: **possible, lower priority than pbrt**: a similar feature ceiling and similar
  scene library (largely the same Bitterli scenes), plus a new dependency.

### Wavefront MTL (alongside the OBJ we already read)

- Where it is used: everywhere OBJ is; McGuire's Computer Graphics Archive (Sponza, San Miguel,
  and others, per-model licences) ships OBJ+MTL scenes.
- `tobj` already parses MTL; `src/shapes/mesh.rs` ignores the result. Fields: `Kd` diffuse,
  `Ks`/`Ns` specular and exponent, `Ni` index of refraction, `d`/`Tr` opacity, `Ke` emission,
  `illum` model, `map_*` textures.
- Mapping: per-face material from `Kd` (lambertian), `Ke` (diffuse-light), `illum` 3/5/7 or
  `d` < 1 (metal/dielectric). Needs a mesh to carry a material per triangle, which ours does not.
- Verdict: **cheap partial support** once meshes can carry several materials; textures wait
  for UVs.

### glTF 2.0 (Khronos)

- From prior knowledge, not checked in this survey. JSON plus binary buffers (or a single
  `.glb`); node hierarchy with TRS or matrix transforms, triangle meshes, cameras, the
  metallic-roughness PBR material, and lights via `KHR_lights_punctual`. Blender and most
  tools export it. The `gltf` crate reads it.
- Mapping: geometry, transforms, cameras and point lights map; the material is a GGX
  microfacet model with base-colour textures, which we lack.
- Verdict: **later**, as the route for importing modelled assets, once textures and a
  microfacet BRDF exist. Not a source of reference images.

### Not recommended

- POV-Ray SDL: a programming language (`#declare`, `#macro`, `#while`, include files such as
  `colors.inc`), CSG, and a large pigment/finish/normal texture system. Our Whitted material and
  checkered plane come from the same tradition, so a subset (sphere, box, plane, camera,
  `light_source`, `pigment { rgb }`, `finish`) would be easy, but real scene files lean on the
  language and includes, so a subset reads few of them.
- RenderMan RIB, USD, Blender `.blend`, LuxCore, Appleseed: large formats whose scenes need
  shading systems (RSL/OSL, MaterialX, node graphs) far beyond ours. Not checked in detail.


## Recommendation

1. **NFF loader** (small, self-contained, no dependency): gives the SPD for performance
   benchmarking of the BVH. Done: `src/nff.rs`. Cones and cylinders become the `cone` shape of
   the POV-Ray shapes PR (#90), so `rings` and `tree` load once that is merged.
2. **Tungsten importer**: directly serves the reference-comparison workstream, replacing
   hand conversion of Bitterli scenes. Includes a `.wo3` reader. Unsupported BSDFs and
   primitives are reported, not silently dropped. Done: `src/tungsten.rs`.
3. MTL materials on OBJ meshes, once meshes carry per-triangle materials.
4. A pbrt-v4 subset, once textures and a microfacet BRDF land (`plan.md` workstreams on
   materials).

Design: rather than building a `Scene` directly, each importer would translate the foreign file
into our scene JSON (a `serde_json::Value`) and hand that to the existing parser, picking the
importer by extension (`.nff`, `.pbrt`, `.xml`) or, for Tungsten's `.json`, by its top-level
`primitives` key. That keeps one path from JSON to `Scene`, lets a `--dump-scene` option print
the translation for inspection or hand editing (the way `demo/reference/*.json` are kept today),
and makes each importer testable as JSON in, JSON out. Features we cannot represent produce a
warning naming the entity, the way unknown plastic `opacity` does today.

New primitives that more than one format needs: a quad (Tungsten, Mitsuba `rectangle`, pbrt
bilinear patches of four vertices), a cylinder/cone (NFF, Tungsten, Mitsuba, pbrt), and a disk.
