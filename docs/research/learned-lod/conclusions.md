# Conclusions

What this research found, what it changed in the renderer, whether it is research, and how it
compares with the literature. The measurements behind each claim are in the stage notes
([stage0.md](stage0.md) to [stage8.md](stage8.md)), [tradeoffs.md](tradeoffs.md),
[caching.md](caching.md) and [suite.md](suite.md). The papers are in
[bibliography.md](bibliography.md) and [notes/](notes/).

## The answer to the question

The question was whether a far smaller learned or fitted structure could stand in for a leaf
card's stored constituents: the leaves a hit is shaded as, chosen with the full foliage's
probabilities (see [README.md](README.md#the-question)).

**The answer is no, because that part was never the problem.**
- Sixteen representatives a card (`constituents`, stage 1) give the chosen leaf's expected shading
  within 1–2% of all its leaves at level 4 and coarser. The 32-leaf even sample they replaced was
  6–10% off.
- The representatives are grouped by k-means on the leaves' normals and their areas refitted by
  non-negative least squares to the cluster's projected area over directions: the support
  function of its zonotope.
- They take about half the memory the sample did. A learned index or neural proxy would add
  machinery to the part of the card that was already right.

**The error was elsewhere, in four places, in the order they were found:**

| | What was wrong | What fixed it | Stage |
|---|---|---|---|
| 1 | How often a card is hit: cards missed 20–26% of rays at fine levels, and forests were 4.6% too bright | A card sized by its leaves' rasterised coverage over the hemisphere | 0 |
| 2 | How often a coarse card is lit: two to three times as often as its leaves, so forests brightened with coarseness (+13% at `pixels: 8`) | Large clusters shadowed by their own leaves (`self_lit`) | 7 |
| 3 | How often a card is hit from the side: broadleaves' flat coarse cards were hit 0.1–0.5 times as often as their leaves from near the horizontal | Up to three crossed cards for clusters one card can't cover (`crossed`) | 8 |
| 4 | Which rays leave a card: told by the ray's origin, which integrators offset differently; and self-shadowing applied to shadow rays but not to the path's next ray | The integrators name the vertex a ray leaves (`shapes::leaving`); the same draw stops both rays, and a stopped next ray scatters off the cluster's own leaves (`CardSet::stopped`) | 8 |

## Takeaways

### 1. Measure the whole image, under many conditions, before building structure

- **The harness misled more than once.** The research measured clusters and trees against their
  leaves (`src/shapes/lod_research.rs`), and those measurements found every mechanism above. But
  twice they predicted the opposite of what the renders then showed:
  - crossed cards at coarse levels brightened the image;
  - coverage fitted near the horizon changed nothing.
- **What the harness leaves out:** the ground, the sky, and light between trees.
- **What decided each change:** renders against full detail, in linear light and by depth, and
  later the suite of nine conditions ([suite.md](suite.md)). Several changes looked right in one
  view and wrong in another.

### 2. Look for errors that cancel

Three times, one error hid another:
- **Cards were opaque to light from behind by accident.** A shadow ray from a card hit, started a
  hair off it, crossed its own card when the light was behind. That darkened coarse cards, and it
  had to be kept exactly when self-shadowing was added (stage 7).
- **Crossed cards were flattered by a missed test.** Telling a leaving ray by its origin missed
  some rays at grazing angles, even in the path tracer. Those rays met sibling cards and
  darkened the image. Fixed, crossed cards were 1% brighter than first measured (stage 8).
- **Self-shadowing was tuned against an inconsistent estimator.** It stopped shadow rays but not
  the path's next ray. The first self-shadowing credited light the leaves scatter on to each
  other, to make up for what the path then didn't follow. Once the path did follow it, that
  credit counted the light twice (+1.2%).

**Lessons:**
- Light sampling and the path's own sampling must see the same scene: here, one draw for both.
- Results are worth checking on more than one integrator. The CPU kernels, which master made the
  default during this work, exposed the leaving-ray problem.
- A scene's materials matter to a model's constants. The scattering credit was first tuned for a
  bright green leaf; this scene's needles reflect a quarter as much.

### 3. Most of the performance was outside the cards

| `forest-aerial`, from the air | master | Now |
|---|---|---|
| Default `pixels` | 1 | 2 |
| FLIP against full detail (600 × 350, 64 spp) | 0.0585 | 0.0557 (measured before the last two fixes) |
| Peak memory | 699 MiB | 420 MiB |
| Build | 188 s | 65 s, or 1.8 s from the forest's `"cache"` |
| Render | 135 s | 101–123 s (`pixels` 2–1) |

Where the build and memory gains came from:
- **Building:** parallel card building and placement, and skipping noise fields in clearings.
- **The cache:** levels and placements kept on disk, keyed by what makes them, with images the
  same to the bit.
- **Placements:** 28 bytes instead of 72.
- **Aerial perspective:** fitted to each segment's length.

**What the cache leaves.** A cached forest's render is mostly BVH traversal: 25% of instructions
in the search over 243,000 placed trees, 14% in their cards. That is about what such a search
should cost, so it isn't waste in this code. Faster would mean the renderer's BVH itself: nodes in
single precision, or four wide. That was out of this work's scope.

### 4. Where LOD stops helping

- **At eye height** LOD saves little: 2.3–2.9 GiB against 3.1 GiB, and 195–222 s against 242 s.
  The nearest trees are seen at full detail and set both. That needs levels between the mesh and
  the finest cards, not coarser cards.
- **Coarser than `pixels: 4`** memory stops falling, at the 250–300 MiB of placements, models'
  materials and memory freed after the build.

### 5. What is left

From the suite's second run, at `pixels: 2`:
- **Slightly dark everywhere:** −1.5% to −2.8% from the air.
- **Straight down and sparse forests** are 8–9% worse in FLIP than before the last fix. The low
  sun (−9%) and autumn broadleaves (−5%) are better.
- **The likely cause:** one visibility number a card, averaged over the sky, shadows light from
  above a little too much.
- **The candidate fix:** a lit fraction by the light's elevation, a few bytes a card.

## Is it worth merging?

Yes. Each renderer change is backed by renders against full detail. The default is more accurate
than master's with less memory, coarse settings become usable, and the cache changes no image.
Before merging, weigh:

- **The default changes** (`pixels` 1 → 2). Scenes without an explicit `pixels` render
  differently, and more accurately. The demo scenes set `1` and are unchanged.
- **A new implicit contract** (`shapes::leaving`). An integrator that shades vertices must name
  each one around the rays it traces, or crossed and self-shadowing cards misbehave, though
  without crashing. The path tracer and the CPU kernels do. A test checks the cards, not that a
  new integrator remembers.
- **Bias that remains:** −1.5% to −2.8% from the air, of one sign.
- **Research hooks left in:**
  - the switches `LOD_CROSSED` and `LOD_SELF_LIT_FROM` (both in the cache's key);
  - the ignored harness tests in `src/shapes/lod_research.rs`;
  - `scripts/lod-suite.py`, `compare-linear.py` and `lod-comparison-sheets.py`.
- **Size.** Most of the diff is the regenerated shader (`src/gpu/pathtrace.wgsl`) and these notes.
- **Not done:** the demo PNGs weren't re-rendered.

## Is any of it research?

Two findings could be. Neither is established until the prior work below is read.

**1. Measured transmittance in real crowns** ([stage5.md](stage5.md)).
- *What was measured:* in six crowns, two transmittances. `T_u` is for light arriving from
  outside; `T_c` is for light leaving a leaf.
- *What failed:* the reciprocal relation between them, `T_c = T_u′ / T_u′(0)`, mispredicts
  self-shadowing by 0.20–0.42. Non-exponential transport (d'Eon 2018; Bitterli et al. 2018) uses
  it to predict one from the other. Leaves clump on twigs, which breaks the stationarity the
  relation assumes. A gamma law fitted to each curve separately is within about 0.01.
- *Why it matters:* that is a concrete, checkable statement about an assumption the theory rests
  on, for a medium that matters in practice.

**2. A decomposition of aggregate-LOD error, with a fix for each part.**
- *The parts:* hit probability, then the constituent distribution, then self-shadowing, then
  consistency of rays leaving a cluster. Each was measured apart from the others.
- *The fixes:*
  - support-function fitting of a cluster's representatives, which beats one SGGX lobe for
    broadleaves and beats an even sample;
  - the requirement that a cluster's self-shadowing stop the path's next ray as it stops shadow
    rays.
- *How novel:* this is an engineering study, the material of a journal article on practice (JCGT)
  or a talk, more than a new technique. The consistency requirement is Bitterli et al.'s
  framework applied correctly. Naming the vertex a ray leaves is the familiar practice of
  skipping a ray's own primitive, in another form.

**Before claiming either:**
- **Read the closest prior work, which could not be fetched** ([log.md](log.md)):
  - Cook et al. 2007, *Stochastic Simplification of Aggregate Detail*, the nearest prior art;
  - Loubet and Neyret 2017, hybrid mesh–volume levels of detail;
  - van Antwerpen et al. 2026, *Massive Dynamic Foliage*, the state of production.

  Any of them may contain the decomposition or the fixes.
- **Widen the evaluation.** So far it is one tree generator (Weber and Penn's), one renderer, one
  family of scenes, at 32–64 samples per pixel.
- **Compare against another renderer's references,** and against the prior methods themselves.

## How it compares with the literature

| Work | What it does | How this work relates |
|---|---|---|
| SGGX microflakes (Heitz et al. 2015) | One ellipsoid lobe per voxel for an aggregate's normals | One lobe is within 1.5% for conifers at coarse levels but 4–9% off for broadleaves. Sixteen fitted representatives were within 1–2% for both, at small cost (stage 0–1). |
| Visible normals (Heitz 2014) | Choosing a microfacet by `a\|n·d\|` | The rule a card uses to choose the leaf a hit is shaded as. Stage 0 found the leaf a ray actually meets first has, on average, the same response (within 0.5%). |
| Non-exponential transport (d'Eon 2018; Bitterli et al. 2018) | Separate transmittances for light arriving and light leaving correlated media | Self-shadowing is their leaving transmittance, measured per cluster. Measured crowns break the relation that predicts it from the arriving one (stage 5). |
| Hybrid mesh–volume LoDs (Loubet and Neyret 2017) | Meshes near, prefiltered volumes for sub-pixel aggregates | The volume prototype matched the corrected cards from the air and did slightly better from the side, at 50% more render time; not adopted (stage 6). Their paper wasn't read. |
| Stochastic simplification (Cook et al. 2007) | Removes leaves and enlarges the rest, with a contrast correction | The opposite move: cards consolidate leaves rather than thinning them. Not compared; the paper couldn't be fetched. |
| Out-of-core massive models (Wald et al. 2004) | Precomputed binary data, memory-mapped, with proxies for what isn't loaded | Their averaged-normal proxies were "overly bright", as the original cards were. The forest cache follows their design: binary, keyed, loaded whole before tracing. |
| Nanite (Karis et al. 2021) | Streamed cluster hierarchies chosen per view | Calls aggregates such as foliage a weak point; this work is a ray-traced take on that problem. |
| Learned indexes and neural LOD (about 22 papers: Kraska et al., N-BVH, NIF, LSNIF and others) | Learned structures standing in for stored data or geometry | None applies them to sampling an aggregate's constituents, which was this research's starting gap. The measurements suggest the need is small: a fitted 16-leaf set already does it. |

## Summary

A well-measured improvement worth merging. It answers the original question in the negative:
learned structures for constituents aren't needed, because the error was in hit probability,
self-shadowing and leaving rays. It has one or two findings that could become a short paper,
once the three unread papers are read and the evaluation is widened.
