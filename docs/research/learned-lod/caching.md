# Keeping a forest's levels on disk

A forest's build grew each distinct tree in full and simplified it to the levels of detail its
placements need, every run. On `forest-aerial` at `pixels: 2` that took most of an 83 s build,
and set its memory peak. A tree's levels depend only on what makes them, so they can be kept.
With `"cache": "<directory>"` on a forest (`src/procedural/tree_cache.rs`), they are written
once and read back.

## What is kept

- **`tree-<key>.bin`, a tree's header.** It holds what choosing a placement's level and making the
  tree's materials need without its mesh:
  - the size of its pieces;
  - its bounds;
  - each leaf's autumn state;
  - its parts: bark, dead bark, and the leaves of each material.
- **`tree-<key>-<part>-<level>.bin`, one part at one level** (`lod::Detail::encode`):
  - cards in single precision, with their constituents and lit fractions;
  - a level's own leaves and simplified bark in double precision.

  BVHs are rebuilt on reading, which is fast next to growing a tree.
- **The key** is the FNV hash (`world::key`) of everything that makes the tree:
  - its parameters;
  - its seed;
  - its forest entry;
  - the materials it names, since the leaves' albedo shapes their cards' self-shadowing.

  The level format is checked by a magic number.
- **Writes** go whole to a temporary file renamed into place, as `world` writes its stages, so runs
  can share a directory.
- **Exactness.** A level is used as decoded whether it was read or just made. So a render is the
  same with the cache, without it, and with it empty. On `forest-aerial` the three images are
  bit-identical.

## Measured

`forest-aerial`, `pixels: 2`, 600 × 350, 16 spp, one machine:

| | Build | Peak memory | Resident while rendering | Render |
|---|---|---|---|---|
| No cache | 83 s | 392 MiB | 368 MiB | 27 s |
| Cache empty (filled by this run) | 78 s | 401 MiB | 377 MiB | 26 s |
| **Cache full** | **37 s** | **313 MiB** | **281 MiB** | 27 s |

- **Size.** The cache is 109 MB in 965 files, for 84 trees at the levels this view uses.
- **The rest of a warm build** was placing the trees. Stack samples put 80% of it in Perlin noise:
  the clearing and species fields evaluated for each candidate position (`place_favouring`).

## Placing the trees

`forest-aerial` throws about 20 million candidate positions, 30 for each tree that could fit at
the closest spacing. Most pass the cheap first check against neighbours, which uses the smallest
radius any tree can have (about 1.4 m here). Each then evaluated 15 noise fields: the clearings'
and one for each of 14 species. Two exact changes, both keeping the same 243,016 trees and a
bit-identical image:

- **The species' fields are skipped for a candidate in a clearing.** Clearings are never filled,
  so the 22% of candidates landing in them paid for all 15 fields to the end, though the first
  decides them. Warm build 46 → 22 s, back to back.
- **The neighbour check scans only cells within reach of its circle,** on cells a quarter as
  wide. It had scanned nine cells as wide as the largest pair of radii. 22 → 20 s.

**Placements are kept too.** They are stored as `placements-<key>.bin`, keyed by the forest
(without its `"lod"` and `"cache"`) and every geometry the scene defines, its variables
substituted, since one of them is the ground. That key is conservative: any edit to a geometry
places the trees again, but a camera or render setting doesn't. A heightmap image is named, not
hashed, as for resumed renders. A forest in an ecosystem, or standing on a landscape, isn't kept.

| `forest-aerial`, `pixels: 2` | Build |
|---|---|
| Before this work | 83 s |
| No cache, faster placement | 65 s |
| Levels and placements written | 27 s |
| **Both read back** | **1.8 s** |

The cache is 123 MB, 14 MB of it placements.

## From the literature

See [notes/streaming.md](notes/streaming.md). Of six papers, two could be read: Wald, Dietrich and
Slusallek 2004, and the Nanite course notes. Four were blocked: Pharr et al. 1997, Christensen
et al. 2003, Yoon et al. 2006, and Eisenacher et al. 2013.

What the two read support:

1. **Precomputed binary data, memory mapped, is the published approach.** Wald et al. 2004
   (Sec. 4) map theirs with `mmap`. Most of their machinery exists only because their model was
   30–40 GB against 6 GB of memory.
2. **Keep the small index resident.** Both systems keep the selection metadata always loaded:
   Wald's proxies, Nanite's culling hierarchy and root page. Here that is each tree's header.
3. **Load whole units before use, and don't fault inside render threads.**
   - Avoiding page faults in render threads cost Wald et al. 16–26% of frame rate (Table 3).
   - Substituting a coarser level while a finer one loads would make a batch render depend on
     timing.
   - Here, the levels a camera needs are known before tracing, so they are loaded then.
4. **Stand-ins change the image.** Wald et al.'s averaged-normal proxies are "overly bright"
   (Sec. 5.2), the same direction as the LOD brightening found in this research.

## Next

- **Memory mapping in place of reading.** It needs the in-memory layout on disk, BVH included.
  Nanite's experience is that a compact disk format (4.6 GB against 7.7 GB in memory) can pay
  more than zero-copy loading.
- **Levels loaded when a ray first reaches a tree,** if scenes outgrow memory.
- **The browser builder.** `assets::read` already serves files the page provides, so it could
  fetch cached levels instead of growing trees in WebAssembly.

## Where a cached forest's render goes

With the build down to seconds, the forest's cost is in the render. Measured on `forest-aerial`
at `pixels: 2` by instruction counts (callgrind, 200 × 116 at 2 spp; run-to-run timings vary
±6% on this machine):

| | Share of all instructions |
|---|---|
| Searching the 243,000 placed trees (nearest hits / shadow rays) | 15.7% / 9.1% |
| Searching and testing a tree's cards | 13.8% |
| A tree's list of parts (bark, dead bark, leaf materials) | 3.8% |
| Bark | 1.6% |

Counted per nearest-hit search:
- 32 nodes of the trees' BVH;
- 2.6 trees entered;
- 10 nodes inside each tree;
- about 3 cards tested per card set searched.

That is about what a well-built tree over 243,000 boxes should cost (its depth is about 18),
so no structural waste is left.

**Tighter boxes for placed trees** were tried, with an image the same to the bit. A placed tree's
box now comes from its model's inner boxes, transformed (`Geometry::transformed_bounds`, passed
through `PartOf` and `CardSet`, and from the items themselves for a set of a few). Before, it was
the model's box turned, which for a tree turned 45° is half as wide again as its crown. That
saves only 1.2% of instructions, 3.7% of the trees' search.

**What's left is cost per node.** The nodes are 56 bytes in double precision, two children at a
time, about 27 MB for the trees' BVH, more than fits in cache. Options:
- **Nodes in single precision** (conservatively rounded, about 32 bytes);
- **a four-wide BVH**, half the depth, four boxes tested together;
- **one BVH per model** in place of the list of parts over their own BVHs.

Each changes the BVH the whole renderer uses, or the forest's model structure.
