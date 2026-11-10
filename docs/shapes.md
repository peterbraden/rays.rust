# Shapes

The scene file's `objects` take these geometry types besides `sphere`, `plane`, `box`,
`triangle`, `mesh` and `smoothmesh`. They follow POV-Ray's shapes of the same names, and use the
same coordinates (except that POV-Ray is left-handed: see `text`). Any of them can be moved,
turned and stretched (see [Placing shapes](#placing-shapes)), given a `material`, and (the solid
ones) combined with others (see [Combining shapes](#combining-shapes-csg)).

![Every shape](../demo/scenes/shapes.png)

[`demo/scenes/shapes.json`](../demo/scenes/shapes.json) has an example of each.

Points are `[x, y, z]`, 2D points `[a, b]`. Fields marked * are required; the rest show their
defaults.

## Quadric shapes

| Type | Fields | |
|---|---|---|
| `cylinder` | `base`*, `cap`*, `radius`*, `open: false` | Closed by flat ends unless `open`. |
| `cone` | `base`*, `base_radius`*, `cap`*, `cap_radius`*, `open: false` | A truncated cone. |
| `disc` | `center`*, `normal`*, `radius`*, `hole_radius: 0` | Flat, with an optional hole. Can be an area light. |

## Polynomial surfaces

Solved exactly as polynomials along the ray. Inside is where the polynomial is negative.

| Type | Fields | |
|---|---|---|
| `torus` | `major`*, `minor`* | About the y axis, centred on the origin. |
| `quadric` | `squares: [A, B, C]`, `mixed: [D, E, F]`, `linear: [G, H, I]`, `constant: J`, `bounds` | A x² + B y² + C z² + D xy + E xz + F yz + G x + H y + I z + J = 0. |
| `poly`, `cubic`, `quartic` | `order` (for `poly`), `coefficients`, or `terms`; `bounds` | `coefficients` in POV-Ray's order (x², xy, xz, x, y², yz, y, z², z, 1 for order 2), or `terms: [[c, i, j, k], ..]` for c xⁱ yʲ zᵏ. |

The polynomial surfaces may be unbounded; `bounds: {"min": [..], "max": [..]}` clips them to a
box.

## Outlines and revolutions

| Type | Fields | |
|---|---|---|
| `polygon` | `points`* | Flat, in the plane of its 3D points. A loop closes when its first point repeats; a loop inside another is a hole. |
| `prism` | `points`* (2D, [x, z]), `height1`*, `height2`*, `spline: "linear"`, `sweep: "linear"`, `open: false`, `steps: 16` | An outline in the xz plane swept along y. `spline: "bezier"` takes four points per segment. `sweep: "conic"` scales the outline by its height, towards an apex at the origin. |
| `lathe` | `points`* (2D, [radius, y]), `spline: "linear"` | The profile spun about y. `"cubic"` is a Catmull-Rom curve through all but the first and last points; `"bezier"` takes four points per segment. |
| `sor` | `points`* (2D, [radius, y]), `open: false` | Surface of revolution: squared radius is a cubic in y through all but the first and last points. Heights must increase, and the squared radius must stay positive between points. |
| `sphere_sweep` | `spheres`* (`[x, y, z, radius]`), `spline: "linear"`, `steps: 16` | A sphere moved along a path. `"cubic"` passes through all but the first and last spheres; `"b_spline"` passes near them. Curves are sampled into `steps` straight pieces per segment. |

## Fields and functions

| Type | Fields | |
|---|---|---|
| `blob` | `threshold`*, `components`* | Components are `{"type": "sphere", "center", "radius", "strength": 1}` or `{"type": "cylinder", "base", "cap", "radius", "strength": 1}`. Each adds strength × (1 − d²/r²)² within its radius; the surface is where the total reaches the threshold. Negative strengths carve away. |
| `isosurface` | `function`*, `threshold: 0`, `contained_by`, `max_gradient: 1.1`, `accuracy: 0.001`, `open: false` | The surface where the function of x, y, z equals the threshold. `contained_by` is `{"box": {"min", "max"}}` (default the cube from -1 to 1) or `{"sphere": {"center", "radius"}}`. `max_gradient` must be at least the function's steepest slope, or holes appear. |
| `superellipsoid` | `e`*, `n`* | (\|x\|^(2/e) + \|y\|^(2/e))^(e/n) + \|z\|^(2/n) = 1, in the cube from -1 to 1. |
| `julia_fractal` | `c`* (`[a, b, c, d]`), `slice: 0`, `max_iteration: 20`, `accuracy: 0.0001` | The quaternion Julia set of q² + c, at fourth component `slice`. |

Functions are written as text in x, y and z (u and v for `parametric`): numbers, `+ - * /`, `^`
for powers, parentheses, `pi`, `e`, and `sin cos tan asin acos atan atan2 sinh cosh tanh sqrt abs
exp ln log floor ceil sqr pow min max mod noise3d select`.

## Meshes made from other shapes

These are turned into triangles when the scene loads.

| Type | Fields | |
|---|---|---|
| `height_field` | `src` (an image) or `function` (of x and z) with `resolution: [128, 128]`; `smooth: false`, `water_level: 0` | Over the unit square in x and z, heights 0 to 1 from the image's brightness. The image's top row is at z = 1. |
| `parametric` | `x`*, `y`*, `z`* (functions of u and v), `u: [0, 1]`, `v: [0, 1]`, `resolution: [64, 64]` | |
| `bicubic_patch` | `points`* (16: four rows of four), `steps: 16` | A Bezier patch. |
| `text` | `font`* (a .ttf or .otf file), `string`*, `thickness: 1`, `offset: [0, 0]`, `steps: 8` | Letters about one unit high on the baseline y = 0 from x = 0, from z = 0 to z = thickness. In this right-handed renderer they read left to right from the +z side; turn them 180° about y (`"rotate": {"pitch": 180}`) to face a camera on the -z side. |

## Placing shapes

Any shape can have `"transform": [steps]`, applied in order, like POV-Ray's transformations on
an object:

```json
{ "type": "torus", "major": 1, "minor": 0.25, "material": "GOLD",
  "transform": [ { "scale": [1, 1, 0.5] }, { "rotate": [90, 0, 30] }, { "translate": [0, 1, 0] } ] }
```

| Step | |
|---|---|
| `{"translate": [x, y, z]}` | Move. |
| `{"rotate": [x, y, z]}` | Turn by degrees about the x axis, then y, then z, each anticlockwise looking down the axis towards the origin (the right-hand rule; POV-Ray, being left-handed, turns the other way). `{"roll": x, "pitch": y, "yaw": z}` names the same angles. |
| `{"scale": s}` or `{"scale": [x, y, z]}` | Stretch about the origin. Negative values mirror; zero is refused. |
| `{"axis_rotate": {"axis": [x, y, z], "angle": a}}` | Turn by `a` degrees about any axis through the origin. |
| `{"matrix": [12 numbers]}` | Any affine transformation, in POV-Ray's order: three rows of the linear part, then the translation, so (x, y, z) goes to x row₁ + y row₂ + z row₃ + row₄. It must not flatten the shape. |

The steps move the shape whole, including what it is `clipped_by`. Order matters: translating
then rotating swings the shape round the origin, while rotating then translating turns it in
place. Uniformly scaled emitters can still be sampled as area lights after any rotation.

The `{"type": "transform", "item": shape, ...}` wrapper takes the same list as `"steps"`, or the
older fixed form `"scale"`, then `"rotate": {"roll", "pitch", "yaw"}`, then `"translate"`.

Materials and bump patterns stay fixed in the world: a moved shape moves through its texture
rather than taking it along.

## Combining shapes (CSG)

| Type | Fields | |
|---|---|---|
| `union` | `items`* | All the shapes, keeping the surfaces where they overlap (seen through glass). |
| `merge` | `items`* | All the shapes, without the surfaces inside one another. |
| `intersection` | `items`* | Where the shapes all overlap. |
| `difference` | `items`* (or `a`*, `b`*) | The first shape with the others cut away. |

![CSG](../demo/scenes/csg.png)

[`demo/scenes/csg.json`](../demo/scenes/csg.json) has an example of each. Combinations nest, and
can be placed with `transform` steps like any shape. Any shape can also have:

- `"inverse": true`: turned inside out, so that its inside is everything outside it. An
  intersection with an inverse shape is a difference.
- `"clipped_by": shape`: the parts of its surface outside the other shape cut away, leaving it
  open. Unlike an intersection, this adds no surface.

A shape inside a combination (or a `transform`) can have its own `material` or `medium`, which
then applies to the surface it contributes: the cut faces of a difference take the cutting
shape's material. The innermost material wins; shapes without one use the object's.

`{"type": "plane", "normal": [x, y, z], "distance": d}` is a half-space, everything on the
side of the plane n·p = d that the normal faces away from, for cutting shapes in two. (The
`"y"` form of `plane` is a ground plane, best kept out of CSG.)

Combinations work as in POV-Ray: each shape's surface is kept where the others say it should
be, by whether it is inside them. So the shapes in an `intersection`, `difference` or `merge`
should be solids: closed, with an inside. `disc`, `polygon`, `height_field`, `parametric` and
`bicubic_patch` have no inside, and open meshes and `open` cylinders, cones and sors have none
that holds together, so they won't cut or be cut cleanly. Where two shapes' surfaces coincide
exactly, either may show; let cutting shapes overlap a little.
