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

## Flora by biome

On a landscape with a climate, an ecosystem can take its plants from the biomes instead of a
list: `"flora": true` (`terrain::flora`).

```json
{ "type": "ecosystem", "seed": 4, "surface": "landscape", "flora": true }
```

The climate gives every point a biome (tundra, taiga, temperate forest, grassland, steppe,
cold and hot desert, savanna, rainforest, ice), its edges wandering with the weather from year
to year, and lower with height as the air cools. A table says what grows in each, from the
species the library has:

| biome | trees | shrubs | ground |
|---|---|---|---|
| tundra | | juniper, shrub (dwarf, 0.6 m) | tussock, fescue, heather |
| taiga | Norway spruce, balsam fir, Scots pine, aspen | juniper | fescue, heather, bracken, litter |
| temperate forest | oak, black tupelo, aspen, Scots pine, spruce; willow by water | hazel, hawthorn, shrub | meadow, bent, bracken, litter |
| grassland | a few oaks, aspens and pines; willow by water | hawthorn | meadow grasses and flowers |
| steppe | Utah juniper; willow by water | sagebrush, rabbitbrush, juniper, prickly pear | fescue, tussock, wild oat |
| cold desert | Utah juniper, Joshua tree | sagebrush, rabbitbrush | sparse tussock |
| hot desert | saguaro, Joshua tree | creosote bush, ocotillo, barrel cactus, prickly pear, agave | |
| savanna | stone pine (for acacia), palm | shrub | savanna grass |
| rainforest | palm, black tupelo | shrub | bracken, litter |

Each plant has an abundance in each biome it belongs to, and is a population of its own, so
where one biome gives way to the next its plants give way to the next's. Its rule is its
abundance in the biome at each point, on ground no steeper than it holds, off ice, dunes and
playas, a fifth as thick on bare rock; trees below the treeline, shrubs below the snow; willows only where water gathers
(`wetness`). Trees come first, then shrubs, then each biome's ground plants, so each grows in
the room those before it leave. Only the biomes found about the camera (12 x 12 points over
300 m) choose plants, and the load prints them:

```
Flora: tundra 100% about the camera; 3 populations: juniper, shrub, 3 ground
```

`"flora"` can also be `{"strata": ["trees", "shrubs", "ground"], "reach": 300, "samples": 12}`,
to grow only some layers, or survey further. Populations the ecosystem lists itself (rocks,
say) are placed after the flora's. Nothing grows within `"clear"` metres of the camera (6 with
a flora, 0 otherwise), so it doesn't stand inside a tree.
