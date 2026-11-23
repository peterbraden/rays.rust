# Bibliography

Each entry gives its citation, how much we read and what it contributes. Numbers are copied from
the paper's own tables. Section and equation references, with longer quotations, are in the
[notes](notes/).

**Status key.** **Full text**: read from the PDF. **Abstract**: only the abstract, the project
page or a citing paper; claims beyond that are marked unverified. **Not fetched**: the host was
blocked here; anything said about it is from memory and unverified.

## Learned data structures

| Paper | Status |
|---|---|
| Kraska, Beutel, Chi, Dean, Polyzotis. *The Case for Learned Index Structures*. SIGMOD 2018. doi:10.1145/3183713.3196909, arXiv:1712.01208 | Full text |
| Mitzenmacher. *A Model for Learned Bloom Filters, and Optimizing by Sandwiching*. NeurIPS 2018. arXiv:1901.00902 | Full text |
| Vaidya, Knorr, Mitzenmacher, Kraska. *Partitioned Learned Bloom Filters*. ICLR 2021. arXiv:2006.03176 | Full text (arXiv v2) |
| Ferragina, Vinciguerra. *The PGM-index: a fully-dynamic compressed learned index with provable worst-case bounds*. PVLDB 13(8):1162–1175, 2020. arXiv:1910.06169 | Full text (both versions) |
| Nathan, Ding, Alizadeh, Kraska. *Learning Multi-dimensional Indexes* (Flood). SIGMOD 2020. arXiv:1912.01668 | Full text |
| Ding, Nathan, Alizadeh, Kraska. *Tsunami*. PVLDB 14(2), 2020. arXiv:2006.13282 | Full text (main sections) |
| Yang et al. *Deep Unsupervised Cardinality Estimation* (Naru). PVLDB 13(3), 2019. arXiv:1905.04278 | Full text |
| Müller, McWilliams, Rousselle, Gross, Novák. *Neural Importance Sampling*. ACM TOG 38(5), 2019. arXiv:1808.03856 | Full text (method) |
| Liu, Fischer, Yoo, Ritschel. *Neural Bounding*. SIGGRAPH 2024. arXiv:2310.06822 | Full text |
| Potamias, Ploumpis, Zafeiriou. *Neural Mesh Simplification*. CVPR 2022 | Full text |
| Figueiredo, He, Bako, Kalantari. *Neural Importance Sampling of Many Lights*. arXiv:2505.11729, 2025 | Abstract |
| Li, Lu, Zheng, Yang, Pan. *LISA: A Learned Index Structure for Spatial Data*. SIGMOD 2020 | Not fetched |
| Hsu, Indyk, Katabi, Vakilian. *Learning-Based Frequency Estimation Algorithms*. ICLR 2019 | Not fetched |
| Ludwig, Campen, Ritschel et al. *Strictly Conservative Neural Implicits*. Pacific Graphics 2024. doi:10.1111/cgf.15241 | Not fetched |

- **Kraska et al.** A model plus an auxiliary structure keeps the original's guarantees.
  - RMI error bounds are stored per last-stage model and measured over the stored keys only
    (Sec. 2, Sec. 3.3). The hybrid B-tree fallback is Algorithm 1, but it is not in the reported
    results.
  - The learned Bloom filter is a classifier plus an "overflow" filter for its false negatives,
    with FPR_O = FPR_τ + (1 − FPR_τ)·FPR_B.
  - On URLs (Sec. 5.2) it saved 36% at 1% FPR (2.04 → 1.31 MB) and 15% at 0.1%.
- **Mitzenmacher.** A learned Bloom filter's FPR depends on the query distribution (Sec. 3.2,
  Theorem 4).
  - Sandwiching: FPR = α^{b1}(F_p + (1 − F_p) α^{b2/F_n}), and the optimal b2 is a constant.
  - The **learned Bloomier filter** (Sec. 6, Sec. 8) is a key → value map with exceptions stored
    exactly. It is the closest formal analogue to a model returning constituents. Values are
    expensive, so the model pays only if it gets most of them right.
- **Vaidya et al.** Under a relaxed problem, the most space saved grows linearly with the KL
  divergence between the key and non-key score distributions (eq. 7–10). This is a derivation,
  not a theorem.
- **Ferragina & Vinciguerra.** The tolerance ε comes first and holds for every key in a segment's
  range. Their construction finds the fewest segments in O(n) (Lemma 1, after O'Rourke).
  Published results:
  - 83× less space than a static B+-tree;
  - 71% faster in 1140× less space, dynamic;
  - mean absolute error 226 ± 139, against RMI's 892 ± 3729.
- **Flood and Tsunami.** The answers are exact because every scanned record is filtered. The
  learned parts change only the cost.
- **Naru.** An autoregressive model gives exact probabilities, and its progressive sampling is
  unbiased *with respect to the model* (Theorem 1). The "entropy gap" (KL, in bits per tuple) is
  the error that sampling cannot remove: 4.23 → 0.84 bits as the model grows from 0.6 to 3.8 MB.
- **Neural Importance Sampling.** An exact q(X) is needed for an unbiased estimator. It is defined
  on continuous domains only.
- **Neural Bounding.** Zero false negatives is a goal, reached empirically: "fewer than 1 FN in 100
  million queries", "we lack any proof". A network test took 70.82 ms against 2.58 ms for boxes on
  the same batch.
- **Neural Mesh Simplification.** It learns a probability per triangle, but thresholds it at 0.5 at
  inference. QEM is more accurate; this method is about 10× faster.
- **Neural Importance Sampling of Many Lights.** A learned distribution over clusters of a stored
  discrete set, with exact sampling within clusters. A proposal, not a replacement for the data.

## Neural ray queries and neural LOD

| Paper | Status |
|---|---|
| Fujieda, Kao, Harada. *Neural Intersection Function*. CGF 42(8), HPG 2023. arXiv:2306.07191 | Full text |
| Fujieda, Kao, Harada. *LSNIF: Locally-Subdivided Neural Intersection Function*. PACMCGIT 8(1), 2025. doi:10.1145/3728295, arXiv:2504.21627 | Full text |
| Kao, Makowski, Fujieda, Harada. *Voxel Deformation-Aware Neural Intersection Function*. EG 2026 short. arXiv:2604.24666 | Full text |
| Weier, Rath, Michel, Georgiev, Slusallek, Boubekeur. *N-BVH: Neural ray queries with bounding volume hierarchies*. SIGGRAPH 2024. doi:10.1145/3641519.3657464, arXiv:2405.16237 | Full text |
| Weier, Zirr, Kaplanyan, Yan, Slusallek. *Neural Prefiltering for Correlation-Aware Levels of Detail*. ACM TOG 42(4), 2023. doi:10.1145/3592443 | Full text |
| Wang, Lu, Chen, Wang. *Neural Explicit Shape Representation* (NeuOctree). arXiv:2610.38441, 2026 | Full text |
| Bako, Sen, Kaplanyan. *Deep Appearance Prefiltering*. ACM TOG, 2023. doi:10.1145/3570327, arXiv:2211.05932 | Full text |
| Stadter, Hofmann, Stamminger. *Neural Volumetric Level of Detail for Path Tracing*. VMV 2024. doi:10.2312/vmv.20241197 | Full text |
| Wu, Lu, Xu, Hašan, Ramamoorthi. *8DNA: 8D Neural Asset Light Transport by Distribution Learning*. SIGGRAPH 2026. arXiv:2604.25129 | Full text |
| Takikawa et al. *Neural Geometric Level of Detail*. CVPR 2021. arXiv:2101.10994 | Abstract |
| Takikawa et al. *Variable Bitrate Neural Fields*. SIGGRAPH 2022. arXiv:2206.07707 | Abstract |
| Müller, Evans, Schied, Keller. *Instant Neural Graphics Primitives*. ACM TOG 41(4), 2022. arXiv:2201.05989 | Abstract |
| Edavamadathil Sivaram, Li, Ramamoorthi. *Neural Geometry Fields for Meshes*. SIGGRAPH 2024. doi:10.1145/3641519.3657399 | Abstract |
| Zeltner et al. *Real-Time Neural Appearance Models*. ACM TOG 43(3), 2024. arXiv:2305.02678 | Abstract |
| Kuznetsov et al. *NeuMIP*. ACM TOG 40(4), 2021. arXiv:2104.02789 | Abstract |
| Nikolaev, Mozokhin, Rodionov, Frolov. *Double Bounded Neural Ray Queries*. IJCAI 2026 | Abstract |

- **NIF.** Shadow rays only, trained online from the current view, up to 35% faster.
  - Networks: 2×64 (outer) and 3×48 (inner), with an L2 loss.
  - About 321 kB per object. No foliage tested.
- **LSNIF.** One network per object, trained offline in about 15 s.
  - Outputs: occlusion (BCE, threshold 0.5), distance, normal, albedo, and a **softmax material
    index trained on hard labels and used as one prediction**.
  - Primary rays are rasterised. It has no LOD.
  - Compression: 106.2× on Hairballs against a compressed BVH (525.1× uncompressed); 6.24 MB for
    Statues.
  - Botanic Cornell Box (foliage): 23.40 MB against a 3.8/0.77 GB BVH, FLIP 0.108–0.142, errors
    "mostly due to missing intersections".
- **N-BVH.** A cut of the scene's BVH, with a hash grid and a 4×64 MLP inside its nodes.
  - Compression: 42× on the subset the network represents and 13× over the Fig. 1 scene; more than
    1000× on the neural part alone, 5–100× over whole scenes.
  - 2–4× slower at high quality.
  - Assumes the geometry in a node is convex or concave, which offsets secondary-ray origins when
    there are few nodes.
- **Weier 2023.** Voxel LODs with two networks.
  - Visibility: a sigmoid probability between points on a voxel's boundary, thresholded per voxel
    by F-measure. Used as a raw probability "is susceptible to light leaks".
  - Appearance: spherical harmonics for directions plus a hash grid.
  - Table 2: Bay Cedar 54.14 MB against 752.92 MB, Pandanus 53.81 MB against 1.40 GB.
  - 7–40 min training. Slower than path tracing the full geometry.
  - Pandanus doesn't conserve energy.
- **Bako.** Per voxel: a 4D phase table, a directional albedo and a 16×16 coverage mask per
  direction, which carries correlation.
  - About 3 KB per voxel; direct light only.
  - 0.5–2 days of data generation on 256 GPUs.
- **Stadter.** Lighting baked into a NeRF, then voxelised.
  - The fine levels exceed the mesh: PBRT Tree 0.19 GB against 3.08 GB at LOD1.
  - Landscape capture took 131 h.
  - Biased.
- **8DNA.** A normalizing flow over where light leaves an asset, trained by throughput-weighted
  likelihood and sampled at render time.
  - About 2 h of training. No foliage.
  - The closest *method* to "learn a distribution, sample it per hit", but its distribution is
    over directions, not identities.
- **NeuOctree.** Feed-forward construction in 0.05 s, so no per-asset training. Hit, normal and
  albedo; no identities, no foliage.

## Prefiltered and volumetric LOD, and stochastic simplification

| Paper | Status |
|---|---|
| Cook, Halstead, Planck, Ryu. *Stochastic Simplification of Aggregate Detail*. ACM TOG 26(3), SIGGRAPH 2007 | Not fetched |
| Heitz, Dupuy, Crassin, Dachsbacher. *The SGGX Microflake Distribution*. ACM TOG 34(4), 48:1–48:11, SIGGRAPH 2015. doi:10.1145/2766988 | Full text |
| Heitz. *Understanding the Masking-Shadowing Function in Microfacet-Based BRDFs*. JCGT 3(2), 2014 | Full text (Sec. 3) |
| Heitz, Neyret. *Representing Appearance and Pre-filtering Subpixel Data in Sparse Voxel Octrees*. HPG 2012. doi:10.2312/EGGH/HPG12/125-134 | Full text |
| Loubet, Neyret. *Hybrid mesh-volume LoDs for all-scale pre-filtering of complex 3D assets*. CGF 36(2), EG 2017 | Not fetched |
| Loubet, Neyret. *A new microflake model with microscopic self-shadowing for accurate volume downsampling*. CGF 37(2), EG 2018. doi:10.1111/cgf.13346 | Full text |
| Prus, Eisenacher, Stamminger. *Level-of-Detail for Production-Scale Path Tracing*. VMV 2015. doi:10.2312/vmv.20151260 | Full text |
| Zhou, Huang, Ramamoorthi, Sen, Yan. *Efficient Scene Appearance Aggregation for Level-of-Detail Rendering*. arXiv:2409.03761 (ACM TOG 44(1), 2025) | Full text (arXiv) |
| Fabre, Castillo, Rodriguez-Pardo, Lopez-Moreno. *Fast Voxelization and Level of Detail for Microgeometry Rendering*. arXiv:2604.13191, 2026 | Full text |
| Condor, Hermann, Yurtsever, Didyk. *Gabor Fields*. ACM TOG 45(4), 2026. doi:10.1145/3811369 | Full text |
| Lacewell, Burley, Boulos, Shirley. *Raytracing Prefiltered Occlusion for Aggregate Geometry*. RT 2008 | Not fetched |
| Neyret. *Modeling, Animating, and Rendering Complex Scenes Using Volumetric Textures*. IEEE TVCG 4(1), 1998 | Abstract |
| Décoret, Durand, Sillion, Dorsey. *Billboard Clouds for Extreme Model Simplification*. SIGGRAPH 2003 | Abstract |
| Yoon, Lauterbach, Manocha. *R-LODs*. Visual Computer 22, 2006 | Abstract (see [Out-of-core and streamed geometry](#out-of-core-and-streamed-geometry)) |
| Gobbetti, Marton. *Far Voxels*. SIGGRAPH 2005 | Abstract |
| Bruneton, Neyret. *Real-time Realistic Rendering and Lighting of Forests*. CGF 31(2), 2012 | Abstract |
| van Antwerpen, Gautron, Biddulph, Kubisch, Schmid, Stich. *Real-time Path Tracing of Massive Dynamic Foliage*. HPG 2026. doi:10.1145/3820021 | Abstract and press (dl.acm.org blocked) |

- **SGGX.**
  - σ(ω) = √(ωᵀSω) (Eq. 11), and the visible normals are D_ω = ⟨ω, m⟩D/σ(ω) (Eq. 21). The latter
    is exactly the card's a|n·d|, and its footnote 2 says it assumes visibility is independent of
    orientation.
  - Fitting (Sec. 4.3): take the eigenvectors of the second moment Σ, with eigenvalues σ²(ω_k)
    along them. It matches σ along three directions only, and S ≠ Σ (footnote 1).
  - Storage is 6 B per voxel (Eq. 19), and visible normals are sampled exactly by Algorithm 1.
  - Its main stated limitation is that density is prefiltered without correlation.
- **Heitz & Neyret 2012.** A-buffer masks for occlusion correlated along a ray.
  - Colour correlated with depth has a closed form (Eq. 17):
    ā(v,l) = ā + a_s[2(Λv + Λl + 1)/(Λv + Λl + 2) − 1].
- **Loubet & Neyret 2018.** A microflake medium with self-shadowing A(ω):
  - attenuation σ_t = Aρ∫D⟨ω·m⟩;
  - single scattering scaled by A;
  - local multiple scattering σ_ms = α_ms(σ_t − σ_ss/α_ss);
  - fitted per voxel by ray casting (Eq. 7.6–7.16; Eq. 7.13's "arg max" is a least-squares minimum).
  - The scalar A was as good as the anisotropic one on cedar and pine.
  - Losing self-shadowing "leads to overly bright LoDs" (Fig. 2c).
- **Prus et al.** "randomly draw a normal during shading" from a per-voxel normal distribution:
  prior art for the card's per-hit normal.
- **Zhou et al.** Not neural: a factored aggregated BSDF, truncated ellipsoids, and interior and
  boundary visibility tables.
  - Forest: 16.2 M (175.2 M instanced) triangles, 2.5 → 1.06 GB, 16,064 s.
  - Direct lighting only.
- **Fabre et al.** SGGX-H: greedily merges the two SGGX lobes nearest by Wasserstein distance
  between sampled histograms. Tested on hibiscus, gardenia and grass.
- **Gabor Fields.** Per segment, chooses at random which groups of kernels to integrate and
  reweights. Unbiased for one segment, biased under multiple scattering.

## Correlated transport

| Paper | Status |
|---|---|
| Jarabo, Aliaga, Gutierrez. *A Radiative Transfer Framework for Spatially-Correlated Materials*. ACM TOG 37(4), 2018. arXiv:1805.02651 | Full text |
| d'Eon. *A reciprocal formulation of non-exponential radiative transfer. 1: Sketch and motivation*. arXiv:1803.03259 | Full text (Sec. 2.3) |
| Bitterli, Ravichandran, Müller, Wrenninge, Novák, Marschner, Jarosz. *A radiative transfer framework for non-exponential media*. ACM TOG 37(6), 2018. doi:10.1145/3272127.3275103 | Not fetched |
| Vicini, Jakob, Kaplanyan. *A Non-Exponential Transmittance Model for Volumetric Scene Representations*. ACM TOG 40(4), 2021. doi:10.1145/3450626.3459815 | Not fetched (form second-hand, from Speierer & Jarabo, arXiv:2603.02887) |
| Miller, Chen, Lai, Gkioulekas. *Objects as Volumes: A Stochastic Geometry View of Opaque Solids*. CVPR 2024. arXiv:2312.15406 | Full text (Sec. 3) |
| Nilson 1971; Ross 1981; Chen & Black 1992; Chen & Cihlar 1995 (canopy gap fraction, G-function, clumping index) | From memory and secondary sources |

- **Jarabo et al.** With a gamma-distributed density, T(t) = (1 + σt/β)^−α (Eq. 20), with α and β
  from the mean and variance of the density (Eq. 19).
- **d'Eon.** A path starting on a scatterer has T_c(s) = T_u′(s)/T_u′(0) (Eq. 7–12). For the gamma
  law, T_c = (1 + σt/β)^−(α+1).
- **Vicini et al.** T = lerp([1 − τ]₊, e^−τ, γ), a blend of linear and exponential transmittance
  (second-hand).
- **Miller et al.** Transmittance is exponential if and only if the indicator along the ray is
  Markov (Theorem 4). Then σ = |ω·∇v|/v, which factors as density times projected area (Eq. 12–14).

## Out-of-core and streamed geometry

| Paper | Status |
|---|---|
| Wald, Dietrich, Slusallek. *An Interactive Out-of-Core Rendering Framework for Visualizing Massively Complex Models*. EGSR 2004, pp. 81–92. doi:10.2312/EGWR/EGSR04/081-092 | Full text |
| Karis, Stubbe, Wihlidal. *Nanite: A Deep Dive*. SIGGRAPH 2021 course, Advances in Real-Time Rendering in Games (slides with speaker notes) | Full text (LOD, streaming, compression and results closely; rasterizer and materials skimmed) |
| Pharr, Kolb, Gershbein, Hanrahan. *Rendering Complex Scenes with Memory-Coherent Ray Tracing*. SIGGRAPH 1997, pp. 101–108. doi:10.1145/258734.258791 | Not fetched |
| Christensen, Laur, Fong, Wooten, Batali. *Ray Differentials and Multiresolution Geometry Caching for Distribution Ray Tracing in Complex Scenes*. CGF 22(3), Eurographics 2003, pp. 543–552. doi:10.1111/1467-8659.t01-1-00702 | Not fetched (abstract from search results) |
| Yoon, Lauterbach, Manocha. *R-LODs: Fast LOD-based Ray Tracing of Massive Models*. The Visual Computer 22, pp. 772–784, 2006. doi:10.1007/s00371-006-0062-y | Not fetched (abstract from search results) |
| Eisenacher, Nichols, Selle, Burley. *Sorted Deferred Shading for Production Path Tracing*. CGF 32(4), EGSR 2013, pp. 125–132. doi:10.1111/cgf.12158 | Not fetched (abstract from search results) |

- **Wald et al.** Precomputed BSPs and geometry stored in binary and mapped with `mmap()`; the
  render threads detect missing tiles through a hash table and cancel the ray instead of faulting.
  - 4–8 fetcher threads; fetch priority from voxel size over distance, in 8 queues; "second
    chance" eviction from about 80% memory use.
  - The manager costs 16–26% of frame rate with everything resident (Table 3).
  - Proxies for missing subtrees: 18 directions of averaged colour and normal, 80 bytes each,
    66 MB to 1.2 GB (Table 1); averaged normals make them "overly bright".
  - 30–40 GB on disk, about one day of preprocessing; per-view working sets 36 MB to 2.3 GB
    (Table 2).
- **Nanite.** Cluster-group DAG with a unique, parallel cut from monotonic errors.
  - Streams cluster groups in fixed 128 KB pages (~1% slack); root page and the whole culling
    hierarchy always resident; requests prioritised by LOD error, low-priority pages evicted.
  - Lumen in the Land of Nanite: 433M input triangles, memory format 7.67 GB, disk format 4.61 GB,
    5.6 bytes per Nanite triangle, 11.4 per input triangle.
  - "Not great with aggregates" (grass, leaves, hair); shadows rasterised, not ray traced.
- **Pharr, Christensen, Yoon, Eisenacher.** Unread; see the
  [notes](notes/streaming.md) for what is known second-hand.
