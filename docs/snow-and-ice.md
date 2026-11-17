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

## Demo scenes

- `demo/scenes/snow-ice.json`: a snowfield against a low sun, with boulders under fresh snow,
  blocks of white, bubbly and blue glacier ice, and mounds of old snow, firn and dirty snow.
- `demo/scenes/glacier.json`: an ice cap over the alpine landscape in a colder climate.
