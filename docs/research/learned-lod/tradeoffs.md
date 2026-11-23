# Tradeoffs: memory, accuracy and speed

The card fixes in this research aimed at better accuracy for the same memory as the naive cards.
This sweep places every scheme and setting on one map, to tell which changes push the frontier
out and which only move along it. Raw rows are in [tradeoffs.csv](tradeoffs.csv).

**Setup.**
- Scene and references:
  - `forest-aerial` at 600 × 350, seen from the air at 64 spp, and from low down (camera
    `[0, 90, -900]` looking at `[0, 40, 600]`) at 32 spp.
  - Each view was compared with a full-detail render of the same view.
- Accuracy is measured two ways:
  - mean FLIP (`--compare`);
  - brightness bias, the mean linear luminance against full detail from
    `scripts/compare-linear.py` (clamped at the reference's 99.9th percentile, near and far split
    at the median depth).
- Memory is the peak resident set at the end of the build; times are the renderer's own.
- Machine and binaries:
  - One machine, one run at a time.
  - Binaries: master (naive cards), this branch (current cards) and the research variants' commits.
- Research variants' build times predate the parallel build, so only their memory, render time
  and accuracy compare directly.

## From the air

| Scheme | `pixels` | Peak MiB | Build s | Render s | FLIP | Bias |
|---|---|---|---|---|---|---|
| Full detail | — | 3,979 | 49 | 354 | 0 | 0 |
| Naive (master) | 0.5 | 1,389 | 200 | 165 | 0.0527 | +3.9% |
| Current | 0.5 | 1,334 | 97 | 146 | 0.0499 | −3.0% |
| Naive | 1 | 699 | 188 | 135 | 0.0585 | +4.6% |
| Current | 1 | 629 | 87 | 123 | 0.0543 | −1.3% |
| Naive | 2 | 428 | 189 | 119 | 0.0704 | +6.8% |
| Current | 2 | 390 | 76 | 101 | 0.0660 | +2.9% |
| Naive | 4 | 327 | 194 | 103 | 0.0925 | +10.5% |
| Current | 4 | 328 | 66 | 89 | 0.0896 | +8.3% |
| Naive | 8 | 291 | 187 | 93 | 0.1191 | +14.0% |
| Current | 8 | 326 | 60 | 76 | 0.1175 | +13.0% |
| Crossed + measured A ([stage3](stage3.md)) | 1 | 865 | 357* | 135 | **0.0454** | +0.3% |
| Compact crossed ([stage4](stage4.md)) | 1 | 664 | 311* | 159 | 0.0548 | −3.4% |
| Volumes from L5 ([stage6](stage6.md)) | 1 | 625 | 244* | 134 | 0.0543 | −2.5% |
| Volumes from L6 | 1 | 629 | 234* | 130 | 0.0544 | −1.7% |

\* before the parallel build.

FLIP counts both images' 64-spp noise, so even a perfect LOD would not score 0. That floor was not
measured.

## From low down

| Scheme | `pixels` | Peak MiB | Render s | FLIP | Bias |
|---|---|---|---|---|---|
| Full detail | — | 3,991 | 90 | 0 | 0 |
| Naive | 1 | 858 | 44 | 0.0313 | +0.4% |
| Current | 1 | 781 | 38 | 0.0296 | −0.4% |
| Naive | 2 | 490 | 37 | 0.0368 | +0.3% |
| Current | 2 | 447 | 32 | 0.0363 | −0.4% |
| Crossed + measured A | 1 | 1,041 | 44 | **0.0251** | +0.3% |
| Volumes from L4 | 1 | 788 | 46 | 0.0282 | −0.4% |

## Reading it

**What moves the frontier:**
- **Card sizing by hemisphere coverage** ([stage0](stage0.md)).
  - At every `pixels` it gives lower FLIP for the same or less memory: 0.0543 against 0.0585 at
    p1, and 0.0660 against 0.0704 at p2.
  - It brings the bias at p1 from +4.6% to −1.3%.
- **Merged constituents** (at most 16, NNLS areas): about 10% less memory at p0.5–p2, at no
  accuracy cost.
- **Speed work.** It doesn't change the image.
  - Build: 2.2–3.1× faster.
  - Render: 9–18% faster, from adaptive sky steps, Möller–Trumbore cards and timing pixels only
    for `--time-map`.
- **Crossed cards with measured self-shadowing** move it most.
  - FLIP 0.0454 at 865 MiB. Plain cards need `pixels: 0.5` and 1,334 MiB to reach 0.0499.
  - They render no slower than plain cards. From low down they are the best too.

**What slides along it:**
- **`pixels`.** Going from p1 to p2 trades 38% less memory and 18% less render time for 22% more
  FLIP.
- **The memory floor.** Beyond p4, memory stays at about 330 MiB, the rest of the scene, while
  error keeps climbing.

**What doesn't help:**
- **Compact crossed cards** land where plain p1 is, slower and darker.
- **Volumes** do the same, with a slower build: whatever limits accuracy at p1, they don't fix it.

## Where the next push could come from

1. **The coarse levels' brightness.**
   - The bias climbs from −1.3% at p1 to +2.9% at p2, +8.3% at p4 and +13% at p8.
   - The p1 render mostly uses L4–L7, so the drift comes from L5 and coarser cards, which stage 0
     measured hit 4% more often than their leaves before the sizing fix.
   - Fixing it would make p2 a usable default. It is likely the largest lever on speed and memory
     together, because it moves the left end of the curve down.
2. **Crossed cards on the parallel build**, measured at p1 and p2. Crossed cards only at fine
   levels, where they are needed, could keep most of the accuracy for less memory.
3. **The 330 MiB floor.** That memory is terrain, ground cover and per-placement records (5–10
   million instances at p4–p8). It needs a breakdown before an f32 instance layout or per-model
   merged BVHs can be judged.
4. **The LOD build's overhead** over full detail, 87 s against 49 s, from building card levels. It
   hasn't been profiled yet.
5. **SGGX lobes for coarse conifer levels** ([stage0](stage0.md)). They matter only once the
   floor comes down.

## Why coarser than `pixels: 2` gains little

Measured on `forest-aerial` (600 × 350, 64 spp; memory mid-render from glibc's `malloc_stats` and
`/proc`):

| | Peak at build | Resident while rendering | Live heap | Render |
|---|---|---|---|---|
| No forest | 65 MiB | — | 64 MiB | 55 s |
| `pixels: 2` | 400 MiB | 370 MiB (297 after `malloc_trim`) | — | 103 s |
| `pixels: 4` | 337 MiB | 316 MiB (246 after `malloc_trim`) | 163 MiB | 89 s |
| `pixels: 8` | 334 MiB | 310 MiB | — | ~83 s |

**Memory.** Going from `pixels: 2` to 4 cuts the cards built from 660 k to 145 k, which saves only
about 60 MiB. At `pixels: 4` the forest's own live data is about 100 MiB:
- 243,000 placements and the BVH over them;
- the 84 models' materials, with 21 MiB of per-leaf autumn states sized for the full-detail trees;
- the cards themselves.

None of the per-tree part depends on the level. The rest of the peak is memory freed after the
build: growing full-detail trees, several at a time. Of 318 MB that glibc holds mid-render, only
163 MB is in use; `malloc_trim` returns about 70 MiB of it.

**Time.** Without the forest the scene renders in 55 s, with its sky, aerial perspective, terrain and
soil. The forest adds 48 s at `pixels: 2` and 34 s at 4. Much of that is traversing the 243,000
placements, which costs the same at every level.

**What would lower the floor:**
- trimming or reusing the allocator's memory after the build (resident memory only);
- fewer full-detail trees in flight while building (the peak);
- smaller placements (f32, about 32 bytes instead of 72) and a compressed BVH over them;
- and for time, the rest of the scene.

### Compact placements, measured

A placement is now 28 bytes rather than 72: its model as a number into the stand's list, with a
mirror bit, and its position, turn and scale in single precision; arithmetic stays in double.

Measured back to back with the previous binary, 64 spp:

| | Resident while rendering | Render | FLIP |
|---|---|---|---|
| `pixels: 2`, before → after | 383 → 341 MiB | 110 → 110 s | 0.0627 → 0.0625 |
| `pixels: 4`, before → after | 309 → 288 MiB | 95 → 93 s | 0.0752 → 0.0752 |

The FLIP difference comes from positions rounded to single precision, which changes the noise
pattern.

**Grouping trees in the BVH's leaves** (2 or 4 a leaf, to cut nodes) was within noise:
274–350 MiB and 87–113 s.

**Neither moves render time, and the build's peak is unchanged.** Traversing the placements is not
where the forest's time goes, and the peak is set by growing the full-detail trees. The next gains
in memory are at the build, not in the stand.
