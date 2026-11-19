# Placement by rules: ecosystems

![An alpine meadow: tussock grass, junipers and boulders below a snowy range](../demo/scenes/alpine-meadow.png)

*`demo/scenes/alpine-meadow.json`: one `ecosystem` on the landscape of `alpine-layers.json`,
standing in the valley at 2,960 m. Junipers where the ground is gentle, in patches; boulders
in clusters, in the room the junipers leave; tussock and fescue grass on all but the steepest
ground, thinning out with distance.*

Trees, shrubs, rocks and grass are placed by **rules**: function graphs
([displacement.md](displacement.md)) from 0 to 1, read on the ground where each thing would
stand, with its normal. Where a rule is 1 things grow as thickly as they can; where it is 0,
not at all. A rule can use:

- `slope` and `altitude` (on a landscape, by the planet's up and height above it), `aspect`
  (which way the ground faces);
- noise (`fractal`, `cells`), to break up edges and make patches;
- on a landscape, its simulation's fields (`field`: sediment, wetness, flow, ice,
  `snowline`, `treeline`, temperature, precipitation) and its climate's biome (`biome`, eg.
  `{"node": "biome", "is": ["taiga", "temperate_forest"]}`).

Every placing object takes its rule as `"where"`: a `forest`, a `scatter` (also as `density`),
`grass` (where it multiplies its conditions' own `max_slope` and altitude limits). Each can
stand on a geometry, or on the scene's landscape (`"surface": "landscape"`; grass,
`"heightmap": "landscape"`), after the landscape in the scene's objects.

```json
"where": { "node": "multiply", "inputs": [
    { "node": "field", "field": "treeline", "from": 50, "to": -150 },
    { "node": "slope", "from": 35, "to": 25 },
    { "node": "biome", "is": ["taiga", "temperate_forest"] } ] }
```

## Ecosystems

```json
{ "type": "ecosystem", "seed": 3, "surface": "landscape", "populations": [
    { "forest": { "reach": 150, "spacing": 7, "species": [{ "species": "juniper" }], "where": {...} } },
    { "scatter": { "reach": 100, "spacing": 3, "items": [{ "boulder": {}, "size": [0.15, 1.2] }] } },
    { "grass": { "reach": 30, "density": 350, "species": ["tussock", "fescue"], "where": {...} } } ] }
```

An ecosystem places its populations together, in the order listed, so that they compete for
the ground (`procedural::ecosystem`), as Deussen et al. place plants (SIGGRAPH '98):

- Each tree or rock claims a disc where nothing after it grows: its `base`, a fraction of its
  spacing radius for trees (0.25, the trunk and roots), of its size for rocks (0.9).
- Trees shade a disc around them, their `canopy` (1.3 of their spacing radius), where what
  comes after grows `shade` (0.6) less thickly, deepest by the trunk.
- So list the largest first: trees, then shrubs, then rocks, then grass.

Each population is written as its own object would be, and takes the ecosystem's `surface`,
`seed` and `area` unless it has its own. Without an `area`, it covers the ground about the
camera, `reach` metres out (300 for forests, 150 for scatters, 25 for grass), and grass thins
out over the last 40% of that (`fade`), in a circle, so there is no edge to see. Grass grows on
the ecosystem's surface itself, sampled every `cell` metres (0.25).

Trees far off cost little: give the forest a `"lod"` (see [adaptive meshes](adaptive-meshes.md)).
