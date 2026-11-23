# Surface layers

![Before and after: the same glaciated range, with its fixed ground cover and with surface layers](surface-layers-comparison.png)

*`demo/scenes/alpine-layers.json` (bottom of each pair) against the landscape's built-in ground
cover (top). The same evolved terrain, light and camera: a cold high range, its glaciers grown by
the landscape simulation. The layers put snow where it is above the snowline and level enough to
lie (lower on north faces), bare ice on the glaciers below it, talus where the simulation left
thin colluvium at the foot of slopes, frost-shattered debris on gentle high ground, alpine grass
on the sediment of the basins, and sedge where water gathers on the flats.*

Real ground is a stack of slightly different things: rock in bands, scree under cliffs, grass
where soil holds, wet ground where water gathers, snow only where it can lie. That variety, more
than any one material, is what makes Terragen's terrain read as photographic. Its Surface Layers
stack materials, each masked by altitude, slope and fractal breakup. Ours do the same with the
[function graph](displacement.md), and can also be masked by what the landscape simulation
computed, which Terragen can't.

On a landscape curved round its planet (drawn at full scale, or with a `planet` layout), the
graph's `slope`, `altitude` and `aspect` are judged by the local up and the height above the
planet's surface (`Fields::vertical`), not by the scene's y, so they hold however far from the
scene's origin the ground is.

## The material

```json
"Ground": { "type": "surface_layers", "layers": [
    { "material": "Rock" },
    { "material": "Scree", "mask": { "node": "multiply", "inputs": [
        { "node": "field", "field": "sediment", "from": 0.05, "to": 1 },
        { "node": "slope", "from": 40, "to": 33 } ] } },
    { "material": "Snow", "mask": { "node": "field", "field": "snowline", "from": -100, "to": 100,
        "breakup": { "node": "fractal", "scale": 700, "amplitude": 150 } }, "coverage": 1 }
] }
```

Layers are listed from the bottom up; the first is the base. Each other layer covers what is
below it by its `mask` (a graph, 0 to 1) times its `coverage`. As with `mix`, each scattering
picks the layer on top stochastically, so each keeps its own BSDF (a `rock` with fractal relief
on the rock layer, a plain Lambertian for snow) and the result is unbiased. Masks above the
chosen layer are the only ones evaluated.

A layer may glow (a `diffuse-light`, say, for lava). Where a light is sampled, its layers'
emissions are added by how much of the surface each is on top of there
(`MaterialModel::emitted_in`, the masks evaluated at the sampled point), which is what
scattering there gives on average, so sampling the light agrees with hitting it.

## Fields

A landscape (`"type": "landscape"`, a scaled patch or full scale) takes `"material"`: a
material for all its ground but rivers and lakes, in place of its fixed cover classes. Graphs
evaluated on it, in `surface_layers` masks, a `mix`'s amount or its `displacement` (see
[displacement](displacement.md#on-a-landscape)), can read its fields with
`{"node": "field", "field": name}` (smoothly 0 to 1 between `from` and `to` if given, the raw
value otherwise; either moved first by a `breakup`; `default`, 0, on surfaces without fields):

| Field | Meaning |
|-------|---------|
| `sediment` | Loose sediment over the rock, metres: alluvium in basins, colluvium at the foot of slopes |
| `ice` | Glacier ice, metres |
| `lava` | Volcanic rock under the sediment, metres |
| `flow` | log10 of the area draining through the point, m² |
| `wetness` | Topographic wetness index, ln(a / tan β): high in hollows and on flats, low on ridges |
| `convexity` | Metres above the mean of the neighbouring cells: ridges positive, gullies negative (depends on the simulation's cell size) |
| `elevation` | Metres above sea level, the planet's curvature and the scale taken out |
| `snowline`, `treeline` | Metres above the snowline and treeline, from the climate model (or by latitude) |
| `temperature`, `precipitation` | The climate's mean annual °C and mm a year (full scale only; on a scaled landscape, `precipitation` is the simulation's, with the rain the terrain makes) |
| `rock` | The rock exposed, by number: 0 sediment, 1 shale, 2 sandstone, 3 limestone, 4 quartzite, 5 schist, 6 gneiss, 7 granite, 8 basalt (docs/geology.md) |
| `hardness` | How hard that rock is to rivers, from -1.4 (sediment) to 1.5 (basalt) |
| `playa` | 0 to 1: the dry floor of a closed basin, where its terminal lake comes and goes (docs/geology.md) |
| `till` | Glacial sediment, metres: till beneath the ice and moraines at its margins (part of `sediment`) |

Each is kept at the simulation's cell corners and interpolated by Catmull-Rom splines, so
masks follow the landscape without showing its grid (but `rock` and `hardness`: the bedrock's
structure is interpolated, and the rock looked up at the point's own height, so beds are sharp
bands across the slopes); at full scale the nested patches are
blended as their surfaces are. The thresholds in the demo came from looking at the fields
first: render a `mix` of two `flat` materials with the field `remap`ped to its amount.
