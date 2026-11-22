# Research: learned and fitted levels of detail for aggregate geometry

This is a research note, not a feature. No renderer behaviour changes here. It records:

- the question;
- what the literature does, checked against the papers' full texts wherever we could get them;
- where the gap is;
- the experiments that would test a contribution, in case it becomes a paper.

The sources are in [bibliography.md](bibliography.md). The working notes behind them, with
section, equation and table references, are in [notes/](notes/). How the reading was done, and
what is still unchecked, is in [log.md](log.md).

## The question

The question, as first put:

> For distant mesh items we already simplify with a LOD implementation. Are there approaches in
> the literature for learned data structures, so a learned level of detail structure can be
> implemented? The triangles of a mesh are added, and a far smaller learned data structure can
> give the constituents with a similar probability to keeping them in memory.

In this renderer that means the leaf cards of [adaptive meshes](../../adaptive-meshes.md)
(`src/shapes/lod.rs`). A distant tree's leaves are gathered into clusters, and each cluster becomes
one rectangular card. A card keeps up to 32 of its leaves, its *constituents*: an octahedral normal,
a relative area and a face, 12 bytes each. A ray that hits the card is shaded as one constituent,
chosen with probability

    P(k | d) = a_k |n_k · d| / Σ_j a_j |n_j · d|,

which is the visible-normal distribution of a microflake medium whose flakes are the leaves
(SGGX, Heitz et al. 2015, Eq. 21; Heitz 2014, Eq. 16 with G1 = 1).

**The question, sharpened:** can a far smaller structure, learned or fitted, stand in for each
card's stored constituents? It would have to return

1. the probability that a ray meets the cluster;
2. which original constituent it meets: a sample with (close to) the full foliage's probabilities,
   carrying that leaf's own state (autumn colour, dead leaf, which leaf material);

and ideally fix what the cards get wrong now.

## Why it matters here: the baseline

These numbers are from `docs/adaptive-meshes.md`. The scene is `forest-aerial.json`: 243,016
trees of 84 distinct models, 600 × 350, 64 spp, 4 cores.

| | Full detail | LOD, `pixels: 1` |
|---|---|---|
| Triangles built / placed | 16.8 M / 24.7 G | 2.6 M / 0.16 G |
| Peak memory | 3.9 GB | 0.67 GB |
| Render | 288 s | 113 s |
| Mean FLIP vs full | 0.035 (seed noise) | 0.057 |

Two targets follow from it:

- **Memory.** Of the LOD scene's 0.67 GB, constituents are about 235 MB, cards 95 MB and card BVHs
  90 MB. That is about 19.6 M constituents over about 2.4 M cards, roughly 8 per card on average.
  The constituents are the largest single item, and they are exactly what the question would
  replace.
- **Bias.** The LOD forest is about 3% brighter than full detail: 1–2% far away, 3–4% near. The
  doc attributes this to self-shadowing the cards lose, because a card is one sunlit layer while a
  cluster's leaves shade one another.
  - The literature names the same symptom. Loubet & Neyret 2018 (Sec. 1, Fig. 2c) say that keeping
    transparency while losing local self-shadowing "leads to overly bright LoDs". Weier et al. 2023
    report "noticeable undesired brightening" of Hybrid LoD on their Oak Tree.
  - So a replacement that also models correlation could improve accuracy, not just save memory.
  - **Stage 0 found otherwise for this scene** ([stage0.md](stage0.md)). The bias, 4.5% in linear
    light, comes from cards being hit 20–26% less often than their leaves at the fine levels.
    Enlarging the cards removes it (−0.5%). Self-shadowing would predict a bias growing with
    distance, and the bias falls with distance.

## What the literature does

Five lines of work bear on the question. Details and numbers are in
[bibliography.md](bibliography.md).

### 1. Learned data structures (databases)

These papers come from databases, not graphics:

- Kraska et al., *The Case for Learned Index Structures* (SIGMOD 2018), and the learned Bloom
  filters, sandwiched (Mitzenmacher, NeurIPS 2018) and partitioned (Vaidya et al., ICLR 2021).
- Error-bounded learned indexes: the PGM-index (Ferragina & Vinciguerra 2020).
- Multi-dimensional learned indexes: Flood (2020) and Tsunami (2020).

Across them, **a model predicts and a small exact structure makes the answer right**:

- the RMI's per-model min/max error (measured over the stored keys only, Sec. 3.3), with a B-tree
  hybrid as the fallback;
- the learned Bloom filter's backup filter for the model's false negatives;
- Mitzenmacher's learned Bloomier filter, a value lookup whose exceptions are stored exactly
  (Sec. 6), which is the closest formal analogue to "a model returns a card's constituents";
- PGM's tolerance fixed first, with the fewest segments that meet it (Lemma 1, after O'Rourke
  1981);
- Flood's exact filtering of every scanned record, where the model decides only how much is scanned.

Two cautions carry over:

- A learned filter's false-positive rate is defined only with respect to a query distribution
  (Mitzenmacher, Sec. 3.2, Theorem 4).
- When an answer is a *distribution*, a model can be exact only with respect to itself. Naru
  (Yang et al. 2019) samples its autoregressive model without bias "with respect to the model's
  distribution"; its "entropy gap" is the error that sampling cannot remove.

**No paper we found applies learned indexes or learned Bloom filters to ray queries, BVHs or a
mesh's constituents.** This rests on arXiv searches, recorded in [log.md](log.md).

### 2. Neural ray queries (graphics)

These networks stand in for a bottom-level BVH or for an asset's geometry:

- NIF (Fujieda et al., HPG 2023);
- LSNIF (I3D/HPG 2025) and its deformable successor VD-NIF (EG 2026 short paper);
- N-BVH (Weier et al., SIGGRAPH 2024);
- NeuOctree (2026);
- Double Bounded Neural Ray Queries (IJCAI 2026; not read).

What they have in common:

- **Deterministic hits.** Each regresses one hit per ray, thresholded at 0.5 or at a tuned value,
  then a distance, a normal and an albedo.
- **Compression.** Large for dense surface assets:
  - N-BVH: 42× on the geometry it represents and 13× on its Fig. 1 scene, more than 1000× on the
    neural part alone, 5–100× over whole scenes.
  - LSNIF: 106.2× against an estimated compressed BVH, 525.1× against an uncompressed one, on
    Hairballs.
- **Slower renders.** N-BVH is 2–4× slower at high quality, and Weier 2023's prototype "cannot yet
  compete with raw real-time path tracing".
- **Foliage is where they are weakest.** LSNIF's foliage errors are "mostly due to missing
  intersections", on its Botanic Cornell Box at FLIP 0.108–0.142.

LSNIF's material head is the only categorical output over identities in this group. It is a softmax
over the handful of materials attached to an object, trained on hard labels and used as one
predicted index. It covers materials, not primitives, and it is never sampled.

### 3. Prefiltered and volumetric LOD

These turn sub-pixel geometry into something else:

- **Volumes:** Neyret 1998; Heitz & Neyret 2012; Loubet & Neyret 2017; the SGGX microflakes
  (Heitz et al. 2015); Fabre et al. 2026's SGGX hierarchy.
- **Aggregated BSDFs with visibility tables:** Zhou et al. 2024/25, not neural.
- **Learned versions:**
  - Bako et al. 2023: a CNN-encoded phase function, albedo and 16×16 coverage mask per voxel.
  - Weier et al. 2023: a correlation-aware neural LOD, with voxel-to-voxel visibility from a
    network thresholded per voxel.
  - Stadter et al. 2024: lighting baked into a NeRF, then voxelised.
- **8DNA (2026):** a normalizing flow over where light leaves an asset, trained by
  throughput-weighted likelihood on path samples and sampled at render time.

What they share:

- **Appearance is averaged.** Per-leaf identity (material, colour) is lost.
- **Correlation is the hard part.** The correlation-aware ones (Heitz & Neyret 2012, Bako 2023,
  Weier 2023, Zhou 2025) are also the expensive ones:
  - Weier 2023 trains for 7–40 minutes per asset;
  - Zhou precomputes for 16,064 s on its Forest;
  - Bako needs 0.5–2 days of data generation on 256 GPUs.
- **Closest prior art to the card's per-hit normal.** Prus et al. (VMV 2015) "randomly draw a
  normal during shading" from a per-voxel normal distribution.

### 4. Correlated transport theory

This is what a correlation term has to reproduce:

- **Non-exponential transmittance:**
  - Jarabo et al. 2018 give closed forms for gamma-distributed density, T(t) = (1 + σt/β)^−α.
  - Vicini et al. 2021 blend linear and exponential transmittance.
- **Where a path starts.** Bitterli et al. 2018 and d'Eon 2018 distinguish paths that start
  uncorrelated with the medium from paths that start on a scatterer:

      T_c(t) = T_u'(t) / T_u'(0).

  A shadow ray leaving a leaf is the second kind, and is attenuated more. This is
  self-shadowing, stated formally.
- **When exponential transport is exact.** Miller et al. 2024 (Theorem 4): exactly when the
  indicator function along a ray is Markov, which a clump of thin opaque plates is not.
- **Microscopic self-shadowing.** Loubet & Neyret 2018 add a factor A(ω) to microflakes, fitted at
  build time by ray casting. Their scalar, isotropic version was as accurate as the anisotropic
  one on cedar and pine.
- **Canopy remote sensing** gives the same physics as measured statistics:
  - gap fraction P(θ) = exp(−G(θ) Ω L / cos θ), with Ross's G-function and Nilson's clumping
    index Ω;
  - the hotspot;
  - photon recollision probability.

### 5. Stochastic simplification and production practice

- **Non-learned prior art for picking real leaves at random:**
  - Cook et al. 2007: prune a random, stratified subset and scale the survivors to keep the area.
    Not fetched (pixar.com is blocked here).
  - Gabor Fields (Condor et al. 2026) choose which groups of primitives to integrate per segment,
    and reweight.
- **Real-time foliage:** NVIDIA's HPG 2026 paper (van Antwerpen et al.: cluster LOD, opacity
  micromaps, far-field aggregates; 60 M plants at 60 fps) is the state of the art, and is not
  learned. Read only through its abstract and press, since dl.acm.org is blocked here.
- **Film renderers:** largely brute force with instancing:
  - Disney's Hyperion "has yet to find an acceptable approach" to automatic LOD;
  - Weta's Manuka avoids aggressive LOD for quality.

## The gap

Every method we read covers at most one half of the question (table below). None:

- (a) keeps a categorical distribution over an aggregate's *original constituents*, calibrated to
  their visible area;
- (b) *samples* it per hit, so shading stays unbiased over the original leaves rather than taking
  one predicted answer;
- (c) uses it in place of stored constituent lists, with a correlation term for self-shadowing.

| Work | Returns original constituents | Sampled by visible share | Correlated visibility | Compact model |
|---|---|---|---|---|
| Cook et al. 2007; our cards | yes | yes (cards) | no | no (stores them) |
| SGGX volumes (Loubet & Neyret 2017; Fabre 2026) | no (normals only) | yes (normals) | no / 2018: scalar A | yes |
| Weier 2023, Bako 2023, Zhou 2025 | no (averaged appearance) | no | yes | yes |
| LSNIF material head | materials, not primitives | no (one prediction) | no | yes |
| 8DNA | no (exit directions) | yes (a flow) | inside the asset | yes |
| **Proposed** | **yes (or a class of them)** | **yes, exact pmf** | **yes (fitted A or T_c)** | **yes** |

**Limits of this claim:**

- It is limited to the papers in the bibliography. Several sources could not be fetched: Cook 2007,
  Loubet & Neyret 2017, Lacewell et al. 2008, van Antwerpen et al. 2026, and LISA.
- Training a categorical head by cross-entropy on *sampled* one-hot hits equals, in expectation,
  training against the soft visible-area distribution. So "soft labels" alone is not a contribution.
  The contribution would be the combination: calibrated per-hit sampling of real constituents,
  correlation, and measured error bounds in place of the stored lists.

Two smaller observations appear to be open as well:

- **Fitting the projected-area function in a norm over directions.** A card's
  σ_Z(d) = ½ Σ a_k |n_k · d| is the support function of a zonotope (the flakes' projection body).
  SGGX's √(dᵀSd) is an ellipsoid's.
  - SGGX fits by matching σ only along the three eigenvectors of the second moment (Sec. 4.3;
    footnote 1: S ≠ Σ), and filters by averaging σ².
  - No paper we read fits σ in L1 or L2 over directions.
  - A reduced zonotope fixes this: a few representative constituents with areas re-solved by
    non-negative least squares. It keeps sampling exact and keeps the representatives' own state.
- **A measurable error bound per card,** in the PGM sense:
  - σ_Z is Lipschitz in d with constant at most Σ a_k. So the largest error of a model over a
    δ-net of directions, plus 2δ Σ a_k, bounds its error over all directions.
  - The total variation between the model's constituent distribution and the card's bounds the
    bias of any bounded shading response: |E_p f − E_q f| ≤ 2 TV(p, q) sup |f|.

  This is our own derivation, not from a paper.

## The proposal, staged

Each stage is useful without the next, and each is measured against full detail and the current
cards.

0. **Instrument.** On `forest-aerial`, measure per card:
   - how many constituents it has;
   - the support-function error of each candidate model;
   - the unoccluded fraction F(ω) from each leaf toward sampled directions, against its own
     cluster's leaves;
   - the brightness gap by distance, in linear light.

   This tests the doc's diagnosis of the 3% before anything is built.
1. **Reduced zonotope.** Keep m ≪ 32 representatives per card, chosen by k-medoids on ±n weighted
   by a, with areas re-fitted by non-negative least squares so that σ matches over sampled
   directions.
   - Sampling stays exact and each representative keeps its face.
   - Measure memory against the TV and σ errors.
2. **Fitted lobes and palettes.** One SGGX lobe, or a 2–4-lobe mixture, per card (6 B per lobe, with
   Algorithm 1 sampling), plus a per-tree palette of faces with per-lobe class weights.
   - Compare with stage 1 at equal bytes.
3. **Self-shadowing** (lower priority after stage 0: measure the residual once hit probability is
   fixed). Loubet & Neyret 2018's scalar A per card plus α_ms, fitted from single-bounce
   escape energies cast at build time (their Eq. 7.10–7.16; Eq. 7.13 read as a least-squares
   minimum).
   - Alternatively, a per-constituent depth fraction g, weighted g^(Λ(v)+Λ(l)) as in Heitz &
     Neyret 2012, Eq. 17.
   - The test: does it close the 3% gap?
4. **Tolerance first, exceptions exact.** Take a per-card error tolerance and choose the cheapest
   representation that meets it: exact list, reduced zonotope, or lobes. Keep classes the model
   under-represents (rare dead leaves, a second leaf material) in an exact exception list, as a
   learned Bloom filter keeps its false negatives.
5. **Learned, only if 1–4 leave memory on the table.** One small model per species, shared by its
   variants and placements, mapping a card's latent code to its constituent distribution (a
   categorical over the palette, conditioned on d).
   - The model gives an exact pmf, so error stays measurable, unlike a thresholded classifier.
   - Training must be deterministic and pure Rust, for the keyed world cache and the wasm build.
   - Inference must be cheap in f32 for the GPU.

## What it would cost, and what to watch

- **Consistency.** The level is chosen per placed tree so that camera, shadow and bounce rays meet
  the same surface. Every stage above keeps the card's geometry, so this holds.
  - Neural hit queries would not keep it: their hits are regressed, and N-BVH documents
    secondary-ray origins offset where the geometry is concave.
- **Determinism.** A constituent's choice is a hash of the ray today. Any model must be a pure
  function of its inputs, so resumable renders stay identical.
- **Speed.** A 32-entry scan is tens of operations. An MLP is around 10⁴, and these papers get
  their speed from tensor cores, which rust-gpu doesn't expose. The fitted models of stages 1–3
  are cheap. A learned model would have to earn its cost in memory.
- **Instancing already does much of the work.** NVIDIA's HPG 2026 foliage paper finds that, at
  forest scale, instance and top-level memory grow while per-model LOD memory barely does. Our
  placements are about 60 B each, 15 MB for the forest. So the constituents remain worth
  attacking, but the payoff is bounded by 235 MB.

## Pictures

Each sheet is a set of renders of `forest-aerial` against full detail. The right column maps
brightness in linear light (red brighter, blue darker, to ±15%), blurred to show bias rather
than noise. The sheets are made by `scripts/lod-comparison-sheets.py` from the stage renders.

- [From the air](images/aerial.png), and [a crop](images/aerial-crop.png): full detail, the old
  cards, this branch's cards, and the volume prototype.
- [From 90 m up](images/low.png): trees side on.
- [Crossed cards](images/crossed.png): crossed cards alone shadow themselves (stage 2); with
  measured self-shadowing (stage 3).

## Status

**What the branch ships.** The renderer changes the measurements justified:
- cards sized by their leaves' measured coverage (stage 0's fix);
- fewer, better constituents (stage 1).

It also carries the measurement harness. The crossed-card experiments of stages 2–4 are in the
branch's history (commits `ac51d87` to `f56d824`) and written up in their notes, not in the code.


- [x] Literature survey, five clusters (first pass from search snippets: blocked hosts).
- [x] Full-text verification of 30 papers (second pass, paper hosts allowed); corrections folded
      in.
- [x] Fixed the SGGX citation in `src/shapes/lod.rs` and `docs/adaptive-meshes.md`: ACM TOG 34(4),
      SIGGRAPH 2015, doi:10.1145/2766988, not EGSR or CGF.
- [x] Stage 0 measurements ([stage0.md](stage0.md)): the brightening is a hit-probability
      deficit, not self-shadowing; memory sits at L2–L3; compact models beat the 32-leaf sample
      at L4 and coarser.
- [x] Fix card hit probability per card, over directions: the card sizing fix sizes each card by its
      leaves' rasterised coverage over the hemisphere. Brightness vs full +4.6% → −1.2%, FLIP
      0.0585 → 0.0545, build about 10% longer ([stage0.md](stage0.md#follow-up-the-fix)).
- [x] Stages 1 and 4 ([stage1.md](stage1.md)): merge a leaf's triangles exactly, and
      keep 16 NNLS-refitted representatives in place of a 32-leaf sample. Constituents about
      halved, error 1–2% (from 6–10%), peak 700 → 618 MiB, render −14%.
- [x] Side views and crossed cards ([stage2.md](stage2.md)): broadleaf cards miss 34–82% of
      their leaves' hits from the side at L3+; crossed cards fix the hit rate but self-shadow
      artificially and darken the image 13–23% (reverted). A cluster LOD must match both the
      transmittance of rays entering it and of rays leaving it.
- [x] Both transmittances ([stage3.md](stage3.md)): crossed cards for rays entering a cluster,
      measured self-shadowing for rays leaving it. Best FLIP from both viewpoints (0.0454 from
      the air, 0.0251 from low down) but 40% more memory and twice the build: needs a compact
      crossed-card primitive.
- [x] Compact crossed card ([stage4.md](stage4.md)): memory back near the single card's (664 MiB), but
      quality hangs on how far a ray counts as leaving a cluster; crossed cards over-shadow rays
      starting anywhere near them, since clumping reaches across a crown. Transmittance has to
      depend on distance from a ray's origin: the principled route is a non-exponential volume.
- [x] Transmittance in a crown, measured ([stage5.md](stage5.md)): `T_c` (from a leaf) is heavy
      tailed and not predicted from `T_u` by the reciprocal non-exponential relation (errors
      0.20–0.42); a gamma law fitted to each separately fits within about 0.01.
- [x] Coarse clusters as volumes with measured transmittance ([stage6.md](stage6.md), prototype
      behind `LOD_VOLUME_TRIANGLES`): as good as the corrected cards from the air, slightly
      better from the side, 50% slower from the air. Self-shadowing must be measured from the
      leaves hits are on (the hotspot correlation), not all leaves.
- [x] Memory, accuracy and speed across every scheme and `pixels` ([tradeoffs.md](tradeoffs.md)):
      the coverage fix, merged constituents, the speed work and crossed cards move the frontier;
      `pixels` slides along it; compact crossed cards and volumes don't help. Next: the coarse
      levels' brightening (+3% at p2 to +13% at p8).
- [ ] The residual darkening from low down (−3% to −6% at mid range), common to cards and
      volumes; SGGX lobes and palettes.
- [ ] Read what could not be fetched (see [log.md](log.md)).
