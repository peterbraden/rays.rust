# Space: stars, the Milky Way, nebulae and galaxies

The `space` background fills the sky beyond the atmosphere with a procedurally generated
universe (`src/procedural/space/`). It is a function of direction alone, infinitely far
off, generated from a seed: the same seed gives the same sky from anywhere in the scene.

```json
"background": { "type": "space" }
```

With no skysphere it is what every escaping ray sees (`demo/scenes/deep-space.json`). With a
skysphere, it shows through the air, dimmed by the air's transmittance along the view ray
and hidden by the ground (`night-orbit.json`). Other backgrounds are still hidden by the
skysphere, as before.

## Brightness: physical by default

Everything is on a physical scale, relative to the sun's irradiance: the scene's `sun`
(its luminance) if it has one, else `sun_irradiance`, else 1. Stars have magnitudes, and the
Milky Way, nebulae and galaxies surface brightnesses in magnitudes per square arcsecond, all
against the sun's apparent magnitude, -26.74. So they are as faint as in life: the Milky
Way's brightest clouds are about 10^-8 of the sun's irradiance per steradian, a sixth
magnitude star 10^-13 of it. Beside a sunlit planet (exposure 1) they don't show, as they
don't in photographs from orbit; seeing them takes a night sky's long exposure:

```json
"output": { "exposure": 1.5e7 }
```

for a scene with no sun (or `2.5e6` beside a sun of intensity 6, as in `night-orbit.json`).
For looks rather than physics, `brightness` scales all of it, and each part has its own
`brightness` (a multiplier) and `surface_brightness` (magnitudes) to set.

## Skipping it where the sky outshines it

Seen through a skysphere, space is looked at only where it could matter: for each view ray,
a cheap bound on its luminance (no noise, no stars generated: per-cell star fluxes tabulated
when the scene loads, the Milky Way's profile, the nebulae and galaxies in that direction) is
compared with the sky's own light, and if it is under `threshold` (default 0.001) of it, the
space and the air's transmittance aren't computed. By day that is almost everywhere; at night
almost nowhere. `"threshold": 0` computes it everywhere.

## Options

Every part is on by default; set it to `false` to turn it off.

| Key | Default | |
| --- | --- | --- |
| `seed` | 0 | Seeds every part |
| `brightness` | 1 | Scales all of it (1 is physical) |
| `sun_irradiance` | the scene's sun, else 1 | The photometric reference |
| `threshold` | 0.001 | Through a skysphere: skip space where it could add less than this share of the sky's light |
| `galactic_pole` | `[-0.35, 0.55, 0.75]` | The direction perpendicular to the Milky Way's plane |
| `galactic_centre` | `[0.9, 0.2, -0.3]` | Towards the galactic centre (made perpendicular to the pole) |
| `stars` | `{}` | See below |
| `milky_way` | `{}` | |
| `nebulae` | `{}` | An object for random ones, or a list |
| `galaxies` | `{}` | An object for random ones, or a list |

### `stars`

| Key | Default | |
| --- | --- | --- |
| `count` | 40000 | Stars in the whole sky |
| `magnitude_limit` | 8 | The faintest star's magnitude; counts grow as 10^(0.47 m) towards it |
| `brightness` | 1 | Scales their light from the physical |
| `size` | half a pixel | The standard deviation of a star's image, in degrees |
| `saturation` | 1 | 0 white stars, 1 their black-body colours |
| `galactic_concentration` | 3 | How much denser stars are in the galactic plane and bulge |

Stars are drawn about a pixel wide (from the camera's field of view and the image height),
carrying their whole flux, so they are sampled smoothly at any resolution.

### `milky_way`

| Key | Default | |
| --- | --- | --- |
| `surface_brightness` | 21 | Of the disc at the galactic centre, before the bulge and star clouds, in mag/arcsec² (the brightest clouds come out near 20) |
| `brightness` | 1 | Scales the glow |
| `dust` | 1 | The optical depth of the dust lanes, which dim and redden what lies behind them |
| `disc_colour`, `bulge_colour` | white-blue, yellow | |

### `nebulae`

Random ones are scattered along the galactic plane, crowded towards the centre:

| Key | Default | |
| --- | --- | --- |
| `count` | 16 | |
| `brightness` | 1 | Scales their light; each is up to 1.2 magnitudes fainter than its kind's default at random |
| `size` | `[3, 12]` | The range of their radii, in degrees (planetary nebulae are an eighth of that) |
| `kinds` | `[0.55, 0.2, 0.1, 0.15]` | The shares of emission, reflection, planetary and dark nebulae |

Or list them, each with `type` (`"emission"`: hydrogen-alpha red with an oxygen-III teal core,
threaded with dust; `"reflection"`: blue; `"planetary"`: a small ring about a white dwarf;
`"dark"`: dust alone), `direction`, `radius` (degrees), `surface_brightness` (mag/arcsec² where
densest; defaults 21 emission, 22 reflection, 19 planetary), `brightness` (a multiplier), `dust`
and `seed`:

```json
"nebulae": [
  { "type": "emission", "direction": [0.2, 0.3, 1], "radius": 8 },
  { "type": "dark", "direction": [0.25, 0.2, 1], "radius": 4, "dust": 3 }
]
```

### `galaxies`

Random ones are placed away from the galactic plane (whose dust would hide them):

| Key | Default | |
| --- | --- | --- |
| `count` | 40 | |
| `brightness` | 1 | Scales their light; their cores are 18.3 to 19.8 mag/arcsec² |
| `size` | `[0.5, 3]` | The range of their radii, in degrees; smaller ones are commoner |
| `spirals` | 0.65 | The share that are spirals; the rest are ellipticals |

Or list them, each with `type` (`"spiral"` or `"elliptical"`), `direction`, `radius`
(degrees), `inclination` (degrees from face on), `position_angle` (degrees, of the major axis
on the sky), `surface_brightness` (of the core, mag/arcsec², default 18.5; a disc's centre is
2.3 magnitudes fainter, near Freeman's law), `brightness` (a multiplier), `arms` (spirals) and
`seed`:

```json
"galaxies": [ { "type": "spiral", "direction": [1, 0.2, 0], "radius": 6, "inclination": 55 } ]
```

## How it works

- **Stars** are generated per cell of a cube map over the sphere: each cell's count is Poisson,
  in proportion to its solid angle and the density of stars in its direction; magnitudes are
  drawn from the cumulative star counts, colours from black bodies (Planck's law through the
  CIE colour-matching functions) at temperatures of the spectral classes. A direction looks
  only at the stars in the cells around it.
- **The Milky Way** is an exponential disc in galactic latitude, widening away from the centre,
  with a bulge, mottled into star clouds by noise; its dust is a narrower layer broken into
  filaments by domain-warped noise.
- **Nebulae** are patches of domain-warped noise inside ragged edges, coloured by the emission
  lines of their gas (or scattered starlight, or none), with dust of their own.
- **Galaxies**: spirals are thin inclined discs, the line of sight followed to where it crosses
  the disc, with logarithmic arms and a bulge; ellipticals have a Sérsic profile.
- Light composes as it would: stars, glow and galaxies are seen through the dust (reddened, as
  extinction falls with wavelength), and the emission nebulae glow in front.
