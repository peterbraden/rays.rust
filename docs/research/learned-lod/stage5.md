# Stage 5: transmittance in a crown, measured

Stage 4 concluded that a stand-in's transmittance must depend on how far a ray is from where
it started. Before building a volume for the coarsest levels, this stage measures that
dependence on real tree geometry. It tests whether non-exponential transport theory could
predict it from a cluster's statistics.

## The theory to test

For a medium whose scatterers are spatially correlated (clumped), there are two
transmittances:
- `T_u(s)`, for a path that starts uncorrelated with the scatterers;
- `T_c(s)`, for a path that starts on a scatterer.

They are related reciprocally (d'Eon 2018, Eq. 7–12; Bitterli et al. 2018):

    T_c(s) = T_u'(s) / T_u'(0).

With a gamma-distributed density (Jarabo et al. 2018, Eq. 20),
`T_u = (1 + s/λ)^-α` gives `T_c = (1 + s/λ)^-(α+1)`.

If foliage followed this, a crown could be a volume of three parameters per cluster, and its
self-shadowing would follow from how it looks from outside.

## Measurement

`lod_transmittance` in `src/shapes/lod_research.rs`, run by hand:

    cargo test --release --lib lod_transmittance -- --ignored --nocapture

For one tree of each of six species, it casts 20,000 rays per kind against all the leaves:
- **Entering the crown's box** from outside.
- **From points uniform in the box, every way.** This is the theory's uncorrelated origin.
- **From points on the leaves, drawn by area, every way** (`T_c`). It is measured again
  passing through the ray's own leaf.

Free paths ending at the box are censored, and the survival curves come from Kaplan-Meier.
Gamma laws are fitted by least squares on `T`. Raw values are in
[stage5-transmittance.csv](stage5-transmittance.csv).

| Species | Crown | `T_u` within: λ, α, fit RMS | `T_c` near / at 2 m | `T_c` predicted from `T_u`: RMS error | Gamma fitted to `T_c`: λ, α, RMS |
|---|---|---|---|---|---|
| Quaking aspen | 13 m | 4.1 m, 1.01, 0.008 | 0.69 at 0.11 m / 0.22 | 0.20 | 0.056 m, 0.40, 0.010 |
| California black oak | 32 m | 158 m, 2.36, 0.002 | 0.72 at 0.26 m / 0.61 | 0.30 | 0.007 m, 0.09, 0.003 |
| Weeping willow | 20 m | 8.3 m, 0.88, 0.008 | 0.40 at 0.17 m / 0.23 | 0.31 | 0.008 m, 0.27, 0.008 |
| Scots pine | 28 m | 43 m, 1.80, 0.007 | 0.42 at 0.23 m / 0.32 | 0.42 | 0.003 m, 0.18, 0.008 |
| Norway spruce | 28 m | 11 m, 0.79, 0.008 | 0.42 at 0.23 m / 0.26 | 0.30 | 0.011 m, 0.26, 0.008 |
| Hazel | 7 m | 4.3 m, 1.02, 0.006 | 0.81 at 0.06 m / 0.29 | 0.24 | 0.024 m, 0.27, 0.018 |

## Findings

1. **The reciprocal relation fails for foliage.**
   - From uncorrelated points within the crown, `T_u` is fitted well by a gamma law (RMS error
     at most 0.008).
   - But the `T_c` predicted from it is wrong by 0.20–0.42.
   - Predicting from rays entering the box does worse, since that `T_u` is S-shaped: rays cross
     the box's empty margins before the foliage. Exponential (Beer-Lambert) transport predicts
     `T_c = T_u`, also 0.31–0.52 off.
   - The theory assumes a statistically uniform medium. A crown isn't one. Its clumping is
     hierarchical (leaves on twigs, twigs on branches, branches in a crown), as canopy
     remote sensing describes (Chen & Cihlar 1995).
2. **`T_c` is heavy-tailed and drops at short range.**
   - 19–60% of rays leaving a leaf are blocked within a few tenths of a metre, by the leaf's
     own twig.
   - After that `T_c` falls slowly: 22–61% of rays still get through at 2 m.
   - **Conifers' tufts shadow themselves.** Passing through the ray's own leaf (needle tuft)
     raises pine's and spruce's `T_c` by about 0.1. Broadleaf leaves don't shadow themselves:
     the result is unchanged.
3. **A gamma law fitted to `T_c` directly fits it well** (RMS error 0.003–0.018). The fits have a
   very short length (3–56 mm) and a small exponent (0.09–0.40), which is what a heavy tail
   looks like.

## What it means for a volume at coarse levels

- **Two independent laws.** A non-exponential crown volume can't derive its self-shadowing from
  its appearance from outside, but two gamma laws describe both curves to within about 0.01:
  - `(λ_u, α_u)` for paths entering or starting uncorrelated;
  - `(λ_c, α_c)` for paths leaving a leaf.
- **Reciprocity is lost.** Pairing them independently breaks the reciprocity the theory keeps:
  light would not travel the same both ways through the crown. Vicini et al. 2021 accept that
  for scene representations. Whether it shows in renders is a question for the next
  experiment.
- **This agrees with stages 3 and 4,** which found `A` (self-shadowing) unpredictable from
  coverage, and correlation reaching across a crown. Here that is quantified as a function of
  distance.

For a paper, this is a result about foliage as a participating medium in its own right:
- measured `T_u` and `T_c` for six species;
- the reciprocal non-exponential relation failing;
- two-parameter gamma laws fitting each curve separately.

## Next

The next step is a crown volume for the coarsest levels, rendered with the two measured laws.
It would draw free flights by inverting each `T`, which is closed form for a gamma law. It
needs:
- a medium whose free-flight sampling depends on whether the path starts on a leaf. The
  renderer's media assume exponential transport (delta tracking), so this is new code;
- the crown's appearance: SGGX normals or the constituents, as cards have now;
- comparison against full detail and the shipped cards, from above and from the side, as
  before.
