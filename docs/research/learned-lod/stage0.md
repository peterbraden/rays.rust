# Stage 0: measuring the cards

Before any new structure is built, these measurements check what the cards get right and wrong,
and where the memory goes. There are two kinds:

- per-cluster measurements against the leaves each card stands for;
- renders of `forest-aerial` at full detail and with LOD, compared in linear light.

## Summary

1. **The 3% brightening comes from hit probability, not self-shadowing.**
   - In linear light the LOD forest is 4.5% brighter than full detail: 7% near, 2% far.
   - Measured on isolated clusters, cards are hit 20–26% less often than their leaves at the fine
     levels (L2–L4), and about as often at L6 and coarser.
   - Enlarging every card's area by 1.2 brings the whole image to −0.5% of full detail, every depth
     bin within ±1.7%.
   - The bias falls with distance, as the hit-probability deficit does. Self-shadowing within a
     cluster would predict the opposite: its deficit *grows* with distance, from 13% at L2 to 54%
     at L7.
   - Skylight alone shows the same bias as sunlight (+3.7% against +5.8%).
   - So the diagnosis in `docs/adaptive-meshes.md`, and the one in this research's first plan
     (after Loubet & Neyret 2018), was wrong for this scene.
2. **The `a|n·d|` choice is right for the leaf a ray sees.** Seen from a direction, the leaf a ray
   actually hits first has, on average, the same response as a leaf chosen by `a|n·d|`: within
   0.5%, and 2.6% at worst. Hiding behind other leaves, which the visible-normal model ignores,
   doesn't bias it.
3. **Constituent memory sits at the fine levels, where clusters are small.**
   - Per tree, constituents take 1.6 MB at L2, 1.2 MB at L3, 0.5 MB at L4, and 0.13 MB at L5.
   - Clusters there average 8 and 28 leaves, so the 32-leaf cap rarely bites and the stored lists
     are nearly exact.
4. **At coarse levels the 32-leaf cap is itself an error, and compact models beat it.**
   - From L4 on, a card's 32 kept leaves are an even sample of hundreds or thousands. The sample
     alone gives a 6.5–8.8% mean error in the expected response.
   - An 8-leaf reduced zonotope with refitted areas gives 3.2–3.6%, at a quarter of the memory.
   - One SGGX lobe (6 bytes) gives 2.6–4.4%, and under 1.5% for conifers at L4 and coarser.
   - None of these models biases the mean by more than 0.5%. Shading every hit with the card's own
     normal does: +1–6% bias and 30–40% error.

## Clusters against their leaves

`src/shapes/lod_research.rs` is run by hand:

    cargo test --release --lib lod_research -- --ignored --nocapture

It grows one tree each of six species: quaking aspen, California black oak, weeping willow,
Scots pine, Norway spruce and hazel. Their leaves are clustered at levels 2–7 exactly as
`consolidate` does it (the grouping is factored out as `groups`). Then, for about 150 clusters per
tree and level:

- **Response.** The expected response of the chosen leaf is computed for 11 view directions from
  above and 7 light directions from the upper hemisphere. The response is reflected
  `0.42·max(0, n'·l)` plus transmitted `0.36·max(0, −n'·l)`, with `n'` facing the viewer: the
  forest's summer leaf. It is compared against the full cluster chosen by `a|n·d|`, weighting each
  cluster by its projected area.
- **Models.** The card's kept leaves (today); one SGGX lobe fitted as Heitz et al. 2015, Sec. 4.3,
  and sampled by their Algorithm 1; reduced zonotopes of 4 and 8 original leaves, with merged
  areas or areas refitted by non-negative least squares to the projected area over 128
  directions; and the card's normal alone.
- **Hit probability.** 96 parallel rays per view direction are cast at the cluster's leaves and
  at its card.
- **Self-shadowing.** For the leaf each ray hits, the response is computed unshadowed and shadowed
  toward the light by the cluster's own leaves.

Results are averaged over the six species. Raw rows are in
[stage0-clusters.csv](stage0-clusters.csv).

| Level | Leaves per card | Kept | Constituents per tree | Hit: card / leaves | Shadowing in cluster |
|---|---|---|---|---|---|
| 2 | 7.8 | 7.6 | 1,596 KB | 0.742 | −12.7% |
| 3 | 27.6 | 18.6 | 1,241 KB | 0.767 | −18.9% |
| 4 | 137 | 27.3 | 510 KB | 0.831 | −27.4% |
| 5 | 852 | 31.0 | 133 KB | 0.907 | −35.9% |
| 6 | 6,674 | 31.7 | 29 KB | 1.040 | −44.3% |
| 7 | 53,048 | 32.0 | 5 KB | 1.041 | −54.4% |

Mean absolute error of each model's expected response, against all the cluster's leaves:

| Level | Kept (≤ 32) | SGGX, 1 lobe | 4 leaves, refitted | 8 leaves, refitted | Card normal |
|---|---|---|---|---|---|
| 2 | 0.3% | 7.5% | 4.8% | 1.1% | 29.8% |
| 3 | 2.7% | 6.2% | 8.1% | 2.3% | 35.8% |
| 4 | 6.5% | 4.4% | 9.5% | 3.2% | 38.8% |
| 5 | 7.8% | 3.3% | 9.8% | 3.4% | 39.6% |
| 6 | 8.8% | 2.8% | 9.7% | 3.4% | 40.2% |
| 7 | 7.6% | 2.6% | 9.5% | 3.6% | 40.5% |

Notes on the models:

- **SGGX depends on the foliage.** It is best for conifers: Scots pine and Norway spruce are at
  1.2–1.5% at L4 and 0.1–0.8% at L5–L7. It is worst for broadleaves (4.3–9.1%), whose clusters'
  normals a single ellipsoid fits poorly.
- **Refitting helps.** Refitting the reduced zonotope's areas lowers the largest projected-area
  error, from 10–12% to 7–9% for 8 leaves. No paper we read fits the support function this way
  (see [README](README.md)).
- **Bias.** Every model's mean bias is under 0.5% except the card normal's.

## Renders: full detail against LOD

`forest-aerial` was rendered at 600 × 350 with 16 spp, at full detail and with `"lod": {"pixels": 1}`
(trees at levels 2–8 as `[0, 1, 623, 6498, 25361, 60311, 74030, 68567, 7625]`, mostly L4–L7).
Each render was written as linear EXR, and they were compared by `scripts/compare-linear.py`:

- mean luminance by depth bins of the full render's depth image;
- both images clamped at the full render's 99.9th percentile, since single sun-glint fireflies
  reached 2,735 against a median of 0.023;
- standard errors by the jackknife over rows.

| Depth (m) | LOD / full | Sky only | Sun only | Cards × 1.2 area / full |
|---|---|---|---|---|
| 766–959 | 1.073 ± 0.004 | 1.070 | 1.074 | 0.999 ± 0.003 |
| 959–1144 | 1.067 ± 0.003 | 1.060 | 1.070 | 0.992 ± 0.003 |
| 1144–1328 | 1.048 ± 0.003 | 1.044 | 1.049 | 0.992 ± 0.003 |
| 1328–1812 | 1.068 ± 0.003 | 1.039 | 1.083 | 1.006 ± 0.002 |
| 1812–2380 | 1.039 ± 0.003 | 1.025 | 1.048 | 0.993 ± 0.003 |
| 2380–3666 | 1.022 ± 0.003 | 1.019 | 1.025 | 0.984 ± 0.002 |
| **All** | **1.045** | **1.037** | **1.058** | **0.995** |

How the columns were made:

- **Sky only:** both scenes rendered without the sun, the sky kept at the brightness the sun gave
  it.
- **Sun only:** the full render minus the sky-only render (light transport is linear in its
  sources).
- **Cards × 1.2:** a temporary, uncommitted change multiplying each card's area by 1.2.

The depth images agree: with LOD, rays end 1–3 m deeper on average, so the cards let more of them
through.

### Reading it

**Two predictions:**
- A hit-probability deficit predicts a bias largest where levels are finest (near) and gone where
  cards match their leaves (L6 and beyond, far). That is what the renders show.
- Missing self-shadowing predicts the opposite, growing with distance. The renders don't show it.

**The intervention.** Enlarging the cards, which is what the measured deficit calls for, removes
the bias. One uniform factor slightly over-corrects the far trees (−1.6%), as expected where cards
already matched.

**Where the deficit comes from.** A card's area is set from its leaves' projected area *along the
card's normal*, `R(1 − e^(−S/R))`. From other directions a flat card's projection shrinks as
`|n·d|`, but a cluster of leaves at many angles shrinks much less (its zonotope against a disc).
At fine levels, where clusters are a few leaves in a twig's spray, the gap is 20–26% averaged over
views from above. At coarse levels clusters span many orientations, overlap more, and the overlap
formula catches up.

**Caveats:**
- The 1.2 factor was chosen after seeing the deficit. It is one global number, and enlarging cards
  also makes them shadow more. A fix should match each card's hit probability to its own leaves
  over directions, then be checked again on more than one scene and sun angle.
- 16 spp, one seed, one scene.
- Leaves were treated as opaque occluders when measuring self-shadowing (transmission through
  them is not counted), so the shadowing column is an upper bound on what cards lose. The render
  comparisons say it matters much less than hit probability here.

## What this changes in the plan

- **A fix for the renderer, independent of any learned structure.** Make each card's hit
  probability match its leaves over the directions it is seen from. For example, size the card
  from the leaves' mean coverage over the hemisphere rather than along its normal, or store a
  coverage factor per card. This is a small, testable change, worth its own PR.
- **For memory**, the target is L2–L3. There, clusters have 8–28 leaves, and the options are an
  8-leaf refitted zonotope at L3 (2.3% error, about 2.3× smaller) or SGGX lobes (6–7.5% error,
  15–37× smaller). At L4 and coarser, replacing the 32-leaf sample is an accuracy gain at little
  memory cost: an 8-leaf zonotope or an SGGX lobe is both smaller and more accurate than today's
  sample.
- **Self-shadowing (stage 3) drops in priority** until the hit probability is fixed and the
  residual is measured again.
- **For a paper,** stage 0 adds two things:
  - an empirical decomposition of LOD error for aggregate cards: hit probability vs constituent
    distribution vs self-shadowing, with the measurements to tell them apart;
  - support-function fitting of constituent sets, which beats both moment-matched SGGX for
    broadleaves and uniform subsampling.

## Follow-up: the fix

PR #171 sizes each card so it is hit as often as its leaves. It rasterises the union of the
leaves' projections along 8 directions spread over the hemisphere, and gives the card the same
mean cross-section (dividing by a flat card's mean `|n·d|`). Two earlier attempts:

- matching along the normal and a 60° ring left a 5–15% deficit;
- matching over the whole sphere with a box-overlap model overshot by up to 3× at coarse levels.

The hemisphere version brings cards to 0.99–1.06 of their leaves' hit probability at L2–L5. On
`forest-aerial` (600 × 350, 64 spp):

| | Mean FLIP vs full | Brightness near / far / all | Build |
|---|---|---|---|
| Before | 0.0585 | +7.4% / +2.2% / +4.6% | 189 s |
| After | 0.0545 | −2.5% / −0.7% / −1.2% | 208 s |

The research branch carries the same change, so later measurements here use the corrected cards.
What remains, slightly dark near the camera, is now small enough that the self-shadowing term of
stage 3 could be measured against it.
