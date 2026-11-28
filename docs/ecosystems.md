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
to year, and lower with height as the air cools. Biomes meet in ecotones, not along lines: the
climate that decides them is blurred by patchy noise a few hundred metres across and by a
dither (`scenery::Ground::wandering_climate`), so near the line each biome holds ground in
tongues and islands within the other, and each tree's candidate draws its own. Rain changes over
tens of kilometres, so ecotones between wetter and drier biomes are kilometres wide; up a slope
the air cools faster, so there they are a few hundred metres. Each biome has several plant communities,
and which grows where is decided region by region: each community has a smooth noise of its
own, about 4 km across, and holds the ground where its noise leads its biome's others, fading
into the next over an ecotone about a kilometre wide, where their plants mix. So every community has a fair share of its biome, and
different valleys of one climate grow different woods (`terrain::flora::COMMUNITIES`):

| biome | communities (their main plants; ground plants) |
|---|---|
| tundra | dwarf-shrub heath (dwarf birch, juniper; heather, cottongrass); alpine meadow (tussock, fescue, buttercup, daisy); krummholz (mountain pine, juniper) |
| taiga | spruce-fir forest (Norway spruce, balsam fir, birch); pine heath (Scots pine, birch, juniper; heather); larch and birch (larch, silver birch, dwarf birch; fireweed, sedge) |
| temperate forest | beech wood (beech, oak, hazel); oak and hazel (oak, birch, aspen, hazel, hawthorn; meadow, buttercup); maple and tupelo (sugar maple, tupelo, black oak); montane mixed forest (spruce, beech, larch, pine) |
| grassland | flower meadow (hawthorn; meadow grasses, daisy, oxeye, buttercup, clover); parkland (scattered oak, aspen, birch, hawthorn); tall-grass prairie |
| steppe | sagebrush steppe; pinyon-juniper woodland; ponderosa parkland; Mediterranean scrub (olive, stone pine, prickly pear; wild oat, poppy) |
| cold desert | sagebrush desert; Joshua tree woodland; pinyon desert |
| hot desert | saguaro desert (saguaro, ocotillo, cacti, creosote); creosote flats; agave slopes |
| savanna | acacia savanna (acacia, thornbush); baobab savanna; eucalypt woodland; palm savanna |
| rainforest | lowland rainforest (dipterocarp and kapok emergents over rain trees, tupelo, palm, tree fern); cloud forest (tree fern, montane oak); palm forest |

Plants of wet ground grow in the communities that have them only where water gathers
(`wetness`): willows and grey alders by temperate and boreal rivers, mesquite in desert washes.
The tundra's shrubs are stunted (to 0.6 m on the heath, 1.5 m as krummholz).

Each community's trees are one forest, its species weighted by their abundance there, so they
mix as the community says and stand as thickly as its woods; its rule is the community's weight,
on ground no steeper than its trees hold, off ice, dunes and playas, sparser where thin soil on a
steep slope shows rock, and below the treeline. Each community's shrubs are another (below the
snow; by darts out to half the trees' reach and on a grid out to half their `far`), and plants
of wet ground one each. Trees come first, then shrubs, then each community's
ground plants, so each grows in the room those before it leave.

Trees stand out to the horizon as instances: by dart throwing out to `reach` (300 m) from the
camera, thinning out over its last third, and from there out to `far` (3 km) on a jittered grid
(`"placement": "grid"` on a forest: one candidate per cell of the closest spacing, kept by the
rule and by how much room its species needs, as thickly as darts fill the ground; thirty times
fewer candidates, and no test against neighbours, which nobody sees a few hundred metres off),
fading in as the near trees thin out. Their levels of detail make the far ones cards. About
100,000 trees over 3 km cost three minutes and 2 GB. Only the communities found as far as the
trees reach (their weights at 24 x 24 points) choose plants, and the load prints them:

```
Flora: oak and hazel (temperate_forest) 71%, beech wood (temperate_forest) 29% about the camera; 13 populations: ...
```

`"flora"` can also be `{"strata": ["trees", "shrubs", "ground"], "reach": 300, "far": 3000, "samples": 24}`,
to grow only some layers, or survey further. Populations the ecosystem lists itself (rocks,
say) are placed after the flora's.

An ecosystem on a landscape hands it the crowns of the trees it placed (`terrain::canopy`), and
the landscape draws forest floor under them (`scenery::Ground::cover`): humus and litter, red-
brown needles under conifers and browner leaves under broadleaves, moss where it isn't dry,
shaded with the soil material's `forest_floor`; only cliffs stay bare. Materials and placement
rules after the ecosystem read the same as a field, `canopy` (0 to 1). So does the ground's own
relief, its rock's ledges and scree's stones softened to a fifth under the trees: scaled towards
zero, which its displaced cells' bounds (found before the trees are planted) all allow for.

Beyond the trees planted about the camera (past `far`), the landscape draws woods of its own from the same
communities (`flora::Regions::woods`: how much of the ground their trees' crowns cover, and how
much of that is conifers'), so the forests carry on to the horizon: a canopy material of crowns
(`Bare::Woods`, `Bare::Conifers`: `cells` domes 8 m apart for broadleaves, 5 m for conifers,
lit by their relief, dark in the gaps between), thinner and thicker from place to place, below
the treeline and off cliffs. Within the planted trees' reach the ground between them is open,
and the two fade into each other as the trees thin out towards it (`canopy::Slot::planted`).
Nothing grows within `"clear"` metres of the camera (12 with a flora, 0 otherwise), so it
doesn't stand inside a tree.
