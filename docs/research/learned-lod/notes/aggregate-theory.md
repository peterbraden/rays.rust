# Literature check: leaf cards, SGGX lobes and self-shadowing

Context: `src/shapes/lod.rs` and `docs/adaptive-meshes.md`. Distant leaves become rectangular cards. A card's plane comes from Σ a n nᵀ, its area is R(1−e^(−S/R)), and it keeps up to 32 constituent leaves. A hit is shaded as one constituent, chosen with probability ∝ a|n·d|. The LOD forest renders about 3% brighter than full detail.

PDFs and text dumps are under `(local downloads)` (p.pdf, p.txt from `-layout`, raw.txt plain).

## How to read this report

- **FULL TEXT READ** means I read the whole paper and the equations below are copied from it.
- **FROM MEMORY (UNVERIFIED)** means I could not get the PDF. Treat those statements as unchecked.
- Access notes:
  - graphics.pixar.com and the cloudfront host behind rgl.epfl.ch return proxy 403.
  - hal.science and inria.hal.science serve an Anubis proof-of-work bot wall. I did not try to get around it.
  - cs.dartmouth.edu, graphics.stanford.edu and semanticscholar cannot be reached.
  - The GitHub tools only cover this repository.
  - diglib.eg.org's DSpace REST API (`/server/api/core/bitstreams/<id>/content`) served the EG papers that have open bitstreams.

---

## 1. Cook, Halstead, Planck, Ryu, "Stochastic Simplification of Aggregate Detail" (2007)

**Citation.** ACM TOG 26(3) (SIGGRAPH 2007), article 79. doi:10.1145/1275808.1276476.

**Status: COULD NOT FETCH.** graphics.pixar.com returns proxy 403 (CONNECT tunnel failed). The paper is not on arXiv. Prus et al. 2015 (paper 13 below) only restates it in one line: "discard parts of aggregate geometry and adjust the remaining objects to have similar lighting behaviour".

**From memory (unverified):**
- Elements are given a fixed, reproducible random order. The last fraction is pruned, and elements fade out smoothly at the boundary so levels don't pop.
- The kept fraction u comes from the screen size.
- Survivors are enlarged by 1/u in area, so the total area stays the same.
- A colour correction keeps the pixel's statistics. Fewer, larger elements change the variance of the pixel average, so element colours are moved relative to the mean by a factor derived from u.
- Overlap and depth complexity are discussed: scaling the area by 1/u alone over-covers when elements overlap.

I cannot give the exact contrast formula or the overlap treatment. The current docs cite this paper only for the idea of fewer, larger elements keeping the visible area, and that much is safe.

## 2. Heitz, Dupuy, Crassin, Dachsbacher, "The SGGX Microflake Distribution" (2015)

**Citation.** ACM TOG 34(4), 48:1–48:11 (SIGGRAPH 2015). doi:10.1145/2766988.

**Status: FULL TEXT READ** (research.nvidia.com PDF).

The paper is in ACM TOG (SIGGRAPH 2015), not EGSR or CGF. Loubet & Neyret 2018 cite it as "ACM Trans. Graph. 34, 4 (July 2015), 48:1–48:11". The comment in `lod.rs` says "EGSR 2015, doi:10.1111/cgf.12667", and the citation in `docs/adaptive-meshes.md` should also be checked. **That citation is wrong.**

**Microflake framework (Sec. 3):**
- Eq. 2: σ_t(ω) = ρ σ(ω).
- Eq. 3: σ_s = α ρ σ(ω).
- Eq. 4: σ(ω_i) = ∫_Ω ⟨ω_i, ω_m⟩ D(ω_m) dω_m, with D symmetric and unnormalised. ⟨·,·⟩ is the clamped dot product over the whole sphere.
- Eq. 5: f_p = (1/σ(ω_i)) ∫ p(ω_m, ω_i→ω_o) ⟨ω_i, ω_m⟩ D dω_m.
- Eq. 8, reciprocity: σ(ω_i) f_p(ω_i→ω_o) = σ(ω_o) f_p(ω_o→ω_i).

**SGGX definition (Sec. 4.2):**
- Eq. 9: S = (ω1 ω2 ω3) diag(S11, S22, S33) (ω1 ω2 ω3)ᵀ, where S_kk = σ²(ω_k), the squared projected areas along the principal axes.
- Eq. 11: σ(ω_i) = √(ω_iᵀ S ω_i).
- Eq. 12: D(ω_m) = 1 / (π √|S| (ω_mᵀ S⁻¹ ω_m)²). D is not normalised in general and its norm has no closed form.
- Surface-like lobe, Eq. 13–14: σ²(ω1) = σ²(ω2) = σ², σ²(ω3) = 1, so S = n nᵀ + σ²(I − n nᵀ).
- Fibre-like lobe, Eq. 15–16.

**Fitting from arbitrary distributions (Sec. 4.3):**
1. Take the eigenvectors of the second-moment matrix Σ = E[m mᵀ] under D (Eq. 18).
2. Take the eigenvalues as σ²(ω_k), with σ(ω_k) computed by Eq. 4 along those three eigenvectors.

The fit is "exactly the same projected area as the input data in these three directions but not necessarily in other directions". It is exact when the input already is an SGGX lobe. Footnote 1: S ≠ Σ in general, though they share eigenvectors.

So using Σ a n nᵀ to find the card's frame matches SGGX. But SGGX's eigenvalues are projected areas, not eigenvalues of Σ.

**Filtering (Sec. 4.4):** In practice they MIP-map S linearly, averaging the 6 coefficients. This is "equivalent to averaging the squared projected areas" and keeps S positive definite. It is "not optimal" and slightly smooths, as shown in Fig. 8.

**Storage (Eq. 19):** σ_x = √S_xx, σ_y, σ_z, r_xy = S_xy/√(S_xx S_yy), r_xz, r_yz, at 1 byte each, so 6 B per voxel. Eq. 20 reconstructs S. The quantised form is not linearly interpolable; interpolate S itself.

**VNDF (Sec. 5.1):**
- Eq. 21: D_ωi(ω_m) = ⟨ω_i, ω_m⟩ D(ω_m) / σ(ω_i), which is normalised (Eq. 22).
- Footnote 2: this normalisation "assumes that the visibility and the orientation of the normals are independent", the same assumption as Smith masking.
- Eq. 23: f_p = ∫ p D_ωi dω_m.
- Eq. 24: f_p^spec = D(ω_h) / (4 σ(ω_i)).
- Eq. 25: the diffuse phase function, estimated stochastically by Eq. 26: sample ω_m from D_ωi, then evaluate (1/π)⟨ω_o, ω_m⟩.

**VNDF sampling (Algorithm 1):**
1. Build a frame (ω_k, ω_j, ω_i) and project S into it to get S_kji.
2. Compute three vectors:
   - M_k = [√|S_kji| / √(S_jj S_ii − S_ji²), 0, 0]
   - M_j = (1/√S_ii) [−(S_ki S_ji − S_kj S_ii) / √(S_jj S_ii − S_ji²), √(S_jj S_ii − S_ji²), 0]
   - M_i = (1/√S_ii) [S_ki, S_ji, S_ii]
3. Draw u = √U1 cos 2πU2, v = √U1 sin 2πU2, w = √(1 − u² − v²).
4. ω_m = normalize(u M_k + v M_j + w M_i), rotated back to world space.

This sampling is exact, needs 2 random numbers, and is χ²-validated. The phase sampling has weight 1 because flakes have no shadowing term (Sec. 5.3, footnote 3).

**Results:**
- Tree and hairball at 1024³ (about 1.9–2 GB) reduce to about 15 MB at "SGGX LOD3, density LOD2" for distant views.
- Mitsuba speedups of 25% to 2–3×.
- GPU renderer: 1.15 M paths/s.

**Limitation they name:** "the most important limitation ... is the way we prefilter density". Visibility correlation along paths and correlation of visibility with material are ignored. Downsampling density "generally alters the result".

## 3. Heitz, "Understanding the Masking-Shadowing Function in Microfacet-Based BRDFs" (2014)

**Citation.** JCGT 3(2), 32–91, 2014.

**Status: FULL TEXT READ** (jcgt.org) for Sec. 3.

**Eq. 16:** D_ωo(ω_m) = G1(ω_o, ω_m) ⟨ω_o, ω_m⟩ D(ω_m) / cos θ_o. It is normalised by Eq. 14/18, and the average radiance is Eq. 17: L = ∫ L(ω_o, ω_m) D_ωo dω_m.

**Smith model:** "assuming a microsurface on which the orientation of visible normals is independent of the probability of masking".

**Fit with the code:** the card's ∝ a|n·d| is the volume (microflake) VNDF, SGGX Eq. 21. It is the JCGT definition with G1 dropped: leaves have no masking within the cluster. Citing JCGT is fine, but the exact match is SGGX Eq. 21.

## 4. Heitz & Neyret, "Representing Appearance and Pre-filtering Subpixel Data in Sparse Voxel Octrees" (2012)

**Citation.** HPG 2012, 125–134. doi:10.2312/EGGH/HPG12/125-134.

**Status: FULL TEXT READ** (via diglib).

**Per voxel:**
- The macroscopic SDF h̄ and the variance σ_h² of the microscopic height.
- The macro normal n̄ and slope variance σ_n² (anisotropic: σ_nx², σ_ny²), a Gaussian NDF.
- Per attribute: mean ā and a_s, the correlation of the attribute with depth.
- About 15–20 B per voxel. Variances are stored so they interpolate linearly.

**Correlation along the ray (Sec. 4.3, Fig. 4):** α-blending α_d = α_[zd,zd+1](1 − α_{d−1}) is wrong for correlated fragments, because it thickens silhouettes. They replace it with A-buffer-style bitmasks of 128 Poisson samples (four 32-bit ints). The bitmasks come from a 256×256 table indexed by the local plane (θ, v), with the test ω_x cos θ + ω_y sin θ ≤ v, where v = (n_z l_d − h̄) / (r_d √(n_x² + n_y²)).

**Correlation of an attribute with visibility (Sec. 5.5):**
- Smith visibility V(v, h) = g(h)^Λ(v) (Eq. 4).
- g(h) = 1 − ½ erfc(h / (√2 σ_h)) (Eq. 5).
- Λ(v) is the Smith Λ (Eq. 6).
- Attributes are modelled as a(h) = ā + a_s s(h) with s = 2g − 1 (H5bis).
- Eq. 15: ā(v, l) = ∫ a(h) V(v, h) V(l, h) P(h) dh / ∫ V V P dh.
- **Closed form, Eq. 17:** ā(v, l) = ā + a_s [2 (Λ(v) + Λ(l) + 1) / (Λ(v) + Λ(l) + 2) − 1].

**Fit:** a_s = ∫ s(h(x)) a(x) dx / ∫ s²(h(x)) dx, the projection of a onto s.

**Numbers:**
- Under 1% error on a Gaussian (Perlin) height map; about 5% at grazing angles on a real texture.
- 40–60 fps without shadows and 10–25 fps with them, at 512².
- The conclusion names foliage-like content that "behave[s] like volumes at a distance" as future work.

## 5. Loubet & Neyret, "Hybrid mesh-volume LoDs for all-scale pre-filtering of complex 3D assets" (2017)

**Citation.** CGF 36(2), 431–442 (Eurographics 2017). doi:10.1111/cgf.13124.

**Status: COULD NOT FETCH.**
- HAL hal-01468817 is behind the Anubis bot wall on both hal.science and inria.hal.science.
- The diglib item (uuid 577c02e8…) has no open bitstream.
- Loubet & Neyret 2018 only cites it as using microflakes for foliage.

**From memory (unverified):**
- A local analysis splits each asset into parts that stay a mesh (large, smooth surfaces relative to the LoD voxel) and sub-voxel parts (foliage, thin parts) that are voxelised into SGGX microflake volumes.
- Volume density comes from the projected area of the sub-voxel geometry, so the volume's mean transparency matches.
- Results include trees and foliage at several LoDs.

I cannot give the split criterion, the density formula or the numbers.

## 6. Loubet & Neyret, "A new microflake model with microscopic self-shadowing for accurate volume downsampling" (2018)

**Citation.** CGF 37(2), 111–121 (Eurographics 2018). doi:10.1111/cgf.13346.

**Status: FULL TEXT READ** (diglib, with the supplemental).

**Motivation (Fig. 2, Sec. 1):**
- Linear density prefiltering gives the wrong transparency.
- Lowering density to fix transparency also lowers local self-shadowing, which leads to "overly bright LoDs".
- This is the same symptom as the cards' 3% over-brightness.

**Model (Sec. 4.2).** A(ω) ∈ (0, 1] with A(ω) = A(−ω); 1 − A(ω) is the probability that a flake is shadowed or masked by a neighbour in direction ω.

| Eq. | Quantity | Formula |
|---|---|---|
| 4.2 | Attenuation | σ_t(ω) = A(ω) ρ ∫ D(m)⟨ω·m⟩ dm |
| 4.3 | Single scattering (unmasked and unshadowed) | σ_ss(ω) = ρ α_ss A(ω) ∫ A(ω′) D(m)⟨ω·m⟩ dm, with ω′ = 2m(m·ω) − ω |
| 4.4 | Same, over outgoing directions | (ρ α_ss A(ω)/4) ∫ A(ω′) D(ω_h) dω′ |
| 4.5 | Single-scattering phase | f_ss = ρ α_ss A(ω) A(ω′) D(ω_h) / (4 σ_ss(ω)), reciprocal |
| 4.6 | Local multiple scattering | σ_ms(ω) = α_ms ρ A(ω) ∫ (1 − A(ω′)) D⟨ω·m⟩ dm = α_ms (σ_t − σ_ss/α_ss) |
| 4.7 | Multiple-scattering phase, roughly diffuse | f_ms(ω′) = σ_ms(ω′) / ∫ σ_ms |

The model passes the white furnace test.

**Simplified model, isotropic A (Sec. 4.3):**
- Eq. 4.8: σ_t = A ρ ∫ D⟨·⟩.
- Eq. 4.9: σ_ss = α_ss A σ_t.
- Eq. 4.10: f_ss is the standard specular microflake phase.
- Eq. 4.11: σ_ms = α_ms σ_t (1 − A).
- Eq. 4.12: f_ms ∝ σ_t(ω′).
- Implemented with SGGX (Sec. 6), using a precomputed ∫ σ_t over pairs of normalised eigenvalues.

**Anisotropic A, Eq. 5.1:** A(ω) = ωᵀ S_A ω, an SGGX-like matrix with eigenvalues ≤ 1, stored as 6 parameters. Closed forms exist with D written as cos²ⁿ and sin²ⁿ lobes (Eq. 5.2–5.4). With SGGX D there is no closed form for σ_ss.

**Fitting (Sec. 7), per low-res voxel, independently:**
1. ρ′ comes from the measured directional transparencies T(ω_X, ω_Y, ω_Z) of the input block.
   - Iso: the mean of −log T_i / (L ∫ D⟨ω_i·m⟩), Eq. 7.6.
   - Aniso: the max of the same (Eq. 7.7), then s_i = −log T_i / (ρ′ L ∫ D⟨ω_i·m⟩) (Eq. 7.8–7.9).
2. Measure the single-scattering escape energy M_ss in the input block and in the low-res voxel.
   - Per ray, Eq. 7.10: P_ss(r) = ∫_{x_min}^{x_max} ∫_{S²} σ_s(x, ω_r) O(x_min, ω_r, x) f(x, ω_r→ω) O(x, ω) dω dx, where O(x1, ω, x2) = exp(−∫ σ_t ds) (Eq. 7.11) and O(x, ω) is the transmittance from x to the block's boundary.
   - Average over N rays along the grid axes (Eq. 7.12).
3. Rescale with ρ = ρ′/γ and A = γ A′. This leaves σ_t unchanged and multiplies σ_ss by γ.
   - γ = arg_g Σ_λ (M_ss^in(λ) − g M_ss(λ))², Eq. 7.13.
   - The paper prints "arg max". Since it is a squared error, arg min is meant, giving g = Σ M^in M / Σ M².
4. α_ms solves α_e^in = Σ_n P_sh(n) (P_ss α_ss + (1 − P_ss) α_ms)ⁿ (Eq. 7.14 and 7.16).
   - P_ss ≈ ∫σ_ss / (α_ss ∫σ_t) (Eq. 7.15).
   - P_sh(n), the probability of exactly n scatterings before leaving, comes from ray casting in the low-res voxel.
   - It slightly underestimates α_e.

**Cost:**
- 40 × 4ⁱ rays per non-empty voxel at LoD i.
- 0.1–6.8 CPU core-hours, against 12–40 h for Zhao et al. 2016.
- Values per voxel: Linear 10, Iso (SGGX) 14, Aniso 21. LoD3 is 0.20%, 0.27% and 0.41% of the input respectively.
- Render time about 13% of full resolution (naïve LoDs about 11%).

**Findings:**
- Anisotropic A "does not improve accuracy significantly in practice". The scalar Iso model is as good on every dataset they tested, including cedar and pine foliage.
- Accuracy is limited by albedos and phase functions.

**Limitations:**
- A is symmetric, but real self-shadowing in dense voxels is strongly forward and weakly backward.
- f_ms is too diffuse, because multiple-scattered light leaves dense voxels backwards.
- It is inaccurate for very dense inputs such as hair.

## 7. Vicini, Jakob, Kaplanyan, "A Non-Exponential Transmittance Model for Volumetric Scene Representations" (2021)

**Citation.** ACM TOG 40(4) (SIGGRAPH 2021). doi:10.1145/3450626.3459815.

**Status: COULD NOT FETCH.** rgl.epfl.ch links to d38rqfq1h7iukm.cloudfront.net, which returns proxy 403. GitHub (dvicini/non-exp-transmittance-torch) is outside this session's allowed repositories.

**Second-hand, from Speierer & Jarabo, "Generalized non-exponential Gaussian splatting", arXiv 2603.02887 (Table 1, read):** the "Blended [Vicini et al. 2021a]" transmittance is T(τ) = lerp([1 − τ]₊, exp(−τ), γ), with γ ∈ [0, 1]. That is, a mix of linear (negatively correlated, surface-like) and exponential transmittance in optical depth τ.

I could not check the exact parameterisation in the original, for example any rescaling of τ in the linear term.

## 8. Jarabo, Aliaga, Gutierrez, "A Radiative Transfer Framework for Spatially-Correlated Materials" (2018)

**Citation.** ACM TOG 37(4), Article 78 (SIGGRAPH 2018). doi:10.1145/3197517.3201282. arXiv 1805.02651.

**Status: FULL TEXT READ** (arXiv, 24 pp including the supplement).

**Basic relations:**
- Eq. 3: Σ(t) = p(t) / (1 − ∫₀ᵗ p) = p(t)/T(t).
- Eq. 4: binary mixture example, p(t) = μ₁ p_τ(μ₁) e^(−μ₁t) + μ₂ p_τ(μ₂) e^(−μ₂t).
- Extended GBE (Eq. 8–12): separate Σ_S (scattered light) and Σ_Qj (each source), with T_S = exp(−∫Σ_S) and T_Qj = exp(−∫Σ_Qj).
- Boundary conditions: light entering, surfaces and media interfaces act as uncorrelated sources reset to t = 0.

**Positive correlation (Sec. 5.2):**
- Eq. 15: general beam form.
- Eq. 16: T(t) = ∫ e^(−μt) p_τ(μ) dμ.
- Eq. 17: Σ(t) = ∫ μ e^(−μt) p_τ dμ / ∫ e^(−μt) p_τ dμ.
- Eq. 18: μ p_τ(μ) = C p_C(C) σ̄.
- Eq. 19: a gamma concentration p_C(C) = β^α C^(α−1) e^(−Cβ) / Γ(α), with α = C̄²/Var(C) and β = C̄/Var(C).

**Closed forms (T(t) = M(−σt), the moment-generating function):**
- Eq. 20: **T(t) = (1 + σt/β)^(−α)**.
- Eq. 21: p(t) = (ασ/β)(1 + σt/β)^(−(1+α)).
- Eq. 22: Σ(t) = (ασ/β)(1 + σt/β)^(−1).
- Directional dependence: p_C(C; ω), σ(ω) (supplement S.2).
- Negative correlation: shown with linear transmittance.
- Fitting: C̄ and Var(C) from voxelised high-resolution volumes (Fig. 11).

**Correlated versus uncorrelated origin:** the paper treats this through different Σ per source. In its local model it does **not** state the derivative relation; that is d'Eon's (paper 9).

## 9. Bitterli, Ravichandran, Müller, Wrenninge, Novák, Marschner, Jarosz, "A radiative transfer framework for non-exponential media" (2018)

**Citation.** ACM TOG 37(6), 225 (SIGGRAPH Asia 2018). doi:10.1145/3272127.3275103.

**Status: COULD NOT FETCH.** cs.dartmouth.edu and the authors' pages cannot be reached, and it is not on arXiv.

**The relation, verified instead in d'Eon, "A reciprocal formulation of non-exponential radiative transfer. 1: Sketch and motivation", arXiv 1803.03259v6 (FULL TEXT READ, Sec. 2.3).** Bitterli et al. adopt the same reciprocal pairing (Speierer & Jarabo cite the two together for it).

- Eq. 7: p_u(s) = X_c(s) / ∫₀^∞ X_c. The uncorrelated free-path pdf is the normalised correlated transmittance.
- Eq. 8: ∫ X_c = ⟨s⟩_c, the mean correlated free path.
- Eq. 11: X_u(s) = 1 − ∫₀ˢ p_u.
- Eq. 12: p_c(s) = ⟨s⟩_c X_u''(s).

Therefore **T_c(s) = −⟨s⟩_c T_u′(s) = T_u′(s)/T_u′(0)**, with μ̄ := 1/⟨s⟩_c = −T_u′(0). So "T_c = −T_u′/μ̄" is **correct**, with μ̄ the inverse mean free path between correlated collisions.

Example with Jarabo's gamma law as T_u: T_c(t) = (1 + σt/β)^(−(α+1)). Paths that start on a leaf are attenuated more than paths that enter from outside. This is the formal statement of foliage self-shadowing.

## 10. Miller, Chen, Lai, Gkioulekas, "Objects as Volumes: A Stochastic Geometry View of Opaque Solids" (2024)

**Citation.** CVPR 2024. arXiv 2312.15406v2.

**Status: FULL TEXT READ** (Sec. 3).

**Theorem 4.** In a stochastic opaque solid with vacancy v(x) = P{I(x) = 0}, the free flight along a ray is exponential **if and only if** the indicator restricted to the ray is a continuous-time, discrete-space Markov process (Eq. 11). It is reversible and reciprocal iff:

- Eq. 12: **σ(x, ω) = |ω·∇v(x)| / v(x) = |ω·∇ log v(x)|**.
- Eq. 13: this equals σ∥ σ⊥, with density σ∥ = ‖∇v‖/v and projected area |ω·n|, where n = ∇v/‖∇v‖.

**Generalised forms:**
- Eq. 14: σ⊥_D(x, ω) = ∫ |ω·m| D_x(m) dm, with D normalised (following Jakob, not SGGX).
- Eq. 16: σ = σ∥ σ⊥_D.
- Eq. 17: σ⊥_mix = α|ω·n| + (1 − α)/2.
- Eq. 26, implicit surfaces: v = Ψ(s f), and σ = s ψ(s f) ‖∇f‖ / Ψ(s f) · σ⊥_D.

**Relevance:** a random arrangement of leaves is generally not Markov along a ray, since a leaf is a thin opaque plate. So exponential transmittance through a clump of cards is an approximation, and the paper names when it holds.

## 11. Lacewell, Burley, Boulos, Shirley, "Raytracing Prefiltered Occlusion for Aggregate Geometry" (2008)

**Citation.** IEEE Symposium on Interactive Ray Tracing (RT08), 2008. doi:10.1109/RT.2008.4634627.

**Status: COULD NOT FETCH.** graphics.stanford.edu and cs.utah.edu cannot be reached, and it is not on arXiv. Prus et al. 2015 restate it in one line: they "prefilter aggregate geometry from different view points resulting in less variance in soft shadows".

**From memory (unverified):** directional opacity is precomputed per BVH node of aggregate geometry (projected onto spherical harmonics or a direction table) and used by shadow rays only. Combining opacities between nodes assumes they are uncorrelated.

## 12. Meng, Papas, Habel, Dachsbacher, Marschner, Gross, Jarosz, "Multi-scale modeling and rendering of granular materials" (2015)

**Citation.** ACM TOG 34(4), 49 (SIGGRAPH 2015).

**Status: COULD NOT FETCH** (Dartmouth and Disney hosts cannot be reached).

One line: explicit grains near the camera; volumetric path tracing and diffusion at larger scales, with parameters fitted from the grains. Jarabo et al. (read) note it validates its uncorrelated-RTE approximation against simulated p(t) (their Fig. 6).

## 13. Bonus (found and read): Prus, Eisenacher, Stamminger, "Level-of-Detail for Production-Scale Path Tracing" (2015)

**Citation.** VMV 2015. doi:10.2312/vmv.20151260. **FULL TEXT READ** (diglib).

This is prior art for the card's per-hit choice of normal:
- An SVO with averaged Disney-BRDF parameters and one normal distribution per voxel: vMF (2 B at half precision), ellipse or Gauss.
- "We use the normal distribution to randomly draw a normal during shading", rather than convolving it into the BRDF.
- Slabs bound the geometry.

---

## Synthesis

### (a) Replacing a card's 32 constituents with a lobe or a mixture

**What the code does now.** P(k | d) = a_k|n_k·d| / Σ_j a_j|n_j·d|. This is the microflake VNDF (SGGX Eq. 21; JCGT Eq. 16 with G1 = 1) for the discrete NDF D = Σ_k (a_k/2)(δ_{n_k} + δ_{−n_k}).

**Geometry.** Its projected-area function σ_Z(d) = ½ Σ_k a_k|n_k·d| is the **support function of the zonotope** Z = Σ_k [−½a_k n_k, ½a_k n_k]. SGGX's σ(d) = √(dᵀSd) is the support function of the ellipsoid {x : xᵀS⁻¹x ≤ 1}. More generally, any symmetric nonnegative D gives a support function of a zonoid: the projection body of the flakes, in Minkowski and Cauchy's sense. Ellipsoids are zonoids, but a zonotope with a few dominant generators, such as a cluster of a few flat leaves, is far from any ellipsoid.

**What the papers do and don't do:**
- **No paper I read fits σ in L1, or in any norm over directions.**
- SGGX (Sec. 4.3) **interpolates** σ exactly at the three eigenvectors of the second moment Σ = Σ a n nᵀ. It warns S ≠ Σ.
- SGGX's MIP filtering averages σ² linearly (in S).
- Loubet & Neyret 2018 just reuse linear SGGX prefiltering.
- So moment matching is all the literature offers. My own observation, not from the papers: L1 or L2 fitting of the support function is open.

**Options that preserve P(normal | d) ∝ a|n·d|:**

1. **A single SGGX lobe, 6 B.** Build S from the eigenvectors of Σ a n nᵀ and eigenvalues σ_Z(ω_k)², where σ_Z(ω_k) is the zonotope evaluated along each eigenvector. Sample normals with SGGX Algorithm 1.
   - The hit then follows the SGGX VNDF, not the leaves'. It matches only along the three axes, and for flat clusters D has GGX's long tail.
   - Per-leaf identity is lost: material, autumn colour, front or back face. A card would need one averaged albedo. That drops the correlation between colour and orientation, which Heitz & Neyret 2012 show matters at grazing angles.
2. **A small mixture.** Keep m ≪ 32 representative normals, for example k-medoids on ±n weighted by a, and re-solve their areas b_j ≥ 0 by nonnegative least squares so that ½Σ_j b_j|n_j·d| ≈ σ_Z(d) over sampled d. The fit is linear in b.
   - This is again a zonotope, so sampling and PDFs stay exact for the reduced set. The constituent identity and material of each medoid are kept, and the error is the support-function residual.
   - An SGGX mixture (P(lobe | d) ∝ w_l σ_l(d), then Algorithm 1) is the smooth alternative.
3. **Whatever is chosen, the VNDF normalisation assumes orientation is independent of visibility** (SGGX footnote 2, JCGT Smith). That is exactly what fails inside a clump: top leaves both mask and light more. Fixing this is part (b), not a better lobe.

### (b) Self-shadowing for the ~3% over-brightness

The literature supports this diagnosis directly. Loubet & Neyret 2018 (Fig. 2c, Sec. 1): keeping transparency while losing local self-shadowing "leads to overly bright LoDs". Paths from a chosen constituent leave from the card plane and never meet the cluster's other leaves, so within-cluster shadowing is lost.

Formulas, in order of practicality:

1. **Loubet & Neyret's simplified model: one scalar A per card, plus α_ms.**
   - At a card hit, single scattering toward ω′ (including next-event estimation to the sun) is weighted by A, or by A(ω′) for the anisotropic form.
   - The rest, (1 − A), goes to a diffuse local multiple-scattering lobe with albedo α_ms, so energy is conserved (Eq. 4.9–4.12).
   - Their paper found the scalar version as good as the anisotropic one on foliage.
   - **Build-time measurement:** cast rays at the original leaves of the cluster (they used axis-aligned rays; for leaves use the sun-facing and view hemispheres).
     - Per ray, the energy leaving after exactly one bounce: Eq. 7.10 with surface BSDFs in place of phase functions. Compute this on the leaves, M_ss^in, and on the card with A = 1, M_ss.
     - Take γ = Σ_λ M^in M / Σ_λ M² (Eq. 7.13 read as a least-squares minimum), so A = γ.
     - Fit α_ms from the effective albedo, Eq. 7.14–7.16.
     - Cost: 1 B for A plus 3 B for α_ms per card.
2. **An anisotropic A(ω) = ωᵀ S_A ω** (6 B, Eq. 5.1). Fit it by casting, from each leaf of the cluster, rays toward sampled directions and recording the unoccluded fraction by direction. Fit S_A to that, for example by least squares in A(ω), eigenvalues clamped to ≤ 1. Multiply a card's sun shadow ray by A(l). Their caveat: true self-shadowing is forward-peaked and A is symmetric.
3. **Visibility correlated with depth (Heitz & Neyret 2012, Eq. 17).** Store per constituent its depth fraction g ∈ [0, 1] in the cluster along the card normal (1 B), and weight its contribution by g^(Λ(v)+Λ(l)).
   - In aggregate this is the Smith height-correlated factor: the mean visible-and-lit fraction is ∫₀¹ g^(Λv+Λl) dg = 1/(1 + Λ(v) + Λ(l)).
   - Λ comes from the cluster's "slope" statistics, or is fitted directly from the measured one-direction visible fractions: if the fraction visible from v is 1/(1 + Λ(v)), then Λ(v) = 1/frac − 1.
   - This also keeps the correlation between depth and colour, such as inner and outer leaves.
4. **Correlated-origin transmittance between cards (d'Eon 2018 / Bitterli 2018).**
   - If a crown's uncorrelated transmittance is fitted as T_u (Jarabo's gamma law (1 + σt/β)^(−α), or the measured clumped canopy gap fraction), then a shadow ray from a leaf should use T_c = T_u′/T_u′(0). For the gamma law that is (1 + σt/β)^(−(α+1)).
   - α and β come from the mean and variance of the leaf area density per unit length along rays, measured at build time (Jarabo Eq. 19).
   - This needs a volumetric crown representation, so it fits the coarsest levels (Loubet & Neyret 2017 style) better than surface cards.
   - Miller et al.'s Theorem 4 explains why exponential T is only approximate for plates.

**Recommended build-time measurement for the cards.** For each cluster, trace from points on its leaves toward a set of directions (sun hemisphere), against the cluster's own leaves only:
- the unoccluded fraction F(ω) gives A(ω), or the scalar A = mean F, or Λ(ω) = 1/F − 1;
- the single-bounce escape energy gives γ (Eq. 7.10–7.13).

Then validate by comparing the full-detail forest and the card forest in linear light. The target is the 3% (1–2% far, 3–4% near) gap, which should scale with each cluster's leaf-area density.

### Corrections to the earlier notes and code comments

1. The SGGX citation is wrong. It is ACM TOG 34(4), 48:1–48:11, SIGGRAPH 2015, doi:10.1145/2766988, not "EGSR 2015, doi:10.1111/cgf.12667".
2. SGGX's eigenvalues are squared projected areas, σ²(ω_k) = (Σ ½a|n·ω_k|)² along Σ's eigenvectors. They are not the eigenvalues of Σ a n nᵀ (Footnote 1: S ≠ Σ). Only the frame comes from Σ.
3. The ∝ a|n·d| VNDF matches SGGX Eq. 21 exactly. JCGT Eq. 16 matches with G1 = 1.
4. T_c = −T_u′/μ̄ is verified, with μ̄ = −T_u′(0) = 1/⟨s⟩_c (d'Eon 2018, Eq. 7–12).
5. Loubet & Neyret 2018 Eq. 7.13 prints "arg max" but means the least-squares minimum.
6. Unverified items:
   - Cook 2007's contrast formula, Loubet & Neyret 2017's split and density rules, and Lacewell 2008: I could not fetch any of them.
   - Vicini 2021's T is known only second-hand, as lerp([1 − τ]₊, e^(−τ), γ).
