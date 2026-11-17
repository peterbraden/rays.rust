# Displacement and function graphs

![Before and after: the same heightmap, plain and displaced](displacement-comparison.png)

*`demo/scenes/displaced-terrain.json`: a sandstone escarpment after the Wingate cliffs of the
Colorado Plateau, about 30 m high, seen along its length in raking late-afternoon light, over a
talus apron and a sandy floor. Top, the 1 m heightmap as triangles, with the same materials.
Bottom, the same heightmap displaced at render time by a function graph: buttresses between
weathered joints, alcoves, bedding, fallen blocks on the talus and outcrops in the sand. The
relief casts its own shadows and changes the cliff's outline, which bump mapping can't do. The
dark streaks of desert varnish are a graph too, the amount of a `mix` of two rocks.*

A terrain stops at its heightmap's resolution. Rock faces, scree and ledges read as real only
when their silhouettes and self-shadowing are real, and bump mapping fails exactly where most of
a landscape is seen: at grazing angles and against the sky. Terragen's answer is displacement
evaluated at render time, down to the size of a pixel, driven by a network of functions. This is
ours.

## Function graphs (`src/graph.rs`)

A graph is a small tree of functions written in the scene file as nested JSON: a number, an
expression string, or an object with `"node"`. Define a node once in the scene's `variables` and
reuse it as `"$name"`.

| Node | What it gives |
|------|---------------|
| `fractal` | Perlin noise over octaves from `scale` down to `smallest` metres; `style` `perlin`, `ridged` or `billow`; `roughness`, `lacunarity`, `amplitude`, `seed`, `plan` (2D) |
| `cells` | Worley noise: `style` `f1`, `edges` (cracks between cells) or `stones` (a rounded stone per cell) |
| `strata` | Ledges: beds `step` metres thick, each ending in a riser; `breakup` makes them waver |
| `slope`, `altitude` | Masks from 0 to 1 between `from` and `to` (degrees, metres), with a fractal `breakup` |
| `add`, `multiply`, `min`, `max`, `subtract` | Arithmetic |
| `blend` | `a` where `mask` is 0, `b` where 1 |
| `clamp`, `abs`, `power`, `smoothstep`, `remap`, `terrace` | Shaping |
| `warp` | Domain distortion: `input` read at a point moved by noise (`amount`, `scale`) or by nodes `x`, `y`, `z` |
| `expr` | An expression (`expr.rs`) over `x`, `y`, `z`, `slope` and named `inputs`, clamped to `range` |

A graph can also be a material's texture: wherever a colour texture is read (a `mix`'s
`amount`, an albedo), an object with `"node"` is a graph, evaluated at the hit with its normal.
The demo uses one to put sandstone on steep faces and sand on flats and ledge tops:
`"GROUND": {"type": "mix", "materials": ["SAND", "SANDSTONE"], "amount": {"node": "slope", "from": 28, "to": 38}}`.
Its varnish is plan-view noise (`"plan": true`, so constant down a vertical face, as streaks
are) masked to the steepest, highest rock, mixing a dark rock into the sandstone.

Every node knows its range by interval arithmetic and clamps to it, so a displaced surface can
be bounded before it is evaluated. Fractals and cells are filtered: told the spacing of the
vertices where they are evaluated, they fade out octaves finer than twice that, putting each
octave's mean in its place, so a coarsely diced distant surface doesn't alias or shift.

## Displaced terrain (`src/shapes/displaced.rs`)

```json
{ "type": "terrain", "heightmap": {...}, "material": "ROCK",
  "displacement": { "node": "blend", "a": "$scree", "b": "$rock", "mask": "$outcrops" },
  "displacement_detail": { "pixels": 1, "max_level": 8 } }
```

- **Smooth base.** The heightmap is interpolated by Catmull-Rom splines (`HeightGrid::smooth`),
  continuous in height and slope, so no facets of the grid show.
- **Along the normal.** Points move along the base's normal, so cliffs get relief, not just level
  ground (`"displacement_direction": "vertical"` moves them straight up). The graph sees the
  base's normal, for `slope` masks.
- **Diced to the pixel.** Each heightmap cell is diced into 2^k x 2^k quads, `k` chosen once per
  cell from the camera so a quad is about `pixels` pixels across, up to `max_level`, and no finer
  than half the graph's finest detail (`min_spacing` overrides). A cell's size is the longer of
  its width and its height range, so a 1 m cell spanning 20 m of cliff is diced finely enough
  up the face, not smeared into vertical streaks. As in Reyes (Cook, Carpenter and
  Catmull 1987) and Pharr and Hanrahan's geometry caching (1996), but per cell and lazily: a
  cell is diced the first time a ray comes near it, then kept. Each diced cell keeps a quadtree
  of boxes that rays descend nearest first; a BVH over the cells' conservative bounds finds
  them.
- **Same surface for every ray.** The level depends on the cell, not the ray, so camera, shadow
  and bounce rays all meet the same geometry.
- **No cracks.** Where a cell meets a coarser neighbour, its edge follows the neighbour's; points
  on edges and corners are evaluated with the filter of the coarsest cell sharing them, so both
  sides compute bit-identical points (`test_no_cracks_between_levels` fires 20,000 rays at the
  seams).
- **Memory.** The scene prints how many vertices it could need if every cell were seen; only
  the cells rays reach are diced.

Things placed on a heightmap (grass, rocks, trees) stand on its grid, not on the displaced
surface, so keep displacement small or masked off where they are.

## What next

- Fuller surface layers (more than two materials, masks from the simulations' fields) on the
  same graphs (Terragen gap 2), and media densities.
- Put the `landscape` scenery's clipmap and the desert branch's `mesas` (whose overlook uses
  nested fixed grids down to 8 cm) through displacement, so one coarse grid does.
- Vector displacement for overhangs.
