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

## Parameters

Every parameter is optional.

- **Seed:** `seed` is random if absent or `"random"`.
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

The planet-wide maps take about 13 s to build at the default size. Renders are as costly as
any thick, multiply scattering cloud; `similarity_after` (eg. 2) speeds them up.
