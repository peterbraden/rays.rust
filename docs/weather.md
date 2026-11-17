# Weather

`{"type": "weather"}` generates a whole planet's weather and renders the clouds it makes,
each kind where the weather puts it (`src/weather/`). Its seed is random unless one is given:
the render prints the one it chose, so a sky worth keeping can be pinned with `"seed"`.

```json
{ "type": "planet", "material": "GROUND" },
{ "type": "skysphere", "sun_direction": [0.5, 0.6, -1] },
{ "type": "weather", "map": "weather-map.png" }
```

## How it works

The weather is built in two parts, in a geographic frame:

1. **The climate's background.** This varies by latitude and season:
   - the ITCZ, which follows the sun, with clusters of cumulonimbus;
   - the sinking, drying air of the subtropical highs;
   - the storm tracks;
   - the cold, stable poles.

   The surface temperature sets the tropopause and how unstable the air is. It comes from the
   landscape's climate if the scene has a full-scale `landscape`, or else North's energy
   balance by latitude. Daytime heating stirs up convection over land: it follows the sun's
   direction and peaks `convection_lag` hours after noon.
2. **Weather systems** (`src/weather/systems.rs`), placed at random or where the scene asks:
   - **Extratropical cyclones**, after the Norwegian cyclone model:
     - a warm front, ahead of which the cloud's base rises along the frontal surface (1 in
       150): cirrus about 1000 km ahead, then cirrostratus, altostratus, and nimbostratus with
       rain in the last 300 km;
     - a cold front lined with cumulonimbus, with the warm conveyor belt's long band of cloud
       ahead of it and a band of rain behind;
     - then a clear dry slot, and showery open cells in the cold air beyond;
     - a warm sector of stratus and stratocumulus;
     - a comma head wrapped round the low as it occludes (`stage` 0 to 1).
   - **Anticyclones**: clear skies under a low inversion. Over the sea, stratocumulus sheets
     lie on their eastern flanks and trade cumulus on their western.
   - **Tropical cyclones**: a clear eye, a cumulonimbus eyewall, spiral rain bands, and the
     central dense overcast.

The two parts together give each point a column of air: its humidity, ascent, instability,
inversion, condensation level and tropopause. From the column come its clouds, in the WMO's
families:

| Family | Clouds | Rendered by |
| --- | --- | --- |
| `low` | stratus, stratocumulus, cumulus humilis, based at the condensation level | `fields::Stratiform` (Nubis recipe, base, top, cover and form from the maps) |
| `mid` | altostratus, nimbostratus, altocumulus | `fields::Stratiform` |
| `convective` | cumulus congestus and cumulonimbus towers, anvils spread downwind, overshooting tops, anvil shields over clusters | `fields::Convective` (new) |
| `high` | cirrus along the jets and ahead of warm fronts, thickening to cirrostratus | Hogan and Kew's `GlobalCirrus`, its coverage from the weather |
| `rain` | rain and snow from nimbostratus, shafts beneath cumulonimbus, virga in dry air | `fields::Rain` (new; Marshall-Palmer extinction) |

All the families come from one set of cube-sphere maps of the planet. That keeps them
consistent: the rain falls from the cloud above it, the anvils spread from the storms beneath
them, and the cirrus thickens ahead of the front that brings the rain. The map `"map":
"file.png"` shows a satellite's view of the weather, with fronts (warm red, cold blue,
occluded purple), lows (red), highs (blue), tropical cyclones (orange) and the scene's origin
(yellow).

## Animation

`"time"` (seconds, default 0) runs the weather on, so `scripts/animate.py` can render it:

```sh
scripts/animate.py demo/scenes/weather-orbit.json --start 0 --end 172800 --frames 96
```

The animation script steps every object that has a `"time"`, so give the weather one. Every
part of the weather changes continuously with time:

- **Systems move and live their lives.**
  - Cyclones travel east along the storm tracks (`cyclone_velocity`, [12, 1.5] m/s eastward
    and poleward). They occlude over their `cyclone_lifetime` (4 days), then fill.
  - Highs creep east (`anticyclone_velocity`, `anticyclone_lifetime`).
  - Tropical cyclones drift west and poleward (`tropical_velocity`, `tropical_lifetime`).
    They weaken over land.
  - The random systems live in slots: each is born, grows, matures and dies, then is born
    again elsewhere. Its strength is zero at birth and death, so nothing pops in or out.
  - The scene's own `systems` start as given, and move and age the same way. Each also takes
    a `velocity`.
- **The background changes.** It drifts east (`drift`, 8 m/s), and its synoptic and
  mesoscale patterns change shape over `synoptic_period` (3 days) and `mesoscale_period`
  (12 hours). Each pattern blends two independent noise fields, a cos ωt + b sin ωt, so its
  statistics stay the same at every time. Without a sun, the season advances by one day per
  day.
- **Shapes move and evolve within the moving cover.**
  - Low and mid cloud drift round the planet's axis at their `wind` (8 and 15 m/s). They boil
    as the noise rises through them at `rise` (0.3 and 0.1 m/s).
  - Convective cells drift at the convective `wind` (10 m/s). Each lives for `lifetime`
    (5400 s):
    - its tower rises through the first 40% of its life;
    - a storm that reaches the tropopause then spreads its anvil and rains;
    - the column rains out and fades, then the anvil fades;
    - a new storm starts somewhere else in the cell.
  - The cirrus runs on Hogan and Kew's own clock (`time`, `evolution_period`).
  - Rain streaks fall at `fall_speed` (6 m/s).

The sun is the skysphere's, so the afternoon convection follows wherever the scene puts it.
To animate a day, move the skysphere's `sun_direction` too.

## Parameters

Every parameter is optional.

- **Seed and time:** `seed` is random if absent or `"random"`. `time` is in seconds (see
  Animation), along with the motion parameters listed there.
- **Place and time:**
  - `site` is `{"lat", "lon"}`: by default the landscape's, else 45°N 0°.
  - `sun_direction` defaults to the skysphere's. Its declination sets the season, clamped to
    the axial tilt.
  - `day_of_year` sets the season when there is no sun; it is random by default.
  - `convection_lag` (hours) and `earth_radius`.
- **The air:**
  - `humidity` and `instability` are scales, default 1.
  - `tropopause` is `[equator, pole]` in metres.
  - `synoptic_size` and `mesoscale_size` (m) set the scale of the variations.
  - `itcz` and `storm_track` are latitudes in degrees.
  - `climate` overrides `tilt`, `solar`, `hadley` and `warming` when there is no landscape.
    With one, the landscape's land, mountains and climate are used. Set `"landscape": false`
    to ignore it.
- **Systems:**
  - `cyclones` and `anticyclones` are counts per hemisphere, and `tropical_cyclones` a count
    in all. All three are random by default.
  - `cyclone_size` (m), `warm_front_slope` and `cold_front_slope`.
  - `systems` is a list of systems to add, each `{"type": "cyclone" | "anticyclone" |
    "tropical_cyclone", "lat", "lon"}`. Use `"offset": [east, north]` (metres from the
    scene's origin) in place of `lat` and `lon`. Each also takes `radius`, `intensity`,
    `stage`, `tilt` (degrees) and `eye` (m).
- **Families:** each of `low`, `mid`, `convective`, `high` and `rain` is `false` (leave it
  out) or an object. Every family takes:
  - `amount`, which scales its cover;
  - its look: `density` (extinction per metre), `shape_size`, `detail_size`, `erosion`;
  - how it scatters, as any volume: `albedo`, `g` or `droplet_diameter`, `similarity_after`,
    `diffraction`, `halos`, `rainbow`.

  Some families take more:
  - `convective`: `cell_size` (m between storms), `overshoot` and `anvil_depth`.
  - `high`: Hogan and Kew's cirrus parameters (see `hogan_kew_cirrus`).
  - `rain`: `shower_rate` (mm/h), `streakiness` and `streak_size`.
- **Maps:** `map_size` is the number of texels per cube face (512: about 20 km each). `map`
  and `map_width` write the satellite view described above.

Sunlit clouds' fireflies are tamed by sun probes, on for every volume (`"sun_probes"`; see
`VolumeMaterial::sun_probe` and plan.md's variance section): about 2.4 times the efficiency on
the trade-cumulus scene, unbiased. The planet-wide maps take about 4 s to build at the default size. Renders are as costly as
any thick, multiply scattering cloud; `similarity_after` (eg. 2) speeds them up.
