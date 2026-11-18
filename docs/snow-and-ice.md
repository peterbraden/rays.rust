# Snow, ice and glaciers

Snow and glacier ice are the same stuff, ice, broken up in different ways. Snow is grains a
tenth of a millimetre to a few millimetres across. Glacier ice is a solid full of air bubbles.
Ice hardly absorbs visible light, but every boundary between ice and air scatters it. Light
that enters wanders past many grains or bubbles before it comes back out, and how far it
wanders sets the colour:

- in fine new snow it travels a few millimetres, and nearly all of it comes back;
- in coarse old snow it goes further, and the ice absorbs a little of the red;
- in bubbly glacier ice it travels metres, and most of the red is absorbed on the way. This is
  why glaciers, crevasses and holes dug in snow look blue.

`src/material/snow.rs` works these colours out from ice's measured absorption spectrum, the
size of the grains or bubbles, and the soot and dust in them, rather than from chosen colours.
In spectral mode (`"spectral": true`) it evaluates them at each path's wavelengths. The module's
notes give the physics and the sources.

## Materials

### `snow`

```json
{ "type": "snow", "preset": "settled", "grain_size": 0.3, "soot": 15, "dust": 5,
  "sparkle": 0.06, "scale": 1, "seed": 0 }
```

| preset    | grain (mm) | soot (ppb) | dust (ppm) | what it is |
|-----------|-----------:|-----------:|-----------:|------------|
| `fresh`   | 0.1 | 5   | 1    | new powder |
| `settled` | 0.3 | 15  | 5    | a few days old (the default) |
| `old`     | 1   | 40  | 20   | weeks old |
| `firn`    | 2   | 50  | 100  | last year's snow on a glacier |
| `dirty`   | 1   | 500 | 1000 | late spring snow by a road |

Any field can be overridden. The reflection follows Kokhanovsky and Bréon's analytical snow
model, so snow is brighter looking towards a low sun than with the sun behind you. The
`sparkle` fraction of the surface glints like the flat faces of grains. Each grain is a cell
of the grain's size, `scale` scene units per metre, and has its own facet direction. Close up
the glints are separate points of light; from far off they blend into a sheen. Density doesn't
change the colour of deep snow; grain size and impurities do. Sooty coarse snow darkens much
more than sooty fine snow.

### `ice`

```json
{ "type": "ice", "preset": "glacier", "bubble_radius": 0.6, "bubbles": 4, "soot": 0,
  "dust": 1, "roughness": 0.08 }
```

| preset    | bubble radius (mm) | bubbles per cm³ | what it is |
|-----------|-------------------:|----------------:|------------|
| `white`   | 0.5 | 30   | bubble-rich ice at a glacier's surface |
| `glacier` | 0.6 | 4    | bubbly glacier ice (the default) |
| `blue`    | 0.8 | 0.3  | old, compressed ice whose bubbles have dissolved |
| `clear`   | 0.5 | 0.01 | almost bubble-free: lake ice, an iceberg's core |

The surface reflects light by Fresnel's law for ice (n = 1.31), on GGX microfacets of the
given `roughness`. Beneath it, light diffused by the bubbles comes back through the surface.
The material treats the ice as deep, so clear ice comes out nearly black under its reflection,
as a deep, clear lake looks. For a piece of ice you can see through, use a `dielectric` of
index 1.31 holding an absorbing medium.

### `snow_cover`

```json
{ "type": "snow_cover", "material": "GRANITE", "cover": 0.45,
  "snow": { "preset": "fresh" }, "up": [0, 1, 0], "drift": 0.3, "seed": 0 }
```

Lays snow over another material wherever the surface faces the sky, with wandering edges
(`drift` is the size of the wandering, in scene units). `cover` runs from 0 (no snow) through
0.5 (snow on every surface facing upwards) to 1 (everything except overhangs). `snow` takes
the same fields as a snow material.

## Terrain

- **Mountains** (`"mountains"`) use physical snow above the snowline: settled snow higher up,
  older snow towards the line. `"snow"` takes snow's fields (eg. `{"preset": "fresh"}`), or a
  colour for the old flat diffuse snow.
- **Landscapes** (`"landscape"`) put snow and glacier ice where the simulation and the climate
  leave them:
  - snow is settled high up, and older, coarser and dustier near the snowline, where it turns
    to firn;
  - below the snowline a glacier is bare ice. Most of it is the white ice of the weathered
    surface, with bands of bluer ice showing through, and rock debris increasing towards the
    snout;
  - where the ice steepens, it speeds up, stretches and cracks into crevasses. These run
    across the flow, tens of metres apart, and their walls are deep blue.

  When a landscape loads, it reports how much of the finest patch is under glaciers and where
  the thickest ice is, to help you frame a shot. The climate's `"warming"` (°C, negative for a
  colder world) moves the snowline and the glaciers. `demo/scenes/glacier.json` uses −7 °C,
  about an ice-age climate. Simulating its glaciers takes about a quarter of an hour, and the
  result is cached.

## Icebergs and ice floes

`src/procedural/iceberg.rs` makes icebergs and sea ice that float on the sea at
`"sea_level"` (default 0). Use them with an ocean that has `"planet_radius": 0`, so that its
surface is flat at that level, and `"transparent": true`, so that you can see into it. Most of
an iceberg is under water. Its foot, and the undersides of floes and lumps of ice, glow
turquoise through the water: the sea absorbs red light within a few metres, and blue and green
much further down.

```json
{ "type": "iceberg", "shape": "pinnacle", "size": 110, "location": [170, 470],
  "yaw": 30, "density": 900, "detail": 6, "seed": 3 }
```

An iceberg `size` metres long, centred over `location` ([x, z]), in one of the shape classes
used by ice services:

| shape      | what it is |
|------------|------------|
| `tabular`  | flat-topped and banded; broken from an ice shelf, long and low |
| `blocky`   | flat top, steep vertical sides (the default) |
| `dome`     | smooth and rounded |
| `pinnacle` | one or more spires |
| `wedge`    | one steep face, the top sloping away from it |
| `drydock`  | eroded into a U, a channel between two pillars |

Each shape is fractured into faces, and its walls are broken the way ice breaks: like glass,
in concave scoops that meet at sharp ridges. There are three scales of them: calving scars a
fifth of the berg across, smaller breaks within those, and melt scallops a metre or so wide.
That is how old, melted ice looks. Weathered cliffs are crushed instead: shattered and rough
at every scale, with fresh ice in the hollows between blocks. Which a stretch of wall is
varies along it, and a tabular berg's front is mostly crushed. Finer roughness is bump mapped.
A few narrow, deep crevasses cut the walls, and the top is smooth snow. The mesh is refined to
about 40 cm above the water, so a large berg has several million triangles.

The berg floats as its `density` says: with 900 kg/m³, an eighth of its volume is above the
sea. It is notched at the waterline by the waves, and below the notch a shelf of ice (the ram)
juts out under water.

The ice's colour follows how freshly it was broken. Old faces have weathered to a white,
bubbly crust. Deep in the scars and crevasses the dense blue ice of the glacier's interior
shows. Snow lies on the top, on ledges, and down the top few metres of a tabular berg's cliff
(the firn). All of these are the `ice` and `snow` materials above with different bubbles and
grains.

```json
{ "type": "ice_floes", "center": [0, 80], "extent": [360, 200], "concentration": 0.55,
  "size": [1.5, 35], "thickness": 1.2, "seed": 1 }
```

Floes from `size[0]` to `size[1]` metres across cover a fraction `concentration` of a
rectangle `extent` ([x, z]) wide. As in real pack ice, there are many small floes and few
large ones (for brash ice, mostly small fragments); none touch. Small pieces gather round the
larger ones they broke from, and the field thins out towards its ragged edges. Each floe is a slab with an irregular outline and a rounded top edge.
It floats with a tenth of its thickness above the water, snow on top with patches of bare ice,
and white ice at the sides.

`"style": "brash"` makes brash ice instead. It partitions the sea's surface into Voronoi
cells twice, at two scales, so neither layer shows the other's pattern. `concentration` is the
share of the sea with ice; the rest is open water, gathered in lanes (more so with higher
`clumping`).

- **Slush** (the fine partition): fragments of clear ice a few centimetres thick, a hand's
  width to a metre across. Some overlap and some leave gaps; they lie at slightly different
  heights and tilts, with a few tipped up out of the water. They range from glass-clear, with
  the dark water showing through, to cloudy white.
- **Shards** (the coarse partition, `size` giving the smallest and largest): chunks of white
  ice. Each is its cell's polygon pulled in off-centre, squeezed, sometimes split in two, with
  corners broken off and a ragged outline, under an uneven, hummocked, snowy top. Small cells
  have small shards or none.

Give the background a dark `ground` (the colour of deep water), as rays that pass down
through the clear ice end there.

In a scene with an ocean, icebergs and floes float on its waves: each piece rises and falls
with the surface where it floats, and small pieces tip with its slope. (Pieces as large as the
waves tip less.) `"clumping"` (0 to 1, default 0.5) gathers floes or lumps into clumps
with open water between.

Either object takes a `"material"` or `"medium"` in place of its own ice and snow.

### Overcast sky

`"background": {"type": "overcast", "zenith": [0.95, 1, 1.1], "bank": {"direction": [x, z],
"darkness": 0.1}}` lights the scene with the CIE standard overcast sky: three times as bright
overhead as at the horizon, with no sun. `bank` puts darker cloud low in one direction, for a
dark sky behind bright ice. Ice shows its blue best in this light.

### Transparent water

`{"type": "ocean", "transparent": true, "absorption": [0.38, 0.065, 0.045]}` lets you see into
the sea. A ray refracted into the water is traced down to whatever is submerged and dimmed by
the water's absorption per metre; the water in front of it adds the sea's own colour. Sunlight
reaches submerged ice through the surface, and the light it sends back leaves by refraction.
The default absorption is clear coastal sea water. Raise the blue for greener water; lower it
all for clearer, more tropical water.

## Demo scenes

- `demo/scenes/snow-ice.json`: a snowfield against a low sun, with boulders under fresh snow,
  blocks of white, bubbly and blue glacier ice, and mounds of old snow, firn and dirty snow.
- `demo/scenes/glacier.json`: an ice cap over the alpine landscape in a colder climate.
- `demo/scenes/icebergs.json`: after photographs of Antarctic ice under heavy overcast. Brash
  ice and a worn bergy bit in the foreground, a tabular berg's cliff behind, dark water.
