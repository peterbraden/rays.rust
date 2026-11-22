# Stage 6: coarse clusters as volumes with measured transmittance

Stage 5 measured how far free paths run in a crown. A prototype then put that to use: clusters
at the coarsest levels as volumes, in place of cards, with each volume's transmittance
measured on its own leaves.

## The prototype (`src/shapes/lod_volume.rs`, off unless `LOD_VOLUME_TRIANGLES` is set)

- **Which clusters.** Those of at least `LOD_VOLUME_TRIANGLES` triangles become volumes. With
  3,000 that is about level 6 and coarser; with 600, level 5; with 100, level 4.
- **Shape.** A volume is the cluster's oriented box (from the plane its leaves cover most of and
  their spread). It sits behind a BVH of its own, beside the cards.
- **Two tables.** Each volume stores the survival of free paths at 16 distances over its
  diagonal, as bytes, measured at build time by Kaplan-Meier over 256 rays:
  - for rays entering the box from outside;
  - for rays leaving the points those rays first meet.
- **Rendering.** A ray from outside has its free path drawn from the first table, starting where
  it enters the box; a ray from within draws from the second. The draw inverts the table with a
  hash of the ray and the cluster, so a ray always meets the same volume. A hit is shaded as one
  of the cluster's constituents, as cards are.
- **Why both tables are measured.** Stage 5 showed one can't be derived from the other for
  foliage.

## Where rays leave from matters: the hotspot

The first version measured the leaving table from points on all the cluster's leaves, drawn by
area. The volumes came out too dark, more so the more clusters were volumes. The leaves a hit is
on are those a ray from outside meets first, and they are lit better than leaves within. That is
the correlation between what is seen and what is lit behind a canopy's hotspot (Kuusk 1985;
Bruneton & Neyret 2012).

Measuring the leaving table from those first-hit points instead:

| From the air | FLIP | Brightness |
|---|---|---|
| Cards (shipped) | 0.0543 | −1.2% |
| Volumes at L6+, leaving from all leaves | 0.0547 | −2.1% |
| Volumes at L6+, leaving from leaves seen | 0.0544 | −1.7% |
| Volumes at L5+, leaving from all leaves | 0.0568 | −3.9% |
| Volumes at L5+, leaving from leaves seen | 0.0542 | −2.5% |

A measured self-shadowing for a LOD has to be measured from where hits will be, not averaged
over all leaves. The same correction applies to the cards' `A` of stage 3.

## Results

`forest-aerial` from the air (600 × 350, 64 spp) and from 90 m up (32 spp), against full detail:

| | FLIP air | Brightness air | Render air | FLIP low | Brightness low, mid range | Peak (air) |
|---|---|---|---|---|---|---|
| Cards (shipped) | 0.0543 | −1.2% | 93 s | 0.0296 | −3% to −6% | 618 MiB |
| Volumes L6+ | 0.0544 | −1.7% | 137 s | 0.0295 | −3% to −6% | 630 MiB |
| Volumes L5+ | 0.0542 | −2.5% | 141 s | 0.0290 | −3% to −6% | 628 MiB |
| Volumes L4+ | | | | 0.0282 | −3.5% to −6% | |

- **Quality.** Volumes match cards from the air and do slightly better from the side, where they
  cover broadleaf crowns' sides, which cards miss. FLIP from low down improves from 0.0296 to
  0.0282 with volumes from L4.
- **Speed.** They render about 50% slower from the air, and as fast from the side.
- **Memory** is about the same.
- **The mid-range darkening from low down remains.** Cards and volumes leave it alike, so it
  isn't the cards' coverage from the side. Something else in the LOD causes it: the bark's vertex
  clustering, or the constituents' choice at grazing views. That is the next thing to measure.

## What it shows

A volume with transmittance measured per cluster is a workable stand-in at the coarsest levels.
It is no better than the corrected cards there, and is slower. The research results from it are
these:
- the two measured tables are needed, by stage 5;
- they must be measured from where hits are, for the hotspot.

To be worth shipping, a volume would have to be cheaper to trace, or used where cards fail
(broadleaf crowns seen from the side).
