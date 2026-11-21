# Learned data structures vs. card constituents: literature verification (full texts)

Context (docs/adaptive-meshes.md): a distant tree's leaves become cards; each card keeps up to
32 of its leaves ("constituents", 12 B each: face + octahedral normal); a hit is shaded as one
leaf chosen in proportion to a|n.d| by a ray hash. On the forest scene constituents are ~235 MB
of a 670 MB peak (95 MB cards, 90 MB card BVHs). The question: can a much smaller learned
structure stand in for the constituent lists, giving "the constituents with a similar
probability", as learned indexes / learned Bloom filters stand in for B-trees / Bloom filters?

Method: PDFs fetched with curl into separate directories, converted with `pdftotext -layout`,
read section by section. Section/equation numbers below are those of the versions read.

---------------------------------------------------------------------------------------------

## 1. Kraska, Beutel, Chi, Dean, Polyzotis, "The Case for Learned Index Structures", SIGMOD 2018 (arXiv 1712.01208v3, 30 Apr 2018)

FULL TEXT READ.

Mechanism
- §2: a range index is a model key -> position in a sorted array; a B-tree is "a regression tree
  ... with a min- and max-error (a min-error of 0 and a max-error of the page-size)". The error
  guarantee holds "only ... over the stored keys, not for all possible keys": for a monotonic
  model one runs the model on every stored key and records the worst over- and under-prediction.
  Footnote 2: the model must be monotonic for the bounds to hold for keys not in the set.
  "Second, and more importantly, the strong error bounds are not even needed": a local
  (e.g. exponential) search around the prediction corrects any error, since the data are sorted.
- §3.2 RMI: stage-wise trained hierarchy; each stage's output picks the next-stage model;
  loss L_l = sum (f_l^(floor(M_l f_{l-1}(x)/N))(x) - y)^2. Trained stage-wise, "not fully
  end-to-end" (footnote 4).
- §3.3 Hybrid indexes (Algorithm 1): after training, every last-stage model's max absolute
  error is computed on its records; if above a threshold it is replaced by a B-tree. "we store
  the standard and min- and max-error for every model on the last stage". This "bound[s] the
  worst case performance of learned indexes to the performance of B-Trees". BUT §3.7.1: "we do
  not report on hybrid models" in the main integer results.
- §3.4 Search: model-biased binary search and biased quaternary search, within [min-err,
  max-err]. Guaranteed to find existing keys; for absent keys a non-monotonic RMI may return
  the wrong bound, fixed by widening the search area or exponential search.
- §2.1 cost argument: a model is faster if it has better precision gain than 1/100 per
  ~400 arithmetic ops (B-tree page traversal ~50 cycles x 8 SIMD ops/cycle).

Numbers (exact)
- Abstract: "outperform cache-optimized B-Trees by up to 70% in speed while saving an
  order-of-magnitude in memory over several real-world data sets."
- §3.7.1 (integer data, 2-stage RMI vs B-tree page 128): "up to 1.5 - 3x faster while being up
  to two orders-of-magnitude smaller".
- §5 Bloom filters: 1 billion records ~1.76 GB (presumably 1% FPR); 0.01% FPR ~2.23 GB (as printed).
- §5.1.1 learned Bloom filter = classifier f with threshold tau + "overflow Bloom filter" on
  K_tau- = {x in K | f(x) < tau} (the paper says "overflow"; "spillover" appears in §5.2).
  FPR_O = FPR_tau + (1 - FPR_tau) FPR_B (attributed in footnote 6 to Mitzenmacher);
  they set FPR_tau = FPR_B = p*/2 so FPR_O <= p*. FPR is measured on held-out non-keys,
  i.e. defined relative to a query distribution ("we follow the notion that we want to
  provide a specific FPR for realistic queries").
- §5.2 phishing URLs (1.7M keys): standard Bloom filter at 1% FPR = 2.04 MB; GRU (16-dim, 32-dim
  char embedding) = 0.0259 MB; tau for 0.5% FPR on validation gives FNR 55% (test FPR
  0.4976%); model + spillover filter = 1.31 MB, "a 36% reduction". At 0.1% FPR: FNR 76%,
  3.06 MB -> 2.59 MB, "a 15% reduction". Covariate shift: random-URL-only negatives save 60%,
  whitelisted-only negatives save 21% (both at 1% FPR).
- Appendix E (model-hash variant, §5.1.2): p* = 0.1%, m = 1,000,000 -> 2.21 MB, 27.4% reduction
  (vs 15%); p* = 1% -> 1.19 MB, 41% reduction (vs 36%).

Corrections to earlier claims
- "36% vs 47%": 36% is right (1% FPR). No "47%" appears in this paper (nor in the
  Mitzenmacher or PLBF papers). The other figures are 15% (0.1% FPR), and 41% / 27.4% for the
  model-hash variant in Appendix E. Drop 47%.
- The abstract says "up to 70% in speed" and "an order-of-magnitude in memory"; the 1.5-3x /
  two orders figure is §3.7.1 for specific configurations. Do not mix them.
- The min/max error is stored per last-stage model and is an empirical bound over the stored
  keys only; it is not a generalization guarantee.
- The hybrid B-tree fallback is a design (Algorithm 1) not evaluated in the headline results.

Limitations stated: read-only, in-memory dense sorted array; inserts / paging left to appendix;
FPR of the learned Bloom filter depends on query distribution matching historical queries.

## 2. Mitzenmacher, "A Model for Learned Bloom Filters, and Optimizing by Sandwiching", NeurIPS 2018 (arXiv 1901.00902v1)

FULL TEXT READ.

- Definition 1 (§3.1): learned Bloom filter = (f, tau, B), backup filter B holds
  {z in K : f(z) < tau}.
- §3.2: FPR is "highly dependent on the query set, and is not well-defined independently of the
  queries". Example: 500 keys in [1000,2000] + 500 elsewhere in [0,10^6): uniform queries give
  ~0.0002 (+ backup -> ~0.0004), queries uniform over [0,100000) give 0.0022.
  Definition 2: FPR w.r.t. a query distribution D:
  Pr_{y~D}(f(y) >= tau) + (1 - Pr_{y~D}(f(y) >= tau)) F(B).
  Theorem 4: empirical FPRs X on test set T and Y on query set Q, both drawn from D, satisfy
  Pr(|X - Y| >= eps) <= e^{-Omega(eps^2 min(|T|,|Q|))} (Chernoff). Only guidance if test and
  query distributions match.
- §3.3: insertions go into the backup filter; deletions are not natural; re-learning needs K.
- §4 size of the learned function: FPR = F_p + (1 - F_p) alpha^{b/F_n}; beats a Bloom filter of
  equal total bits when F_p + (1-F_p) alpha^{b/F_n} <= alpha^{b + zeta/m}, i.e.
  zeta/m <= log_alpha(F_p + (1-F_p) alpha^{b/F_n}) - b. alpha ~ 0.6185 (standard Bloom filter).
  Example F_p = 0.01, F_n = 0.5, 1 byte/key: Bloom filter 0.0214; a learned function of <= 3
  bits/key with 5m backup bits gives ~0.0181 (">10% reduction").
- §5 sandwich: initial Bloom filter (b1 m bits) -> f -> backup (b2 m bits).
  FPR = alpha^{b1} (F_p + (1 - F_p) alpha^{b2/F_n}).
  Optimum (eq. 2): b2* = F_n log_alpha( F_p / ((1 - F_p)(1/F_n - 1)) ), a constant independent
  of the total budget b: extra bits go to the initial filter. With F_n = 0.5, F_p = 0.01:
  b2* = (log_alpha 1/99)/2 ~ 6. At b = 8 bits/key FPR 0.010045 -> 0.005012; at b = 10,
  0.010066 -> 0.001917. Gain whenever zeta/m < ~3.36 bits/key (for those parameters).
  The sandwich also makes the structure more robust to query shift.
- §6 / §8 learned Bloomier filter (key -> value): oracle f predicts a value; a Bloom filter
  holds the keys f gets wrong (F_n m of them); hits go to a backup Bloomier filter, which must
  also take the (1-F_n) alpha^b correctly-predicted keys that falsely hit the Bloom filter:
  space zeta + b m F_n + c m (F_n + (1-F_n) alpha^b)(u + r'), FPR F_p(1 - alpha^b) + alpha^b 2^{-r'}.
  "The cost per element in a Bloomier filter is rather high, because the value must be
  stored. Therefore if the false negatives ... can be driven down ... there should be space
  available to pay the zeta bits of the learned function".

Corrections: none to the sandwich formula; note the closed form needs alpha, F_p, F_n treated
as constants and ignores the model size in the b2* optimization. The Bloomier variant (value
retrieval with an exception list) is the closest formal analogue to "model returns a card's
constituents, exceptions stored exactly", and it was not cited before.

## 3. Vaidya, Knorr, Mitzenmacher, Kraska, "Partitioned Learned Bloom Filters", ICLR 2021 (arXiv 2006.03176v2, Oct 2020)

FULL TEXT READ (arXiv v2; the ICLR camera-ready may differ in wording/figures).

- §3.2: score space split into k regions [t_{i-1}, t_i] with their own backup FPR f_i;
  minimize sum_i |S| (G(t_i) - G(t_{i-1})) c log2(1/f_i) + model size subject to
  sum_i (H(t_i) - H(t_{i-1})) f_i <= F, f_i <= 1 (G: key score CDF, H: non-key query score CDF).
- §3.3.1 (relaxed problem, f_i <= 1 dropped, KKT, Appendix A): f_i = F (G(t_i)-G(t_{i-1})) /
  (H(t_i)-H(t_{i-1})) (eq. 7). Substituting gives space
  c|S| log2(1/F) - c|S| D_KL(g_hat, h_hat) + model size (eq. 9; the printed eq. 9 drops a c),
  so savings over a Bloom filter = c |S| D_KL(g_hat, h_hat) - model size (eq. 10).
  This is a derivation, not a stated "Theorem", and holds for the relaxed problem; it is also
  the optimum of the general problem when F (G diff)/(H diff) < 1 in every region (§3.3.3).
  Thresholds maximizing D_KL found by DP in O(N^2 k) (§3.3.2); KL tried as a training loss
  without significant gain over MSE.
- §4.1 (random forest, k = 5, model size = pickle size): at equal space (400 Kb, 500 Kb,
  3000 Kb) PLBF has 22x, 26x, 3x lower FPR than sandwiching and 8.5x, 9x, 1.9x lower than
  AdaBF on synthetic, URLs, EMBER. At FPR 0.001 sandwiching uses 8.8x, 3.3x, 1.2x and AdaBF
  6x, 2.5x, 1.1x the space of PLBF. F1 of the models: 0.99, 0.97, 0.85. §4.2: 4-6 regions
  suffice; 25 regions give no noticeable further gain.

Correction: phrase as "under the relaxation, the maximum space saving is linearly proportional
to the KL divergence of the key and non-key score distributions over the regions (eq. 10)",
not as a theorem.

## 4. Ferragina & Vinciguerra, "The PGM-index", PVLDB 13(8):1162-1175, 2020

FULL TEXT READ, both arXiv 1910.06169v1 (Oct 2019, titled "The PGM-index: a multicriteria,
compressed and learned approach to data indexing") and the PVLDB version
(vldb.org/pvldb/vol13/p1162-ferragina.pdf, titled "The PGM-index: a fully-dynamic compressed
learned index with provable worst-case bounds"). Cite the PVLDB title; its numbers differ.

- Definition 1 (PVLDB): segment s is epsilon-approximate on [k_i, k_{i+r}] if
  |f_s(x) - rank(x)| <= epsilon for ALL x in U with k_i <= x <= k_{i+r} (not just stored keys).
- Lemma 1: optimal (minimum number of segments) piecewise linear epsilon-approximation in
  O(n) time and space, via the streaming convex-hull algorithm of O'Rourke (1981). Lemma 2:
  every segment covers at least 2 epsilon keys.
- Recursive construction: build the optimal PLA on the keys, take each segment's first key,
  build the optimal PLA on those, until one segment. Query = one binary search in a 2 epsilon
  window per level. Theorem 1: Theta(m) space, O(log m) time, O(log_c m) I/Os (c >= 2 epsilon),
  m = minimum number of epsilon-approximate segments.
- §4 distribution-aware PGM: per-key y-range min{1/p_i, epsilon}; O(m) space, O(H) average
  query time (H = entropy of the query distribution) (Theorem 2 in v1).
- Tolerance-first: v1 §6: the PGM-index, "instead of fixing the structure beforehand and
  inspecting the errors afterwards, ... is dynamically and optimally adapted to the input data
  distribution while guaranteeing the desired epsilon-approximation". With epsilon = 512 it
  needed ~32K segments, MAE 226 +- 139, vs RMI with as many second-stage models MAE
  892 +- 3729 (3.9x more).

Numbers
- arXiv v1 abstract: improves FITing-tree space by 63.3% and B-tree "by more than four orders
  of magnitude". Real datasets: 37.7%-63.3% fewer segments than FITing-tree's greedy PLA;
  synthetic: 20.8%-69.4%.
- PVLDB abstract: "improves the space occupancy of existing traditional and learned indexes by
  up to three orders of magnitude"; static: matches a cache-optimised B+-tree's query
  performance in "two orders of magnitude (83x) less space"; dynamic: "improves the query and
  update time performance of a B+-tree by up to 71% within three orders of magnitude (1140x)
  less space". Figure: fastest CSS-tree 341 MiB matched by a 4 MiB PGM (83x); fastest B+-tree
  874 MiB matched by an 87 KiB PGM (four orders of magnitude).

Correction: if the earlier note quoted 63.3% / "four orders of magnitude" as the PVLDB
abstract, that is the 2019 preprint; the published abstract says "up to three orders of
magnitude", 83x and 1140x.

## 5. Nathan, Ding, Alizadeh, Kraska, "Learning Multi-dimensional Indexes" (Flood), SIGMOD 2020 (arXiv 1912.01668v1)

FULL TEXT READ.

- §3.2: grid over d-1 dimensions (columns placed at equal steps of each dimension's learned
  CDF, "flattening") plus a sort dimension inside cells. Query: (1) projection to intersecting
  cells, (2) refinement of each cell's range on the sort dimension, (3) scan: "scan and process
  the records that match the filter". Goal: "find and process exactly the points within this
  hyper-rectangle". Exactness comes from filtering every scanned record against the predicate;
  the learned parts only decide how much is scanned (speed, never correctness).
- §5.2 refinement model: a piecewise linear CDF model per cell, constrained to be a lower
  bound P(v) <= D(v), with average absolute error per segment below delta; greedy slicing;
  local search corrects mispredictions. "it is difficult to build an RMI with a target error
  bound" (why they chose the PLM).
- Layout learned from a sample query workload with a learned cost model.
- Numbers: "up to three orders of magnitude faster" than other multi-dimensional indexes; sales
  dataset 3x over a tuned clustered column index and 72x over Redshift's Z-encoding; TPC-H
  workload 61x faster than the clustered column index, 3x faster than Z-encoding.
- §6: multi-dimensional CDFs not used: hard to keep monotone; conditional CDFs did not help
  significantly but grew the index.

## 6. Ding, Nathan, Alizadeh, Kraska, "Tsunami: A Learned Multi-dimensional Index for Correlated Data and Skewed Workloads", PVLDB 14(2), 2020 (arXiv 2006.13282v1)

FULL TEXT READ (main sections). Flood's successor. A Grid Tree (a small decision tree that
partitions space by query skew) has an Augmented Grid in each region; Augmented Grids replace
Flood's independent CDFs with functional mappings F: Y -> X for monotonically correlated
dimensions, stored with error bounds so that a range in Y maps to a guaranteed range in X
(§5.2.1), and conditional CDFs CDF(Y|X) for generic correlations. As in Flood, scanned points
are filtered exactly. Abstract: up to 6x faster queries and up to 8x smaller index than Flood;
up to 11x faster and 170x smaller than optimally tuned traditional indexes. Transferable idea:
an error-bounded mapping is used conservatively (widen by the bound, then filter exactly).

## 7. Li, Lu, Zheng, Yang, Pan, "LISA: A Learned Index Structure for Spatial Data", SIGMOD 2020

COULD NOT FETCH: dl.acm.org is blocked, q-zh.github.io unreachable from this sandbox
(connection refused at the proxy), and an arXiv search found no preprint (see §12). From
memory only, unverified: a learned mapping of spatial keys to 1-D with a shard prediction
function, supporting point/range/kNN queries and insertions/deletions. Do not cite numbers.

## 8. Hsu, Indyk, Katabi, Vakilian, "Learning-Based Frequency Estimation Algorithms", ICLR 2019

COULD NOT FETCH: openreview.net served a "Verifying your browser" challenge (HTTP 403) for both
the forum page and the PDF; arXiv search found no preprint (see §12). From memory only,
unverified: a learned heavy-hitter oracle routes predicted-heavy items to unique buckets
(exact counters) and the rest to a Count-Min / Count-Sketch; error bounds under Zipfian
frequencies; evaluated on internet traffic and search query data. This is the "learned oracle
+ exact exception storage + classical sketch for the rest" pattern. Do not cite numbers.

## 9. Liu, Fischer, Yoo, Ritschel, "Neural Bounding", SIGGRAPH 2024 Conference Papers (arXiv 2310.06822v5)

FULL TEXT READ.

- §3.1: cost per region r: 0 for TP/TN, alpha for FN, beta = 1 for FP, with alpha = infinity
  needed for conservativeness; intractable (infinite, discontinuous, zero gradients), so they
  use a dynamically weighted asymmetric loss (Algorithm 1) that ramps up FN weight over
  training; neural hierarchies and an early-exit variant (abstract: queries 25% faster).
- Query types: point (2D, 3D, 4D), ray, plane and box queries (§4); "two, three and 4D point
  queries, 2D and 3D range queries as well as queries of dynamic scenes, including scenes with
  multiple degrees of freedom" (§1). Also applied to fitting k-DOPs ("OurkDOP").
- Zero false negatives is EMPIRICAL, not proven. §4: "Because of numerics ... might be
  non-conservative on a hidden test set"; they add an epsilon to outputs and "find fewer than
  1 FN in 100 million queries on hidden test sets". Limitations: "our method neither provides
  an algorithmic guarantee to have no FNs ... Instead, we heuristically incentivize
  conservativeness"; "Our results are only almost always conservative ... we lack any proof".
  Per-object training; slower to fit than closed-form bounds.
- Abstract: "up to an order of magnitude fewer false positives than traditional methods".
  Timing table: OurNN 70.82 ms vs AABox 2.58 ms per batch (throughput 0.14 vs 3.88) — the
  network test is far slower per query than a box test.

Correction: the abstract's "ensuring that the number of false negatives is strictly zero" is a
design goal; the paper itself says it is not guaranteed.

## 10. Ludwig, Campen, Ritschel et al., "Strictly Conservative Neural Implicits", Pacific Graphics 2024

COULD NOT FETCH / NOT FOUND. dl.acm.org and Wiley (Computer Graphics Forum) are not reachable from
here, and arXiv has no matching preprint: ti:conservative AND ti:neural AND ti:implicit -> 0;
au:Campen AND au:Ritschel -> 0; abs:"conservative" AND abs:"neural implicit" AND
abs:"interval arithmetic" -> 0. Its content (from memory, UNVERIFIED: conservativeness
enforced by construction, e.g. range/interval bounds on the network, rather than by Neural
Bounding's empirical loss) cannot be checked here; cite it only after reading it, or leave it out.

## 11. Potamias, Ploumpis, Zafeiriou, "Neural Mesh Simplification", CVPR 2022, pp. 18583-18592 (openaccess.thecvf.com)

FULL TEXT READ.

- Three modules: Point Sampler (GNN "DevConv" scores vertices, sampled by an extension of
  random sampling), Edge Predictor (sparse attention; simplified adjacency A_s = S A S^T keeps
  the original topology), Face Classifier: candidate triangles from A_s get a prior
  p_init_t = (A_s[i,j] + A_s[i,k] + A_s[j,k]) / 3; three TriConv layers on a k-NN graph of
  triangle barycentres with relative position encodings, softmax, output p_t = probability the
  triangle is in the simplified mesh.
- Losses (§3.4): probabilistic Chamfer distance (eq. 6, weighting point distances by p);
  probabilistic surface distance (forward term to barycentres of the original surface, reverse
  term penalising low-probability regions that would leave holes); triangle collision
  (penalty proportional to faces penetrated), edge crossings, overlap.
- The probabilities are a training relaxation: at inference "produced by selecting only the
  faces with a probability above 0.5", then for each edge the two incident faces with highest
  probability (manifold). Not used as a sampling distribution at render time.
- Data: TOSCA (80 meshes), same split as their earlier work. Table 1 at 5/10/20/50%:
  proposed CD 1.02/0.42/0.19/0.06, QEM 0.45/0.22/0.11/0.05; NE 0.19/0.15/0.11/0.06 vs QEM
  0.14/0.10/0.07/0.03; non-watertight edges 2.17-3.57% vs QEM 0.00; time 4.1-4.4 vs QEM
  45.6/31.1/28.4/25.6 (units as printed). "up to 10x faster than the optimized QEM method and
  at least 100x faster than its faster differentiable counterpart". QEM is better on every
  appearance metric except Laplacian error. Limitation: topology/manifoldness "can not be
  explicitly guaranteed".

Correction: "per-triangle probability" is right, but it is an inclusion probability for a
one-shot thresholded selection, not a stochastic LOD; the method loses to QEM on accuracy and
wins on speed and differentiability.

## 12. arXiv searches (export.arxiv.org API)

Phrase queries checked against known papers: ti:"case for learned index structures" -> 1712.01208;
ti:"neural bounding" -> 2310.06822, so zero counts below are real zeros for these phrases
(arXiv metadata only, not full text; ACM-only and journal-only papers are not covered).

Zero hits (negatives):
- abs:"learned index" AND abs:"ray tracing" / AND abs:BVH / AND abs:mesh / AND abs:"point cloud"
  / AND abs:spatial AND abs:"bounding volume" / AND abs:"level of detail"
- abs:"learned bloom filter" AND abs:rendering / AND abs:geometry / AND abs:spatial
- abs:"neural" AND abs:"primitive" AND abs:"stochastic" AND abs:"level of detail"
- abs:"neural" AND abs:"aggregate" AND abs:"level of detail" AND abs:foliage
- abs:"neural" AND abs:"mesh" AND abs:"level of detail" AND abs:"ray tracing"
- abs:"stochastic simplification" AND abs:"aggregate"
- ti:"learning-based frequency estimation" (Hsu et al. has no arXiv version);
  ti:LISA AND ti:spatial AND ti:learned (LISA has no arXiv version)
- "learned alias table": abs:"alias table" AND abs:neural -> 1 unrelated hit (NLP entity
  aliases); abs:"alias method" AND abs:learned -> 1 hit (a GAN recommender that uses alias
  sampling, not a learned alias table). Nothing on learned alias tables.
- abs:"learned sketch" AND abs:sampling -> 2 hits, both about drawings, irrelevant.

Hits that are relevant (abstracts read, full texts NOT read):
- Figueiredo, He, Bako, Kalantari, "Neural Importance Sampling of Many Lights", arXiv
  2505.11729 (2025). The closest thing found to "learned distribution over a discrete set of
  primitives": a network predicts a light-selection distribution per shading point, trained
  online by minimizing KL to the target; for hundreds or thousands of lights it predicts the
  distribution over CLUSTERS of a light hierarchy, and existing methods sample lights within
  clusters; residual learning on top of an existing distribution. Pattern: learned top level,
  exact bottom level, and the lights themselves are still stored (it is a proposal, not a
  replacement for the data).
- Fujieda, Kao, Harada, "Neural Intersection Function", arXiv 2306.07191 (2023): an MLP
  answers secondary-ray visibility instead of traversing the lower BVH; up to 35% less
  secondary-ray time for direct illumination. "LSNIF: Locally-Subdivided Neural Intersection
  Function", arXiv 2504.21627 (2025): replaces bottom-level BVHs with a network (sparse hash
  grid) outputting visibility, hit point and material index; "memory footprint reduction of up
  to 106.2x compared to a compressed BVH". Closest to "replace stored primitives by a learned
  structure in ray tracing", but answers are approximate (no exactness guarantee in the
  abstract) and per object.
- Litalien et al., "Neural Product Importance Sampling via Warp Composition", arXiv 2409.18974
  (2024): normalizing flows for products of environment light and BSDF, a small neural head
  warp composed with a large tabulated (discretized) tail warp.
- Discrete flows exist in ML (e.g. "Reliable Categorical Variational Inference with Mixture of
  Discrete Normalizing Flows", 2006.15568) but nothing applies them to choosing primitives.
- Also found: "Deep Appearance Prefiltering" (2211.05932) and NeuMIP (2104.02789), learned
  appearance LOD; not about keeping or sampling constituents.

Conclusion of the search: no paper was found that combines learned indexes, learned Bloom/
Bloomier filters or learned sketches with BVHs, mesh LOD, or sampling primitives in rendering.
The nearest are neural many-light sampling (a learned proposal over a stored discrete set),
neural intersection functions (learned replacement of stored geometry, approximate), and
Neural Bounding (learned conservative bounds, empirical).

## Also: Müller, McWilliams, Rousselle, Gross, Novák, "Neural Importance Sampling", ACM TOG 38(5) 2019 (arXiv 1808.03856v5)

FULL TEXT READ (method sections). Requirements stated in §1: sample generation must be fast and
"given a sample X_i, we require an efficient and exact evaluation of its corresponding
probability density q(X_i) — a necessity for evaluating the unbiased estimator" F = E[f/q].
NICE-style coupling layers with piecewise-linear / piecewise-quadratic warps give tractable
(triangular) Jacobians, so both sampling and exact density evaluation; one-blob input
encoding; KL or chi^2 optimized from unnormalized MC estimates; for path guiding, one-sample
MIS with the BSDF with a learned selection probability c (§6), which also keeps the density
non-zero wherever the BSDF is. Continuous domains only; nothing on discrete sets.

## Also: Yang et al., "Deep Unsupervised Cardinality Estimation" (Naru), PVLDB 13(3) 2019 (arXiv 1905.04278v2)

FULL TEXT READ (sections 1-6). A deep autoregressive model (MADE / Transformer) factorizes the
table's joint exactly by the product rule, P(A1)P(A2|A1)..., trained by maximum likelihood
(cross-entropy over the tuples, eq. 2); the KL "entropy gap" H(P, P_hat) - H(P), in bits per
tuple, is the monitoring metric. Range queries by progressive sampling (Algorithm 1): sample
column by column from the model's conditionals restricted to the query region, multiply the
in-region masses; Theorem 1: unbiased (with respect to the model's distribution). Budgets:
DMV 11.6M rows, 13 MB (1.3% of the table); Conviva-A 4.1M rows, 3 MB (0.7%). Abstract:
"single-digit multiplicative error at tail, an up to 90x accuracy improvement over the second
best method". Table 6: entropy gap 4.23 / 2.25 / 1.01 / 0.84 bits per tuple for 0.6 / 1.1 /
2.7 / 3.8 MB models. Limitation: the estimate is exact only for the model's distribution; any
entropy gap is model error that sampling does not remove.

---------------------------------------------------------------------------------------------

## Synthesis: what transfers to a learned LOD that must give constituents with the right probabilities

What a card has to deliver. A hit picks leaf i with p(i | d) = a_i |n_i . d| / sum_j a_j |n_j . d|
and then shades with that leaf's normal and face state. So the payload is a distribution over
(normal, face state) that depends on d, a continuous 2D query. A "far smaller structure" must
reproduce that distribution, not a membership bit or a position. The nearest data-structure
analogue is therefore the learned Bloomier filter (Mitzenmacher §6/§8: key -> value with an
exception list), not the learned Bloom filter or the range index; and its own conclusion
applies: values are the expensive part, so a model pays only if it gets most values right.

Principles that transfer

1. Model + exact exception list (Kraska §3.3 hybrid; learned Bloom/Bloomier backup filters;
   Hsu et al.'s exact counters for predicted heavy hitters, unverified here). Per card: store a
   compact model when it reproduces the card's distribution within tolerance, otherwise keep
   the explicit constituents (the hybrid RMI's "replace the model by a B-tree if max error >
   threshold", Algorithm 1). The worst case is bounded by today's memory, as the hybrid RMI is
   bounded by the B-tree.

2. Error bounds measured exhaustively on static data (Kraska §2: bounds "only over the stored
   keys", found by running the model on every key). Cards are built once from a fixed set of
   leaves, so the error of every card's model can be measured at build time against its exact
   constituents. This is the strongest point of the analogy: there is no unseen data, unlike a
   Bloom filter's queries. The catch is the continuous query d: the error must hold over all
   directions (camera, shadow and bounce rays all meet the same card, docs/adaptive-meshes.md),
   so measure it over the sphere (dense sampling, or bounds, since a|n.d| is piecewise linear
   in d), not over camera directions. Mitzenmacher §3.2 shows what goes wrong otherwise: error
   measured on one query distribution says little about another.

3. Tolerance first, structure second (PGM-index: "instead of fixing the structure beforehand and
   inspecting the errors afterwards", fit the minimum number of pieces meeting epsilon; it beat
   RMI's MAE 3.9x at equal model count). Choose the tolerance (e.g. a maximum KL or total
   variation between the model's and the true p(normal, state | d)), then fit each card's model,
   or split the card, until it meets the tolerance; cards that can't go to the exception list.
   Naru's "entropy gap" (KL in bits per tuple, Table 6) is a ready-made per-card metric.

4. Learned parts decide cost, exact parts decide correctness (Flood/Tsunami: learned layout and
   error-bounded mappings choose what to scan; every scanned record is filtered exactly).
   Transfers only partly: a learned proposal can choose WHICH constituents to fetch, but the
   exact filter/weight needs the constituents in memory, which removes the memory saving.

5. Support coverage, i.e. no false negatives (Bloom filters; Neural Bounding's FN = 0 goal).
   Every leaf state that shows some visible area must keep non-zero probability: a model that
   drops rare states (the few autumn or dead leaves, a second leaf material) biases colour
   in a way averaging can't fix. Store rare states exactly (a short list of (state, weight) per
   card) and model the common bulk; this is the heavy-hitter split turned round. Neural
   Bounding shows a loss alone gives only empirical coverage ("fewer than 1 FN in 100 million
   queries", "we lack any proof"); coverage should be by construction (explicit list, or a
   mixture with a uniform-over-states component), not by training.

6. Exact-pdf generative models enable unbiased reweighting (NIS §1: exact q(X) "a necessity for
   evaluating the unbiased estimator"; Naru Theorem 1). This holds when the target f can be
   evaluated and q is only a proposal (neural many-lights does exactly that, keeping the lights).
   Here the target p(i | d) is the stored data being discarded, so reweighting to it is
   impossible without keeping it. A model with an exact pdf is unbiased only with respect to
   itself; the gap to the true distribution is bias, to be bounded by (2)-(3), not removed.

7. Hierarchy: learned coarse level, exact fine level (RMI stages; neural many-lights: a network
   over clusters, exact sampling within clusters). For cards: model the distribution over a few
   groups (e.g. normal lobes x leaf-state classes) and keep a handful of representative exact
   constituents per group.

Where the analogy breaks

- Membership/position vs. payload. Learned indexes and Bloom filters save space because the
  answer (a position, a bit) is cheap and the model learns structure in the keys. Constituents
  are payload (normal + face state, 12 B); "learning" them means predicting them, so the
  render then shows the model's leaves, not the originals.
- The "learned" structure doesn't have to be neural. The per-card model the card already needs
  is a small parametric distribution: an SGGX (6 numbers, already cited in the doc) or a few
  lobes for normals, plus a short (state, weight) table. Neural Bounding's table shows
  why per-hit networks are a poor fit: 70.82 ms vs 2.58 ms for boxes on the same batch.
  The literature's lesson is the discipline (bounded error, exceptions, tolerance-first), not
  the network.
- Query distribution. A learned Bloom filter's FPR is defined only w.r.t. a query distribution
  (Mitzenmacher §3.2). The card's distribution must be right for every d and for every ray
  type, and a far card is seen from very few directions per pixel, so errors don't average out
  over queries as an FPR does.
- What error means. An index's error costs time (a longer search) and a Bloom filter's costs
  a wasted lookup, both corrected exactly afterwards. A wrong constituent distribution changes
  the picture and nothing corrects it afterwards: the error is bias in radiance.
- The budget. 32 x 12 B = 384 B per card, vs 40 B per card and the BVH share. Even a perfect
  model saves at most the 235 MB of constituents; Mitzenmacher's §4 accounting (model bits per
  key + exception bits per key vs. the plain structure) is the right way to decide whether a
  given model pays.

## Corrections to the earlier snippet-based pass (summary)
- Kraska URL result: 36% at 1% FPR (2.04 MB -> 1.31 MB, FNR 55%); 15% at 0.1% (3.06 -> 2.59 MB).
  There is no 47%. Appendix E's model-hash variant: 41% and 27.4%.
- Kraska abstract: "up to 70% in speed while saving an order-of-magnitude in memory"; the
  "1.5-3x faster, up to two orders of magnitude smaller" is §3.7.1.
- Kraska min/max error: empirical, per last-stage model, over stored keys only; valid for absent
  keys only if monotonic (footnote 2). Hybrid B-tree fallback: proposed (Algorithm 1), not in
  the reported results.
- Kraska's backup is called an "overflow Bloom filter" in §5.1.1 ("spillover" in §5.2).
- PLBF: the KL result is a derivation under a relaxation (eq. 7-10), not a theorem.
- PGM: the PVLDB title and numbers differ from the 2019 preprint (published abstract: up to
  three orders of magnitude, 83x static, 1140x and 71% dynamic; preprint: 63.3% vs FITing-tree,
  four orders vs B-tree).
- Neural Bounding: zero false negatives is empirical; the paper says it has no proof.
- Neural Mesh Simplification: the triangle probabilities are thresholded at 0.5 at inference,
  not sampled; QEM is more accurate on CD and normal error, the method is ~10x faster.
- Not verified (could not fetch): LISA (SIGMOD 2020), Hsu et al. (ICLR 2019, OpenReview's browser
  check), "Strictly Conservative Neural Implicits" (PG 2024; no arXiv version found).
