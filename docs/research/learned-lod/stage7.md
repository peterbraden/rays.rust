# Stage 7: why coarse levels are too bright

The [tradeoff sweep](tradeoffs.md) found the corrected cards slightly dark at `pixels: 1`
(−1.3%) and brighter the coarser they get: +2.9% at 2, +8.3% at 4, +13% at 8. Fixing that drift
would make the coarse settings, which are cheaper in both memory and time, usable.

## Whole trees, not clusters

Stage 0 measured clusters one at a time. `lod_tree_levels` (in `src/shapes/lod_research.rs`)
measures whole trees, so it counts what clusters do to each other:
- parallel rays from views 10–40° below the horizontal, as `forest-aerial`'s camera sees;
- for each tree at each level: the fraction of rays that hit it, the fraction of those hits lit
  by the scene's sun through the rest of the tree, and the mean response of what is hit;
- each quantity as cards / leaves, for six species.

| Level | Hit | Lit | Response |
|---|---|---|---|
| 2 | 0.95–1.07 | 1.00–1.24 | 0.99–1.00 |
| 4 | 0.89–1.09 | 1.08–1.66 | 0.98–1.02 |
| 5 | 0.90–1.10 | 1.14–1.83 | 0.99–1.02 |
| 6 | 0.93–1.11 | 1.28–2.47 | 0.98–1.02 |
| 7 | 0.80–1.25 | 1.37–3.49 | 0.98–1.03 |

(Ranges over the six species.)

**Coverage and response are close.** What is wrong is how often a hit is lit: at L6–L7 a card
is lit two to three times as often as the leaves it stands for. Leaves deep in a cluster are
shaded by the cluster's other leaves, and a flat card can't shade itself.

Stage 0 had seen this, as shadowing within a cluster rising from 13% at L2 to 54% at L7. The
renders then didn't show it, because at `pixels: 1` the hit-probability deficit dominated. With
coverage fixed, it is what's left at the coarse levels.

## Shadowing a card by its own cluster

**The mechanism.** A ray leaving a card is now stopped with probability `1 − A`, where `A` is the
card's cluster's lit fraction. The test is in `CardSet::occludes`:
- a ray counts as leaving a card when the card crosses its line within a millimetre of its
  origin, or a thousandth of the card's size;
- the draw hashes the ray, as choosing a constituent does;
- `A` is stored as a byte per card, measured at build time by `self_lit`.

**The measurement.** `self_lit` casts rays from 16 directions over the sphere at the cluster's
leaves. From each leaf hit it casts toward 4 directions over the upper hemisphere, turned per ray.
A cluster of more than 2,048 triangles is measured on an even sample of them, each enlarged to
keep their total area, which bounds the build's time and memory.

### Attempt 1: `A` as the unshadowed fraction

`A` was first the plain fraction of leaf hits whose ray toward the light got through the cluster.

| `pixels` | FLIP before | FLIP after | Bias before | Bias after |
|---|---|---|---|---|
| 1 | 0.0543 | 0.0644 | −1.3% | −5.8% |
| 2 | 0.0660 | 0.0684 | +2.9% | −4.9% |
| 4 | 0.0896 | 0.0772 | +8.3% | −3.5% |
| 8 | 0.1175 | 0.0938 | +13.0% | −2.7% |

The drift is gone: the bias is now within 3% from `pixels: 1` to 8. But every setting is 3–6%
too dark, and `pixels: 1` is worse.

### Attempt 2: calibrating each level to the whole tree

Each level was scaled so that the tree as a whole is lit, and hit, as often as its leaves. The
measurement covered views from 10° to 85° below the horizontal and lights over the sky.

This didn't work:
- **Lit only:** 3–5% dark at `pixels: 1` and 2 (FLIP 0.0613 and 0.0664).
- **Card sizes too:** worse. Cards seen from above are over-covered and from the side
  under-covered, so one scale can't fit both.
- **Memory:** the whole-tree leaves built per level raised peak memory by 140 MiB.

The code was removed.

### Why still dark: light the leaves pass on

**The cause.** A green leaf reflects 0.42 and transmits 0.36 of the light it gets. A leaf shaded by
its neighbours still receives much of their light, scattered on: light a card's self-shadowing
throws away.

**How it was tested.** `lod_tree_scattering` path traces whole trees, cards against leaves, in the
sun with up to eight bounces at the leaf's reflectance and transmittance. Bounces carry 26–46%
of what leaves send to the viewer.

**Shadow strength.** Applying a strength `s` to the binary shadow, as `1 − s(1 − A)`, the strength
that matches varies:
- by species: oak about 0, Scots pine about 0.75;
- by level: aspen 0 at L3 rising to 0.65 at L6.

At `s = 0.5`, cards fall within 0.89–1.20 of the leaves, against 0.93–1.59 without
self-shadowing and 0.49–0.90 at full strength.

The renders at `s = 0.5` agree:
- FLIP: 0.0570, 0.0667, 0.0829 and 0.1049 at `pixels` 1, 2, 4 and 8;
- bias: −0.9%, +2.0%, +5.8% and +8.9%.

A thin cluster passes most of what it stops on to the leaves behind; a thick one absorbs more
of it over many bounces. So the right strength grows with the cluster's size.

### Attempt 3: `A` with the light scattered within the cluster

**The measurement.** Rather than fitting a strength, `self_lit` now measures it. For each leaf
hit and light:
- **Without the cluster:** the leaf's response to the light, `ρ max(0, n·l) + τ max(0, −n·l)`, as
  if nothing were in the way.
- **With it:** that response if the ray toward the light gets through, plus a three-bounce random
  walk among the cluster's own leaves, each vertex lit if its ray toward the light gets through.
- `A` is the ratio of the two, capped at 1.

Light that leaves the cluster isn't followed: cards already scatter it between themselves.

**In the harness**, whole trees come within 0.89–1.24 of the leaves at L3–L7. The exception is
weeping willow at L7, at 1.45, where its cards are hit 1.25 times as often as its leaves. The
level drift is gone; what remains follows each species' coverage.

**In renders** (`LOD_SELF_LIT=cluster`, from the air at 64 spp):

| `pixels` | FLIP: current / scattering `A` | Bias: current / scattering `A` | Peak MiB | Build s | Render s |
|---|---|---|---|---|---|
| 1 | 0.0543 / 0.0562 | −1.3% / −1.4% | 629 / 633 | 87 / 121 | 123 / 132 |
| 2 | 0.0660 / **0.0651** | +2.9% / +1.6% | 390 / 394 | 76 / 94 | 101 / 104 |
| 4 | 0.0896 / **0.0823** | +8.3% / +5.6% | 328 / 341 | 66 / 76 | 89 / 94 |
| 8 | 0.1175 / **0.1063** | +13.0% / +9.2% | 326 / 335 | 60 / 66 | 76 / 87 |

From low down, FLIP is 0.0298 against 0.0296 at `pixels: 1`, and 0.0375 against 0.0363 at 2.

Measured this way:
- FLIP at `pixels` 4 and 8 falls by 8–10%;
- the drift with coarseness falls by about a third;
- `pixels: 1` and the low view barely change.

Costs:
- builds take 10–34 s longer;
- renders take 3–14% longer, so far unexplained: the test for a ray leaving its own card shares
  the card's intersection test;
- the scattering is computed for one green leaf, not the scene's leaf material.

It stayed a research option at this point; see [the resolution](#the-resolution-the-leaves-own-albedo-and-large-clusters-only)
below.

## What's left: coverage from low angles

**The remaining drift is coverage.** `lod_tree_levels` finds coarse levels hit less often than
their leaves from shallow views: 0.80–0.94 at L7–L9 for the broadleaves, whose coarsest levels
are one or a few cards per tree.

**Why one card falls short.** A card's size matches its leaves' cross-section averaged over the
hemisphere about its normal. A tree's crown, though, is round: from the side it shows far more
than a flat, mostly horizontal card does at `|n·d|`. A distant tree is seen from near the
horizon, and the sun's shadow of it falls at the sun's elevation. So the card lets too much
sunlit ground show through, and casts too small a shadow.

**Next: crossed cards at the coarse levels only.** Crossed cards fix exactly this (stage 3), and
at coarse levels a tree has so few cards that tripling them costs almost no memory. The memory
that made crossed cards expensive at `pixels: 1` is at the fine levels, where one card does well.

## Crossed cards at coarse levels: a negative result

**What was tried.** Clusters of 512 pieces or more (about L5 and coarser) may get up to three
crossed cards, fitted as in stage 3 (`crossed`; research option `LOD_CROSSED=<pieces>`).
- A ray leaving a crossed cluster passes its sibling cards, in both `intersects` and
  `occludes`.
- The cluster's lit fraction stands in for what its leaves do to that ray.
- The cluster a ray leaves is found by a short search of the cards about its origin
  (`CardSet::leaving`), done only in card sets that have a crossed cluster.

**Coverage was as hoped.** In `lod_tree_levels`, the coarse levels' hit ratios from shallow views
came to about 1 (examples below are single cards → crossed):

| Tree, level | Single card | Crossed |
|---|---|---|
| Aspen L7 | 0.80 | 0.99 |
| Spruce L8 | 0.79 | 0.99 |
| Hazel L7 | 0.83 | 1.04 |
| Oak L7 | 0.91 | 1.07 |

**But the renders are brighter, not darker** (with the scattering-aware lit fraction, from the
air):

| `pixels` | FLIP: single / crossed | Bias: single / crossed | Cards built |
|---|---|---|---|
| 1 | 0.0562 / 0.0577 | −1.4% / +1.3% | 2.57 M / 2.59 M |
| 2 | 0.0651 / 0.0733 | +1.6% / +6.3% | 660 k / 687 k |
| 4 | 0.0823 / 0.0887 | +5.6% / +9.4% | 145 k / 171 k |
| 8 | 0.1063 / 0.1043 | +9.2% / +10.6% | 31 k / 51 k |

Without the lit fraction, crossed cards are worse still: +18.8% and +21.8% at `pixels` 4 and 8.

**Memory and time are unaffected**, as expected: 342–398 MiB, within a few MiB of single cards.

**Reading it.** The path-traced harness predicted this. With crossed cards, the coarse trees
send 1.1–1.5 times their leaves' light to the viewer (aspen L6 1.53, hazel L6 1.31), since more
of the tree is hit and each hit is lit as often as before.

So the remaining brightening is **in the trees' own radiance**, not in sunlit ground showing
through:
- **The scene is mostly conifers.** Scots pine, spruce and fir carry over half the weight.
- **Their cards are too bright even as single cards.** They come to 1.07–1.24 of their leaves in
  the path-traced harness, hit 5–10% more often than their needles from the views that matter.

**What it points to.** Matching the light a tree sends back means matching coverage and the lit
fraction as functions of direction, not on average over the hemisphere. A distant tree is seen
near the horizon and lit from above, and one number per card can't hold both. Candidates:
- weighting the coverage fit toward the directions distant trees are seen from;
- making the lit fraction depend on the light's elevation (a few bytes per card).

## Coverage fitted near the horizon: no change

Card sizes were fitted over a band of directions within 60° of the horizontal (in the tree's frame),
where distant trees are seen from, instead of the hemisphere about each card's normal.

**In `lod_tree_levels`.** The broadleaves' whole-tree hit ratios came right at every level: 0.95–1.01,
against 0.80–0.94. Conifers stayed 4–9% over at L3–L5 whatever the band, so that excess isn't about
direction.

**In renders** it changed nothing that matters:

| `pixels` | FLIP: hemisphere → band | Brightness: hemisphere → band |
|---|---|---|
| 1 | 0.0562 → 0.0580 | −1.4% → −2.0% |
| 2 | 0.0651 → 0.0663 | +1.6% → +1.2% |
| 4 | 0.0823 → 0.0837 | +5.6% → +5.4% |
| 8 | 0.1063 → 0.1084 | +9.2% → +9.2% |

The code was removed.

## The resolution: the leaves' own albedo, and large clusters only

### The scene's leaves are dark

`self_lit` assumed a bright green leaf: reflectance 0.42 and transmittance 0.36. The scene's
leaves, as luminance, are far darker:
- needles reflect about 0.1 and transmit about 0.05;
- broadleaves reflect about 0.15 and transmit about the same.

So the scattering it credited was several times too much. That was also why the plain binary shadow
(attempt 1) did best at the coarse settings.

**The fix.** Each leaf material's reflectance and transmittance (luminance of its healthy colour) now
reach `consolidate_with` through `Detail::Aggregate`'s `Albedo`. With every card shadowed:

| `pixels` | FLIP: current → own albedo | Brightness: current → own albedo |
|---|---|---|
| 1 | 0.0543 → 0.0613 | −1.3% → −4.7% |
| 2 | 0.0660 → 0.0653 | +2.9% → −3.4% |
| 4 | 0.0896 → **0.0746** | +8.3% → **−1.0%** |
| 8 | 0.1175 → **0.0934** | +13.0% → **+1.5%** |

The coarse settings are fixed. The fine ones became too dark.

### The cards had been shadowing themselves by accident

A shadow ray from a card hit starts a hair off the card, along the chosen leaf's normal. When the light
is behind the card, the ray crosses its own card just beyond its start and is blocked. So every card has
been opaque to light from behind.

Detecting a ray leaving its own card removed that. With no measured shadowing, the forest became
brighter: +19% at `pixels: 8`, against the current +13%. For cards without a measured lit fraction, the
old behaviour is now kept exactly: with shadowing off the image matches the current cards to the last
digit (FLIP 0.1175, +13.0%).

### Shadowing only large clusters

For small clusters, the accidental back-face shadow serves better than the measured one, which
over-darkens them. The likely cause is that neighbouring cards already cast much of a small cluster's
shadow.

**The gate.** Measured shadowing applies only to clusters of `SELF_LIT_FROM` pieces or more. From the
air at 64 spp:

| `pixels` | Current: FLIP / brightness | From 512: FLIP / brightness | **From 2048 (default)**: FLIP / brightness |
|---|---|---|---|
| 1 | 0.0543 / −1.3% | 0.0567 / −3.3% | 0.0547 / −2.1% |
| 2 | 0.0660 / +2.9% | 0.0634 / −2.7% | **0.0627 / −0.6%** |
| 4 | 0.0896 / +8.3% | 0.0744 / −0.9% | **0.0752 / +0.3%** |
| 8 | 0.1175 / +13.0% | 0.0934 / +1.5% | **0.0937 / +1.6%** |

From low down it is unchanged: FLIP 0.0297 against 0.0296 at `pixels: 1`, and 0.0367 against 0.0363
at 2.

**Cost.**
- Memory is the same: a byte per card.
- Builds change by −3 to +5 s.
- Renders, timed back to back with the current binary, came out −6% at `pixels: 1` and +6% at 2, which
  is within run-to-run noise.

Earlier timings that looked 4–14% slower were the machine drifting between runs. Rerunning the current
binary later gave 73 s at `pixels: 8`, not its first 76 s.

**What it moves.**
- **Accuracy.** FLIP drops 5% at `pixels: 2`, 16% at 4 and 20% at 8, and the brightness drift with
  coarseness is gone: within ±2% from `pixels: 1` to 8.
- **Frontier.** At the old `pixels: 4`'s memory (about 330 MiB), the error closes 61% of the gap to
  the old `pixels: 2` (0.0752, between 0.0896 and 0.0660 at 390 MiB). The coarse settings become
  usable.

### Caveats

- One scene and one sun, at 64 spp.
- The 2048 gate was chosen from two candidates on this scene.
- The albedo is a luminance, and the scattering is computed per card, not per colour channel.
- Leaves' autumn colours aren't used; the albedo is each material's healthy colour.
