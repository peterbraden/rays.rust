# Geology

A `landscape`'s bedrock is no longer a uniform stack of hard and soft layers. It is a
sedimentary cover folded along the mountain belts, over a crystalline basement, with granite
plutons. Rivers and glaciers each erode every rock at a rate of their own. The code is in
`src/terrain/geology.rs` (with the sources it follows); `Planet::structure` places the geology
on the planet, and `Terrain` carries it through the landscape's evolution.

![The patch over seed 0's most mountainous ground at time zero: shaded relief (left) and the rock exposed (right)](geology-folds.png)

![The same ground after 100,000 years of uplift, rivers and glaciers (128 cells of 400 m): strike ranges along the anticlines, their relief held down by landslides to what the rock can stand, sediment filling the synclinal valleys, and beds repeated by the thrusts](geology-evolved.png)

## Rocks, rivers and ice

| Rock | Rivers | Glaciers | Why they differ |
|------|--------|----------|-----------------|
| shale | 2.2 | 1.8 | weak; little to quarry |
| sandstone | 1.0 | 1.3 | |
| limestone | 0.8 | 1.2 | strong against gravel, but soft to abrade |
| quartzite | 0.45 | 0.85 | the hardest for rivers, but brittle and closely jointed: ice plucks it |
| schist | 1.4 | 1.9 | splits along its foliation |
| gneiss | 0.62 | 0.67 | |
| granite | 0.57 | 0.45 | massive, widely jointed: domes and roches moutonnées |
| basalt (lava) | 0.3 | 1.5 | columnar joints are quarried readily |
| sediment | 3 | 3 | |

These are erodibilities relative to average bedrock, multiplying the stream power law's K for
rivers and the abrasion coefficient K_g for glaciers, at the default `contrast` of 5. For
rivers the order follows rock strength (Sklar and Dietrich 2001). For glaciers it follows how
easily the rock is quarried along its joints as well as how hard it is to abrade (Hallet 1996,
Iverson 2012, Krabbendam and Glasser 2011, Dühnforth et al. 2010). The order follows those
sources. The values are model choices (`Rock::scores`), and `contrast` scales them all.

Hillslopes creep in proportion to the rivers' erodibility too: weak rock weathers faster into
soil that creeps, so slopes on shale relax while hard beds hold cliffs.

Rock is also weaker where it is fractured. Folding cracks an anticline's crest open as its
outer arc is stretched, and thrusts are flanked by damage zones. Molnar, Anderson and Anderson
("Tectonics, fracturing of rock, and erosion", JGR 112, 2007) argue that fracturing sets much
of rock's erodibility. Quarrying depends on it more than abrasion does, so glaciers make more
of it than rivers: the most fractured rock is `1 + fracturing` times as erodible to glaciers
(2.5 by default) and `1 + 0.35 fracturing` times to rivers (Whipple, Hancock and Anderson,
GSA Bulletin 112, 2000, find plucking dominant in jointed rock). So glaciers dig out
anticline crests and thrust zones, while rivers barely notice them.

Slopes can only stand so steep before they fail, and how steep depends on the rock (Schmidt
and Montgomery, "Limits to relief", Science 270, 1995): where rivers or glaciers cut down
faster than the slopes can follow, the slopes steepen until they fall in landslides. Each rock
has a critical slope over a simulation's cells (`Rock::critical_slope`, after Selby's rock
mass strength), from loose debris and shale at about 32° to massive granite at 43° (mean slopes
in active ranges level off at about 30°: Montgomery and Brandon 2002),
lowered where the rock is fractured. Steeper slopes fail onto their steepest neighbour, and the
debris lies as sediment at their feet (`fluvial::landslide`). Ice holds up the walls it
carves, so they don't fail while it is there; when it melts, walls in weak rock fall in, and
only strong rock keeps the steep sides of glacial troughs. At time zero the noise topography is
cut down to what its rock holds, without debris, as if worn back before it began
(`fluvial::cap_slopes`). `slope_strength` scales every
limit (0 for no landslides). The limits are for cells of 300 m; as slopes measured over
longer distances are gentler, they are steeper on finer grids and gentler on coarser, as
(300 m / spacing)^0.2, so nested patches agree roughly on how steep the ground stands.

Glaciers erode only where their bed is at the melting point: thin ice on cold summits and
plateaus freezes to its bed and protects it, while thick, warm-based valley glaciers dig
troughs between (`glacier`'s "Frozen beds").

Limestone dissolves. Exposed, it is lowered in proportion to the rain (`karst_rate`, 40 mm a
thousand years per metre of annual rain), the rock leaving in solution; and most of the rain
on it sinks underground (`infiltration`, 70%), follows the drainage beneath the limestone and
rises at springs where the limestone ends. Limestone uplands lose their surface streams
(dry valleys), while the rivers below still get the water. So limestone stands as plateaus
in dry climates and is worn down in wet ones.

So the same mountains erode differently under the two processes. Rivers leave quartzite and
basalt standing as ridges and cut valleys along the shale. Glaciers dig into quartzite, basalt
and schist, and leave granite as domes.

## The column

Above the basement (gneiss, or schist in places) lies a sedimentary cover about
`cover_thickness` thick, of hard beds (sandstone or limestone) alternating with shale, the pair
averaging `strata_thickness`. Bed thicknesses and which bed lies at a given depth vary over
hundreds of kilometres. In the mountain belts the basement was brought up by erosion before the
simulation began (`exhumation`), and the cover near it is metamorphosed: sandstone to
quartzite, shale to schist. Granite plutons rise into the cover as domes, mostly in the belts
and volcanic arcs (`plutons`).

Oceanic crust, where the plates' crust is oceanic (or, without plate tectonics, where the
continents' noise is below sea level), is basalt all the way down: the sea floor and the
volcanic islands on it.

Uplift carries the rock up with the ground (`Terrain::exhumed`), so erosion that keeps pace
with uplift cuts into older rock.

## Folds

In the mountain belts the cover is folded, with axes along the belt (across the direction of
shortening, which is taken to be the direction in which the belt's strength changes fastest).
The folds are phasor noise (Tricard et al. 2019) following that direction: long parallel
trains that end or fork in places, rising and falling along their axes as doubly plunging
anticlines. Folding shapes the ranges in three ways:

- **Relief at time zero**: a share of the folds' structural relief (`fold_relief`) is
  topography, and the ridges are raised on the anticlines and lowered in the synclines, so
  the ranges line up along the folds.
- **Growth**: anticlines are uplifted faster than synclines (`fold_growth`).
- **Vergence**: the folds lean towards the foreland, their steep limbs facing away from the
  belt's centre (`fold_vergence`), so the ranges have steep fronts and gentler backs. They are
  upright at the belt's centre, where real orogens fan out both ways.
- **Thrusts**: where the folds lean, their steep limbs break along thrusts, each sheet riding
  up over the next towards the foreland (`thrust_throw`), so the same beds crop out again
  behind each thrust, hard against soft. The thrusts offset the strata but not the initial
  relief, and die out where the folds end.
- **Lithology**: as erosion cuts down, the folded beds crop out in bands along the belt.
  Breached anticlines expose older rock in their cores, ringed by younger beds.

The belts keep shortening: their thrust sheets carry the rock towards the foreland
(`shortening_rate`, 5 mm/yr in the strongest belts; the Zagros shortens at about 10), folds,
beds and ground together, through the belt's uplift, which stays where it is. Over a few
million years that is further than a fold's wavelength, so divides migrate and folds pass
through the zone of uplift. The ground moves by whole cells as the rock passes each (moving
it by fractions of a cell would blur it), and each cell's rock is that of where it came
from, so rock enters from the planet's geology beyond the patch's edge.

Folds narrower than about two cells of a simulation are left out of it, so a planet-wide
simulation sees only the broad structure, and the nested patches add the folds.

## In a scene

```json
{
  "type": "landscape", "seed": 0, "time": 200000, "scale": 1000,
  "domain": {"type": "patch", "cells": 192, "spacing": 300},
  "geology": {"fold_wavelength": 6000, "fold_amplitude": 2000, "contrast": 8},
  "lava_glacial_erodibility": 1.5,
  "geology_map": "geology.png"
}
```

- `geology`: `strata_thickness` (600 m), `contrast` (5), `fold_amplitude` (1500 m),
  `fold_wavelength` (8000 m), `fold_relief` (0.5), `fold_growth` (0.5), `fold_vergence` (0.6), `thrust_throw` (800 m), `fracturing` (1.5), `slope_strength` (1), `shortening_rate` (0.005 m/yr), `karst_rate` (4e-5), `infiltration` (0.7), `warp_amplitude`
  (400 m), `cover_thickness` (3000 m), `exhumation` (4000 m), `plutons` (1; 0 for none). The
  older top-level `strata_thickness`, `strata_contrast` and `fold_amplitude` are still read.
- `sediment_erodibility` (3), `lava_erodibility` (0.3): to rivers; `sediment_glacial_erodibility`
  (3), `lava_glacial_erodibility` (1.5): to glaciers.
- `geology_map`: also write a map of the rock exposed, in the customary colours of geological
  maps, over shaded relief (for a full-scale landscape, of the finest patch).

Bare rock in the default ground cover takes the colour of the rock at the point's own height,
so beds show as bands across cliffs. In function graphs, `field` nodes can read `rock` (its
number) and `hardness`; see [surface layers](surface-layers.md).

## Plateaus and deserts

Away from the mountain belts, broad regions of continent are raised as a whole without
folding (`Planet::plateau`, `plateau_uplift` 0.2 mm/yr, standing `plateau_height` 800 m higher
at time zero), so their strata lie flat; the Colorado Plateau's rivers cut down through them at
0.1-0.5 mm/yr. In a desert, hillslopes creep slowly (creep scales with the rain, from a tenth
to twice the rivers' reference), so the slopes are bare rock and cliffs keep their edges,
retreating as the soft rock beneath them is cut away and they fail. A patch `"near":
"plateau"` is centred on a plateau in the subtropics capped by a hard bed over soft: there,
over a few million years, canyons cut through the caprock and widen, leaving flat-topped,
caprock-covered tablelands between them. Isolated mesas and buttes need scarps to retreat
further than that, over tens of millions of years (real scarps retreat 0.5-2 km a million
years). A standalone patch's edges are held as base level while it rises, which cuts gullies
all round its rim.

Deserts' basins are often closed. With a climate, each lake keeps a water balance: the
runoff of its catchment (rain less what the land evaporates, by Budyko's curve) against
open-water evaporation (from 0.5 m/yr at 0 °C to 2 m/yr in a hot desert) less the rain on it.
A lake that would lose more than flows in never spills: its basin keeps its water and its
sediment, the rivers below it lose its catchment, and its water stands where evaporation
balances the inflow, as a shallow salt lake or none at all, leaving a flat playa of lake beds
where a wet year's lake spreads (three times the mean inflow, `WET_YEAR`: the basin's lowest
ground, not the slopes of the whole depression) (the module docs of `terrain::simulation`, "Closed basins"; `evaporation` scales it, 0 for
none). Maps show salt lakes in green-blue and playas in white; rendered, playas are salt crust
and clay pan cracked into polygons, and terminal lakes shallow brine, and graphs can read a
`playa` field.

## Rifts: basin and range

Where a continent is stretched, its brittle upper crust breaks along normal faults into tilted
blocks: half-grabens, each block's back sloping gently up to a crest at the fault, and dropping
at the fault's scarp to the floor of the basin beyond (the Basin and Range, whose ranges are
spaced 25-35 km apart; the flanks of the East African rift). Away from the mountain belts and
the plateaus, provinces several hundred kilometres across are stretched (`Planet::extension`)
and broken into such blocks (`Planet::fault_blocks`, `geology::fault_blocks`): a sawtooth in
phasor noise along the direction of extension, which turns slowly across a province, the
faults `rift_spacing` apart (25 km), their traces wandering, the ranges ending and taking over
from each other en echelon, and the faults dipping one way or the other by domains, with
symmetric horsts and grabens between. At time zero the crests stand up to `rift_relief`
(1200 m) above the basins, whose floors are at the regional level, and the faults keep
slipping, the crests rising and the basins sinking relative to each other at up to
`rift_rate` (0.4 mm/yr; the Wasatch fault's is 1-2). The blocks carry and tilt their strata,
and the rock along the faults is fractured. Patches `"near": "rift"` are centred on a
stretched province in the subtropics.

![A stretched province after a million years (128 cells of 1 km): tilted ranges with their scarps facing west, half-grabens filling with alluvium, and playas against the faults where the basins are closed](geology-basin-range.png)

In a dry climate the basins are closed (above): their floors fill with the ranges' alluvium,
and their lowest ground, against the fault where the hanging wall is tilted down, is a playa,
as Death Valley's is against the Black Mountains.

## Deposits: till, moraines and sand

What erosion takes is laid down again somewhere, and the sediment remembers how. Glaciers
carry half of what they quarry in and on their ice towards the margin, lodging some as till
beneath the ice where it melts and dumping the rest at the margin, where it heaps into
moraines at each stand of the ice through the glacial cycle (`glacier::carry_debris`; the
module docs' "Debris"); the rest goes to the rivers in meltwater. `Terrain::till` keeps how
much of each cell's sediment is glacial: maps draw it lilac-grey, the ground cover as grey
rubble grown over below the treeline, and graphs read it as `till`.

In deserts the wind takes the sand out of freshly laid alluvium, leaving its gravel as a lag
that armours the ground (desert pavement), and carries it along the prevailing wind, dropping
it where the wind slackens: in the lee of ranges and ridges, and where plants take hold
(`terrain::aeolian`, `aeolian` in a scene; `Terrain::sand`, a `sand` field). At full scale
the sand is drawn as dunes: ridges about 400 m apart across the wind, with gentle windward
slopes and slip faces down their lee, breaking into barchans where the sand is thin
(`Relief::dunes`). Real sand seas gather over hundreds of kilometres and millions of years,
more than a patch covers; a patch's sand lies in sheets and ramps against the ranges.

## Validation

`terrain::simulation::tests::test_channels_and_slopes_are_steeper_on_hard_rock` (ignored; run
with `cargo test --release test_channels_and_slopes -- --ignored --nocapture`) evolves a
mountain patch for 400,000 years without glaciers and checks what real ranges show: channels
on rock resistant to rivers are steeper for their drainage (their steepness index
k_sn = S A^0.5, Kirby and Whipple 2012), and so are hillslopes. On seed 0, k_sn is 525 on hard
rock and 295 on weak, and hillslopes 0.55 and 0.52 (29° and 27°). Over a million years a
mountain patch's mean slope (500 m cells) settles at a median of 25° (real ranges: 20-25° at
that resolution, levelling off at about 30° on finer grids), and river networks follow Hack's
law with exponents of 0.50-0.58 (0.5-0.6 in real basins). The parameters' values are otherwise not
calibrated against real landscapes.

## Not modelled

The direction of shortening comes from the belt's shape, not from the plates' motion (where
plates converge obliquely, real belts mostly partition the motion between strike-slip faults
and shortening across the belt, so folds run along the belt anyway, as here); thrusts offset
the strata but don't stand as scarps at time zero; fracturing comes only from the folds and
thrusts (not from unloading, cooling or weathering);
karst is only solution and sinking streams (no caves, dolines or collapse), and there is no soil whose depth limits how fast the rock beneath it weathers (at a
simulation's cell sizes, creep is far slower than bare rock weathers, so it would rarely bind).
