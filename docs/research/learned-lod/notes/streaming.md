# Checking the out-of-core and streamed-geometry papers against their full texts

The question: what does the literature say about keeping a forest's level-of-detail geometry on
disk and loading it instead of rebuilding it? This is for an on-disk cache of rays.rust's leaf cards
(docs/adaptive-meshes.md). The renderer instances about 243,000 trees from 84 distinct models. Each
model is simplified at build time into levels of cards, and a level is chosen per placed tree from
the camera's pixel size. Building every level takes 70–90 s per run, and the memory peak (growing
the full-detail trees) dominates the run. The plan is to store each (model, level) as a
memory-mapped file keyed by a hash of its inputs, loaded instead of rebuilt; later, perhaps, to
fault levels in on demand when rays reach a tree.

Topics looked for: geometry caches and eviction, resolution per ray (ray differentials) against per
object, ray reordering for coherence when geometry is out of core, on-disk formats and memory
mapping, consistency of shadows across levels, and measured costs.

I downloaded each paper into its own folder under `(local downloads)` and converted it with
`pdftotext -layout`. **Only two of the six were reachable: Wald et al. 2004 (read in full) and the
Nanite course slides with speaker notes (all 155 pages converted; the LOD, streaming, compression,
results and future-work sections read closely, the rasterizer and material sections skimmed).** The
other four were blocked by the proxy or a login wall (list at the end). For those, nothing below is
a number from the paper: what is said comes from search-engine snippets or abstracts and is marked
*unverified*.

---

## Main answers for the cache design

1. **Memory mapping precomputed binary structures is the published approach, and it was simple.**
   Wald et al. precompute "all static data structures (e.g. BSP index structures etc.)", "store
   them on disk in binary form, and map them into the address space via mmap()" (Sec. 4). The OS
   does the paging and "never pages in any data (e.g. shading information) that might not be
   required". That is the plan for our levels, with one difference: their data was 30–40 GB on disk
   for 6 GB of RAM (Sec. 6.1–6.2), so faulting was the problem they solved. Our levels presumably
   fit in memory; the gain is the skipped build, not working-set control.
2. **A page fault inside a render thread is what they had to avoid, and doing so cost 16–26% of
   frame rate** (Wald Table 3). Their hybrid scheme checks every pointer dereference for a tile
   crossing, looks tiles up in a hash table, cancels a ray that would fault, and has 4–8 fetcher
   threads read the tile with `madvise()`. A whole-file `mmap` of a level we will certainly use
   needs none of this; it matters only for the later "fault in on demand" step.
3. **Eviction in both systems is approximate LRU driven by use, with a priority from screen
   importance.** Wald: "second chance" (a clock over a referenced bit), starting at about 80% of
   memory, with fetch priority proportional to the BSP voxel's size and inversely to its distance
   from the camera, in 8 discrete queues (Sec. 4.3–4.4). Nanite: page requests come out of the
   culling traversal with "priority based on LOD Error", are re-emitted for resident pages to
   refresh their priority, and "low-priority pages" are evicted; shadow views request at lower
   priority.
4. **Keep the coarsest level and the selection metadata always resident.** Nanite's root page "is
   always resident, so we always have something to render", and "for simplicity, we always keep the
   full hierarchy resident" so that a new object "can immediately request data from all the levels
   it needs" instead of level by level, which would multiply IO latency "by the number of levels we
   are off". Wald loads all proxies (80 bytes each, 66–344 MB) before tracing and keeps them in
   physical memory, because "Otherwise, we would only replace paging for BSP nodes by paging for
   proxies" (Sec. 5.4). For us: the per-model level table, card counts and bounds belong in one
   small always-loaded index; the coarsest level of each of the 84 models could be kept resident
   as the fallback.
5. **A coarse stand-in used when data is missing changes the image: it is a quality trade, not a
   cache detail.** Wald's proxies average normals per direction, which "results in overly bright
   proxies" (Sec. 5.2). This is the same direction as rays.rust's LOD brightening. Their remedy for
   occlusion inside a proxy is to build it by ray casting from outside so only visible triangles
   contribute: "just randomly picking a triangle is not appropriate, as it would lead to the proxy
   getting yellowish" (Sec. 5.3). In a deterministic offline render, substituting a coarser level
   when a finer one isn't loaded yet would make the image depend on timing. A cache that loads
   synchronously, or loads everything before rendering, keeps the image identical.
6. **Granularity: stream at the unit that must be complete to be usable, packed in fixed-size
   pages.** Nanite streams at cluster-group granularity because "Full group needed before a cluster
   in the group can be rendered", fills fixed 128 KB pages at cluster granularity (clusters "up to
   about 2KB"), and gets "~1% slack". Wald found earlier voxel-sized units of "several thousand
   triangles" "far too large" and prone to fragmentation, and used 16 KB or 64 KB tiles instead
   (Sec. 2, 4.2). For us a (model, level) file is the natural complete unit: a tree can only be
   intersected once its level is whole.
7. **Format: an LZ-friendly disk format apart from the in-memory format paid off for Nanite.**
   Lumen in the Land of Nanite: raw 25.90 GB, memory format 7.67 GB (6.77 GB compressed, "only
   about 10% smaller"), disk format 4.61 GB, i.e. 5.6 bytes per Nanite triangle and 11.4 bytes per
   input triangle (results slide). Memory-mapping, however, requires the in-memory layout on disk,
   uncompressed. That is the trade: mmap for zero-copy loading, or a compact disk format with a
   decode step.
8. **Nothing read here measures shadow consistency across levels for ray tracing.** Nanite uses
   rasterized virtual shadow maps with the same LOD selection at shadow-map resolution, and explains
   that hardware ray tracing "isn't flexible enough" for its LOD logic. Wald calls the caching impact
   of shadows "surprisingly low" but says "an exact discussion of these effects is beyond the scope
   of this paper" (Sec. 6.6). Christensen et al. 2003, the paper that addresses per-ray resolution
   for secondary rays, could not be fetched.

---

## 1. Wald, Dietrich, Slusallek: "An Interactive Out-of-Core Rendering Framework for Visualizing Massively Complex Models"

**FULL TEXT READ.** Eurographics Symposium on Rendering 2004, pp. 81–92, A. Keller and H. W. Jensen
(eds.). Eurographics Digital Library handle 10.2312/EGWR/EGSR04/081-092 (the PDF came from
diglib.eg.org).

**Setting.** The Boeing 777 model: about 350 million triangles, "without using instantiation"; 12 GB
as bare float triples; "30–40 GByte including acceleration structures" (Sec. 2). Hardware: one
dual 1.8 GHz Opteron 246 with 6 GB RAM, Linux 2.4.25, a WD2500JB IDE disk at "roughly 50–55 MB/s"
(Sec. 6). The ray tracer is OpenRT with BSP trees.

**Method**
- **Memory management by the OS (Sec. 4).** Precomputed BSPs and geometry are stored in binary and
  mapped with `mmap()`. Using real pointers means address translation is done by the hardware MMU,
  and a race with eviction costs at worst a page fault, not a crash.
- **Detecting faults before they happen (Sec. 4.1–4.2).** Pages are grouped into tiles and kept in a
  hash table of tile addresses. A table entry is the tile's 64-bit base address, with bit 0 for
  "available" and bit 1 for "recently referenced". Hashing 128 GB of RAM in 4 KB tiles needs 32M
  entries; 16 KB or 64 KB tiles need 8M or 2M. Calling `mincore()` per access was ruled out as too
  costly. They rely on two observations: a loaded page stays in memory for a while, and pages are not
  paged out while free memory remains.
- **Fetching (Sec. 4.3).** A ray that would touch a missing tile is cancelled and the tile is queued.
  4–8 fetcher threads read tiles with `madvise()` and mark them available. Priority is proportional
  to the BSP voxel's size and inversely to its distance to the camera, mapped linearly into 8 queues
  "relative to the minimum and maximum priorities of the previous frame". Counting rays per tile was
  rejected as too costly (shared counters).
- **Eviction (Sec. 4.4).** "Second chance", as the Linux swapper: an evictor cycles through the tile
  table clearing the referenced bit, and evicts a tile it finds still unreferenced on its next visit,
  discarding the pages with `madvise()`. Eviction starts "at a memory utilization of ∼80%". A full
  LRU "would be too expensive".
- **Overhead control (Sec. 4.5).** Each dereference checks whether it crosses a tile boundary
  relative to the previous access (a few bit operations); only crossings consult the table. Mutexes
  are taken only when adding a new tile to the table or a request to a queue.
- **Layout (Sec. 5.1).** BSP nodes are stored in tile-sized blocks filled breadth-first (after Havran
  1999) rather than depth-first, so fewer pointers cross tiles and the proxies are spread more
  evenly.
- **Proxies (Sec. 5.2–5.4).** A proxy stands for each BSP subtree whose pointer crosses into another
  tile, and is used when a ray is cancelled there.
  - It is directional only: 18 directions (an octahedron subdivided once), each with an averaged
    diffuse colour (5+6+5 bits) and an averaged normal (16 bits), interpolated barycentrically.
  - Built by tracing about 10K Halton-distributed rays from outside, so that only visible triangles
    contribute.
  - 80 bytes each: 72 bytes of directional data, a float of surface area for prioritised loading, and
    a 32-bit subtree ID. Found through an STL map keyed by the parent node's address, so OpenRT's own
    structures are unchanged.
  - A fault on triangles or shading data (not BSP nodes) uses "the last proxy encountered during
    traversal".

**Numbers**
- **Table 1 (number and memory of proxies):**

  | BSP | 16 KB tiles | 64 KB tiles |
  |---|---|---|
  | deep | 15.6M, 1.2 GB | 4.3M, 344 MB |
  | shallow | 833K, 66 MB | 383K, 30 MB |

  The defaults are shallow BSPs with 16 KB tiles (66 MB). Sec. 5.1 says this gives "roughly 1.1
  million proxies", which does not match Table 1's 833K.
- **Preprocessing (Sec. 6.1).** Splitting stops at 2–4 million triangles per voxel, so each block's
  BSP can be built in core with a cost function. Everything (unpacking, conversion, splitting, BSP
  building, proxies) takes "in the order of one day on a single PC", and 30–40 GB on disk (deep BSPs
  about 40 GB, shallow about 30 GB, Sec. 6.2). Proxy building parallelises trivially by voxel.
- **Cache (Sec. 6.2).** 4–5 GB of the 6 GB is the geometry cache: "each individual view fits into
  cache, and the proxies only have to bridge the loading latencies when changing views."
- **Startup (Sec. 6.3).** Parsing the voxel list and building a top-level BSP takes "at most a few
  seconds"; reading all proxies takes "several seconds to a few minutes". Loading a view's working
  set can take "up to several minutes".
- **Table 2 (working set per reference view, MB):**

  | BSP | overview | engine | wheels | cockpit | cabin |
  |---|---|---|---|---|---|
  | deep | 2,300 | 145 | 40 | 122 | 254 |
  | shallow | 2,150 | 215 | 36 | 105 | 236 |

- **Table 3 (frames per second, all tiles resident, without and with the memory manager):**

  | BSP | overview | engine | wheels | cockpit | cabin |
  |---|---|---|---|---|---|
  | shallow, without | 2.7 | 2.4 | 5.3 | 2.0 | 2.1 |
  | shallow, with | 1.9 | 1.8 | 4.0 | 1.5 | 1.6 |
  | shallow overhead | 26% | 25% | 24% | 25% | 23% |
  | deep, without | 4.9 | 3.6 | 9.0 | 4.0 | 4.0 |
  | deep, with | 4.1 | 2.9 | 7.1 | 3.1 | 3.2 |
  | deep overhead | 16% | 19% | 21% | 22% | 20% |

  This is the worst case for the manager (everything resident, deep traversals). The comparison
  could only be made for static views, since "even small camera movements lead to long paging
  stalls if the MMU is turned off".
- **Table 4.** 640×480: 1.5–4.0 fps (shallow), 2.9–7.1 fps (deep); 1280×1024: 0.5–1.3 and 0.9–2.3
  fps. No antialiasing. The headline "3–7 fps" and Fig. 4's per-view rates (4.1, 2.9, 7.1, 3.1, 3.2)
  are the deep-BSP numbers, although the text says shallow BSPs are the default.
- No hit rates or miss counts are reported.

**Limitations (stated)**
- Proxies have no positional information, so a proxy covering many pixels looks like a coloured box;
  large camera moves show "blocky artifacts" until data loads.
- Averaged normals over-estimate the cosine to the viewer, so proxies are too bright.
- The image while loading depends on what has loaded: the design trades correctness for
  interactivity.
- Shadows are shown (Fig. 6, a light attached to the viewer) but not measured. There is no
  discussion of shadow rays hitting proxies or of consistency between levels.
- Earlier work it replaces (Wald et al. 2001) suspended rays until data arrived; this "can hide the
  loading latency only within the duration of one frame", and a camera change "often triggers
  thousands of disk read requests" (Sec. 2).

**For us.** This is the clearest precedent for "precompute, write binary, mmap". Its machinery
exists because 30–40 GB didn't fit in 6 GB and the renderer was interactive. A batch render that
maps each needed (model, level) file whole before tracing avoids both the fault checks (16–26%) and
the timing-dependent proxies. If levels are later faulted in on demand, the lessons are: detect
misses at a coarse unit, not per pointer; never stall a render thread on IO; keep a small,
always-resident fallback; prioritise by projected size; and expect that a fallback built by
averaging normals will brighten. Their fix for occlusion inside a proxy (build it from rays cast from
outside) is the same concern as choosing a card's constituents by visible area.

## 2. Karis, Stubbe, Wihlidal: "Nanite: A Deep Dive"

**SLIDES AND SPEAKER NOTES READ** (PDF from advances.realtimerendering.com, 155 pages). SIGGRAPH 2021
course *Advances in Real-Time Rendering in Games*. No DOI. The LOD build, LOD selection, streaming,
compression, results and future-work sections were read closely; the rasterizer, visibility buffer
and material sections were skimmed. A rasterizer, not a ray tracer.

**Method**
- **Clusters and the DAG.** Clusters of 128 triangles. Groups of clusters are merged, simplified to
  half the triangles with locked group boundaries, and split again; because group boundaries change
  between levels the result is a DAG, not a tree. The simplifier is edge collapse with a quadric error
  metric; its error estimate "is later projected on screen to a number of pixels of error".
- **Selection.** A cluster is drawn when its parent's projected error is above the threshold and its
  own is at or below it. All clusters of a group store "the same unioned error value and sphere
  bounds", so they decide the same way without communication ("Same input => same output"). Errors
  are forced to be monotonic (parent ≥ child), so the cut is unique and can be evaluated in parallel.
  The target is under one pixel of error, with TAA blending sub-pixel differences.
- **Streaming.** "Conceptually similar to virtual texturing: GPU requests needed data. CPU fulfills
  them." The loaded data must always be "a valid cut of full DAG", so no cracks.
  - The streaming unit is the cluster group, since a cluster can't be drawn until its whole group is
    present.
  - Fixed-size pages avoid fragmentation. Pages are filled with groups by spatial locality; groups are
    split at cluster granularity (clusters "up to about 2KB", groups of 8 to 32 clusters) into
    consecutive pages, giving "~1KB average slack per page! (~1% slack for 128KB page)". A split
    group is active only when all its parts are loaded.
  - The root page holds the top of the DAG and is always resident.
  - The full culling hierarchy (metadata about cluster groups, "tiny") is always resident, so a
    traversal can look past the streaming cut and request every level needed at once.
  - Requests are emitted during culling with a priority from LOD error, for the primary view and, at
    lower priority, shadow views; they are re-emitted for resident pages to keep priorities current.
    The CPU reads them back asynchronously, adds missing DAG dependencies, issues IO for the highest
    priority pages, evicts low-priority ones, then installs pages and patches GPU pointers.
- **Compression.** Two formats: a memory format decoded at random access for rendering, and a disk
  format designed to compress well under a byte-based (hardware) LZ coder and transcoded on the GPU
  ("~50GB/s with fairly unoptimized code on PS5"). Values are quantised and stored with the minimum
  number of bits for their range.
- **Tiny instances.** "Instance memory adds up fast: 10M * float4x3 = 457MB." Far away, meshes switch
  to "visibility buffer imposters": 12×12 view directions in an octahedral atlas, 12×12 pixels each,
  storing depth and triangle ID, "40.5KB per mesh always resident". "Merged unique proxies must
  replace instances at extreme distances"; switching can sometimes be noticed as a pop.

**Numbers**
- **Results slide, Lumen in the Land of Nanite:** 433M input triangles, 882M Nanite triangles. Raw
  data 25.90 GB ("Full floats, byte indices, implicit tangents"); memory format 7.67 GB, 6.77 GB
  compressed; compressed disk format 4.61 GB, about 20% better than Early Access; 5.6 bytes per Nanite
  triangle, 11.4 bytes per input triangle; "1M triangles = ~10.9MB on disk". Compressed sizes use
  Kraken level 5 (notes).
- **Pipeline numbers (one frame):** main pass 896,322 instances before culling, 3,668 after; post
  pass 102,804 and 365. 199,420 clusters, 25,041,711 triangles rasterised; the notes say a standard
  UE4 path "would have to rasterize over a billion triangles", and that the 25M "is consistent
  throughout the demo".
- **Timing:** about 2.5 ms for culling and rasterising the whole visibility buffer and about 2 ms for
  the material pass, at an average of about 2496×1404 upsampled to 4K. The per-pass table prints
  "Main Pass: InstanceCull 108ms", inconsistent with the 2.5 ms total; probably 108 µs.
- **Ray tracing formats:** "HW triangle formats + BLAS currently are 3-7x the size our memory format
  which includes attributes. A direct comparison without attributes the ratio is ~60% worse."
- **Virtual shadow maps:** 16k × 16k, 128×128-texel pages, a 128×128 page table with mips; pages
  marked where screen pixels project, at the mip where one texel matches one pixel; cached pages
  reused.

**Limitations (stated)**
- "Not great with aggregates: Many tiny things becoming a porous volume. Grass, leaves, hair." "The
  property of scaling cost with screen resolution doesn't hold with aggregates such as grass and
  leaves." Prefiltering partial coverage for leaves and grass "is still an open question".
- No translucent or masked materials, no deformation (2021).
- Shadows are rasterised, because DXR "isn't flexible enough" for the LOD logic and custom triangle
  encoding and has no partial BVH updates. "Out of core ray tracing" is listed as future work.
- Cluster LOD selection is isotropic, so glancing surfaces are over-tessellated.

**For us.** Nanite's structure is per-cluster, view-dependent LOD within a mesh; ours is a level per
placed tree. What transfers is the streaming design: a fixed-size unit that must be complete before
use, always-resident coarse data and selection metadata, requests prioritised by projected error,
asynchronous IO with eviction by priority, and a separate compact disk format. Their warning applies
directly: Nanite does not handle foliage, and its far-field answer (imposters, merged proxies) is
an approximation whose switch can pop. Its shadows use the same LOD selection at the shadow map's
resolution, which is one way of keeping shadows consistent with what is seen; it is not a ray-traced
answer.

## 3. Pharr, Kolb, Gershbein, Hanrahan: "Rendering Complex Scenes with Memory-Coherent Ray Tracing"

**NOT READ: could not fetch.** SIGGRAPH 1997, Computer Graphics 31 (Annual Conference Series),
pp. 101–108. doi:10.1145/258734.258791. Hosts tried: graphics.stanford.edu, pharr.org,
pbr-book.org, dl.acm.org (all proxy 403).

What is known only from search snippets and from citations in papers that were read (unverified):
- The search engine's summary describes a geometry cache over regular voxel grids ("geometry
  grids"), a second "scheduling grid" whose voxels each hold a queue of rays, and choosing which
  scheduling voxel to process next so as to reduce what must be loaded into the cache; and a claim
  of rendering scenes of millions of primitives while keeping about ten percent of the scene in
  memory. None of this was checked against the paper.
- Wald et al. 2004 (read) summarise it as "a caching and reordering scheme that reorders the rays in
  a way that minimizes disk I/O", efficient for models much larger than memory but "not easily
  applicable to interactive rendering" (Sec. 2).

**For us (provisional).** The paper to read before any on-demand faulting of tree levels: it is the
original argument for reordering rays so that geometry is touched coherently. It is less relevant to
the first step (load every needed level up front).

## 4. Christensen, Laur, Fong, Wooten, Batali: "Ray Differentials and Multiresolution Geometry Caching for Distribution Ray Tracing in Complex Scenes"

**NOT READ: could not fetch.** Computer Graphics Forum 22(3) (Eurographics 2003), pp. 543–552.
doi:10.1111/1467-8659.t01-1-00702. graphics.pixar.com returned proxy 403; the Eurographics Digital
Library record is reachable but its PDF needs authentication (HTTP 401).

From the abstract as quoted by search results (unverified): coherent rays do well with a standard
geometry cache, incoherent rays need less geometric accuracy, and ray differentials distinguish the
two; the scheme extends multiresolution caching to tessellated surfaces; scenes that would need
about 100 times the cache size fully tessellated render efficiently. No number from the paper is
used in this note.

**For us (provisional).** This is the paper on choosing resolution per ray rather than per object,
and on whether shadow and glossy rays may use coarser geometry than camera rays. rays.rust chooses
a card level per placed tree from the camera; whether secondary rays should use a coarser level,
and what that does to shadows, is the question it answers. It should be read before that decision.

## 5. Yoon, Lauterbach, Manocha: "R-LODs: Fast LOD-based Ray Tracing of Massive Models"

**NOT READ: could not fetch.** The Visual Computer 22, 2006, pp. 772–784 (Pacific Graphics).
doi:10.1007/s00371-006-0062-y. gamma.cs.unc.edu did not resolve or returned 403; sgvr.kaist.ac.kr and
link.springer.com returned 403.

From search snippets (unverified): an R-LOD is a simplified plane with material attributes standing
for the triangles under a kd-tree inner node, with a surface-deviation error projected to screen at
run time; an error metric for primary and secondary rays; cache-oblivious layouts; reported speedups
of about 2× on models with small working sets to 20–50× on very large ones.

**For us (provisional).** Closest in spirit to our cards (a planar stand-in per node, chosen by
projected error) and claims a criterion for secondary rays. Needs reading before any per-ray level
choice.

## 6. Eisenacher, Nichols, Selle, Burley: "Sorted Deferred Shading for Production Path Tracing"

**NOT READ: could not fetch.** Computer Graphics Forum 32(4) (EGSR 2013), pp. 125–132.
doi:10.1111/cgf.12158. media.disneyanimation.com and disneyanimation.com returned proxy 403; the
diglib.eg.org files need authentication (401).

From the abstract as quoted by search results (unverified): large, possibly out-of-core ray batches
are sorted for coherent traversal, and shading is deferred until hits are sorted, removing the need
for shading caches while keeping texture access coherent.

**For us (provisional).** The production version of ray reordering. Relevant only if levels are
faulted in on demand during the render.

## 7. Foliage streaming for ray tracing

Not found in a readable form. van Antwerpen et al., "Real-time Path Tracing of Massive Dynamic
Foliage" (HPG 2026, doi:10.1145/3820021), is on dl.acm.org, which is blocked (as in the earlier pass,
see [aggregate-theory.md](aggregate-theory.md)). The Moana Island scene's pages at
disneyanimation.com are blocked. An arXiv full-text search for out-of-core ray tracing, foliage LOD
for ray tracing and geometry streaming for path tracing found nothing relevant.

---

## Could not be fetched

| Paper | Hosts tried | Result |
|---|---|---|
| Pharr et al. 1997 | graphics.stanford.edu, pharr.org, pbr-book.org | proxy 403 (dl.acm.org known blocked) |
| Christensen et al. 2003 | graphics.pixar.com; diglib.eg.org | proxy 403; record page open, PDF 401 (authentication) |
| Yoon et al. 2006 (R-LODs) | gamma.cs.unc.edu; sgvr.kaist.ac.kr; link.springer.com | DNS failure or 403; 403; 403 |
| Eisenacher et al. 2013 | media.disneyanimation.com, disneyanimation.com; diglib.eg.org | proxy 403; PDF 401 (authentication) |
| van Antwerpen et al. 2026 | dl.acm.org | blocked (earlier pass) |
| Moana Island scene notes | disneyanimation.com | proxy 403 |

No other route around these refusals was tried.
