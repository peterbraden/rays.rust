# Space: stars, the Milky Way, nebulae and galaxies

The `space` background fills the sky beyond the atmosphere with a procedurally generated
universe (`src/procedural/space/`). It is a function of direction alone, infinitely far
off, generated from a seed: the same seed gives the same sky from anywhere in the scene.

```json
"background": { "type": "space" }
```

With no skysphere it is what every escaping ray sees (`demo/scenes/deep-space.json`). With a
skysphere, it shows through the air, dimmed by the air's transmittance along the view ray
and hidden by the ground: by day the sky outshines it, at night and from high up it shows
(`orbit.json`, `limb-sunset.json`, `stratosphere.json`). Other backgrounds are still hidden by
the skysphere, as before.

## Options

Every part is on by default; set it to `false` to turn it off.

| Key | Default | |
| --- | --- | --- |
| `seed` | 0 | Seeds every part |
| `brightness` | 1 | Scales all of it |
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
| `brightness` | 12 | A magnitude-0 star's light, as this radiance over one pixel |
| `size` | half a pixel | The standard deviation of a star's image, in degrees |
| `saturation` | 1 | 0 white stars, 1 their black-body colours |
| `galactic_concentration` | 3 | How much denser stars are in the galactic plane and bulge |

Stars are drawn about a pixel wide (from the camera's field of view and the image height),
carrying their whole flux, so they are sampled smoothly at any resolution.

### `milky_way`

| Key | Default | |
| --- | --- | --- |
| `brightness` | 0.12 | The glow of the disc |
| `dust` | 1 | The optical depth of the dust lanes, which dim and redden what lies behind them |
| `disc_colour`, `bulge_colour` | white-blue, yellow | |

### `nebulae`

Random ones are scattered along the galactic plane, crowded towards the centre:

| Key | Default | |
| --- | --- | --- |
| `count` | 16 | |
| `brightness` | 1 | Scales their light |
| `size` | `[3, 12]` | The range of their radii, in degrees (planetary nebulae are an eighth of that) |
| `kinds` | `[0.55, 0.2, 0.1, 0.15]` | The shares of emission, reflection, planetary and dark nebulae |

Or list them, each with `type` (`"emission"`: hydrogen-alpha red with an oxygen-III teal core,
threaded with dust; `"reflection"`: blue; `"planetary"`: a small ring about a white dwarf;
`"dark"`: dust alone), `direction`, `radius` (degrees), `brightness`, `dust` and `seed`:

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
| `brightness` | 1 | Scales their surface brightness |
| `size` | `[0.5, 3]` | The range of their radii, in degrees; smaller ones are commoner |
| `spirals` | 0.65 | The share that are spirals; the rest are ellipticals |

Or list them, each with `type` (`"spiral"` or `"elliptical"`), `direction`, `radius`
(degrees), `inclination` (degrees from face on), `position_angle` (degrees, of the major axis
on the sky), `brightness`, `arms` (spirals) and `seed`:

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
