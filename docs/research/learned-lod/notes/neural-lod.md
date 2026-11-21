# Checking the neural and learned LOD papers against their full texts

The question: could a much smaller learned or fitted structure return hits, and the original leaves ("constituents") behind them, with the probabilities the full foliage would give? This matters for rays.rust's leaf cards (docs/adaptive-meshes.md). There, each card keeps up to 32 leaves at 12 B each, and a hit is shaded as one leaf chosen in proportion to a|n·d|. The constituents take 235 MB of a 670 MB peak, and the LOD forest renders about 3% brighter than full detail.

I downloaded every PDF to its own folder under `(local downloads)` and converted it with pdftotext, in both reading order (`r.txt`) and layout (`p.txt`). All 12 papers were fetched and read in full: method, results tables, limitations. I skimmed the related-work sections and skipped the references.

---

## Main answer for the novelty claim

**None of the 12 papers outputs a distribution over original primitive or constituent identities that is sampled at render time and calibrated to visible area.** None trains on soft labels over identities either. The nearest pieces are these:

1. **LSNIF's material-index head** is the only categorical output over identities of any kind. It has a softmax over the few *materials* attached to an object (8 + N_mat output neurons). It is trained with categorical cross-entropy on hard labels taken from BVH ray casts. At render time the paper uses "the predicted material index", and nothing is sampled. Quotes:
   - "The material index is an integer value indexing one material attached to the object … We can consider this as a multi-class classification problem, so we use the softmax activation function and the categorical cross-entropy loss."
   - "Other BSDF parameters are determined by looking up the attached material with the predicted material index."

   The paper does not literally say "argmax", but it describes no sampling. VD-NIF inherits the head ("Material is multi-class, occlusion binary; both follow the classification weighting L_cls").
2. **Weier et al. 2023's visibility network** outputs a sigmoid *probability* of mutual visibility between two points on a voxel's boundary. It is trained with BCE on Monte Carlo binary labels, so in expectation it is a calibrated occlusion probability. By default it is **thresholded per voxel**. A stochastic use exists, but only as a comparison, and it leaks light:
   - "In the stochastic alternative, the output of the visibility network is directly interpreted as a probability. For volume-like details, it can sometimes outperform binary classification … For general scenes, however, such a simplified stochastic approach is susceptible to light leaks" (Sec. 6, Fig. 20).
   - The continuous probability is also used for a Russian-roulette traversal: "We use the inferred probability of non-collision in each voxel as a lower bound for the survival probability" (Sec. 4.5).

   This is visibility only: it carries no identity.
3. **8DNA** learns a continuous *distribution* p_θ(x_i, ω_i | x_o, ω_o) of where light leaves an asset, with a normalizing flow. It is trained by throughput-weighted negative log-likelihood on forward path samples, one sample per query: "L_p = E[−β_i log p_θ(x_i, ω_i | x_o, ω_o)]" (Eq. 6). It is sampled at render time (Algorithm 3). This is the closest methodological template, "learn a distribution from Monte Carlo samples, sample it while rendering", but it covers continuous exit positions and directions, not identities. It also keeps the asset's geometry for `SurfaceIntersect`.
4. **Gabor Fields** stochastically chooses *which subsets of primitives* (frequency bands, orientation bins) a path segment intersects, and reweights by the sampling probability: "for each path segment, we stochastically decide which frequency bands of the asset to integrate". This selects groups of kernels for transmittance. It neither picks a constituent to shade nor learns anything.
5. Every other hit query (N-BVH, NIF, NeuOctree) **thresholds a binary hit at 0.5**, or at a tuned threshold, and regresses one distance, normal and albedo. Their outputs carry no identity.

A caution for the write-up: training a categorical head with cross-entropy on *sampled* one-hot hits is, in expectation, the same as cross-entropy against the soft visible-area distribution. "Soft labels" alone therefore isn't a strong difference. What none of these papers does:
- (a) a categorical distribution over an aggregate's *original constituents*, whose target is calibrated to visible area (a|n·d|, or a pixel's coverage);
- (b) *sampling* it per hit, so that shading is unbiased in expectation over the original leaves rather than arg-maxed;
- (c) replacing explicitly stored constituent lists with such a model.

This conclusion is limited to these 12 papers. Cook et al. 2007 (stochastic simplification, which the doc already cites) is the non-learned prior art on choosing leaves at random.

Where the papers report the brightening already seen in rays.rust:
- Weier 2023 quotes Hybrid LoD (Loubet & Neyret 2017) as showing "noticeable undesired brightening" on the Oak Tree at coarse levels.
- Stadter et al. report "light leaking due to approximating fully opaque geometry with exponential transmittance".
- LSNIF's foliage errors are "mostly due to missing intersections".
- Weier 2023 reports that on Pandanus the "overall transported energy is [not] fully conserved".

---

## 1. Weier, Zirr, Kaplanyan, Yan, Slusallek: "Neural Prefiltering for Correlation-Aware Levels of Detail"

**FULL TEXT READ.** ACM TOG 42(4), Article 1 (August 2023, SIGGRAPH), DOI 10.1145/3592443. Code: github.com/WeiPhil/neural_lod.

**Method**
- **Voxel grid (Sec. 4.1).** A sparse multi-level grid. LoD 0 is 512³, and the levels halve down to 8³ (7 LoDs).
- **Decomposition (Sec. 3).** A voxel path space separates the point-to-point visibility V_M(p_o, p_i) from a position-free, far-field "average intra-voxel throughput" φ_V(ω_o, ω_i) (Eq. 3). That throughput acts like a BSDF of the voxel.
- **Appearance network (Sec. 4.2).**
  - Inputs: ω_o and ω_i, encoded as spherical harmonics up to degree 8, and the voxel centre, encoded by a multi-resolution hash grid with one grid level per LoD and table size 2^19. Because voxel centres never coincide across levels, the centre "implicitly encodes the level of detail".
  - Network: an MLP with 4×128 ReLU layers and no output activation, giving φ_V (RGB throughput).
  - Loss: the noise-to-noise relative L2 of Lehtinen et al. Training targets are path-traced estimates inside the voxel (Sec. 5.2).
- **Visibility network (Sec. 4.3).**
  - Inputs: the two boundary points p_o and p_i only, in *object space*. A "recurrent composite encoding" shares one hash grid between both inputs.
  - Network: 4×128 ReLU layers with a **sigmoid output, interpreted "as the probability that the points p_o and p_i are mutually visible"**. Trained with BCE.
  - No LoD or voxel input. Because the encoding is in object space, a segment shared by two levels is the same query (Fig. 8). This is the only sense in which correlation "aggregates across levels": there is no explicit aggregation step.
- **Optimal thresholds (Sec. 4.4).**
  - "we train the visibility network as a classifier that predicts binary point-to-point visibility." The threshold is chosen **per voxel** by maximising a weighted F-measure that favours detecting occlusion.
  - Search: a grid of 100 thresholds in (0, 1] with 1000 visibility samples per voxel, taking 20–120 s for all LoDs. One threshold per voxel is stored in a NanoVDB grid.
  - Per-voxel thresholds remove the light leaks that a fixed 0.5 leaves (Fig. 10).
- **Rendering (Sec. 4.1).** March the sparse grid. In each non-empty voxel, query visibility for the entry-to-exit segment. If it is occluded, sample ω_i, evaluate φ_V, and continue the path from the voxel boundary p_o.
  - The stochastic Russian-roulette traversal survives with about 99.25% probability per voxel at 512³.
  - Directions are not importance sampled: "Our approach does not apply any importance sampling" (Fig. 17).
- **Test assets.** Arcade, **Bay Cedar** and **Pandanus** (from the Moana Island scene), Rover, Fractal; also the **Oak Tree** and **Bare Tree** (against Hybrid LoD) and the tree of Bako et al. (Fig. 19). Foliage is therefore well represented.

**Numbers (Table 2)**

| Scene | Visibility network | Appearance network | Voxel grid | Total | Original | Saving |
|---|---|---|---|---|---|---|
| Arcade | 7.63 MB | 7.60 MB | 4.08 MB | 19.31 MB | 68.28 MB | 71.72% |
| Bay Cedar | 30.09 MB | 15.12 MB | 8.93 MB | 54.14 MB | 752.92 MB | 92.81% |
| Rover | 15.10 MB | 15.12 MB | 9.45 MB | 39.67 MB | 187.41 MB | 78.83% |
| Pandanus | 30.09 MB | 15.12 MB | 8.60 MB | 53.81 MB | 1.40 GB | 96.16% |
| Fractal | 30.09 MB | 15.12 MB | 86.23 MB | 131.44 MB | 1.92 GB | 93.15% |

The abstract says "70–95% compression ratios" and "around 25% of quality improvements over previous work". About seven scalars are stored per voxel on average.
- **Oak Tree against Hybrid LoD:** Hybrid LoD takes 82.94 MB (3.72 MB mesh and textures plus 79.22 MB of volumes). Ours takes 60.36 MB (22.64 MB grid plus 37.72 MB networks), which the paper calls 27% better.
- **Training (Table 1, RTX 3080):** total time ranges from 7m12s (Arcade) to 40m16s (Fractal). Bay Cedar takes 20m49s and Pandanus 21m4s, with 8000–12000 steps to convergence. The abstract says "10–20 minutes of training" for each network.
- **Rendering cost:** Fig. 16 renders 512² images at 1024 spp. Bay Cedar takes 96 s at LoD 0 and 11 s at LoD 6; Pandanus takes 74 s and 18 s. The paper reports speedups "up to 5× between the highest and the lowest levels of detail" and states: "Within our prototype, we cannot yet compete with raw real-time path tracing performance." FLIP for Pandanus is 0.19–0.21 at LoD 0–2.
- **Limitations:**
  - Pandanus is the hardest case: "difficulty in robustly learning visibility and appearance such that the overall transported energy is fully conserved. Accuracy fluctuates across multiple training reruns".
  - The spherical-harmonics directional encoding limits glossy highlights.
  - Runtime importance sampling is needed.
  - Dynamic content is not handled.

**Corrections:** The visibility output is a *probability*, but in use it is binary, through a per-voxel threshold tuned by F-measure, not a fixed 0.5. Correlation across levels comes only from the object-space encoding being shared; it is not aggregated explicitly. Also, Weier's description of Bako et al. ("256 scalars per voxel (~1KB)") does not match Bako's own figure of 797 floats, about 3 KB (see item 6).

## 2. Weier, Rath, Michel, Georgiev, Slusallek, Boubekeur: "N-BVH: Neural ray queries with bounding volume hierarchies"

**FULL TEXT READ.** SIGGRAPH Conference Papers '24, DOI 10.1145/3641519.3657464, arXiv 2405.16237.

**Method**
- **Structure (Sec. 4).** One global hash grid (8 levels from 8³ to 1024³, 4 features per level; only the table size varies) plus a small MLP (4×64 ReLU). The grid is probed at stratified points along each ray's interval through the leaves of a shallow BVH. The leaves are a cut of the scene's ordinary BVH, refined by splitting nodes by error rank r = 2 log q + log p (Sec. 5.1).
- **Outputs.**
  - Visibility: sigmoid output, "thresholding the output against 0.5", BCE loss.
  - Local hit distance: L1 loss.
  - Normal: L1 loss.
  - Albedo: relative L2 loss.
  - Combined loss: L = 2L_vis + 2L_dist + L_normal + L_albedo.
  - Other BSDF parameters are fixed by the user. There is no material identity.
- **LoD (Sec. 5.3).** Several tree cuts are registered during construction. Switching to a coarser cut after the primary hit gives "1.5–2× faster rendering with only slight increase in reconstruction error".

**Numbers**
- **Fig. 1:** "compression rate of 42× for the subset of geometry it represents, and 13× over the entire scene". Original 105 MB + 1780 MB; ours 105 MB + 43 MB.
- **The >1000× claim is real:** "On that portion alone we can achieve over 1000× compression rates over the original geometry footprint. On the entire scene, the compression rates are 5−100×" (Sec. 7, Fig. 7).
- **Table 1 (RTX 3090, 1920×1080).** Columns are path tracing time and memory, then two hybrid configurations (time, memory, FLIP):

  | Scene | Path tracing | Hybrid, fewer nodes | Hybrid, more nodes |
  |---|---|---|---|
  | Chess | 6.6 ms, 329 MB | 10 ms, 36 MB, FLIP 0.057 | 19 ms, 37 MB, FLIP 0.019 |
  | Bonzai | 28 ms, 853 MB | 29 ms, 181 MB, FLIP 0.069 | 102 ms, 186 MB, FLIP 0.023 |
  | Exhibition | 13 ms, 2.69 GB | 14 ms, 181 MB, FLIP 0.027 | 34 ms, 182 MB, FLIP 0.010 |
  | And. Room | 28 ms, 309 MB | 31 ms, 108 MB, FLIP 0.088 | 55 ms, 109 MB, FLIP 0.055 |
  | City Block | 24 ms, 1.55 GB | 22 ms, 180 MB, FLIP 0.039 | 80 ms, 185 MB, FLIP 0.014 |
  | Statuette | 4.9 ms, 642 MB | 3.7 ms, 10.8 MB, FLIP 0.020 | 12 ms, 11.2 MB, FLIP 0.007 |

  Caption: "Our approach achieves 2–4× higher render times." Training takes "a couple of minutes".
- **Prefiltering application (Sec. 8, RTX 3080).** Weier 2023's voxel visibility grid is replaced by the N-BVH; Weier's appearance network is kept. Bay Cedar uses 127 nodes at the coarsest level and 176k at the finest, with a 2^18 hash table.

  | Scene | N-BVH training | N-BVH network + acceleration | Weier '23 training | Weier '23 network + acceleration |
  |---|---|---|---|---|
  | Bay Cedar | 3:25 min | 10.8 + 5.7 MB | 10:57 min | 30.1 + 8.9 MB |
  | Rover | 2:11 min | 10.8 + 0.7 MB | 6:14 min | 15.1 + 9.5 MB |

  The paper claims "easily … a 2× speed improvement". After a hit, the secondary ray must start "on the previously determined appearance-voxel's boundary rather than on the intersected surface".

**Limitations (stated)**
- The model assumes the geometry inside a neural node is convex or concave, which "can lead to incorrect offsets of secondary ray origins when using a low number of N-BVH nodes". It "did not manifest" at 1.2k nodes or more.
- Each node takes a fixed number of encoded points (3).
- No dynamic geometry.
- "we cannot yet compete with the speed of raw path-tracing algorithms".

**Foliage:** only Bay Cedar, in the prefiltering application. Bonzai is "our geometrically most complex scene", but no foliage-specific numbers are given beyond the table above.

**Corrections:** 42× and 13× belong to the Fig. 1 scene. The >1000× applies only to the neural subset's geometry, and over whole scenes the range is 5–100×. On timing, a fair one-line summary is "≈ equal time at reduced quality, 2–4× slower at high quality".

## 3. Fujieda, Kao, Harada: "Neural Intersection Function"

**FULL TEXT READ.** Computer Graphics Forum 42(8) (HPG 2023), arXiv 2306.07191.

**Method**
- **What it targets.** The evaluation uses it only for "secondary ray casting for direct illumination", i.e. shadow rays. Fig. 5 shows primary rays (normal and depth) only as a demonstration.
- **Two networks:**
  - an outer network for rays from outside the AABB, with 2 hidden layers of 64;
  - an inner network for rays from inside, with 3 hidden layers of 48.
- **Inputs and outputs.** Inputs come from 2D feature grids over spherical coordinates of the AABB entry point and the direction, plus a 1D distance grid for the inner network. Leaky ReLU, then a sigmoid "visible probability", **thresholded at 0.5**, trained with **L2** loss (not BCE).
- **Training (Sec. 4.2).** Online, "with rays that are generated from the current viewpoint (camera) … with a first few samples per pixel". Shadow directions come from light importance sampling (Sec. 5). Training takes 0.1–1 s per spp.

**Numbers**
- Up to 35% less secondary-ray casting time (1.53× on Statuette, 10M triangles).
- Fig. 1: Bistro with 30M triangles takes 4.54 ms against 5.27 ms; about 15% in that scene.
- Memory: 256 kB (outer grid) + 65 kB (inner grid) per object at runtime, plus networks shared across objects (18 kB + 21 kB). The paper notes "about 321 kB even for a model with 10M triangles".
- GPU: RX 7900 XTX in Sec. 6; Fig. 1's caption says XT.
- Scenes are Stanford scans, Centaur and Bistro. **No foliage.**

**Limitations:** overfits to the view and lighting; still needs the BVH for training and for primary visibility; low-complexity objects gain nothing (Statuette Low).

**Corrections:** "shadow rays only, online training, 35%" is accurate. The network sizes are 2×64 (outer) and 3×48 (inner). Its loss is L2, not BCE.

## 4. Fujieda, Kao, Harada: "LSNIF: Locally-Subdivided Neural Intersection Function"

**FULL TEXT READ.** Proc. ACM Comput. Graph. Interact. Tech. 8(1), Article 17 (May 2025; HPG), DOI 10.1145/3728295, arXiv 2504.21627.

**Method**
- **Inputs.** Each object is voxelized at 32³ (surface only). DDA finds the ray's crossings of occupied voxel boundaries, at most H = 18. These points are encoded with a 2-level sparse hash grid (64³–128³, table 2^17, features only on voxel boundaries) and concatenated.
- **Network.** One MLP per object, 2×128, leaky ReLU.
- **Outputs and losses (Sec. 5.3):**
  - occlusion: sigmoid, BCE, threshold 0.5;
  - local hit distance: sigmoid, MAE;
  - shading normal: linear, cosine loss;
  - albedo: sigmoid, relative L2;
  - **material index: softmax, categorical CE**, with 8 + N_mat outputs.
  - No texture coordinates.
- **Labels.** Hard values from BVH ray casts. Training rays are 50% from a sphere around the object and 50% from its surface, cosine-distributed, so they don't depend on the view.
- **Primary rays are not used.** "its application to primary rays results in lower accuracy … Therefore, we restrict the application of LSNIF to non-primary rays." Primary visibility comes from a rasterized G-buffer, so vertices and faces are kept.
- **Size and training.** About 1.56 MB per LSNIF: 4 KB voxels, 1,536 KB encoder, about 62 KB MLP. Training is about 15 s per geometry (2000 steps).

**Numbers (Table 1, RX 7900 XTX, 1920×1080, 4 bounces)**
- **106.2× is the Hairballs scene against the *compressed* BVH** (BVH B, estimated from Ylitie et al. 2017). Against the uncompressed 4-wide BVH (BVH A) it is 525.1×. BVH sizes exclude triangles.
- **6.24 MB is the Statues scene** (4 LSNIFs, 18.2M triangles; BVH A 2.5 GB, "0.24%").

| Scene | LSNIF memory | BVH A / BVH B | Time: low quality / high quality / path tracing | FLIP low / high |
|---|---|---|---|---|
| Hairballs | 1.56 MB | 0.8 / 0.16 GB | 45.56 / 65.62 / 84.20 ms | 0.115 / 0.066 |
| Botanic Cornell Box (27.0M triangles, 26 LSNIFs, 15 unique) | 23.40 MB | 3.8 / 0.77 GB | 150.31 / 173.97 / 162.40 ms | 0.142 / 0.108 |
| Botanic Cornell Box Glossy | 23.40 MB | 3.8 / 0.77 GB | 147.35 / 176.47 / 158.76 ms | 0.198 / 0.170 |

The high-quality configuration has an overhead of 1.07–2.06×, except Hairballs, which runs at 0.78×.
- **Foliage:** "The errors in the rendered images with LSNIF are relatively higher … mostly due to missing intersections."

**Limitations (stated)**
- No deformation.
- No texture coordinates.
- Needs rasterization, so perspective cameras only.
- Opaque only.
- A fixed epsilon is used for ray origins.
- "our method does not support levels of detail (LOD)".

**Corrections:** The 106.2× and 6.24 MB come from different scenes, as given above. The paper does say "primary rays not used" (they could be). The material head is a classifier over a handful of materials, trained on hard labels and used as one predicted index.

## 5. Stadter, Hofmann, Stamminger: "Neural Volumetric Level of Detail for Path Tracing"

**FULL TEXT READ.** VMV 2024, DOI 10.2312/vmv.20241197. The PDF came from the diglib bitstream link.

**Method**
- This is not a learned hit query.
- **Capture and training.** For each object, or each grid cell, they render 32 views at 1024² and 1024 spp, with full global illumination baked in. An Instant-NGP NeRF is trained on these, usually without view dependence (about 20,000 iterations, about 2 min).
- **Storage.** The NeRF is voxelized into NanoVDB as density and RGB, 8 B per voxel (32 B with degree-2 spherical harmonics). Colour is averaged weighted by density, and voxels below a density threshold (0.01) are dropped.
- **Rendering.** Ray marching with stochastic texture filtering. Shadow rays are needed only against meshes.

**Foliage numbers**
- **PBRT Landscape:** 23,241 objects, 3.1B triangles without instancing (24M with). 15×6×21 cells, more than 600 occupied.
  - Table 1: capture **131 hours**, training 605 min, voxelization 71 min.
  - "the full large-scale scene with all LOD data did not fit into 10 GB of VRAM".
- **Fig. 4** (ms per spp at 1024², and VRAM; columns Mesh and LOD1–LOD6):

  | Asset | Mesh | LOD1 | LOD2 | LOD3 | LOD4 | LOD5 | LOD6 |
  |---|---|---|---|---|---|---|---|
  | PBRT Tree, time | 40.28 | 42.92 | 38.32 | 23.32 | 12.09 | 6.42 | 3.73 |
  | PBRT Tree, memory | 0.19 GB | 3.08 GB | 0.64 GB | 0.15 GB | 26.3 MB | 6.55 MB | 4.21 MB |
  | Oak Tree, time | 10.99 | 57.62 | 51.96 | 32.26 | 16.42 | 8.10 | 5.09 |
  | Oak Tree, memory | 0.17 GB | 1.01 GB | 0.24 GB | 86.8 MB | 24.8 MB | 6.16 MB | 3.19 MB |

  Fine levels use *more* memory than the mesh.
- **Bias:** 1 spp gives an error similar to 8–16 spp of the mesh, "however, does not converge to the identical result" (Fig. 8).
- **Against Weier 2023:** comparable at fine levels, but it "lags behind for coarser levels of detail, especially levels four to six", because density is isotropic.
- **Limitations:** biased (stochastic gradient descent, exponential transmittance, isotropic inter-voxel visibility, ray marching); no relighting and no instancing (lighting is baked); expensive precomputation; memory grows cubically.

## 6. Bako, Sen, Kaplanyan: "Deep Appearance Prefiltering"

**FULL TEXT READ.** ACM TOG, DOI 10.1145/3570327, arXiv 2211.05932. Weier cites it as Bako et al. 2023.

**What each voxel stores (Sec. 4)**
- A prefiltered *phase function*: a monochrome 4D table over (ω_i, ω_o), N = 128 per slice.
- A directional RGB *albedo* over ω_o.
- A **2D coverage mask α, 16×16, for each outgoing direction**: a 4D table.

**Correlation:** beams carry a "tracked coverage" mask Λ. A voxel contributes the part of its mask not yet covered (Eq. 16): "we do spatially track transparency", but "we do not track color correlations from voxel to voxel".

**Encoding:** CNN encoders produce latent vectors of 256 + 256 + 280 floats (phase or phase slice, coverage, albedo), decoded by residual MLP decoders with TensorRT. "797 floats, just over 3 KB per voxel" (4 KB with all four sub-networks).

**Cost:** data generation "~0.5 to 2 days" on "NVIDIA Volta GPUs (256 in total)"; training "~2 days".

**Scope:** a single bounce (direct lighting only), primary beams. Scenes include Oak (foliage), City and Mossy Rock.

**Limitations:**
- Training per scene.
- Albedo averaged per direction, which brightens the City roof by including the occluded floor.
- Mask resolution limits sub-pixel occlusion.
- Sharp highlights blur.

**Correction:** Weier's "256 scalars per voxel (~1KB)" understates Bako's own figure, about 3 KB.

## 7. Zhou, Huang, Ramamoorthi, Sen, Yan: "Efficient Scene Appearance Aggregation for Level-of-Detail Rendering"

**FULL TEXT READ.** arXiv 2409.03761 (v1 19 Aug 2024; the PDF header reads "October 2024"). **Title correction:** it is not "Appearance-Preserving Scene Aggregation".

**Method.** This is not neural.
- The ABSDF summarises a voxel's surfaces. It has a closed-form factorization (an NDF mixture with up to 4 lobes, diffuse and specular parts) and supports importance sampling.
- A **truncated ellipsoid** (an approximate minimum bounding ellipsoid clipped to the voxel) handles local correlation.
- Long-range correlation comes from precomputed **aggregated interior visibility (AIV, 32² angular)** and **aggregated boundary visibility (ABV, 64²)**, compressed with wavelets (top 10% of coefficients) and CPCA.
- Direct illumination only. Rendering is on the CPU with Embree.

**Forest (Table 2):** 16.2M unique triangles (175.2M instanced), 2.5 GB → **1.06 GB**, precomputation **16,064 s**, maximum resolution 512³, occupancy 2.86%. Other scenes include Palm, Oleander and Coral Reef; Metropolis is 46.9 GB → 5.33 GB.

**Limitations:**
- Direct illumination only.
- Visibility and material are assumed independent inside a voxel (Eq. 6).
- Visibility along two directions is assumed independent (Eq. 25), which fails when camera and light are collocated.

The earlier claims check out (ABSDF, truncated ellipsoids, and the Forest numbers 16.2M/175.2M, 2.5 → 1.06 GB, 16,064 s).

## 8. Wu, Lu, Xu, Hašan, Ramamoorthi: "8DNA: 8D Neural Asset Light Transport by Distribution Learning"

**FULL TEXT READ.** arXiv 2604.25129 (28 Apr 2026).

**What is learned.** The asset's full 8D transport F(x_o, ω_o, x_i, ω_i) is factored as α(x_o, ω_o) · p(x_i, ω_i | x_o, ω_o), per colour channel:
- a **normalizing flow** p_θ for the distribution of where light leaves. The exit is reparameterized to its bounding-box hit u_i in cylindrical coordinates, and a "vector-valued" flow is used.
- an MLP α_θ for the albedo, i.e. the survival probability.
- The direct BSDF term is kept analytic.

**Training.** Throughput-weighted NLL from forward path samples, 1 spp, generated online and resampled 30×, plus L2 on α against the mean throughput.
- Table 3: about 2.23 h total for volumetric assets (0.55 h data, 1.68 h optimization) and 1.78 h for surface assets.

**Rendering.** Exit positions and directions are sampled from p_θ (Algorithm 3), with MIS against emitter sampling. The original asset's surface is still intersected.

**Assets:** Candle, Seal, CurlHair, Hair, Teaset, Cat, Milk, Bunny, Dragon. **No foliage.**

**Limitations:**
- Nothing else may intersect the asset's convex hull.
- No MIS over x_i, so point lights are noisy.
- Flows are slower than MLPs and biased toward smooth distributions.

## 9. Fabre, Castillo, Rodriguez-Pardo, Lopez-Moreno: "Fast Voxelization and Level of Detail for Microgeometry Rendering"

**FULL TEXT READ.** arXiv 2604.13191 (Apr 2026).

**Clustering criterion (SGGX-H, Sec. 4.2).** Greedy agglomerative merging, building a dendrogram. At each step, the **two most similar SGGXs** are merged, where similarity is measured as follows:
- draw N = 5000 samples from each SGGX;
- bin them into histograms of 5 bins per dimension;
- compare the histograms by **Wasserstein distance**, not by SGGX parameters.

The merged lobe is an SGGX fitted to the pooled samples. Each leaf voxel stores one SGGX fitted from its orientation histogram's covariance, and coarser levels use a projection-based density.

**Tests:** vegetation (Hibiscus, Gardenia, grass), hair, a scarf. Nothing is learned or stochastic.

## 10. Condor, Hermann, Yurtsever, Didyk: "Gabor Fields: Orientation-Selective Level-of-Detail for Volume Rendering"

**FULL TEXT READ.** ACM TOG 45(4), Article 62 (July 2026), DOI 10.1145/3811369, arXiv 2602.05081.

**Representation.** A volume is a mixture of Gabor kernels, in a frequency and orientation pyramid. Level of detail comes from pruning primitives.

**Stochastic selection (Sec. 4.5).**
- "for each path segment, we stochastically decide which frequency bands of the asset to integrate". The low-pass band is always included, in a control-variate style.
- For orientation, kernels are binned by orientation, and per ray a selection is importance sampled ∝ exp(−f₀²a²/2)-type dampening, then reweighted by the sampling probabilities.
- Implemented with OptiX ray visibility masks.

**Bias:** "unbiased" for each segment's density integral, but biased in multiple scattering by Jensen's inequality: Bias ≈ Var(τ̂)·T.

**Assets:** volumes (Bunny, clouds). **No foliage.**

## 11. Wang, Lu, Chen, Wang (Peking University): "Neural Explicit Shape Representation" (NeuOctree)

**FULL TEXT READ. The paper exists:** arXiv 2610.38441 (28 Oct 2026).

**Method.** Features are stored at the corners of sparse octree voxels. A shared MLP, with recurrent sampling guided by the intersection, predicts:
- a hit probability, from a sigmoid, accepted above a threshold, with the first accepted candidate taken;
- the intersection position;
- the normal;
- the albedo.

**Results.** It works both per object and as a feed-forward octree CNN from coloured point clouds: "constructs a representation in 0.05 seconds", and transfers from Objaverse to ShapeNet. Across 7 meshes × 4 budgets it has the lowest positional error in 24 of 28 comparisons, against N-BVH and VQAD. No foliage, no LOD study, no identities.

## 12. Kao, Makowski, Fujieda, Harada (AMD): "Voxel Deformation-Aware Neural Intersection Function"

**FULL TEXT READ. The paper exists:** Eurographics 2026 short paper, arXiv 2604.24666.

One line: LSNIF extended to parameterized deformation. Rays in deformed space are mapped back to a canonical rest space, linearized per voxel, so one network serves every pose. It adds scale-invariant (log) distance regression, uncertainty-weighted multi-task losses and positional plus grid encoding. The material head stays a multi-class classifier.

---

## Corrections to the earlier pass, in short

- **Weier 2023:**
  - Visibility is a sigmoid probability *thresholded per voxel* by a weighted F-measure (grid search), not a fixed 0.5.
  - There is no explicit aggregation across levels; levels share one object-space encoding.
  - Compression is 71.72–96.16% (Table 2).
  - Training takes 7–40 min total per asset.
  - The prototype is slower than plain path tracing.
- **N-BVH:**
  - 42× and 13× belong to the Fig. 1 scene.
  - ">1000×" applies only to the neural subset's geometry; over whole scenes it is 5–100×.
  - Rendering is 2–4× slower at high quality.
  - Its only foliage test is Bay Cedar, in the prefiltering application.
- **NIF:** L2 loss, not BCE.
- **LSNIF:**
  - 106.2× is Hairballs against the estimated *compressed* BVH (525.1× against the uncompressed one).
  - 6.24 MB is the Statues scene.
  - Primary rays are deliberately rasterized.
  - No LOD support.
  - On foliage, missed intersections are the main error.
- **Bako:** about 3 KB per voxel, not 1 KB.
- **Zhou:** wrong title in the earlier pass (corrected above); not neural; direct lighting only.
- **NeuOctree and VD-NIF:** both exist; details above.
