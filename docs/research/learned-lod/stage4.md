# Stage 4: a compact crossed card, and how far rays leaving a cluster reach

Stage 3 matched both transmittances with three separate cards per cluster, at 40% more memory.
This stage packs a cluster's crossed cards into one 56-byte primitive. The primitive holds the
cluster's frame and box, up to three rectangle scales, which rectangles it has, and the
cluster's measured self-shadowing `A`. It sits behind one BVH entry, and the box it describes
is the region in which a ray counts as leaving the cluster.

## Results

`forest-aerial` from the air (600 × 350, 64 spp) and from 90 m up (32 spp), against full
detail, in linear light.

| | Peak | Build | Render (air) | FLIP air | Brightness air | FLIP low |
|---|---|---|---|---|---|---|
| #172 (one card) | 618 MiB | 200 s | 93 s | 0.0543 | −1.2% | 0.0296 |
| Stage 3: three separate cards, leaving = their axis-aligned box | 866 MiB | 387 s | 145 s | **0.0454** | +0.3% | **0.0251** |
| Compact, leaving = its oriented box | 664 MiB | 324 s | 175 s | 0.0548 | −3.4% | 0.0269 |
| Compact, side cards full length, oriented box | 661 MiB | 365 s | 208 s | 0.0632 | −5.1% | 0.0293 |
| The same, leaving = axis-aligned box | 666 MiB | 343 s | 230 s | 0.0531 | −3.0% | — |

- **Memory.** The compact primitive brings memory back near #172's.
- **Quality** depends on the region in which a ray counts as leaving a cluster, and so meets its
  self-shadowing `A` instead of the cluster's cards.
- **Why stage 3 looked best.** Its side cards were scaled up without a cap. That made its
  axis-aligned boxes large, so rays from much of a crown skipped nearby clusters' crossed cards.
- **The isolating test.** On the same compact cards, widening the leaving region from the
  oriented box to the axis-aligned one improved FLIP from 0.0632 to 0.0531 and brightness from
  −5% to −3%.
- **Speed.** Every crossed-card variant renders slower than #172, at 145–230 s against 93 s. A
  primitive with three rectangles and a larger box costs more per ray.

## What this says

**Crossed cards are fitted to rays coming from far away.** They are sized so that the hit
probability from outside, `T_u`, matches the leaves'.

**A ray that starts in a crown meets them too densely**, and not only in its own cluster:
- Leaves clump on twigs and twigs on branches. Correlation reaches across a crown, not just a
  cluster, as Chen and Cihlar's hierarchical clumping (shoot, crown, stand) describes for
  canopies.
- So the transmittance a ray sees depends on how far its origin is from the cluster it crosses.
  Two `T`s, one for rays entering and one for rays leaving their own cluster, aren't enough.
  `T` has to depend on the distance from the origin, as non-exponential transport describes:
  in Bitterli et al. 2018 and d'Eon 2018, the free-flight distribution after a scattering
  differs from the one for a fresh ray at every distance, not just at the start.
- **The leaving region is a crude stand-in for that distance dependence.** Widening it helps
  because it approximates the shorter-range correlation correctly. Where to put its edge has no
  principled answer.

## Where this leaves the work

**Shippable now:** #171 and #172, the coverage fix and the constituents.
- They improve brightness and FLIP from above and from the side.
- Memory and render time go down.
- The single card's limits remain: it is weak from the side, and it can't shadow itself.

**Research result:** the three separate effects measured in stages 0–3, and this stage's
finding that a cluster stand-in's transmittance has to depend on where a ray starts.
- Card schemes can't express that, except through a leaving region. Their best result here
  (FLIP 0.0454 from the air, 0.0251 from low down) depended on a heuristic.

**The principled route** is the third option from stage 2: at coarse levels, replace cards by a
volume whose free-flight distribution depends on distance from the origin.
- The non-exponential transport of Jarabo et al. 2018 (a gamma law) and Bitterli et al. 2018,
  with the reciprocal pairing of `T_u` and `T_c`.
- Its parameters measured per cluster at build time, as `A` is here, by casting rays from the
  leaves at several ranges.
- That would make the distance dependence explicit instead of a box edge. It is the natural
  next experiment, and it would be the paper's core if it works.
