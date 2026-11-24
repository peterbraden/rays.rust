# Research log

This log records how the survey was done, so it can be repeated and extended.

## 2026-11-21: survey and verification

**Question.** Can a much smaller learned data structure stand in for a mesh's triangles at a
distance and give "the constituents with a similar probability"? See [README.md](README.md).

**First pass: search snippets only.**
- Five agents each searched one cluster:
  - learned data structures;
  - neural ray queries;
  - neural geometry compression;
  - stochastic and volumetric aggregates;
  - foliage LOD and recent work.
- The session's network blocked the paper hosts (arXiv, ACM, CVF, Eurographics and the authors'
  pages), so every claim came from search-engine extracts or memory.
- That pass mapped the field and found the candidate gap. It also produced numbers that were
  wrong or attached to the wrong paper. None of them were used until checked (second pass).

**Second pass: full texts.**
- A cloud environment for this repository was created allowing these hosts: arxiv.org,
  export.arxiv.org, openaccess.thecvf.com, diglib.eg.org, graphics.cg.uni-saarland.de,
  research.nvidia.com, hal.science, nvlabs.github.io, sites.cs.ucsb.edu, cseweb.ucsd.edu,
  rgl.epfl.ch, graphics.pixar.com, jcgt.org, openreview.net, proceedings.neurips.cc,
  www.vldb.org and others.
- Three agents downloaded each paper into its own directory, converted it with
  `pdftotext -layout`, and read its method, results and limitations.
- 30 papers were read in full. Their notes, with section, equation and table references, are in
  [notes/](notes/):
  - [neural-lod.md](notes/neural-lod.md): 12 papers;
  - [aggregate-theory.md](notes/aggregate-theory.md): 8 papers, plus the list of what could not be
    fetched;
  - [learned-indexes.md](notes/learned-indexes.md): 10 papers, plus the arXiv searches.

**Still blocked during the second pass, even with the hosts allowed:**
- dl.acm.org and graphics.pixar.com: proxy 403.
- hal.science: an Anubis proof-of-work page, which we did not try to get around.
- rgl.epfl.ch's file host (cloudfront): 403.
- openreview.net: a browser check.
- cs.dartmouth.edu, graphics.stanford.edu and q-zh.github.io: not on the allow list.

The papers left unread are listed below.

**Corrections the full texts made to the first pass (abridged):**
- The N-BVH figures are 42× and 13× for its Fig. 1 scene. ">1000×" applies only to the neural part,
  and it is 5–100× over whole scenes.
- LSNIF's 106.2× and 6.24 MB come from different scenes.
- Weier 2023 thresholds its visibility per voxel. It does not aggregate correlation across levels
  explicitly; the levels share an object-space encoding.
- The Kraska learned Bloom filter saved 36%. The "47%" in some summaries is not in the paper.
- The PGM-index's published numbers differ from the preprint's.
- Neural Bounding's zero false negatives is empirical, not proven.
- Zhou et al.'s title is "Efficient Scene Appearance Aggregation…", and the method isn't neural.
- Bako et al. store about 3 KB per voxel, not 1 KB.
- SGGX was published in ACM TOG (SIGGRAPH 2015), not EGSR or CGF. The renderer's own citation was
  wrong and is fixed in this change.

**Searches for the gap.**
- arXiv metadata (export.arxiv.org API):
  - Every combination of "learned index", "learned bloom filter" or "learned sketch" with ray
    tracing, BVH, mesh, LOD, foliage or point clouds returned nothing.
  - So did "learned alias table".
  - Phrase queries were first checked against known papers, so these zeros are real zeros.
- These searches don't cover papers only in the ACM or journal libraries.
- The near misses, all in the bibliography:
  - neural many-light sampling (a proposal over a stored set);
  - NIF and LSNIF (approximate replacements of stored geometry);
  - 8DNA (a sampled distribution over directions).

## Unread (to do)

| Paper | Why it matters | Blocked by |
|---|---|---|
| Cook et al. 2007, Stochastic Simplification | The main non-learned prior art: its contrast correction and how it handles overlap | graphics.pixar.com (403) |
| Loubet & Neyret 2017, Hybrid LoDs | How it splits mesh from volume, its density formula, its foliage numbers | HAL bot wall; no open diglib file |
| van Antwerpen et al. 2026, Massive Dynamic Foliage | The state of the art in production; its memory tables | dl.acm.org |
| Bitterli et al. 2018 | T_c and T_u in the paper itself (checked here in d'Eon 2018) | cs.dartmouth.edu |
| Vicini et al. 2021 | The exact transmittance form (known only second-hand) | cloudfront 403 |
| Lacewell et al. 2008 | Directional opacity per BVH node | graphics.stanford.edu |
| LISA; Hsu et al. 2019; Strictly Conservative Neural Implicits | Spatial learned index; learned sketch; provable conservative bounds | ACM, OpenReview check, Wiley |
| Double Bounded Neural Ray Queries (IJCAI 2026) | Claims up to 300× | ijcai.org not allowed |
| NGLOD, VBNF, Instant NGP, NGF, Zeltner 2024, NeuMIP | Neural compression of surfaces and materials (abstracts only; judged a poor fit for leaf aggregates) | arXiv was blocked in the first pass |

## 2026-11-21: stage 0

Ran `src/shapes/lod_research.rs` (six species, levels 2–7) and rendered `forest-aerial` at
600 × 350 and 16 spp: full detail, LOD, sky-only versions of both, and LOD with cards enlarged
×1.2 by a temporary change. Results and their reading are in [stage0.md](stage0.md). The diagnosis
of the brightening changed, from self-shadowing to hit probability.

## 2026-11-23: out-of-core and streamed geometry

**Question.** What does the literature say about an on-disk cache of the forest's card levels:
memory mapping, eviction, per-ray against per-object resolution, ray reordering, shadow
consistency, measured costs? See [notes/streaming.md](notes/streaming.md).

**Fetched and read:**
- Wald, Dietrich, Slusallek 2004: full text, from diglib.eg.org (an EGSR PDF open there).
- Nanite: A Deep Dive (SIGGRAPH 2021 course): slides with speaker notes, from
  advances.realtimerendering.com.

**Blocked** (no way around a refusal was tried):
- Pharr et al. 1997: graphics.stanford.edu, pharr.org and pbr-book.org: proxy 403.
- Christensen et al. 2003: graphics.pixar.com: proxy 403; diglib.eg.org has the record but its PDF
  needs authentication (401).
- Yoon et al. 2006 (R-LODs): gamma.cs.unc.edu (DNS failure, then 403), sgvr.kaist.ac.kr and
  link.springer.com: 403.
- Eisenacher et al. 2013: disneyanimation.com hosts: 403; diglib.eg.org PDF: 401.
- van Antwerpen et al. 2026 and the Moana Island notes: dl.acm.org and disneyanimation.com.
- An arXiv full-text search (out-of-core ray tracing; foliage LOD for ray tracing; geometry
  streaming for path tracing) found nothing relevant.

**Findings that matter for the cache:**
- `mmap` of precomputed binary structures is the published method (Wald Sec. 4), and was simple.
  Its fault detection cost 16–26% (Table 3), but only because the model did not fit in memory.
- Both systems keep coarse data and selection metadata always resident and prioritise loading by
  projected size or error.
- Stand-ins used while data is missing change the image (Wald's proxies are too bright); a batch
  render should load what it needs before tracing so the image does not depend on timing.
- Nothing read measures ray-traced shadow consistency across levels; that is Christensen et al.
  2003's subject, which is unread.

