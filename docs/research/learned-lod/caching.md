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
- **The rest of a warm build** is placing the trees. Stack samples put 80% of it in Perlin noise:
  the clearing and species fields evaluated for each candidate position (`place_favouring`).
  That is the next target. Placement could be kept too, but its key would have to cover the
  ground it stands on.

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
