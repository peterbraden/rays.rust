# Planets: giants, rings, and rocky and icy worlds

Beyond the scene's own planet and its moon (`docs/moon.md`), any number of other worlds, each
real geometry of its true size at its true distance, generated from a seed and lit
physically by the scene's `sun` (`src/celestial/`):

- `gas_giant`: gas and ice giants, flattened by their spin, their cloud bands grown from
  their winds, with rings if they have them;
- `moon` with a `preset`: rocky and icy bodies (Mercury, Mars, Callisto, Ganymede, Europa,
  Io, or ones generated from a seed);
- `world`: any of these, its kind and everything about it from one seed.

Scenes: `demo/scenes/saturn.json` (the rings' lit face and the planet's shadow across them),
`saturn-shadows.json` (from the rings' unlit side: their shadows on the planet),
`jupiter.json` (with Io and its shadow), and `worlds.json`: a family portrait under one sun, of
giants (a seeded ringed gas giant, a seeded ice giant, Uranus and Neptune), the rocky and icy
presets at one distance (Mercury, Mars, Callisto, Ganymede, Europa and Io), and worlds from seeds.

Their brightness is physical: they reflect the scene's sun, whose `intensity` is the
irradiance where they are. The sun of intensity 6 is the Earth's; at Jupiter it is 6 / 5.2²
= 0.22, at Saturn 0.066, at Uranus 0.016 and at Neptune 0.0066, and the scenes set their
exposure to match, as a camera would.

## Giant planets: `gas_giant`

```json
{ "type": "gas_giant", "preset": "saturn", "seed": 3, "position": [0, 0, 1.5e9], "pole": [0.15, 0.92, -0.36] }
```

| Key | Default | |
| --- | --- | --- |
| `preset` | `gas_giant` | `jupiter`, `saturn`, `uranus`, `neptune`; or `gas_giant` or `ice_giant`, generated from `seed` |
| `seed` | 0 | |
| `position`, `direction`, `orbit` | | Where it is, as for a moon (`docs/moon.md`) |
| `pole` | `[0, 1, 0]` | Its spin axis, in scene coordinates |
| `rotation` | 0 | Degrees about the pole: which longitudes face where |
| `radius`, `polar_radius` | the preset's | Metres; the rings scale with the radius |
| `albedo` | the preset's | Geometric albedo: the map is scaled to give it |
| `rings` | the preset's | `false`, `true` (the preset's, or generated), or `{"preset": "saturn" \| "uranus" \| "neptune"}`, or `{"seed": n}` for generated rings |
| `resolution` | 2048 | Width of the cloud map, texels |
| `flow_time` | 3 | How long the winds have carried the clouds (relative) |
| `turbulence` | the preset's | |
| `shadows` | true | Whether moons and other objects shadow it |

### Its clouds

A giant's face is the top of its clouds, painted by its winds (`src/celestial/bands.rs`). Its
winds are zonal jets, alternating east and west with latitude: Jupiter's two dozen after
Cassini's measurements (Porco et al. 2003), Saturn's broad 450 m/s equatorial jet, Uranus'
and Neptune's single retrograde equatorial current and prograde jets near the poles; a
generated giant's jets are spaced about as the Rhines scale spaces them. The bands follow from
the jets: where the shear between jets turns the flow anticyclonically, gas rises and fresh
ammonia ice makes a bright zone; where it turns cyclonically, the clouds clear and the dark,
coloured depths show as a belt.

The pattern is made by letting the flow carry the clouds: each point of the map is traced
backwards along the wind for a while and takes its colour from where it came from, so the
jets' shear draws mottling into the long streaks and filaments of the real thing. The wind
also has turbulence (the curl of a noise stream function, strongest where the jets shear) and
long-lived vortices that hold their place in the jets and wind the bands' edges round them:
the Great Red Spot and the white ovals on Jupiter, a Great Dark Spot on Neptune, cyclones
crowding the poles. Haze darkens and greys the poles.

### Its light

The cloud deck is a deep layer of scatterers, whose reflected light Hapke's model with
Chandrasekhar's H functions describes (as for the moon, without the opposition surge):
somewhat limb-darkened, and calibrated so the disc's geometric albedo is the planet's
(Jupiter 0.54, Saturn 0.50, Uranus 0.49, Neptune 0.44 in V: Mallama et al. 2017). The ice
giants are blue-green because methane absorbs the red over the long paths light takes through
their air; their colours are those of Irwin et al.'s 2024 true-colour reprocessing of Voyager
and Hubble images, paler than the stretched Voyager pictures of Neptune.

Moons and other objects shadow it, their penumbrae from the sun's disc (Io's shadow on
Jupiter, in `jupiter.json`).

## Rings

A ring is a thin sheet of particles described by its normal optical depth τ(r)
(`src/celestial/rings.rs`). On its lit face its single-scattered light is
ϖ P(α) / 4 · μ₀ / (μ + μ₀) · [1 - exp(-τ (1/μ + 1/μ₀))] of the sun's (as I/F), plus a
multiple-scattering term; from its unlit face, the light scattered through it,
ϖ P(α) / 4 · μ₀ / (μ - μ₀) · [exp(-τ/μ) - exp(-τ/μ₀)]. So the thick B ring is bright from
the sunlit side and dark from the other, the thin C ring and Cassini Division faint on both,
and the planet and stars show through the thin parts (light passes straight through as
exp(-τ/μ)). The rings shadow the planet by exp(-τ/μ₀) where the sunlight crossed them; the
planet shadows the rings.

- **Saturn**: the measured radial profile: the D, C, B and A rings, the Cassini Division,
  the Huygens, Encke and Keeler gaps and the F ring, with ringlets. Icy particles, albedo 0.72
  in the thick rings and darker where thin, reddish, scattering backwards with a strong
  opposition surge; the B ring's I/F near opposition is about 0.4-0.5, as Cassini measured.
- **Uranus**: thirteen narrow, nearly black rings (ε the widest).
- **Neptune**: four faint dusty rings.
- **Generated**: within the Roche limit, a few broad regions of optical depth from 0.01 to
  4, gaps at the 2:1 and 3:2 resonances of moons outside them (as Mimas clears the Cassini
  Division), narrow gaps swept by embedded moonlets, sometimes a narrow shepherded ring
  outside; icy and bright, or dark.

τ is tabulated finely and filtered over each pixel's footprint, so the rings don't alias at
any distance.

## Rocky and icy worlds: `moon` with a `preset`

```json
{ "type": "moon", "preset": "europa", "seed": 5, "position": [0, 0, 1e9], "earthshine": false }
```

Everything about moons in `docs/moon.md` applies (craters, Hapke's photometry, placement,
eclipses); the preset sets the size, craters, colours and what else marks the surface
(`src/celestial/terrain.rs`):

| Preset | |
| --- | --- |
| `moon` | Our Moon (the default) |
| `mercury` | Greyer and darker, smooth plains rather than maria, many rayed craters |
| `mars` | Ochre dust and dark basaltic regions, fewer and worn craters, polar caps |
| `callisto` | Dark ice saturated with craters, bright where fresh ones dug up clean ice |
| `ganymede` | Dark old terrain cut by bright grooved lanes |
| `europa` | Bright ice, almost uncratered, crossed by reddish double-ridged lineae and chaos |
| `io` | No craters: sulfur yellows and oranges, white frost, black paterae, red plume rings, darker poles |
| `rocky` | From the seed: Moon- or Mercury-like, Mars-like, or volcanic |
| `icy` | From the seed: by its age, cracked and young like Europa, grooved, or old and cratered like Callisto |

A world that isn't the scene planet's moon should have `"earthshine": false`. The moon's
other keys (`craters`, `maria`, `albedo`, `photometry`) override the preset's.

## A world from a seed: `world`

```json
{ "type": "world", "seed": 42, "direction": [0.1, 0, 1], "distance": 1e9 }
```

The seed chooses the kind (a quarter gas giants, a fifth ice giants, the rest rocky or icy),
unless `kind` (`gas_giant`, `ice_giant`, `rocky`, `icy`) says, and then everything about it;
other keys (placement, `pole`, `rings`, ...) pass to the object it becomes.

## Not yet

The giants have no atmosphere above their clouds to scatter at the limb (Rayleigh haze,
the methane bands), and their clouds don't drift with time. The rings are lit only by the
sun: not by the planet's light (which lights their unlit side near the planet), nor do they
light the planet. Mars has no air (its thin dusty haze at the limb). Worlds are smooth at the
limb, and none lights the others but the moon's planet.
