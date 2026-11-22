# Stage 1: fewer, better constituents

Stage 0 found where constituent memory sits and how inaccurate the 32-leaf cap was. This stage
replaces the cap with a fitted set of representatives, on this branch after the card-coverage
fix. It follows the proposal's stages 1 and 4: reduced zonotopes, refitted, with a fixed
budget.

## What a card keeps now

1. **Exact merging.**
   - Pieces of one face with one normal, such as the triangles of a flat leaf, become one
     constituent. A hit chooses by `a|n·d|`, which adds, so no choice changes.
   - The broadleaf species' leaves are 4 triangles with about 2 distinct normals, so this halves
     their lists at the fine levels, where the memory is.
   - Conifer needle tufts (5 triangles of 5 normals) don't merge.
2. **Representatives.** A cluster still of more than 16 is stood for by 16:
   - **Grouping:** axial k-means on the normals, from a farthest-point start.
   - **Normals:** each representative takes its group's own normal nearest the group axis.
   - **Faces:** the face of a member drawn by area, so leaf states stay in proportion across
     cards.
   - **Areas:** refitted by non-negative least squares to the cluster's projected area over 32
     directions. That projected area is the support function of the leaves' zonotope, and the
     normaliser of the visible-normal distribution.

## Results

Measured with the stage-0 harness (`stage1-clusters.csv`; the `new` columns). Error is the mean
absolute error of the chosen leaf's expected response, against the full cluster:

| Level | Kept per card, old → new | Error, old → new |
|---|---|---|
| 2 | 2.4–11.4 → 2.4–10.0 | 0–1.0% → 0–0.7% |
| 3 | 10–26 → 4–14 | 0.2–6.0% → 0.0–1.8% |
| 4 | 20–30 → 8–16 | 2.7–8.7% → 0.2–1.9% |
| 5–7 | 27–32 → 13–16 | 4.9–9.8% → 0.8–2.1% |

Mean bias stays under 0.5%. On `forest-aerial` (600 × 350, 64 spp), with the coverage fix's cards:

- peak memory 700 → 618 MiB;
- render 108 → 93 s, faster because a hit chooses among fewer constituents;
- mean FLIP against full detail 0.0545 → 0.0543;
- brightness unchanged at −1.2%.

## What's left for memory

Constituents are now at most 16 × 12 bytes per card, and the fine levels' lists are about half
what they were. The next steps, in order of what they'd save:

- **Smaller constituents.** The 32-bit face could be a 16-bit index into a per-tree palette of
  leaf states, giving 8 bytes per constituent.
- **SGGX lobes for conifers.** At L4 and coarser their clusters are fitted by one SGGX lobe to
  0.1–1.5% error, in 6 bytes against 16 × 12. But a lobe keeps no faces, so leaf states would
  need a palette of classes.
- **A learned decoder** (stage 5), only if the above leaves memory worth the cost.
