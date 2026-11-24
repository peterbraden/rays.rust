# Scatter

Copies of things strewn over a surface: boulders on a moraine, stones on scree, logs in a wood.
They are what Terragen calls populations, and what a forest is for trees.

```json
{ "type": "scatter", "seed": 2, "surface": "landscape",
  "area": { "min": [3500, 15380], "max": [3720, 15620] }, "spacing": 4,
  "density": { "node": "multiply", "inputs": [
      { "node": "field", "field": "sediment", "from": 0.02, "to": 0.6 },
      { "node": "slope", "from": 45, "to": 35 } ] },
  "items": [ { "boulder": { "angularity": 0.7 }, "variants": 5, "size": [0.2, 1.6],
               "sink": 0.15, "tilt": 10, "material": "RockA" } ] }
```

- **Where.** Over `area` (x and z) of the `surface`: a named or inline geometry, a list of them
  (copies stand on the highest), the plane y = 0 by default, or `"landscape"` for the scene's
  landscape object listed before the scatter.
- **How thickly.** `density` (or `where`, as forests and grass call it) is a [function graph](displacement.md) from 0 to 1, evaluated on
  the surface where each copy would stand, with its normal: `slope` and noise masks, and on a
  landscape its simulation's fields, so boulders lie where the simulation left scree. Where it
  is lower, copies are fewer and further apart.
- **How they are spaced.** As a forest's trees (`procedural::forest::place_favouring`): no two
  closer than `spacing` in proportion to their sizes, so big boulders keep their distance and
  small ones fill between them, with `patchiness` grouping items of a kind and `clearings`
  leaving gaps. `max_slope` leaves out steep ground. Alone, that spreads them evenly, filling
  the ground; `clustering` (0 to 1) gathers them into clumps with open ground between, as
  rockfall leaves its blocks: each copy lands beside one already placed with that chance,
  clumps start elsewhere only rarely, and each grows to a size of its own (even up to about
  0.4, clumps from 0.5, strongly clumped by 0.7; a landscape's own boulders take 0.65).
- **Items.** A named or inline `"geometry"` with its `"material"`, or seeded boulders
  (`"boulder": {"angularity", "detail"}`, `variants` of them, granite by default). Each has a
  `weight`, a `spacing` relative to the scatter's, and a `size`: one scale, or `[smallest,
  largest]`, spread evenly between them. `sink` buries each by that fraction of its size;
  `align` (0.7) leans it with the slope, from upright (0) to square to the ground (1); `tilt`
  leans it at random by up to that many degrees besides. Every copy turns to a random heading
  and half are mirrored.
- **Cost.** A copy is a transform of a shared item, so memory goes with the distinct items,
  not the copies: `alpine-closeup.json` strews thousands of boulders from five.

Like a forest, a scatter doesn't draw the surface it stands on: that is an object of its own.

## A landscape's own boulders

A full-scale landscape that draws its own detail (see [displacement](displacement.md),
"Procedural by default") also strews its own boulders about the camera, without a scatter in
the scene (`terrain::object::boulders`; `"boulders": false` for none, `true` to have them with
a displacement graph of the scene's own): fallen blocks on scree below steep ground, erratics in
patches on till, none on sand, playas, alluvial flats or ice. They are the rock most often bare
round the camera, in its `rock` material style and shape (granite rounded, sandstone and
limestone blocky, shale and schist in slabs, basalt angular), out to 30 times the camera's
height (60 to 300 m).
