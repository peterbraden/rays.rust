//! # Lens flare
//!
//! A bright light in front of a camera lights the lens as well as the image. Some of its light
//! reflects off one glass surface, back to another, and forward again to the film: a ghost, an
//! out-of-focus image of the aperture somewhere along the line from the light through the
//! image's centre, one for every pair of surfaces. And the aperture's edges diffract it into a
//! starburst of spikes. Both are drawn here after rendering, from the finished image, like the
//! glare (`glare`).
//!
//! ## Light sources
//!
//! Ghosts are faint, a few parts in a hundred thousand of the light, and only the very brightest
//! things make visible ones: the sun, lamps, a glint of the sun. Pixels brighter than
//! `threshold` (as displayed, after exposure) are grouped into connected regions, and each
//! region (up to `sources` of them, brightest first) is a light: its direction from its
//! brightness-weighted centre, its energy the sum of its pixels. Whatever is hidden behind
//! something doesn't reach the image, so doesn't flare.
//!
//! ## Ghosts
//!
//! After Matthias B. Hullin, Elmar Eisemann, Hans-Peter Seidel and Sungkil Lee,
//! "Physically-Based Real-Time Lens Flare Rendering", ACM Transactions on Graphics 30(4)
//! (SIGGRAPH 2011), doi:10.1145/2010324.1965003. For each pair of surfaces i < j (not the
//! stop), a grid of parallel rays from the light's direction, across the front element, is
//! traced through the lens system (`lens`): forward to j, where it reflects; back to i, where it
//! reflects again; and forward to the film. Each cell of the grid, mapped by its four corners'
//! rays, spreads its share of the light over the quadrilateral it lands on, so a ghost is as
//! bright as its rays are dense. Cells with a corner stopped by an aperture are dropped, which
//! shapes each ghost like the stop (round, or a polygon with blades). The light's energy is
//! shared between the grid's cells that reach the film directly, so the ghosts are in proportion
//! to the light as rendered.
//!
//! What each surface reflects is given by Fresnel's equations, averaged over polarisations, and
//! air-glass surfaces have a single-layer anti-reflection coating: a quarter wave of magnesium
//! fluoride (n = 1.38) at 550 nm, its reflectance found by thin-film interference (Born and
//! Wolf, "Principles of Optics", 7th ed., section 1.6.4). A coated surface reflects a fraction
//! of a percent in the green and more in the red and blue, which tints ghosts purple and
//! amber. Each colour channel is traced at its own wavelength (610, 550 and 465 nm), with glass
//! dispersion estimated from each glass's index (lens prescriptions give no Abbe number): an
//! Abbe number from the trend of the glass map, V = 70 - 60 (n - 1.45) within [25, 70], and
//! Cauchy's equation through n_d.
//!
//! ## Starburst
//!
//! A straight edge of the aperture diffracts light into a thin line perpendicular to it.
//! Far from the centre, along the line, a rectangle w across diffracts a fraction λ / (2π² w θ²)
//! of the light per radian at angle θ (the tail of the sinc² pattern, integrated across the
//! line); each edge of a bladed stop gives half that on each side, and parallel edges add, so a
//! stop of n blades makes n spikes if n is even and 2n if odd. Here w is the entrance pupil's
//! width across its flats. A round stop makes no spikes. Spikes are drawn one pixel wide from a
//! pixel out, for each pixel brighter than the threshold, by convolution, and longer for red
//! than for blue, as λ goes.
//!
//! Both are physical in strength by default; `ghosts` and `starburst` scale them.
use crate::color::Color;
use crate::glare::fft2d;
use crate::lens::{refract, intersect_spherical, LensSystem};
use crate::na::{Vector2, Vector3};
use crate::output::LinearImage;
use rustfft::num_complex::Complex;
use rayon::prelude::*;
use std::f64::consts::PI;

/// Wavelengths (nm) of the red, green and blue channels.
const WAVELENGTHS: [f64; 3] = [610., 550., 465.];

/// The anti-reflection coating: index, and the wavelength it's a quarter wave thick at.
const COATING_INDEX: f64 = 1.38;
const COATING_WAVELENGTH: f64 = 550.;

#[derive(Debug, Clone, PartialEq)]
pub struct Flare {
    /// The lens, focused, with its stop and blades.
    pub lens: LensSystem,
    /// The film's width and height, metres.
    pub film: (f64, f64),
    /// Pixels brighter than this, after exposure, are lights.
    pub threshold: f64,
    /// At most this many lights.
    pub sources: usize,
    /// Scales the ghosts and the starburst (zero for none).
    pub ghosts: f64,
    pub starburst: f64,
    /// Rays across the front element, per side.
    pub grid: usize,
}

/// A light found in the image: its direction in camera space and its energy (the sum of its
/// pixels).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Source {
    pub direction: Vector3<f64>,
    pub energy: Color,
}

/// Reflectance at an interface between media of index `n0` and `n2` (in the order the light
/// crosses them), at cosine `cos0` in the first, averaged over polarisations; with a coating
/// between them of index `n1` and thickness `d` (nm) if `coating`.
pub fn reflectance(cos0: f64, n0: f64, n2: f64, lambda: f64, coating: Option<(f64, f64)>) -> f64 {
    let sin0 = (1. - cos0 * cos0).max(0.).sqrt();
    let cos_in = |n: f64| -> Option<f64> {
        let s = n0 * sin0 / n;
        if s >= 1. { None } else { Some((1. - s * s).sqrt()) }
    };
    let cos2 = match cos_in(n2) {
        Some(c) => c,
        None => return 1.,
    };
    let rs = |ni: f64, ci: f64, nj: f64, cj: f64| (ni * ci - nj * cj) / (ni * ci + nj * cj);
    let rp = |ni: f64, ci: f64, nj: f64, cj: f64| (nj * ci - ni * cj) / (nj * ci + ni * cj);
    match coating {
        None => 0.5 * (rs(n0, cos0, n2, cos2).powi(2) + rp(n0, cos0, n2, cos2).powi(2)),
        Some((n1, d)) => {
            let cos1 = match cos_in(n1) { Some(c) => c, None => return 1. };
            let delta = 4. * PI * n1 * d * cos1 / lambda;
            let phase = Complex::new((-delta).cos(), (-delta).sin());
            let film = |r01: f64, r12: f64| {
                let r = (Complex::new(r01, 0.) + phase * r12) / (Complex::new(1., 0.) + phase * (r01 * r12));
                r.norm_sqr()
            };
            0.5 * (film(rs(n0, cos0, n1, cos1), rs(n1, cos1, n2, cos2)) + film(rp(n0, cos0, n1, cos1), rp(n1, cos1, n2, cos2)))
        },
    }
}

/// A glass's index at `lambda` nm from its index at the d line (see the module's notes).
fn dispersed(nd: f64, lambda: f64) -> f64 {
    if nd <= 1. {
        return nd;
    }
    let v = (70. - 60. * (nd - 1.45)).clamp(25., 70.);
    // n_F - n_C = (n_d - 1) / V = b (1/λ_F² - 1/λ_C²), λ in µm.
    let b = (nd - 1.) / v / (1. / 0.4861f64.powi(2) - 1. / 0.6563f64.powi(2));
    let a = nd - b / 0.5876f64.powi(2);
    a + b / (lambda * 1e-3).powi(2)
}

/// How a ray meets a surface on a ghost's path.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Step {
    Refract,
    Reflect,
}

impl Flare {
    /// The surfaces' positions along the axis, in lens space (film at 0, lens towards -z).
    fn positions(&self) -> Vec<f64> {
        let mut z = -self.lens.front_z();
        self.lens.interfaces.iter().map(|e| { let here = z; z += e.thickness; here }).collect()
    }

    /// The pairs of surfaces (i < j) that make ghosts: neither the stop.
    pub fn ghost_pairs(&self) -> Vec<(usize, usize)> {
        let n = self.lens.interfaces.len();
        let mut pairs = vec![];
        for j in 0..n {
            for i in 0..j {
                if !self.lens.interfaces[i].is_stop() && !self.lens.interfaces[j].is_stop() {
                    pairs.push((i, j));
                }
            }
        }
        pairs
    }

    /// The index of the medium before surface `k` (towards the scene) and after it, at `lambda`.
    fn media(&self, k: usize, lambda: f64) -> (f64, f64) {
        let e = &self.lens.interfaces;
        let before = if k == 0 || e[k - 1].eta == 0. { 1. } else { e[k - 1].eta };
        let after = if e[k].eta == 0. { 1. } else { e[k].eta };
        (dispersed(before, lambda), dispersed(after, lambda))
    }

    /// Trace a ray (camera space, from the scene) along the path for `ghost` (None: straight
    /// through) at `lambda`: where it lands on the film (camera space x, y) and the fraction of
    /// its light that gets there.
    fn trace(&self, z: &[f64], o: &Vector3<f64>, d: &Vector3<f64>, ghost: Option<(usize, usize)>, lambda: f64) -> Option<(Vector2<f64>, f64)> {
        let n = self.lens.interfaces.len();
        let mut path: Vec<(usize, bool, Step)> = Vec::with_capacity(n + 4);
        match ghost {
            None => path.extend((0..n).map(|k| (k, true, Step::Refract))),
            Some((i, j)) => {
                path.extend((0..j).map(|k| (k, true, Step::Refract)));
                path.push((j, true, Step::Reflect));
                path.extend((i + 1..j).rev().map(|k| (k, false, Step::Refract)));
                path.push((i, false, Step::Reflect));
                path.extend((i + 1..n).map(|k| (k, true, Step::Refract)));
            },
        }
        let (mut o, mut d) = (Vector3::new(o.x, o.y, -o.z), Vector3::new(d.x, d.y, -d.z).normalize());
        let mut weight = 1.;
        for (k, forward, step) in path {
            let e = &self.lens.interfaces[k];
            let (t, normal) = if e.is_stop() {
                if d.z.abs() < 1e-12 { return None; }
                ((z[k] - o.z) / d.z, Vector3::zeros())
            } else {
                intersect_spherical(e.radius, z[k] + e.radius, &o, &d)?
            };
            if t <= 0. {
                return None;
            }
            let p = o + d * t;
            if !self.lens.within(e, p.x, p.y) {
                return None;
            }
            o = p;
            if e.is_stop() {
                continue;
            }
            let (before, after) = self.media(k, lambda);
            let (ni, nt) = if forward { (before, after) } else { (after, before) };
            let cos = normal.dot(&-d).abs();
            let coating = (ni == 1. || nt == 1.).then_some((COATING_INDEX, COATING_WAVELENGTH / (4. * COATING_INDEX)));
            let r = reflectance(cos, ni, nt, lambda, coating);
            match step {
                Step::Reflect => {
                    d = d - normal * (2. * d.dot(&normal));
                    weight *= r;
                },
                Step::Refract => {
                    d = refract(&-d, &normal, nt / ni)?;
                    weight *= 1. - r;
                },
            }
        }
        if d.z <= 0. {
            return None;
        }
        let t = -o.z / d.z;
        Some((Vector2::new(o.x + d.x * t, o.y + d.y * t), weight))
    }

    /// The grid's rays for a light from camera-space `direction`: for each vertex of a
    /// (grid + 1)² lattice over the front element, where it lands and with what weight.
    fn lattice(&self, z: &[f64], direction: &Vector3<f64>, ghost: Option<(usize, usize)>, lambda: f64) -> Vec<Option<(Vector2<f64>, f64)>> {
        let g = self.grid;
        let r = self.lens.interfaces[0].aperture;
        let front = self.lens.front_z();
        let d = -direction.normalize();
        (0..(g + 1) * (g + 1)).map(|k| {
            let (i, j) = (k % (g + 1), k / (g + 1));
            let p = Vector3::new(-r + 2. * r * i as f64 / g as f64, -r + 2. * r * j as f64 / g as f64, front);
            // Start a little in front of the lens, back along the ray.
            self.trace(z, &(p - d * 0.01), &d, ghost, lambda)
        }).collect()
    }

    /// Film (camera space) to image position in pixels, (column, row from the top): the image
    /// is inverted on the film, as in `lens::RealisticCamera`.
    fn to_pixel(&self, f: &Vector2<f64>, w: usize, h: usize) -> Vector2<f64> {
        let u = 0.5 - f.x / self.film.0;
        let v = 0.5 - f.y / self.film.1;
        Vector2::new(u * w as f64, (1. - v) * h as f64)
    }

    /// Camera-space direction of image position (column, row) in pixels, by the film's own
    /// pinhole projection.
    fn direction_of(&self, x: f64, y: f64, w: usize, h: usize) -> Vector3<f64> {
        let focal = self.lens.focal_length().unwrap_or(0.05);
        let (u, v) = (x / w as f64, 1. - y / h as f64);
        Vector3::new((u - 0.5) * self.film.0 / focal, (v - 0.5) * self.film.1 / focal, 1.).normalize()
    }

    /// The lights in `img`: connected regions of pixels brighter than the threshold, at most
    /// `sources`, brightest first.
    pub fn find_sources(&self, img: &LinearImage, exposure: f64) -> Vec<Source> {
        let (w, h) = (img.width, img.height);
        let threshold = self.threshold / exposure.max(1e-300);
        let bright: Vec<bool> = img.pixels.iter().map(|c| c.rgb.max() > threshold).collect();
        let mut seen = vec![false; w * h];
        let mut sources = vec![];
        for start in 0..w * h {
            if !bright[start] || seen[start] {
                continue;
            }
            let (mut energy, mut centre, mut weight) = (Color::black(), Vector2::zeros(), 0.);
            let mut stack = vec![start];
            seen[start] = true;
            while let Some(i) = stack.pop() {
                let c = img.pixels[i];
                let l = crate::color::luminance(&c).max(0.);
                energy = energy + c;
                centre += Vector2::new((i % w) as f64 + 0.5, (i / w) as f64 + 0.5) * l;
                weight += l;
                let (x, y) = (i % w, i / w);
                let mut visit = |j: usize| if bright[j] && !seen[j] { seen[j] = true; stack.push(j); };
                if x > 0 { visit(i - 1); }
                if x + 1 < w { visit(i + 1); }
                if y > 0 { visit(i - w); }
                if y + 1 < h { visit(i + w); }
            }
            if weight > 0. {
                let c = centre / weight;
                sources.push(Source { direction: self.direction_of(c.x, c.y, w, h), energy });
            }
        }
        sources.sort_by(|a, b| crate::color::luminance(&b.energy).total_cmp(&crate::color::luminance(&a.energy)));
        sources.truncate(self.sources);
        sources
    }

    /// The ghosts of `source`, as an image of `w` x `h` (pixel-sum units, like the image's).
    pub fn ghosts_of(&self, source: &Source, w: usize, h: usize) -> Vec<Color> {
        let z = self.positions();
        let g = self.grid;
        let cell_ok = |lat: &[Option<(Vector2<f64>, f64)>], i: usize, j: usize| {
            [(i, j), (i + 1, j), (i + 1, j + 1), (i, j + 1)].iter().all(|(a, b)| lat[b * (g + 1) + a].is_some())
        };
        // The light's energy per cell that reaches the film directly.
        let direct = self.lattice(&z, &source.direction, None, WAVELENGTHS[1]);
        let reaching = (0..g * g).filter(|k| cell_ok(&direct, k % g, k / g)).count();
        if reaching == 0 {
            return vec![Color::black(); w * h];
        }
        let jobs: Vec<((usize, usize), usize)> = self.ghost_pairs().into_iter().flat_map(|p| (0..3).map(move |c| (p, c))).collect();
        jobs.par_iter().fold(|| vec![Color::black(); w * h], |mut out, &(pair, c)| {
            let lambda = WAVELENGTHS[c];
            let lat = self.lattice(&z, &source.direction, Some(pair), lambda);
            let per_cell = source.energy.rgb[c] * self.ghosts / reaching as f64;
            for j in 0..g {
                for i in 0..g {
                    if !cell_ok(&lat, i, j) {
                        continue;
                    }
                    let corner = |a: usize, b: usize| lat[b * (g + 1) + a].unwrap();
                    let q = [corner(i, j), corner(i + 1, j), corner(i + 1, j + 1), corner(i, j + 1)];
                    let energy = per_cell * q.iter().map(|(_, w)| w).sum::<f64>() / 4.;
                    if energy <= 0. {
                        continue;
                    }
                    let p: Vec<Vector2<f64>> = q.iter().map(|(f, _)| self.to_pixel(f, w, h)).collect();
                    splat_triangle(&mut out, w, h, c, &p[0], &p[1], &p[2], energy / 2.);
                    splat_triangle(&mut out, w, h, c, &p[0], &p[2], &p[3], energy / 2.);
                }
            }
            out
        }).reduce(|| vec![Color::black(); w * h], |a, b| a.iter().zip(&b).map(|(x, y)| *x + *y).collect())
    }

    /// The entrance pupil's diameter, and the stop's blades.
    fn pupil(&self) -> f64 {
        let focal = self.lens.focal_length().unwrap_or(0.05);
        let stop = self.lens.stop_diameter().unwrap_or(0.01);
        // The stop seen through the elements in front of it: as large as f / N for the
        // f-number the stop gives, N = f / (2 r_stop m); approximated by tracing a ray parallel
        // to the axis that just grazes the stop's edge.
        let z = self.positions();
        let mut lo = 0.;
        let mut hi = self.lens.interfaces[0].aperture;
        for _ in 0..40 {
            let mid = 0.5 * (lo + hi);
            let o = Vector3::new(mid, 0., self.lens.front_z() + 0.01);
            if self.trace(&z, &o, &Vector3::new(0., 0., -1.), None, WAVELENGTHS[1]).is_some() { lo = mid; } else { hi = mid; }
        }
        if lo > 0. { 2. * lo } else { stop.min(focal) }
    }

    /// The starburst of the bright pixels of `img`, added to it.
    fn starburst_of(&self, img: &LinearImage, exposure: f64, pixel_angle: f64) -> Vec<Color> {
        let (w, h) = (img.width, img.height);
        let (n, rotation) = match self.lens.blades {
            Some(b) if self.starburst > 0. && pixel_angle > 0. => b,
            _ => return vec![Color::black(); w * h],
        };
        let threshold = self.threshold / exposure.max(1e-300);
        let bright = |c: &Color| c.rgb.max() > threshold;
        if !img.pixels.iter().any(bright) {
            return vec![Color::black(); w * h];
        }
        let width = self.pupil() * (PI / n as f64).cos();
        let reach = w.max(h);
        let (pw, ph) = ((w + reach).next_power_of_two(), (h + reach).next_power_of_two());
        let mut out = vec![Color::black(); w * h];
        for (c, lambda) in WAVELENGTHS.iter().enumerate() {
            let lambda = lambda * 1e-9;
            // The spikes, centred at the origin and wrapped round.
            let mut kernel = vec![Complex::new(0., 0.); pw * ph];
            for k in 0..n {
                // Each edge's normal; the film's y runs up, the image's rows down.
                let a = rotation + (2 * k + 1) as f64 * PI / n as f64;
                for side in [1., -1.] {
                    let (dx, dy) = (side * a.cos(), -side * a.sin());
                    let mut r = 1.;
                    while r < reach as f64 {
                        let theta = r * pixel_angle;
                        // Per unit length along the spike; steps of half a pixel.
                        let v = lambda / (4. * PI * PI * width * theta * theta) * pixel_angle * 0.5 * self.starburst;
                        let (x, y) = (r * dx, r * dy);
                        let (x0, y0) = (x.floor(), y.floor());
                        let (fx, fy) = (x - x0, y - y0);
                        for (ox, oy, wgt) in [(0., 0., (1. - fx) * (1. - fy)), (1., 0., fx * (1. - fy)), (0., 1., (1. - fx) * fy), (1., 1., fx * fy)] {
                            let (ix, iy) = (((x0 + ox) as i64).rem_euclid(pw as i64) as usize, ((y0 + oy) as i64).rem_euclid(ph as i64) as usize);
                            kernel[iy * pw + ix].re += v * wgt;
                        }
                        r += 0.5;
                    }
                }
            }
            fft2d(&mut kernel, pw, ph, false);
            let mut source = vec![Complex::new(0., 0.); pw * ph];
            for y in 0..h {
                for x in 0..w {
                    let p = img.pixels[y * w + x];
                    if bright(&p) {
                        source[y * pw + x].re = p.rgb[c];
                    }
                }
            }
            fft2d(&mut source, pw, ph, false);
            for (s, k) in source.iter_mut().zip(&kernel) {
                *s *= k;
            }
            fft2d(&mut source, pw, ph, true);
            let size = (pw * ph) as f64;
            for y in 0..h {
                for x in 0..w {
                    out[y * w + x].rgb[c] += (source[y * pw + x].re / size).max(0.);
                }
            }
        }
        out
    }

    /// `img` with its lights' ghosts and starbursts.
    pub fn apply(&self, img: &LinearImage, exposure: f64, pixel_angle: f64) -> LinearImage {
        let (w, h) = (img.width, img.height);
        let mut pixels = img.pixels.clone();
        if self.ghosts > 0. {
            let mut ghosts = vec![Color::black(); w * h];
            for s in self.find_sources(img, exposure) {
                for (a, g) in ghosts.iter_mut().zip(self.ghosts_of(&s, w, h)) {
                    *a = *a + g;
                }
            }
            // Rasterising cells leaves a faint mesh where they meet; ghosts are smooth.
            for (p, g) in pixels.iter_mut().zip(blur(&ghosts, w, h)) {
                *p = *p + g;
            }
        }
        for (p, s) in pixels.iter_mut().zip(self.starburst_of(img, exposure, pixel_angle)) {
            *p = *p + s;
        }
        LinearImage { width: w, height: h, pixels }
    }
}

/// A Gaussian blur of one pixel's standard deviation, separable, keeping the light that would
/// leave the image's edges at them.
fn blur(img: &[Color], w: usize, h: usize) -> Vec<Color> {
    let k: Vec<f64> = (-3..=3).map(|i: i64| (-(i * i) as f64 / 2.).exp()).collect();
    let total: f64 = k.iter().sum();
    let pass = |src: &[Color], horizontal: bool| -> Vec<Color> {
        let mut out = vec![Color::black(); w * h];
        for y in 0..h {
            for x in 0..w {
                let mut acc = Color::black();
                for (o, kv) in (-3..=3).zip(&k) {
                    let (sx, sy) = if horizontal {
                        ((x as i64 + o).clamp(0, w as i64 - 1) as usize, y)
                    } else {
                        (x, (y as i64 + o).clamp(0, h as i64 - 1) as usize)
                    };
                    acc = acc + src[sy * w + sx] * (kv / total);
                }
                out[y * w + x] = acc;
            }
        }
        out
    };
    pass(&pass(img, true), false)
}

/// Spread `energy` evenly over the pixels whose centres lie in triangle abc (in pixels), in
/// channel `c`; a triangle that covers no pixel centre puts it all in the pixel under its
/// centroid.
#[allow(clippy::too_many_arguments)]
fn splat_triangle(out: &mut [Color], w: usize, h: usize, c: usize, a: &Vector2<f64>, b: &Vector2<f64>, d: &Vector2<f64>, energy: f64) {
    let cross = |o: &Vector2<f64>, p: &Vector2<f64>, q: &Vector2<f64>| (p.x - o.x) * (q.y - o.y) - (p.y - o.y) * (q.x - o.x);
    let area = cross(a, b, d);
    let centroid = (a + b + d) / 3.;
    let put = |out: &mut [Color], x: i64, y: i64, e: f64| {
        if x >= 0 && y >= 0 && (x as usize) < w && (y as usize) < h {
            out[y as usize * w + x as usize].rgb[c] += e;
        }
    };
    if area.abs() < 1. {
        put(out, centroid.x.floor() as i64, centroid.y.floor() as i64, energy);
        return;
    }
    let (x0, x1) = (a.x.min(b.x).min(d.x).floor().max(0.) as i64, a.x.max(b.x).max(d.x).ceil().min(w as f64) as i64);
    let (y0, y1) = (a.y.min(b.y).min(d.y).floor().max(0.) as i64, a.y.max(b.y).max(d.y).ceil().min(h as f64) as i64);
    let inside = |x: i64, y: i64| {
        let p = Vector2::new(x as f64 + 0.5, y as f64 + 0.5);
        let (e0, e1, e2) = (cross(a, b, &p), cross(b, d, &p), cross(d, a, &p));
        (e0 >= 0. && e1 >= 0. && e2 >= 0.) || (e0 <= 0. && e1 <= 0. && e2 <= 0.)
    };
    // The triangle's whole area in pixels, so light landing off the image is lost, not
    // crowded onto the part inside it.
    let covered = area.abs() / 2.;
    let mut any = false;
    for y in y0..y1 {
        for x in x0..x1 {
            if inside(x, y) {
                put(out, x, y, energy / covered);
                any = true;
            }
        }
    }
    if !any {
        put(out, centroid.x.floor() as i64, centroid.y.floor() as i64, energy);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flare(blades: Option<usize>) -> Flare {
        let mut lens = LensSystem::named("dgauss.50mm").unwrap();
        lens = lens.clone().with_film_at(lens.focus(1e4).unwrap());
        if let Some(n) = blades {
            lens = lens.with_blades(n, 0.);
        }
        Flare { lens, film: (0.036, 0.024), threshold: 10., sources: 4, ghosts: 1., starburst: 1., grid: 48 }
    }

    #[test]
    fn test_reflectance() {
        // Uncoated glass, at normal incidence: ((n - 1) / (n + 1))², 4% for n = 1.5.
        assert!((reflectance(1., 1., 1.5, 550., None) - 0.04).abs() < 1e-9);
        // A quarter-wave coating of index 1.38 on n = 1.5 at its design wavelength:
        // ((n0 n2 - n1²) / (n0 n2 + n1²))² = 1.3%.
        let d = COATING_WAVELENGTH / (4. * COATING_INDEX);
        let r = reflectance(1., 1., 1.5, 550., Some((COATING_INDEX, d)));
        let expected = ((1.5 - 1.38f64.powi(2)) / (1.5 + 1.38f64.powi(2))).powi(2);
        assert!((r - expected).abs() < 1e-9, "{} vs {}", r, expected);
        // More away from it, at both ends of the spectrum.
        assert!(reflectance(1., 1., 1.5, 420., Some((COATING_INDEX, d))) > r);
        assert!(reflectance(1., 1., 1.5, 700., Some((COATING_INDEX, d))) > r);
        // Total internal reflection.
        assert_eq!(reflectance(0.1, 1.5, 1., 550., None), 1.);
    }

    #[test]
    fn test_dispersion_estimate() {
        for nd in [1.5, 1.67, 1.8] {
            assert!((dispersed(nd, 587.6) - nd).abs() < 1e-3);
            assert!(dispersed(nd, 450.) > dispersed(nd, 650.));
        }
        assert_eq!(dispersed(1., 450.), 1.);
    }

    #[test]
    fn test_direct_path_images_the_light() {
        // Straight through, a distant light 5° off axis lands at about f tan 5° on the opposite
        // side, which is where the image shows it.
        let f = flare(None);
        let z = f.positions();
        let a = 5f64.to_radians();
        let dir = Vector3::new(a.sin(), 0., a.cos());
        let lat = f.lattice(&z, &dir, None, 550.);
        let hits: Vec<Vector2<f64>> = lat.iter().flatten().map(|(p, _)| *p).collect();
        assert!(hits.len() > 100);
        let mean = hits.iter().sum::<Vector2<f64>>() / hits.len() as f64;
        let focal = f.lens.focal_length().unwrap();
        assert!((mean.x + focal * a.tan()).abs() < 0.03 * focal * a.tan(), "{} vs {}", mean.x, -focal * a.tan());
        let px = f.to_pixel(&mean, 300, 200);
        let back = f.direction_of(px.x, px.y, 300, 200);
        assert!((back - dir).norm() < 0.01, "{} vs {}", back, dir);
    }

    /// Ghosts are faint, and their light is a small part of the light's, on the line through
    /// the image's centre.
    #[test]
    fn test_ghosts_are_faint_and_on_the_axis_line() {
        let f = flare(None);
        let (w, h) = (300, 200);
        let source = Source { direction: Vector3::new(0.15, 0., 1.).normalize(), energy: Color::new(1e6, 1e6, 1e6) };
        let g = f.ghosts_of(&source, w, h);
        let total: f64 = g.iter().map(|c| c.rgb.y).sum();
        assert!(total > 0. && total < 0.01 * 1e6, "ghost light {}", total);
        // A light on the horizontal through the centre makes ghosts centred on that line: their
        // light is symmetric about it.
        let mean_row: f64 = g.iter().enumerate().map(|(i, c)| ((i / w) as f64 + 0.5) * c.rgb.y).sum::<f64>() / total;
        assert!((mean_row - 100.).abs() < 1., "ghosts centred on row {}", mean_row);
    }

    #[test]
    fn test_sources_are_found_and_ordered() {
        let (w, h) = (60, 40);
        let mut pixels = vec![Color::new(0.1, 0.1, 0.1); w * h];
        for (x, y, v) in [(10, 10, 1e3), (11, 10, 1e3), (40, 30, 5e4)] {
            pixels[y * w + x] = Color::new(v, v, v);
        }
        let img = LinearImage { width: w, height: h, pixels };
        let s = flare(None).find_sources(&img, 1.);
        assert_eq!(s.len(), 2);
        assert!((s[0].energy.rgb.x - 5e4).abs() < 1e-9 && (s[1].energy.rgb.x - 2e3).abs() < 1e-9);
        // Below and right of the centre.
        assert!(s[0].direction.x > 0. && s[0].direction.y < 0.);
    }

    #[test]
    fn test_starburst_has_spikes_for_blades() {
        let (w, h) = (64, 64);
        let mut pixels = vec![Color::black(); w * h];
        pixels[32 * w + 32] = Color::new(1e8, 1e8, 1e8);
        let img = LinearImage { width: w, height: h, pixels };
        let round = flare(None).starburst_of(&img, 1., 1e-3);
        assert!(round.iter().all(|c| c.rgb.y == 0.));
        // Six blades, a corner at 0°: edge normals at 30°, 90°, ...: a spike straight up.
        let six = flare(Some(6)).starburst_of(&img, 1., 1e-3);
        let up = six[(32 - 10) * w + 32].rgb.y;
        let between = six[(32 - 7) * w + 32 + 7].rgb.y;
        assert!(up > 100. * between.max(1e-12), "up {} between {}", up, between);
        // Falls as 1/θ² along it.
        let far = six[(32 - 20) * w + 32].rgb.y;
        assert!((up / far - 4.).abs() < 0.5, "{} {}", up, far);
        // Longer for red than for blue.
        assert!(six[(32 - 10) * w + 32].rgb.x > six[(32 - 10) * w + 32].rgb.z);
    }
}
