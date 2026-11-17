# Moons

A `moon` object is a world of its own: a sphere of its true size at its true distance (the
Moon is 1737 km in radius, 384,400 km from the Earth's centre), with a surface generated from
a seed and lit physically (`src/celestial/`). Because it is real geometry, what follows from
that does: it rises and sets behind the planet, the air in front of it reddens it near the
horizon and veils it by day, it can hide the sun, and the planet's shadow falls on it.

```json
{ "type": "moon", "seed": 1, "direction": [0.35, 0.5, 1] }
```

Scenes: `demo/scenes/moon.json` (a quarter moon through a telescope), `moonrise.json` (the
full moon rising at dusk), `moon-daytime.json` (a pale gibbous moon in the blue sky),
`moon-crescent.json` (a crescent after sunset, its night side lit by earthshine), and lit by
the moon: `moon-glade.json` (its path of light on the sea) and `moonlit-alpine.json`.

## Moonlight

The moon lights the scene (`celestial::moon::Moonlight`): its whole disc's light, summed
over its sunlit (and earthshine-lit) face as seen from the camera, becomes a disc light in
its direction, sampled by surfaces, water and clouds as the sun is, crossing the same air
(reddened when the moon is low), and shadowed by everything short of the moon. So its light
follows its phase: a full moon gives 2.5·10⁻⁶ of the sun's irradiance (about 0.25 lux
beside the sun's 10⁵), a quarter a tenth of that. A ray that reaches the moon's disc counts
its radiance weighted against that sampling, as rays reaching the sun's disc do.

The skysphere's air scatters moonlight too: the night sky under a moon is the day's sky
(blue overhead, an aureole about the moon) at the moon's share of the sun's light, which a
long exposure shows, as a photograph does (`"lut": false` skies leave it out). By moonlight
a scene wants about 2·10⁵ times a sunlit scene's exposure.

`"light": false` on the moon leaves its light out. Only the first moon in a scene lights it.

## Brightness: physical

The moon reflects the scene's `sun`, so its brightness is set by the sun's: the full moon is
2.5·10⁻⁶ of the sun's irradiance (magnitude -12.7 against -26.7), and its surface about 0.04
of the sun's irradiance per steradian. Beside a sunlit sky (exposure 1) it is the pale disc
it is by day; alone at night it needs a longer exposure (`"output": {"exposure": 3}` for a
sun of intensity 6), and showing its earthshine a far longer one, which burns out the sunlit
crescent, as in a photograph.

Its light is reflected by Hapke's model of regolith (B. Hapke, "Theory of Reflectance and
Emittance Spectroscopy", 2nd ed., 2012): mostly single scattering by the Lommel-Seeliger
law, which is why the full moon is a flat disc rather than a shaded ball, a backward-scattering
phase function, and the opposition surge that makes it brighten sharply at full. Its
parameters were fitted to the Moon's phase curve (Allen's Astrophysical Quantities: V(α) =
V(0) + 0.026α + 4·10⁻⁹α⁴) and geometric albedo (0.12), and the tests check the rendered
moon's whole-disc brightness at full and at quarter against them.

## Where it is

One of:

| Key | |
| --- | --- |
| `direction` | Where it is seen from `from` (default the origin), at `distance` (default 384,400 km) from the planet's centre |
| `orbit` | A Keplerian orbit about the planet, at `time` seconds (below) |
| `position` | Its centre, in scene coordinates |

It keeps one face to its planet (tidal locking); `libration`, `[longitude, latitude]` in
degrees, turns it to show a little of its far side. Its north is towards the scene's y (or its
orbit's pole).

The planet is the skysphere's earth, else the `planet` object's sphere, else the Earth's,
below the origin; or give `"planet": {"centre": [...], "radius": r}`. It hides the moon below
the horizon even without a ground, and shadows it: with the sun behind the planet the moon is
eclipsed, with a penumbra from the sun's disc. (The umbra is black: the light the air bends
into it, which turns an eclipsed moon copper, isn't modelled yet.)

### `orbit`

The six classical elements, in metres and degrees, about a reference plane whose `pole` and
`equinox` direction are given in scene coordinates (defaults y and x). Defaults are the
Moon's mean orbit.

| Key | Default | |
| --- | --- | --- |
| `semi_major_axis` | 384,400 km | |
| `eccentricity` | 0.0549 | |
| `inclination` | 5.145 | |
| `ascending_node` | 0 | Longitude of the ascending node |
| `argument_of_periapsis` | 0 | |
| `mean_anomaly` | 0 | At time 0 |
| `gm` | 3.986e14 | The planet's GM, m³/s²: sets the period (27.3 days for the Moon) |
| `pole`, `equinox` | `[0, 1, 0]`, `[1, 0, 0]` | The reference plane |

## Its surface

Generated from `seed` (`src/celestial/surface.rs`):

- **Craters** of every size from 300 km down to what a pixel resolves, in the numbers of a
  saturated surface, N(>D) = K D⁻² per km², and the shapes Pike measured on the Moon: simple
  bowls below 15 km, flat-floored complex craters with central peaks above, ejecta blankets
  thinning outwards; older ones worn shallower.
- **Basins**, 300 km and up, and **maria**: on the side facing the planet the lava flooded
  the biggest basins and a great irregular lowland, dark and smooth, burying most of the
  craters that were there.
- **Albedo**: dark maria, brighter highlands, bright fresh craters, and the rays of a few
  young craters reaching across the face.
- **Shadows**: near the terminator the relief casts shadows, soft-edged by the sun's disc,
  found by marching over the height field towards the sun.
- **Earthshine**: the planet's sunlit face, a sphere of its albedo at its phase seen from the
  moon, lights the night side.

| Key | Default | |
| --- | --- | --- |
| `seed` | 0 | |
| `radius` | 1737.4 km | Metres |
| `maria` | 1 | How much is flooded: 0 none, 1 as on the Moon, more for more |
| `albedo` | `{"highlands": [0.14, 0.13, 0.112], "maria": [0.083, 0.078, 0.069]}` | Normal albedos (the Moon is a little redder than the sun) |
| `albedo_variation` | 0.15 | The patchiness of the highlands and maria |
| `craters` | `{}` | `density` (K, 0.07), `max` (300 km), `min` (0.5 km), `basins` (30), `rayed` (12), `ray_brightness` (0.5), `fresh_brightness` (0.3), `mare_survival` (0.3: the share of a mare's craters younger than it) |
| `photometry` | the Moon's | Hapke's `b` (0.27), `c` (1), `b0` (1.07), `h` (0.18) |
| `earthshine` | `{"albedo": 0.367}` | The planet's geometric albedo and `color`; `false` for none |
| `shadows` | true | Whether the relief casts shadows |

Craters smaller than a pixel aren't drawn (they would only alias): the pixel's size at the
moon comes from the camera, or `footprint` (km) if the camera has no `angle`.

## Not yet

The limb is a smooth sphere (mountains on it would be a few kilometres, a fraction of a pixel
in most views); the moon is lit only by the sun and its planet; its light is sampled as a
uniform disc (its soft shadows don't follow a crescent's shape), and halos in ice clouds are
the sun's only;
eclipses have no refracted light; the atmosphere doesn't refract it (no flattened moon at the
horizon).
