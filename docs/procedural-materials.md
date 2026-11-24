# Procedural materials

Everything real has fine detail. A noisy render hides its absence; once it converges, surfaces
of one flat colour look like plastic. POV-Ray answered this with a texture language (a
*pattern* coloured by a *colour map*, a *normal* perturbed by a pattern, a *finish*) and large
libraries written in it (`stones.inc`, `woods.inc`, `metals.inc`). This renderer now has the
same pieces, built on its function graphs (`src/graph.rs`), with sources of noise from the
literature, filtering so detail finer than a pixel fades instead of aliasing, and a library of
ready-made materials.

![The material library](material-library.jpg)

The library, by `scripts/material-gallery.py` (each preset alone, on a block 40 cm across and a
sphere, in rows of seven, in the library's order: the first table's first 21, then the seven
overlays, over plain grey stone (`worn_edges` over red paint), then the rest of the first table).

## Using the library

```json
"materials": {
  "TABLE": { "type": "library", "name": "oak" },
  "FLOOR": { "type": "library", "name": "travertine", "scale": 2, "seed": 7, "roughness": 0.15 }
}
```

`scale` sizes every pattern in the material, and its relief with them; `seed` moves every
pattern to another part of its noise, so two objects of one material differ; any other field
replaces the preset's own (here a more polished travertine). Patterns are sized in metres.

Each preset is an ordinary material definition in
[`src/material/library.json`](../src/material/library.json): copy one into a scene to change it.

| Preset | What it is, and what it's made of |
|---|---|
| `carrara_marble` | White marble with grey veins: Perlin's marble, a wave of a gradient bent by turbulence (\|sin\| for veins that are thin and sharp), over cloudy white. Polished. |
| `nero_marquina` | Black marble with a network of white calcite veins: the zero crossings of warped noise (\|noise\| small), finer veins from a second noise. Polished. |
| `travertine` | Cream bands of deposition, warped, with pits drawn out along them: cells stretched by a transform, masked by noise. Pits darker and hollow (the same graph bumps it). Honed. |
| `granite` | Pink granite: crystals of feldspar, quartz and biotite as Worley cells, each coloured by its own random value (`cells` style `id`), warped so they interlock. |
| `slate` | Riven slate: the cleft face stepped by terracing noise, coloured by the same steps. |
| `concrete` | Mottled cement with sand and aggregate (cell ids), and air pores (pits). |
| `terrazzo` | Marble chips in white cement: cells' interiors (`edges`) coloured by their ids. |
| `oak` | Growth rings about a trunk along y (`rings`, a `ramp` wave: pale earlywood darkening to latewood, then the next year), distorted at two scales; the large pores of oak's earlywood as cells stretched along the grain, in the earlywood only, dark and recessed; fibres of Gabor noise. After Liu, Dong, Hašan and Marschner. Varnished. |
| `pine` | Wide pale rings with narrow orange-brown latewood. Matt varnish. |
| `walnut` | Dark, wavy rings with a figure of phasor noise, and small pores throughout. Oiled. |
| `brushed_steel` | Anisotropic roughness, rough across the brushing (x) and smooth along it, varied in streaks of anisotropic Gabor noise, which also bump it slightly. |
| `hammered_copper` | Copper dimpled by a hammer: the relief is F2 of cells, squared; dimples smoother than the metal between. |
| `rusted_iron` | A mix: rust (lambertian, raised and scaly) over dull pitted steel, where noise and cells say. |
| `verdigris_copper` | A mix: green patina over copper, streaked downwards. |
| `galvanized_steel` | Zinc spangle: each crystal (a cell, centimetres across) reflects with its own roughness. |
| `leather` | Pebbled grain: creases between rounded cells, dark and recessed, and fine pores. |
| `cork` | Granules of bark, each its own shade, with dark gaps between. |
| `car_paint` | Glossy red with the faint orange-peel waviness of a sprayed clear coat. |
| `linen` | A plain weave of threads a millimetre apart, in the x-z plane (for cloth lying flat). |
| `terracotta` | Fired clay, mottled, with a gritty surface. |
| `brick_wall` | Bricks in running bond (courses along y), each its own shade, in recessed mortar. |
| `grey_granite` | `granite` with a grey `palette`: white feldspar, grey quartz, black biotite. |
| `weathered_granite` | Granite left outdoors: honed dull, feldspar chalky, iron stains, `grime` in its hollows and `lichen` on top: overlays on a preset. |
| `black_galaxy` | Polished black granite flecked with coppery bronzite that glints (a mix with a conductor, by cell). |
| `quartzite` | Sugary fused quartz grains, faintly banded grey and rose, each grain its own roughness. |
| `engineered_quartz` | A worktop of crushed quartz in white resin, grey specks and tiny mirror flecks. |
| `quartz_crystal` | Clear quartz (n = 1.544) with milky veils of inclusions: a mix of clear and frosted glass by billowed noise; `tint` for smoky quartz. |
| `rose_quartz` | `quartz_crystal` tinted pink. |
| `calacatta_marble` | Warm white with bold, wide grey and gold veins. |
| `verde_marble` | `nero_marquina`'s vein network in a green `palette`: Verde Alpi. |
| `limestone` | Pale cream, mottled, with scattered shell fragments (flattened cells' walls) and pores. |
| `basalt` | Fine dark grey with vesicles: round gas holes, dark and hollow. |
| `dried_mud` | A crust cracked into polygons (cells' walls, warped), the plates curling up at their edges. |
| `asphalt` | Black bitumen packed with grey and pale stones, worn smooth on top. |
| `glazed_tiles` | 15 cm glazed tiles (`brick` with square bricks), each a shade between `color` and `color2`, in grout. |
| `plaster` | Trowelled lime plaster, off-white, with the shallow sweeps of the trowel. |
| `weathered_oak` | Oak's rings silvered grey, the earlywood worn back so the grain stands up, and checks split along it. |
| `maple` | Curly maple: pale, with the rippling flame figure of phasor noise across the grain. |
| `ebony` | Nearly black, with faint brown streaks along the grain, French polished. |
| `aged_bronze` | Brown-black patina, darkest in hollows, rubbed bright on its edges (`worn_edges` over a patina). |
| `cast_iron` | Dark, sand-rough cast metal with a little rust in its hollows. |
| `chipped_paint` | Paint (`color`) flaking in patches and worn on edges, rusted iron beneath. |

Presets have parameters: the variables a preset defines can be set by fields of the same name,
`{"type": "library", "name": "granite", "palette": [[0, [0.1, 0.1, 0.1]], ...]}`, as
`grey_granite` is made. Those worth knowing: `palette` (granite, nero_marquina, oak),
`color` (glazed_tiles, chipped_paint, the overlays), `tint` (quartz_crystal), `amount` (the
overlays).

### Weathering

Overlays lie over another material, `"over"` (a name, or a definition), so anything can be
weathered, and stacked:

```json
"OLD_STEP": { "type": "library", "name": "moss", "amount": 0.5,
              "over": { "type": "library", "name": "grime", "over": "STONE" } }
```

| Overlay | Lies where |
|---|---|
| `dust` | On what faces up, thicker in places (slope, broken up by noise). `amount`, `color`. |
| `grime` | In crevices, corners and where objects meet: the `occlusion` node's short rays find them. `amount`, `color`, `reach` (metres). |
| `rain_streaks` | Dark runs down steep faces, long and thin. `amount`, `color`. |
| `moss` | Raised cushions on what faces up and in damp hollows. `amount`. |
| `lichen` | Round crusts with paler rims, more on faces turned up. `amount` (the share of possible patches), `color`, `rim`. |
| `mud_splash` | Thick at the foot, spattered above, up to `height` metres above the ground (the scene's y = 0). `amount`, `color`. |
| `worn_edges` | Wears through the material on convex edges and corners (the `occlusion` node, cast inwards) to what is `under` it (bare steel by default). `amount`, `width` (metres: about the edges' rounding). |

Overlays are `surface_layers`: the base's bump stays under them (dust lies on the stone's
relief), and a layer with its own bump adds it (moss cushions). Their masks read slope, aspect
and altitude in the scene's directions, whichever way the object is turned, and noise in the
object's own space.
## Natural landscapes

![Natural materials](material-library-natural.jpg)

*The natural set (`scripts/material-gallery.py`): ground and rock as a patch of rough ground with a
half-sunk boulder from eye height, beside hills a few hundred metres across from above; the
natural overlays over a plain grey rock; bark on a trunk.*

These are made for the landscapes: each has structure at every scale from centimetres (crystals,
grains, ripples, lichen) through metres (stains, streaks, colonies of lichen, stones) to tens of
metres (tone), all filtered to the pixel, so the same material serves a boulder at your feet and
a cliff a kilometre away, and far off becomes its true average colour rather than noise. They
read the scene's slope and aspect (water streaks run down steep faces, lichen and moss grow on
what faces up, algae on the shaded side of a trunk) and, on a landscape, its fields.

| Preset | |
|---|---|
| `granite_outcrop` | Weathered granite: grey crystalline rock, mottled, iron-stained, black water streaks down steep faces, grey crustose and yellow-green map lichen on what faces up. |
| `limestone_cliff` | Pale grey crag, ochre where fresh, striped black by cyanobacteria where water runs, fluted by solution (karren) on steep faces, pitted on top. |
| `gneiss` | Folded bands and lenses of pale quartz-feldspar and dark biotite. |
| `schist` | Silvery grey-green, finely foliated, glittering with mica (each flake its own roughness). |
| `scree` | Angular stones in three sizes (35, 11 and 3.5 cm) lying on each other with dark gaps, each its own shade, older ones with lichen. |
| `river_gravel` | Rounded pebbles of many rocks on sand. |
| `dune_sand` | Ripples 11 cm apart across the `wind` (a direction; phasor noise with a sawtooth profile: gentle upwind, steep downwind), heavy minerals dark in their troughs, smooth on slopes steeper than sand rests. |
| `beach_sand` | Pale dry sand with shell fragments; below `waterline` (scene y, metres) dark, glossy and rippled by waves. |
| `mudflat` | Wet glossy mud, water standing in hollows, cracked where it has dried. |
| `oak_bark`, `pine_bark`, `birch_bark`, `beech_bark` | Furrowed ridges with algae on the side away from `shade`; plates and fissures; white with lenticels, black and rough below `rough_below` metres; smooth grey with lichen. Furrows run along y. |
| `map_lichen` (overlay) | Rhizocarpon's yellow-green crusts, tiled by black cracks, in colonies on what faces the sky. |
| `wet` (overlay) | Darker and glossy under water: where a landscape's `wetness` field says water gathers, elsewhere `amount` of it in patches. |

`lichen`, `moss` and `rain_streaks` (above) belong here too.

On a landscape, use them as the layers of its `surface_layers`, masked as before by its fields:

```json
"Ground": { "type": "surface_layers", "layers": [
  { "material": { "type": "library", "name": "granite_outcrop" } },
  { "material": { "type": "library", "name": "scree" },
    "mask": { "node": "field", "field": "sediment", "from": 0.05, "to": 1 } },
  { "material": { "type": "library", "name": "wet", "over": { "type": "library", "name": "mudflat" } },
    "mask": { "node": "field", "field": "flow", "from": 5, "to": 6 } } ] }
```

Seen from a kilometre, centimetre detail only sets a material's average, so it is close to the
ground (a figure on a path, a boulder in the foreground) that these show; from afar, it is the
landscape's own layers and their masks that make it read.

## Patterns and colour maps

Any colour (a lambertian's or plastic's `albedo`, a mix's `amount`, a roughness) can be a
pattern coloured by a map, as POV-Ray's `pigment { pattern color_map { ... } }`:

```json
"albedo": { "pattern": { "node": "wave", "shape": "scallop",
                         "input": { "node": "add", "inputs": [
                           { "node": "gradient", "direction": [1, 0.35, 0.2], "scale": 0.14 },
                           { "node": "fractal", "style": "billow", "scale": 0.35, "amplitude": 1.6 } ] } },
            "color_map": [ [0, [0.32, 0.33, 0.35]], [0.14, [0.7, 0.71, 0.72]], [1, [0.86, 0.86, 0.85]] ],
            "interpolation": "smooth" }
```

The pattern is a function graph (see `src/graph.rs` for every node). Its value picks a place in
the map, whose entries are sorted by value; the colour is blended from the two entries about
it (`"linear"`, `"smooth"`, or `"step"` for hard edges). An entry may itself be a texture,
another pattern too, as POV-Ray's `pigment_map`; a number is a grey level, for roughness maps.

### Sources of detail

Beyond Perlin noise (`fractal`, with `ridged` and `billow` styles) and Worley's cells:

| Node | |
|---|---|
| `gabor` | Sparse Gabor convolution noise (Lagae et al. 2009): Gabor kernels, a Gaussian times a cosine, scattered at random. Its spectrum is a blob about one frequency, so it can be anisotropic (`direction`, with `spread`: brushed metal, grain, fibres) and is filtered exactly. `frequency` or `wavelength`, `bandwidth` (default half the frequency: smaller for more regular, slower), `impulses` (16), `plan`, `seed`, `amplitude`. |
| `phasor` | Phasor noise (Tricard et al. 2019): the same kernels as complex exponentials, keeping the phase of the sum, turned into a `profile` (`sine`, `saw`, `square`): oscillations of constant contrast, as wood figure, fingerprints, sand ripples. |
| `cells` | Worley's noise, now with styles `f2` and `id` (a random value per cell: crystals, chips, granules), a `metric` (`euclidean`, `manhattan`: faceted, `chebyshev`: square) and `jitter` (0: a regular lattice, 1 the default). |
| `gradient`, `rings`, `spherical` | POV-Ray's gradient (distance along `direction`), cylindrical rings (distance from the line through `center` along `axis`), and onion shells, each over `scale`. |
| `wave` | Any input turned into repeating bands, `frequency` per unit of it, `phase`, as POV-Ray's wave types: `shape` `ramp`, `triangle`, `sine`, `scallop` (\|sin\|), `square`; `exponent` sharpens. |
| `checker`, `brick` | POV-Ray's: alternate cubes `size` across; bricks `size` [x, y, z] (default a UK brick in metres) in running bond with `mortar` between, `style` `mortar` (1 in it) or `id` (a value per brick). |
| `occlusion` | The share of `rays` (6) short rays, `radius` (0.05 m) long, that meet something: cast out from the surface, how enclosed it is (`style` `cavity`: crevices, corners, where objects meet, as dirt gathers there); cast in from under it, how near a convex edge it is (`style` `edges`: wear). Directions are hashed from the point, so they converge over a pixel. Costs rays: use it in masks, not in bumps. |
| `transform` | Its `input` scaled, rotated (degrees about x, then y, then z) and moved (`translate`): stretch cells into pores, turn bands. |

### Filtering

A pattern finer than a pixel aliases: it glitters, or shows moiré where a surface recedes.
Patterns are therefore evaluated with the pixel's footprint where the surface is hit (as image
textures are, `textures::footprint`):

- `fractal` octaves finer than the footprint fade to their mean, as they always have on
  displaced terrain;
- Gabor and phasor noise are attenuated by exp(-2π² σ² F²), which is what convolving a kernel
  with a Gaussian of deviation σ does to it at its frequency F (Lagae et al., section 4);
- `wave`, `checker` and `brick` are box filtered exactly, by differences of their integrals
  over the footprint (Apodaca and Gritz's `filteredpulsetrain`); a wave measures how fast its
  input changes across the surface to know how many cycles the footprint spans.

## Bumps that follow the pattern

Colour and relief in real materials come from the same structure: a marble's veins are
softer, a travertine's pits are hollow, oak's pores are both dark and recessed. A bump can be
any graph, as a height in metres:

```json
"normal": { "graph": "$pits", "amount": -0.003 }
```

and the graph can be the one the colours come from (define it once in `variables` and refer to
it by `"$name"` in both). The normal is tilted against the height's gradient across the surface,
taken by central differences half a footprint apart of the graph filtered to that footprint, so
relief finer than a pixel fades rather than sparkling. `amount` scales the heights.

A `"normal"` in a material's definition goes with the material wherever it is used (an
object's `"normal"` adds to it), so a library material carries its relief. In a `mix`, each
material keeps its own: rust raised over pitted steel.

## Roughness maps

`plastic` and `conductor` take a texture for `"roughness"`: a pattern with a map of numbers, so
a surface can be shinier in places (worn, polished, each crystal of galvanizing its own). For a
conductor the map scales `roughness_u` and `roughness_v` (1 unless given), keeping anisotropy.

## Patterns go with their objects

A pattern is read in its object's own space: moved and turned with the object's `transform` (a
plank turned turns its grain, a moving object doesn't slide through its texture), but not
scaled, so a library material keeps its size in metres on any object (`Geometry::pattern_frame`).

## References

- K. Perlin, "An image synthesizer", SIGGRAPH 1985 (marble as turbulence); "Improving noise", SIGGRAPH 2002.
- S. Worley, "A cellular texture basis function", SIGGRAPH 1996.
- J. P. Lewis, "Algorithms for solid noise synthesis", SIGGRAPH 1989 (sparse convolution).
- A. Lagae, S. Lefebvre, G. Drettakis and P. Dutré, "Procedural noise using sparse Gabor convolution", SIGGRAPH 2009.
- A. Lagae et al., "A survey of procedural noise functions", Computer Graphics Forum 29(8), 2010.
- T. Tricard, S. Efremov, C. Zanni, F. Neyret, J. Martínez and S. Lefebvre, "Procedural phasor noise", SIGGRAPH 2019.
- D. S. Ebert, F. K. Musgrave, D. Peachey, K. Perlin and S. Worley, "Texturing and Modeling: A Procedural Approach", 3rd ed., 2003.
- A. A. Apodaca and L. Gritz, "Advanced RenderMan", 2000, chapter 11 (antialiasing procedural textures).
- A. J. Liu, Z. Dong, M. Hašan and S. Marschner, "Simulating the structure and texture of solid wood", SIGGRAPH Asia 2016.
- J. F. Blinn, "Simulation of wrinkled surfaces", SIGGRAPH 1978.
- I. Quilez, "Domain warping", <https://iquilezles.org/articles/warp/>.
- POV-Ray 3.7 reference: "Pattern", "Color Map", "Normal"; and its include files `stones.inc`, `woods.inc`, `metals.inc`.
