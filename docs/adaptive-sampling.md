# Adaptive sampling

With a fixed `supersamples`, every pixel takes the same number of samples, so smooth pixels
(clear sky, a lit wall) cost as much as noisy ones (glossy reflections, indirect light,
caustics). With `"adaptive"`, each pixel keeps sampling only until its noise is low enough, and
`supersamples` becomes the most any pixel takes:

```json
"supersamples": 256,
"adaptive": { "threshold": 0.02, "min_samples": 16 }
```

`"adaptive": 0.02` is short for a threshold with the default minimum (16), and `true` for a
threshold of 0.01. On the command line, `--noise-threshold 0.02` sets or overrides the
threshold (0 turns it off), and `--sample-map map.png` writes each pixel's samples relative to
the most any pixel took, so you can see where they went.

## How it decides

The implementation is in `src/adaptive.rs`, whose module comment has the details.

- Each pixel keeps the sum of its samples' squares as well as their sum, so its standard
  error can be estimated: s² = (Σx² − n m²) / (n − 1), SE² = s² / n (Lee, Redner and Uselton,
  SIGGRAPH 1985).
- That error is judged relative to the displayed value (after the exposure and the film's
  gain), as `--variance` measures noise: relMSE = mean over channels of SE² / (m² + 0.01). A
  pixel is converged when √relMSE is below `threshold`, so 0.01 asks for a 1% standard error.
- Each pixel's relMSE is averaged over its 3 × 3 neighbourhood, because an estimate from a
  few dozen samples is itself noisy. Then the neighbours of every unconverged pixel keep
  sampling too, so the edges of a noisy area aren't left behind (as in Cycles).
- Every pixel takes `min_samples` first. Stopping on a pixel that hasn't yet found a rare,
  bright path darkens it (Kirk and Arvo, SIGGRAPH 1991), and the minimum guards against that.

### Why not the "half image" estimate

Blender's Cycles, after Dammertz et al. (2009), compares the image with one made from every
other sample. That doesn't suit our Owen-scrambled Sobol' points: the first dimension of
sample k is k's bits reversed, so all the even samples lie in one half of the pixel. The two
images would then differ wherever the pixel isn't flat, whether or not it is noisy. The sum of
squares treats the samples as independent. For low-discrepancy points that overstates the
error of smooth integrands, such as antialiased edges, so the rule errs towards taking more
samples.

## Scheduling and determinism

Work is grouped into regions of `chunk_size` square. After each pass over a region
(`samples_per_chunk`² samples per pixel), the region's next pass is decided from its own pixels
alone (`RenderContext::next_pass`). Waiting passes are taken lowest layer first, so the image
fills in evenly. Sample k of a pixel is always the same sample, so:

- the image depends only on the seed, not on threads or scheduling;
- an interrupted render resumes exactly (`.accum`, now format `RAYSACC3` with the sums of
  squares; states from earlier versions start afresh);
- the threshold isn't part of the saved state's key, so a render can be carried on with a new
  threshold or more samples. The result then differs a little from a render made with those
  settings from the start, because pixels that stopped under the old settings changed their
  neighbours' later decisions.

The wavefront integrator renders whole passes over the image, so with `"adaptive"` the path
tracer is used instead.

## Measurements

`demo/benchmarks/glossy-sun.json` (480 × 240), 4 threads.

Noise measured with `--variance 4`, efficiency = 1 / (relMSE × time):

| Settings                         | Time   | relMSE  | Efficiency |
|----------------------------------|--------|---------|------------|
| uniform, 64 spp                  | 2.7 s  | 0.00143 | 256        |
| adaptive 0.04, ≤ 256 spp         | 4.7 s  | 0.00049 | 435        |
| adaptive 0.02, ≤ 256 spp         | 8.4 s  | 0.00033 | 366        |

Against a 2048 spp reference at equal time (display-space RMSE, mean of 3 seeds; the signed
error is the same as uniform's, so there's no sign of early-stopping bias):

| Settings                         | Time   | RMSE   |
|----------------------------------|--------|--------|
| uniform, 112 spp                 | 6.1 s  | 0.0058 |
| adaptive 0.015, ≤ 128 spp        | 6.1 s  | 0.0053 |
| uniform, 200 spp                 | 9.7 s  | 0.0045 |
| adaptive 0.02, ≤ 256 spp         | 9.6 s  | 0.0041 |

This scene is noisy almost everywhere below the horizon, so the gain is modest (about 9% less
error at equal time, worth roughly 1.2× the samples). Scenes with large smooth areas and small
difficult ones gain more.

## Literature

- M. E. Lee, R. A. Redner, S. P. Uselton, "Statistically optimized sampling for distributed
  ray tracing", SIGGRAPH 1985.
- D. Kirk, J. Arvo, "Unbiased sampling techniques for image synthesis", SIGGRAPH 1991.
- H. Dammertz, J. Hanika, A. Keller, H. Lensch, "A hierarchical automatic stopping condition
  for Monte Carlo global illumination", WSCG 2009.
- F. Rousselle, C. Knaus, M. Zwicker, "Adaptive sampling and reconstruction using greedy error
  minimization", SIGGRAPH Asia 2011; "Adaptive rendering with non-local means filtering",
  SIGGRAPH Asia 2012.
- M. Zwicker et al., "Recent advances in adaptive sampling and reconstruction for Monte Carlo
  rendering", Eurographics STAR 2015.
