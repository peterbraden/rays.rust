# Stage 2: from the side, and why crossed cards fail

Everything until now was measured from above, where cards are seen nearly face on. This stage
looks at the forest from low down, where an upward-facing card is seen edge on.

## Cards from the side

The harness (`src/shapes/lod_research.rs`) now also casts rays within 17° of horizontal.
[stage2-clusters.csv](stage2-clusters.csv) has the cards of the coverage fix and the shipped constituents. Card hits per leaf
hit, from above and from the side:

| Species | L2 | L3 | L4 | L5 | L6 | L7 |
|---|---|---|---|---|---|---|
| Quaking aspen | 1.02 / 0.83 | 1.05 / 0.54 | 1.05 / 0.38 | 1.06 / 0.31 | 1.09 / 0.25 | 1.07 / 0.18 |
| California black oak | 1.02 / 0.91 | 1.04 / 0.78 | 1.03 / 0.66 | 1.04 / 0.50 | 1.04 / 0.37 | 1.02 / 0.30 |
| Weeping willow | 1.01 / 1.02 | 1.03 / 0.96 | 1.00 / 1.07 | 0.99 / 1.13 | 0.98 / 1.18 | 0.99 / 1.26 |
| Scots pine | 1.00 / 0.98 | 1.01 / 1.02 | 1.00 / 1.02 | 0.98 / 1.06 | 0.96 / 1.12 | 1.03 / 1.26 |
| Norway spruce | 1.01 / 1.01 | 1.01 / 1.01 | 1.01 / 1.02 | 0.99 / 1.01 | 0.98 / 0.88 | 1.09 / 0.40 |
| Hazel | 1.01 / 0.82 | 1.05 / 0.59 | 1.06 / 0.37 | 1.06 / 0.28 | 1.07 / 0.23 | 1.24 / 0.26 |

- **Broadleaf crowns.** Their leaves face mostly up, and so do their cards. From the side, those
  cards are hit a fifth to two thirds as often as their leaves from L3 on.
- **Willow and conifers** have clusters whose normals spread every way, so their cards are
  tilted and do well from both sides.

## Renders from low down

The view is `forest-aerial` with the camera at 90 m: trees seen from the side, mostly levels
4–7. Renders are 600 × 350 at 32 spp, compared in linear light by depth.

| Depth (m) | Old cards | coverage fix + constituents | Crossed cards |
|---|---|---|---|
| 268–355 | 1.007 | 0.995 | 0.984 |
| 355–394 | 1.083 | 0.985 | 0.856 |
| 394–535 | 1.072 | 0.968 | 0.767 |
| 535–901 | 1.021 | 0.944 | 0.784 |
| 901–3791 | 1.012 | 0.961 | 0.847 |
| **All** | **1.004** | **0.996** | **0.982** |
| Mean FLIP | 0.0313 | 0.0296 | 0.0497 |

From the side, the old cards were 7–8% too bright at mid range, and the corrected ones are 3–6%
too dark. FLIP improves slightly.

## Crossed cards: a negative result

**The attempt.** Each cluster got up to three crossed cards, perpendicular to the axes of its
box:
- their areas were fitted by NNLS to the leaves' rasterised coverage over directions (a zonotope
  of three generators);
- they were refitted against the cards' own union, since crossed cards overlap at a slant;
- a cluster got them only when one card missed its coverage by more than 15%.

From above and from the side, the cards were then hit within about ±5% of their leaves at every
level, and for every species.

**What happened in the images.** They came out far too dark: −13.5% from the air (FLIP 0.100
against 0.054) and −23% at mid range from low down. The cards also took 2.3–3 planes per
cluster, which raised the peak to 819 MiB and the build to 305 s.

**The reading.** Matching what a cluster shows to rays coming from *outside* (the
uncorrelated-origin transmittance `T_u` of Bitterli et al. 2018 and d'Eon 2018) is not enough.
Rays *leaving* a cluster also have to be right (`T_c`).
- A shadow or bounce ray from a hit on one crossed card often meets the same cluster's other
  cards. That is artificial self-shadowing, much stronger than the real leaves' 13–54% from
  stage 0.
- A single flat card has the opposite error: it can never shadow itself, so `T_c = 1`.

Stage 0 measured that the brightening came from `T_u`, the hit deficit. This stage shows what
happens when `T_u` is fixed with geometry that gets `T_c` badly wrong. The crossed cards were
reverted.

**What would work.** A cluster representation needs both transmittances right:
- hit probability from outside matching the leaves';
- self-occlusion for rays leaving it matching the leaves'.

Options:
- crossed cards that ignore their own cluster's cards for rays leaving it, with the real
  self-shadowing added as a measured factor (Loubet & Neyret 2018's `A`, or the stage-0 lit
  fraction). This needs the ray to carry where it left from;
- a direction-dependent stochastic opacity on a single card: hit with probability
  `p(d) = coverage(d) / (C |n·d|)`, capped at 1. This fixes `T_u` from oblique views but not
  edge on;
- a volumetric representation for the coarsest levels (Loubet & Neyret 2017), with the
  non-exponential `T_c` of Jarabo et al. 2018 and Bitterli et al. 2018.

For a paper this is a useful result in its own right: an aggregate LOD has to match two
transmittances, and these measurements separate them.
