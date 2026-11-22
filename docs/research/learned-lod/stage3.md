# Stage 3: matching both transmittances

Stage 2 found that a cluster's stand-in has to get two different things right:

- **Rays entering it** must hit it as often as they would hit the leaves. This is the
  uncorrelated-origin transmittance `T_u`.
- **Rays leaving it** must be shadowed by it as often as by the leaves. This is the
  correlated-origin transmittance `T_c` (Bitterli et al. 2018; d'Eon 2018).

A single card gets `T_c` wrong one way: it can't shadow itself. Crossed cards get it wrong the
other way: they shadow each other far more than leaves do. This stage gets both right.

## The scheme (experimental, on the research branch)

- **For `T_u`: crossed cards.** Each cluster gets up to three cards across its box, sized by NNLS
  to its leaves' rasterised coverage over directions, and refitted against the cards' own union.
  A cluster gets extra cards only if one card misses its coverage by more than 15%.
- **For `T_c`: rays leaving don't meet the cards.** A ray starting within a cluster's bounds
  doesn't test that cluster's cards. Instead, a shadow ray is blocked by its own cluster with
  probability `1 − A`. The draw hashes the ray and the cluster, as constituents are chosen now.
- **Measuring `A`.** `A` is the fraction of light a leaf gets past its cluster's other leaves. It
  is measured at build time by casting 24 rays from points on the leaves, drawn by area, toward
  directions over the upper hemisphere (`self_lit`). It is stored as a byte per cluster, with the
  cluster's bounds.
- **`A` can't be estimated from coverage.** An estimate from the box's coverage seen from outside,
  half the box's depth by Beer-Lambert, gave 0.85–0.95 against the measured 0.19–0.95. Leaves
  clump on their twigs, so a leaf's neighbourhood is far denser than its cluster's box.
  Measured `A` per level, averaged over the six species:

| Level | 2 | 3 | 4 | 5 | 6 | 7 |
|---|---|---|---|---|---|---|
| `A` | 0.84 | 0.78 | 0.67 | 0.55 | 0.45 | 0.34 |

## Results

`forest-aerial` from the air at 600 × 350 and 64 spp, and from 90 m up at 32 spp. Each is
compared with full detail in linear light.

| | From the air: FLIP | Brightness, near / far / all | From low down: FLIP | Brightness at mid range |
|---|---|---|---|---|
| Old cards | 0.0585 | +7.4% / +2.2% / +4.6% | 0.0313 | +7% to +8% |
| #171 + #172 (one card, sized) | 0.0543 | −2.5% / −0.7% / −1.2% | 0.0296 | −3% to −6% |
| Crossed cards alone | 0.1001 | −19% / −9% / −13.5% | 0.0497 | −15% to −23% |
| **Crossed cards, `T_c` by measured `A`** | **0.0454** | **−1.1% / +2.2% / +0.3%** | **0.0251** | **+1% to +5%** |

Matching both transmittances gives the lowest error from both viewpoints: FLIP down 16% from the
air and 15% from low down, against #172.

**What it costs** against #172, from the air:
- peak memory 618 → 866 MiB;
- build 200 → 387 s;
- render 93 → 145 s.

The causes:
- 2.3–3 card planes per cluster instead of one, so about 2.6× the cards and their BVH;
- per-cluster bounds (24 B);
- the build-time ray casting;
- the cards' union fitting.

## What it would take to ship

- **A compact crossed-card primitive.** One BVH entry holding a cluster's frame and its three
  plane sizes: about 56 bytes against 3 × 40, plus one leaf in the BVH. Its bounds would serve
  as the cluster's, so no separate 24-byte box would be needed.
- **A cheaper build.** For example, measure `A` for one cluster in several (it varies smoothly
  with level and species), and rasterise fewer directions.
- **Direction-dependent `A`.** A scalar was enough here, as Loubet & Neyret found for their
  microflake self-shadowing. The hotspot (light and view aligned) would need the correlation
  between the two directions.

## For a paper

The contribution would be a level of detail for aggregates that matches, and measures, three
things separately:

1. **The hit probability from outside (`T_u`).** Measured coverage, rasterised, replaces a
   random-overlap estimate. Crossed cards fitted as a zonotope cover views from the side.
2. **The constituent distribution of a hit.** Original leaves are kept, merged exactly, with
   representatives refitted to the support function of their zonotope. Error is 1–2%, against
   6–10% for an even sample of 32.
3. **The transmittance of rays leaving a cluster (`T_c`).** It is measured, because clumping
   makes it unpredictable from the cluster's coverage, and applied stochastically in place of
   the cluster's own geometry.

The measurements separate the three errors, and the renders show that each one is needed:
- fixing `T_u` alone moved the error from too bright to slightly dark;
- fixing `T_u` from the side without `T_c` darkened the image 13–23%;
- with both, the image is within a few percent of full detail from above and from the side.
