# Stage 8: crossed cards, again, for the view from low down

## Where the low view goes wrong

Seen from 90 m up (`forest-aerial`'s low camera), the near half of the image matched full
detail. The far half was 4.5% too dark at `pixels: 2`, as before stage 7.

`lod_tree_levels` with views from 0° to 6° below the horizontal (`LOD_VIEW_DOWN=0,2`), cards hit /
leaves hit:

| Species | L3 | L4 | L5 | L6 | L7 |
|---|---|---|---|---|---|
| Quaking aspen | 0.74 | 0.53 | 0.40 | 0.30 | 0.10 |
| California black oak | 0.79 | 0.63 | 0.44 | 0.34 | 0.27 |
| Hazel | 0.67 | 0.41 | 0.29 | 0.18 | 0.11 |
| Scots pine | 1.06 | 1.07 | 1.10 | 1.13 | 1.04 |

**The broadleaves' coarse cards nearly vanish from the side.** Their leaves face mostly up, so the
cards lie mostly flat, and from near the horizontal they are seen almost edge on. Conifers'
needles face every way and their cards don't. (The single card's size matches its leaves
averaged over the hemisphere about its normal, which views from above dominate.)

## Crossed cards, with the leaves' own albedo

Crossed cards fix exactly this (stage 3), but at the coarse levels in stage 7 they made the image
brighter from the air. That test assumed a bright green leaf for the light leaves pass to each
other, several times what this scene's dark leaves pass on. Since then:
- `self_lit` uses each leaf material's own albedo;
- a crossed cluster's own leaves always shadow a ray leaving it (`Clusters::lit`), since that
  ray passes the cluster's other cards, whatever the cluster's size.

A cluster of at least `CROSSED_FROM` pieces gets up to three crossed cards. It gets them only if
one card would miss its leaves' cross-section over directions by more than 15% (`crossed`).

From the side, crossing clusters of 256 pieces or more brings the broadleaves to about 1 at
L5–L7. Aspen L5 is 1.07, hazel L6 1.11, and oak L5 still 0.51.

`forest-aerial` at 64 spp from the air and 32 spp from low down:

| Crossed from | FLIP, air, `pixels: 2` | Bias, air | FLIP, low, `pixels: 2` | Far half, low | Peak MiB (air) | Build |
|---|---|---|---|---|---|---|
| none (stage 7) | 0.0627 | −0.6% | 0.0367 | −4.5% | 393 | 82 s |
| 256 pieces | 0.0597 | +2.4% | 0.0348 | +6.7% | 405 | 76 s |
| **64 pieces (default)** | **0.0557** | +1.1% | **0.0309** | +5.0% | 420 | 91 s |
| 16 pieces | 0.0554 | +1.0% | 0.0284 | +4.6% | 451 | 98 s |

With crossing from 64 pieces at every setting, from the air:

| `pixels` | FLIP before → after | Peak MiB before → after | Bias after |
|---|---|---|---|
| 1 | 0.0547 → 0.0489 | 637 → 682 | −0.2% |
| 2 | 0.0627 → 0.0557 | 393 → 420 | +1.1% |
| 4 | 0.0752 → 0.0695 | 333 → 337 | +2.3% |

**It moves the frontier.** At `pixels: 2` the forest is now nearly as accurate as it was at
`pixels: 1` (0.0557 against 0.0547), with a third less memory. Render time is unchanged.

**Why 64.** At 16 pieces, little more is gained from the air, for another 31 MiB and a slower
build. Smaller clusters are mostly one twig's spray, which a single card stands for.

**What's left.** The far half of the low view overshoots to about +5%, where it was −4.5% before.
The whole image is within 0.2% (low) and 1.1% (air). Oak at L5 is still under-covered from the
side: one card passes the 15% test averaged over the hemisphere, but not from the side.

**The cache.** A level's format is now `RAYSLOD2`, so cached single-card levels are made again.
With the cache, renders stay bit-identical, and a warm build is 1.7 s.

## Telling a ray that leaves a card, under any integrator

**How a card recognised a leaving ray.** Crossed cards and self-shadowing depend on telling a ray
that leaves a card from one that arrives at it. A card did that by the ray's origin: within 1 mm,
or a thousandth of the card's size, of its own plane.

**Why that broke.** After master's merge, forests render by default with the kernels on the CPU,
which start a ray off a surface by 4 millionths of its distance from the camera: centimetres for
these trees. So a ray leaving a crossed card wasn't recognised. It met the cluster's other cards,
and lost most of its light to this scene's dark leaves:
- at full detail the two integrators agreed within 0.2%;
- under LOD the kernels were 0.8% darker, and 2–3% darker in the near half's median.

**The fix: the integrators name the vertex** (`src/shapes/leaving.rs`):
- **What is named:** its point, which lies on the surface. Each placed tree passes it on in its
  model's frame (`forest::Placed`).
- **Who names it:**
  - the path tracer at each surface vertex (`trace::trace_intersection`);
  - the kernels' host around each host vertex (`kernels::host::shade`), and around the path's
    next ray. The vertex's point travels in the path's state until the next round
    (`kernels::host::HOST_VERTEX`, read before `pathtrace::find_all`).
- **How a card uses it:** its owner is the cluster whose card holds that point
  (`CardSet::leaving`, `Card::holds`). The ray's origin only has to be within reach of the vertex.

`forest-aerial`'s `aerial` suite case at `pixels: 2`, 480 × 280, 32 spp, against the path tracer's
full detail:

| | Path tracer | Kernels on the CPU |
|---|---|---|
| Crossed cards (default) | +2.6%, FLIP 0.0634 | +2.7%, FLIP 0.0631 |
| Single cards only | −0.4%, FLIP 0.0665 | −0.1%, FLIP 0.0667 |

The integrators now agree to the 0.1–0.25% they differ by at full detail.

**Crossed cards are brighter than measured above.** The old test missed some rays leaving at
grazing angles, even in the path tracer. Those rays met sibling cards and darkened the image.
Recognising them properly makes crossed cards about +2.6% bright here, against −0.4% for single
cards. Their FLIP is still lower (0.0634 against 0.0665), so they stay the default, but the
figures in the tables above, and in the suite's first run, were helped by that accident.
