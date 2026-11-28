//! A numerical wave tank: water waves in two dimensions (a vertical section) followed to the
//! moment a breaker's jet touches down, by fully nonlinear potential flow.
//!
//! The water is inviscid and its flow irrotational, so its velocity is the gradient of a
//! potential φ that satisfies Laplace's equation. The tank is closed: a free surface above, a
//! bed below (flat, or sloping up to a reef), and walls at either end. Only the boundary is
//! discretised, by straight panels with φ and its normal derivative q constant on each
//! (collocated at the panels' midpoints, their integrals exact), and solved by Green's
//! identity, c φ(x) = ∫ q G - φ ∂G/∂n ds with G = -ln r / 2π: φ is known on the surface and q
//! (zero) on the bed and walls, and the solve gives the rest. The surface is a line of
//! Lagrangian markers carrying φ, moved by the velocity (φ's slope along the surface and q
//! across it) and their φ changed by Bernoulli's equation at zero pressure,
//! Dφ/Dt = |u|²/2 - g y, by fourth-order Runge-Kutta. Every few steps the surface is smoothed by
//! Longuet-Higgins and Cokelet's five-point filter, which stops the sawtooth instability of
//! such schemes (the mixed Eulerian-Lagrangian method of M. S. Longuet-Higgins and E. D. Cokelet,
//! "The deformation of steep surface waves on water. I. A numerical method of computation",
//! Proc. R. Soc. Lond. A 350, 1976; in a closed tank over a bed as S. T. Grilli, J. Skourup and
//! I. A. Svendsen's numerical wave tank, "An efficient boundary element method for nonlinear
//! water waves", Eng. Anal. Bound. Elem. 6, 1989).
//!
//! Units are any consistent ones; `g` sets them (9.81 for metres and seconds).

use crate::na::{DMatrix, DVector, Vector2};

/// The tank: its bed (a polyline from the left wall, x = 0, to the right one, x = `length`,
/// heights below the still water's level at y = 0), gravity, and the free surface's markers
/// (left to right, the first on the left wall and the last on the right) with the potential at
/// each, and the time.
#[derive(Clone, Debug)]
pub struct Tank {
    pub bed: Vec<Vector2<f64>>,
    pub length: f64,
    pub g: f64,
    pub surface: Vec<Vector2<f64>>,
    pub phi: Vec<f64>,
    pub time: f64,
    /// The bed's and walls' panels' length, about.
    pub solid_spacing: f64,
}

/// A boundary panel: its ends (in counterclockwise order round the water, so the water is on
/// its left and its outward normal on its right), and whether it is on the free surface (the
/// index of the panel between surface markers `k` and `k + 1`) or solid.
#[derive(Clone, Copy, Debug)]
struct Panel {
    a: Vector2<f64>,
    b: Vector2<f64>,
    surface: Option<usize>,
}

impl Panel {
    fn mid(&self) -> Vector2<f64> { (self.a + self.b) * 0.5 }
    fn length(&self) -> f64 { (self.b - self.a).norm() }
}

/// ∫ ln r ds along the straight panel from `a` to `b`, r the distance from `p`.
fn log_integral(p: Vector2<f64>, a: Vector2<f64>, b: Vector2<f64>) -> f64 {
    let l = (b - a).norm();
    let t = (b - a) / l;
    let s0 = (p - a).dot(&t);
    let d = (p - a).perp(&t);
    // ∫ ln √(u² + d²) du = u ln √(u² + d²) - u + d atan(u / d).
    let f = |u: f64| {
        let r2 = u * u + d * d;
        let log = if r2 > 0. { 0.5 * u * r2.ln() } else { 0. };
        let at = if d.abs() > 1e-300 { d * (u / d).atan() } else { 0. };
        log - u + at
    };
    f(l - s0) - f(-s0)
}

/// The angle the panel from `a` to `b` subtends at `p`, counterclockwise positive: ∫ (r · n) /
/// r² ds with n the panel's outward normal (to its right).
fn subtended(p: Vector2<f64>, a: Vector2<f64>, b: Vector2<f64>) -> f64 {
    let (u, v) = (a - p, b - p);
    (u.perp(&v)).atan2(u.dot(&v))
}

/// The three-point derivative of `f` at the middle of three samples at arc lengths `s`.
fn derivative(s: [f64; 3], f: [f64; 3]) -> f64 {
    let (h1, h2) = (s[1] - s[0], s[2] - s[1]);
    -h2 / (h1 * (h1 + h2)) * f[0] + (h2 - h1) / (h1 * h2) * f[1] + h1 / (h2 * (h1 + h2)) * f[2]
}

impl Tank {
    /// A tank `length` long over `bed` (heights at increasing x from 0 to `length`), its surface
    /// `markers` points evenly along it with elevation `eta(x)` and potential `phi(x)`.
    pub fn new(bed: Vec<Vector2<f64>>, g: f64, markers: usize, eta: impl Fn(f64) -> f64, phi: impl Fn(f64) -> f64) -> Tank {
        let length = bed.last().unwrap().x;
        let surface: Vec<Vector2<f64>> = (0..markers).map(|k| {
            let x = length * k as f64 / (markers - 1) as f64;
            Vector2::new(x, eta(x))
        }).collect();
        let phi = surface.iter().map(|p| phi(p.x)).collect();
        let solid_spacing = 2. * length / (markers - 1) as f64;
        Tank { bed, length, g, surface, phi, time: 0., solid_spacing }
    }

    /// The boundary's panels, counterclockwise round the water: the bed left to right, the
    /// right wall up, the surface right to left, the left wall down.
    fn panels(&self, surface: &[Vector2<f64>]) -> Vec<Panel> {
        let mut out = vec![];
        let line = |a: Vector2<f64>, b: Vector2<f64>, spacing: f64, out: &mut Vec<Panel>| {
            let n = ((b - a).norm() / spacing).ceil().max(1.) as usize;
            for i in 0..n {
                let (u, v) = (i as f64 / n as f64, (i + 1) as f64 / n as f64);
                out.push(Panel { a: a + (b - a) * u, b: a + (b - a) * v, surface: None });
            }
        };
        let h = self.solid_spacing;
        for w in self.bed.windows(2) { line(w[0], w[1], h, &mut out); }
        let m = surface.len();
        line(*self.bed.last().unwrap(), surface[m - 1], 0.5 * h, &mut out);
        for k in (0..m - 1).rev() {
            out.push(Panel { a: surface[k + 1], b: surface[k], surface: Some(k) });
        }
        line(surface[0], self.bed[0], 0.5 * h, &mut out);
        out
    }

    /// The normal derivative of the potential, q, on each surface panel (between markers k and
    /// k + 1), for the surface at `surface` with potential `phi`: Green's identity collocated at
    /// every panel's midpoint, solved for q on the surface and φ on the bed and walls.
    fn normal_velocity(&self, surface: &[Vector2<f64>], phi: &[f64]) -> Vec<f64> {
        let panels = self.panels(surface);
        let n = panels.len();
        let tau = std::f64::consts::TAU;
        let known: Vec<f64> = panels.iter().map(|p| match p.surface {
            Some(k) => 0.5 * (phi[k] + phi[k + 1]),
            None => 0.,
        }).collect();
        let mut m = DMatrix::<f64>::zeros(n, n);
        let mut rhs = DVector::<f64>::zeros(n);
        for (i, pi) in panels.iter().enumerate() {
            let x = pi.mid();
            for (j, pj) in panels.iter().enumerate() {
                // φ's coefficient (c δ + the double layer) and q's (the single layer).
                // (∂G/∂n integrates to minus the angle the panel subtends over 2π: a constant φ,
                // with q = 0, then satisfies the equation, ½ less the half turn the rest of the
                // boundary subtends from a point on it.)
                let a = if i == j { 0.5 } else { -subtended(x, pj.a, pj.b) / tau };
                let b = if i == j {
                    let l = pj.length();
                    -(l * (0.5 * l).ln() - l) / tau
                } else {
                    -log_integral(x, pj.a, pj.b) / tau
                };
                match pj.surface {
                    // q unknown on the surface: a φ_j + ... - b q_j = 0.
                    Some(_) => { m[(i, j)] = -b; rhs[i] -= a * known[j]; },
                    // φ unknown on the bed and walls, where q = 0.
                    None => m[(i, j)] = a,
                }
            }
        }
        let solution = m.lu().solve(&rhs).expect("the wave tank's boundary system is singular");
        let mut q = vec![0.; surface.len() - 1];
        for (j, p) in panels.iter().enumerate() {
            if let Some(k) = p.surface { q[k] = solution[j]; }
        }
        q
    }

    /// The velocity of each surface marker, and the rate of change of its potential, for the
    /// surface at `surface` with potential `phi`.
    fn rates(&self, surface: &[Vector2<f64>], phi: &[f64]) -> (Vec<Vector2<f64>>, Vec<f64>) {
        let q = self.normal_velocity(surface, phi);
        let m = surface.len();
        let mut s = vec![0.; m];
        for k in 1..m { s[k] = s[k - 1] + (surface[k] - surface[k - 1]).norm(); }
        let mut u = vec![Vector2::zeros(); m];
        for k in 1..m - 1 {
            let ss = [s[k - 1], s[k], s[k + 1]];
            let tx = derivative(ss, [surface[k - 1].x, surface[k].x, surface[k + 1].x]);
            let ty = derivative(ss, [surface[k - 1].y, surface[k].y, surface[k + 1].y]);
            let t = Vector2::new(tx, ty).normalize();
            // The outward normal: to the right of the counterclockwise direction, which runs
            // right to left along the surface, so to the left of increasing k.
            let normal = Vector2::new(-t.y, t.x);
            let phi_s = derivative(ss, [phi[k - 1], phi[k], phi[k + 1]]);
            let (h1, h2) = (s[k] - s[k - 1], s[k + 1] - s[k]);
            let phi_n = (h2 * q[k - 1] + h1 * q[k]) / (h1 + h2);
            u[k] = t * phi_s + normal * phi_n;
        }
        // At the walls the water moves up and down them: the vertical velocity extrapolated from
        // the two markers beside each (the panels nearest the corner where the surface meets
        // the wall are the least accurate).
        u[0] = Vector2::new(0., 2. * u[1].y - u[2].y);
        u[m - 1] = Vector2::new(0., 2. * u[m - 2].y - u[m - 3].y);
        let dphi = (0..m).map(|k| 0.5 * u[k].norm_squared() - self.g * surface[k].y).collect();
        (u, dphi)
    }

    /// One step of `dt` by fourth-order Runge-Kutta.
    pub fn step(&mut self, dt: f64) {
        let (x0, p0) = (self.surface.clone(), self.phi.clone());
        let advance = |x: &[Vector2<f64>], p: &[f64], u: &[Vector2<f64>], d: &[f64], h: f64| -> (Vec<Vector2<f64>>, Vec<f64>) {
            let m = x.len();
            let mut xn: Vec<Vector2<f64>> = (0..m).map(|k| x[k] + u[k] * h).collect();
            // The end markers stay on the walls.
            xn[0].x = 0.;
            xn[m - 1].x = self.length;
            (xn, (0..m).map(|k| p[k] + d[k] * h).collect())
        };
        let (u1, d1) = self.rates(&x0, &p0);
        let (x1, p1) = advance(&x0, &p0, &u1, &d1, 0.5 * dt);
        let (u2, d2) = self.rates(&x1, &p1);
        let (x2, p2) = advance(&x0, &p0, &u2, &d2, 0.5 * dt);
        let (u3, d3) = self.rates(&x2, &p2);
        let (x3, p3) = advance(&x0, &p0, &u3, &d3, dt);
        let (u4, d4) = self.rates(&x3, &p3);
        let m = x0.len();
        let u: Vec<Vector2<f64>> = (0..m).map(|k| (u1[k] + (u2[k] + u3[k]) * 2. + u4[k]) / 6.).collect();
        let d: Vec<f64> = (0..m).map(|k| (d1[k] + 2. * (d2[k] + d3[k]) + d4[k]) / 6.).collect();
        let (x, p) = advance(&x0, &p0, &u, &d, dt);
        self.surface = x;
        self.phi = p;
        self.time += dt;
    }

    /// A step's length: a fraction `courant` of the time the fastest of the surface's markers,
    /// or a long wave in the tank's deepest water, takes to cross the closest two.
    pub fn time_step(&self, courant: f64) -> f64 {
        let (u, _) = self.rates(&self.surface, &self.phi);
        let fastest = u.iter().map(|v| v.norm()).fold(0f64, f64::max);
        let deepest = self.bed.iter().map(|p| -p.y).fold(0f64, f64::max);
        let speed = fastest.max((self.g * deepest).sqrt());
        let closest = self.surface.windows(2).map(|w| (w[1] - w[0]).norm()).fold(f64::INFINITY, f64::min);
        courant * closest / speed
    }

    /// Longuet-Higgins and Cokelet's five-point smoothing of the surface's markers and their
    /// potential, which removes the shortest (sawtooth) waves the markers can carry and hardly
    /// touches the rest; the two markers at either end are left as they are.
    pub fn smooth(&mut self) {
        let m = self.surface.len();
        let filter = |f: &[f64], k: usize| (-f[k - 2] + 4. * f[k - 1] + 10. * f[k] + 4. * f[k + 1] - f[k + 2]) / 16.;
        let xs: Vec<f64> = self.surface.iter().map(|p| p.x).collect();
        let ys: Vec<f64> = self.surface.iter().map(|p| p.y).collect();
        let ps = self.phi.clone();
        for k in 2..m - 2 {
            self.surface[k] = Vector2::new(filter(&xs, k), filter(&ys, k));
            self.phi[k] = filter(&ps, k);
        }
    }

    /// Redistributes the surface's markers along it (keeping their number and the two on the
    /// walls), closer where it curves sharply: a Lagrangian surface's markers crowd at a
    /// breaker's jet and thin out elsewhere, until the steps they allow grow too short to go on.
    /// Their spacing goes as 1 / (1 + `refine` κ ℓ), κ the curvature and ℓ the markers' mean
    /// spacing times a hundred, smoothed and limited so neighbouring spacings differ by no more
    /// than a fifth; positions and potentials are carried over by cubic interpolation in arc
    /// length (Catmull-Rom), which keeps the surface's curvature where straight lines would cut
    /// it.
    pub fn regrid(&mut self, refine: f64) {
        let m = self.surface.len();
        let mut s = vec![0.; m];
        for k in 1..m { s[k] = s[k - 1] + (self.surface[k] - self.surface[k - 1]).norm(); }
        let total = s[m - 1];
        let ell = 100. * total / (m - 1) as f64;
        // Curvature at each marker, from the turning of the two segments beside it.
        let mut w = vec![1.; m];
        for k in 1..m - 1 {
            let (a, b) = (self.surface[k] - self.surface[k - 1], self.surface[k + 1] - self.surface[k]);
            let turn = a.perp(&b).atan2(a.dot(&b)).abs();
            let kappa = turn / (0.5 * (a.norm() + b.norm())).max(1e-12);
            // At most six times as close as on average, so the steps they allow stay long enough.
            w[k] = (1. + refine * kappa * ell / 100.).min(6.);
        }
        // Smoothed, so the spacing changes gradually.
        for _ in 0..4 {
            let v = w.clone();
            for k in 1..m - 1 { w[k] = 0.25 * v[k - 1] + 0.5 * v[k] + 0.25 * v[k + 1]; }
            w[0] = w[1];
            w[m - 1] = w[m - 2];
        }
        // The cumulative weight, placed at equal steps.
        let mut cum = vec![0.; m];
        for k in 1..m { cum[k] = cum[k - 1] + 0.5 * (w[k - 1] + w[k]) * (s[k] - s[k - 1]); }
        let targets: Vec<f64> = (0..m).map(|i| {
            let c = cum[m - 1] * i as f64 / (m - 1) as f64;
            let j = cum.partition_point(|&v| v < c).clamp(1, m - 1);
            let t = (c - cum[j - 1]) / (cum[j] - cum[j - 1]).max(1e-300);
            s[j - 1] + (s[j] - s[j - 1]) * t
        }).collect();
        // Catmull-Rom on arc length.
        let at = |f: &dyn Fn(usize) -> f64, x: f64| -> f64 {
            let j = s.partition_point(|&v| v <= x).clamp(1, m - 1) - 1;
            let (s0, s1) = (s[j], s[j + 1]);
            let h = (s1 - s0).max(1e-300);
            let t = ((x - s0) / h).clamp(0., 1.);
            let slope = |k: usize| {
                let (a, b) = (k.saturating_sub(1), (k + 1).min(m - 1));
                (f(b) - f(a)) / (s[b] - s[a]).max(1e-300)
            };
            let (m0, m1) = (slope(j) * h, slope(j + 1) * h);
            let (t2, t3) = (t * t, t * t * t);
            (2. * t3 - 3. * t2 + 1.) * f(j) + (t3 - 2. * t2 + t) * m0 + (-2. * t3 + 3. * t2) * f(j + 1) + (t3 - t2) * m1
        };
        let (old, phi) = (self.surface.clone(), self.phi.clone());
        let mut surface = vec![Vector2::zeros(); m];
        let mut potential = vec![0.; m];
        for (i, &x) in targets.iter().enumerate() {
            surface[i] = Vector2::new(at(&|k| old[k].x, x), at(&|k| old[k].y, x));
            potential[i] = at(&|k| phi[k], x);
        }
        surface[0] = old[0];
        surface[m - 1] = old[m - 1];
        potential[0] = phi[0];
        potential[m - 1] = phi[m - 1];
        self.surface = surface;
        self.phi = potential;
    }

    /// Whether the surface has come back on itself: two markers within `gap` of each other
    /// that aren't near each other along it (a plunging jet touching down on the water before
    /// it).
    pub fn touched_down(&self, gap: f64) -> bool {
        let m = self.surface.len();
        let mut s = vec![0.; m];
        for k in 1..m { s[k] = s[k - 1] + (self.surface[k] - self.surface[k - 1]).norm(); }
        let p = &self.surface;
        // (Or passed through it within a step: two segments crossing.)
        let cross = |o: Vector2<f64>, a: Vector2<f64>, b: Vector2<f64>| (a - o).perp(&(b - o));
        let crossed = |i: usize, j: usize| {
            cross(p[i], p[i + 1], p[j]) * cross(p[i], p[i + 1], p[j + 1]) < 0. && cross(p[j], p[j + 1], p[i]) * cross(p[j], p[j + 1], p[i + 1]) < 0.
        };
        (0..m).any(|i| (i + 1..m).any(|j| s[j] - s[i] > 10. * gap && ((p[j] - p[i]).norm() < gap || (j + 1 < m && i + 1 < j && crossed(i, j)))))
    }

    /// The water's energy: kinetic, ½ ∫ φ q ds over the surface (the bed and walls, where q is
    /// zero, adding none), and potential about the still water, ½ g ∫ y² dx.
    pub fn energy(&self) -> (f64, f64) {
        let q = self.normal_velocity(&self.surface, &self.phi);
        let mut kinetic = 0.;
        let mut potential = 0.;
        for (k, qk) in q.iter().enumerate() {
            let (a, b) = (self.surface[k], self.surface[k + 1]);
            kinetic += 0.5 * 0.5 * (self.phi[k] + self.phi[k + 1]) * qk * (b - a).norm();
            potential += 0.5 * self.g * 0.5 * (a.y * a.y + b.y * b.y) * (b.x - a.x);
        }
        (kinetic, potential)
    }

    /// Whether the surface has overturned: somewhere a marker lies to the left of the one
    /// before it (the face has passed the vertical and the crest hangs over it).
    pub fn overturned(&self) -> bool {
        self.surface.windows(2).any(|w| w[1].x < w[0].x)
    }

    /// The highest point of the surface.
    pub fn crest(&self) -> Vector2<f64> {
        *self.surface.iter().max_by(|a, b| a.y.total_cmp(&b.y)).unwrap()
    }

    /// The bed's depth below still water at `x`.
    pub fn depth(&self, x: f64) -> f64 {
        for w in self.bed.windows(2) {
            if x >= w[0].x && x <= w[1].x {
                let t = (x - w[0].x) / (w[1].x - w[0].x).max(1e-12);
                return -(w[0].y + (w[1].y - w[0].y) * t);
            }
        }
        -self.bed.last().unwrap().y
    }

    /// The two markers where the surface has come back on itself (the jet's tip and the water
    /// it lands on): the closest pair more than `apart` from each other along the surface.
    fn closure(&self, apart: f64) -> (usize, usize) {
        let m = self.surface.len();
        let mut s = vec![0.; m];
        for k in 1..m { s[k] = s[k - 1] + (self.surface[k] - self.surface[k - 1]).norm(); }
        let mut best = (f64::INFINITY, 0, 0);
        for i in 0..m {
            for j in i + 1..m {
                let gap = (self.surface[j] - self.surface[i]).norm();
                if s[j] - s[i] > apart && gap < best.0 { best = (gap, i, j); }
            }
        }
        (best.1, best.2)
    }

    /// The tube a plunging jet has closed, once it has touched down: the air between the jet's
    /// underside, the face and the water the tip lands on.
    pub fn tube(&self) -> Tube {
        let (i, j) = self.closure(0.1);
        let ring: Vec<Vector2<f64>> = self.surface[i..=j].to_vec();
        // Area, centroid and second moments of the enclosed region, by Green's theorem.
        let (mut a, mut cx, mut cy, mut ixx, mut iyy, mut ixy) = (0., 0., 0., 0., 0., 0.);
        for k in 0..ring.len() {
            let (p, q) = (ring[k], ring[(k + 1) % ring.len()]);
            let c = p.x * q.y - q.x * p.y;
            a += c / 2.;
            cx += (p.x + q.x) * c / 6.;
            cy += (p.y + q.y) * c / 6.;
            ixx += (p.y * p.y + p.y * q.y + q.y * q.y) * c / 12.;
            iyy += (p.x * p.x + p.x * q.x + q.x * q.x) * c / 12.;
            ixy += (p.x * q.y + 2. * p.x * p.y + 2. * q.x * q.y + q.x * p.y) * c / 24.;
        }
        let centre = Vector2::new(cx / a, cy / a);
        let (ixx, iyy, ixy) = (ixx / a - centre.y * centre.y, iyy / a - centre.x * centre.x, ixy / a - centre.x * centre.y);
        let mean = 0.5 * (ixx + iyy);
        let spread = (0.25 * (ixx - iyy).powi(2) + ixy * ixy).sqrt();
        // The long axis: the direction of the largest spread of x and y about the centroid.
        let tilt = -0.5 * (2. * ixy).atan2(iyy - ixx);
        let top = ring.iter().max_by(|p, q| p.y.total_cmp(&q.y)).unwrap();
        // The water above the tube's top: up from it to the jet's upper surface (the markers
        // before the tip's).
        let roof = self.surface[..i].windows(2).filter_map(|w| {
            let (lo, hi) = if w[0].x < w[1].x { (w[0], w[1]) } else { (w[1], w[0]) };
            (top.x >= lo.x && top.x <= hi.x && hi.x > lo.x).then(|| lo.y + (hi.y - lo.y) * (top.x - lo.x) / (hi.x - lo.x) - top.y)
        }).filter(|&d| d > 0.).fold(f64::INFINITY, f64::min);
        // The jet's thickness halfway from the tube's top to the tip: from the underside there
        // to the nearest point of its upper surface.
        let t = ring.iter().position(|p| p == top).unwrap();
        let mid = ring[t / 2];
        let lip = self.surface[..i].iter().map(|p| (p - mid).norm()).fold(f64::INFINITY, f64::min);
        Tube {
            area: a.abs(), centre, aspect: ((mean + spread) / (mean - spread)).sqrt(), tilt,
            width: ring.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max) - ring.iter().map(|p| p.x).fold(f64::INFINITY, f64::min),
            height: top.y - ring.iter().map(|p| p.y).fold(f64::INFINITY, f64::min),
            roof, lip,
        }
    }
}

/// The tube of air a plunging breaker's jet closes as it touches down, measured on the
/// tank's section.
#[derive(Clone, Debug)]
pub struct Tube {
    pub area: f64,
    pub centre: Vector2<f64>,
    /// The ratio of its long axis to its short one, by its second moments: what surfing
    /// studies call the vortex ratio.
    pub aspect: f64,
    /// The long axis's angle above the horizontal, rising away from the shore, towards the
    /// wave's back (radians).
    pub tilt: f64,
    pub width: f64,
    pub height: f64,
    /// The water above the tube's highest point, up to the jet's upper surface.
    pub roof: f64,
    /// The jet's thickness halfway from the tube's top to its tip.
    pub lip: f64,
}

/// What a breaker did in the tank: when and where its face went vertical, the depth there and
/// the crest's height above still water, and the tube it closed (None if it never touched
/// down), with the surface at the end.
#[derive(Clone, Debug)]
pub struct Breaking {
    pub overturned: f64,
    pub crest: Vector2<f64>,
    pub depth: f64,
    pub tube: Option<Tube>,
    pub time: f64,
    pub tank: Tank,
}

/// A solitary wave `height` high in water 1 deep (g = 1) running up a slope of 1 in `slope`
/// to a reef flat 0.1 deep, followed until its jet touches down. `spacing` is the markers'
/// mean spacing along the surface.
pub fn break_on_slope(height: f64, slope: f64, spacing: f64, mut each: impl FnMut(&Tank)) -> Breaking {
    let (toe, reef) = (12., 0.1);
    let top = toe + (1. - reef) * slope;
    let length = top + 8.;
    let bed = vec![Vector2::new(0., -1.), Vector2::new(toe, -1.), Vector2::new(top, -reef), Vector2::new(length, -reef)];
    let (eta, phi) = solitary(height, 1., 5., 1.);
    let mut tank = Tank::new(bed, 1., (length / spacing) as usize + 1, eta, phi);
    let mut broke: Option<(f64, Vector2<f64>, f64)> = None;
    let mut steps = 0;
    let mut tube = None;
    while steps < 40000 {
        let dt = tank.time_step(0.3);
        if dt < 5e-5 { break; }
        tank.step(dt);
        steps += 1;
        if steps % 5 == 0 { tank.smooth(); }
        // Markers redistributed to follow the jet, closer where it curves.
        if steps % 10 == 0 { tank.regrid(3.); }
        if broke.is_none() && tank.overturned() {
            let c = tank.crest();
            broke = Some((tank.time, c, tank.depth(c.x)));
        }
        each(&tank);
        // (A run gone unstable: markers thrown out of the tank.)
        if tank.surface.iter().any(|p| !(p.y.abs() < 2.) || !(p.x > -1. && p.x < length + 1.)) { break; }
        if broke.is_some() && tank.touched_down(0.02) {
            tube = Some(tank.tube());
            break;
        }
        // A wave that has reached the reef flat's end without overturning has spilled or
        // surged.
        if broke.is_none() && tank.crest().x > length - 3. { break; }
    }
    let (overturned, crest, depth) = broke.unwrap_or((f64::NAN, tank.crest(), f64::NAN));
    Breaking { overturned, crest, depth, tube, time: tank.time, tank }
}

/// A solitary wave of height `height` in water `depth` deep, its crest at `x0`, by first-order
/// (Boussinesq) theory: η = H sech²(k (x - x0)), k = √(3H / 4h³), travelling at c = √(g (h + H))
/// with the depth-averaged velocity c η / h, so the surface potential c H / (h k) tanh(k (x -
/// x0)). Steep ones shed a small tail as they settle to the exact form.
pub fn solitary(height: f64, depth: f64, x0: f64, g: f64) -> (impl Fn(f64) -> f64, impl Fn(f64) -> f64) {
    let k = (3. * height / (4. * depth.powi(3))).sqrt();
    let c = (g * (depth + height)).sqrt();
    let eta = move |x: f64| height / (k * (x - x0)).cosh().powi(2);
    let phi = move |x: f64| c * height / (depth * k) * (k * (x - x0)).tanh();
    (eta, phi)
}

/// The sections of a breaker for drawing one (`breaker::Breaker` with `"model":
/// "simulated"`): a solitary wave H/h = 0.45 run up a slope of 1 in `slope` until its jet
/// touches down, and `frames` of its surface from as long before its face overturns as it
/// takes from then to touchdown, evenly in time, each `points` points along the surface
/// (closer where it curves) from three crest heights behind its crest to two and a half in
/// front. In units of the crest's height above the still water as it overturns, x shoreward
/// from a point moving with the wave (the crest as it overturns, at the speed it had then).
pub fn breaker_frames(slope: f64, frames: usize, points: usize) -> Vec<Vec<Vector2<f64>>> {
    let mut record: Vec<(f64, Vec<Vector2<f64>>)> = vec![];
    let b = break_on_slope(0.45, slope, 0.08, |tank| record.push((tank.time, tank.surface.clone())));
    assert!(b.tube.is_some(), "the wave on 1:{} never closed a tube", slope);
    let crest_at = |t: f64| {
        let k = record.partition_point(|r| r.0 < t).min(record.len() - 1);
        *record[k].1.iter().max_by(|a, b| a.y.total_cmp(&b.y)).unwrap()
    };
    let (t1, t2) = (b.overturned, b.time);
    let t0 = t1 - (t2 - t1);
    let unit = b.crest.y;
    let speed = (crest_at(t1).x - crest_at(t1 - 0.5).x) / 0.5;
    (0..frames).map(|f| {
        let t = t0 + (t2 - t0) * f as f64 / (frames - 1) as f64;
        let k = record.partition_point(|r| r.0 < t).min(record.len() - 1);
        let surface = &record[k].1;
        let origin = b.crest.x + speed * (record[k].0 - t1);
        let (from, to) = (origin - 3. * unit, origin + 2.5 * unit);
        let first = surface.iter().position(|p| p.x >= from).unwrap().saturating_sub(1);
        let last = (surface.len() - 1 - surface.iter().rev().position(|p| p.x <= to).unwrap() + 1).min(surface.len() - 1);
        let line = &surface[first..=last];
        let m = line.len();
        let mut s = vec![0.; m];
        for i in 1..m { s[i] = s[i - 1] + (line[i] - line[i - 1]).norm(); }
        // Closer where the surface curves (the jet's tip), at most seven times.
        let mut w = vec![1.; m];
        for i in 1..m - 1 {
            let (a, c) = (line[i] - line[i - 1], line[i + 1] - line[i]);
            let kappa = a.perp(&c).atan2(a.dot(&c)).abs() / (0.5 * (a.norm() + c.norm())).max(1e-12);
            w[i] = 1. + (0.5 * kappa * unit).min(6.);
        }
        for _ in 0..4 {
            let v = w.clone();
            for i in 1..m - 1 { w[i] = 0.25 * v[i - 1] + 0.5 * v[i] + 0.25 * v[i + 1]; }
        }
        let mut cum = vec![0.; m];
        for i in 1..m { cum[i] = cum[i - 1] + 0.5 * (w[i - 1] + w[i]) * (s[i] - s[i - 1]); }
        (0..points).map(|j| {
            let c = cum[m - 1] * j as f64 / (points - 1) as f64;
            let i = cum.partition_point(|&v| v < c).clamp(1, m - 1);
            let u = (c - cum[i - 1]) / (cum[i] - cum[i - 1]).max(1e-300);
            let p = line[i - 1] + (line[i] - line[i - 1]) * u;
            Vector2::new((p.x - origin) / unit, p.y / unit)
        }).collect()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_boundary_integrals_are_exact() {
        // Against a fine midpoint sum.
        let (a, b, p) = (Vector2::new(0.2, -0.1), Vector2::new(1.3, 0.4), Vector2::new(0.5, 0.9));
        let n = 20000;
        let (mut log, mut angle) = (0., 0.);
        let l = (b - a).norm();
        let t: Vector2<f64> = (b - a) / l;
        let outward = Vector2::new(t.y, -t.x);
        for i in 0..n {
            let y = a + (b - a) * ((i as f64 + 0.5) / n as f64);
            let r = y - p;
            log += r.norm().ln() * l / n as f64;
            angle += r.dot(&outward) / r.norm_squared() * l / n as f64;
        }
        assert!((log_integral(p, a, b) - log).abs() < 1e-6, "{} {}", log_integral(p, a, b), log);
        assert!((subtended(p, a, b) - angle).abs() < 1e-6, "{} {}", subtended(p, a, b), angle);
    }

    #[test]
    fn test_the_solve_gives_a_known_potentials_normal_velocity() {
        // φ = cosh k(y + h) cos kx in a tank 2 long, 1 deep, k = π / 2: no flow through the bed
        // or walls, and at the flat surface q = ∂φ/∂y = k sinh(kh) cos(kx).
        let (h, len) = (1., 2.);
        let k = std::f64::consts::PI / len;
        let tank = Tank::new(vec![Vector2::new(0., -h), Vector2::new(len, -h)], 1., 81, |_| 0., |x| (k * h).cosh() * (k * x).cos());
        let q = tank.normal_velocity(&tank.surface, &tank.phi);
        let mut worst: f64 = 0.;
        for (i, qi) in q.iter().enumerate() {
            let x = 0.5 * (tank.surface[i].x + tank.surface[i + 1].x);
            let exact = k * (k * h).sinh() * (k * x).cos();
            // (Constant panels are least accurate at the corners where the surface meets the
            // walls: within 6% at the panels beside them, the wall markers' velocity extrapolated
            // from further in; elsewhere 1%.)
            let corner = i < 2 || i + 3 > q.len();
            worst = worst.max((qi - exact).abs() / if corner { 6. } else { 1. });
            if i % 10 == 0 { println!("x {:.3} q {:.5} exact {:.5}", x, qi, exact); }
        }
        assert!(worst < 0.02 * k * (k * h).sinh(), "{}", worst);
    }

    #[test]
    fn test_a_small_standing_wave_keeps_its_period_and_energy() {
        // A standing wave in a closed tank 2 long and 1 deep, its first mode: η = a cos(k x)
        // cos(ω t), k = π / 2, ω² = g k tanh(k h); it starts at rest, so φ = 0.
        let (g, h, len, a) = (1., 1., 2., 0.005);
        let k = std::f64::consts::PI / len;
        let omega = (g * k * (k * h).tanh()).sqrt();
        let period = std::f64::consts::TAU / omega;
        let mut tank = Tank::new(vec![Vector2::new(0., -h), Vector2::new(len, -h)], g, 41, |x| a * (k * x).cos(), |_| 0.);
        let e0 = tank.energy();
        let dt = period / 200.;
        // The left wall's elevation through one period: it should come back to a, through -a
        // at half the period.
        let mut least = (f64::INFINITY, 0.);
        for i in 1..=200 {
            tank.step(dt);
            if i % 10 == 0 { tank.smooth(); }
            let y = tank.surface[0].y;
            if y < least.0 { least = (y, tank.time); }
        }
        let e1 = tank.energy();
        println!("period {:.4}; trough at {:.4} ({:.5}), back to {:.5} of {}; energy {:?} -> {:?}", period, least.1, least.0, tank.surface[0].y, a, e0, e1);
        assert!((least.1 / period - 0.5).abs() < 0.01, "{} of {}", least.1, period);
        assert!((least.0 + a).abs() < 0.03 * a, "{}", least.0);
        assert!((tank.surface[0].y - a).abs() < 0.03 * a, "{}", tank.surface[0].y);
        let (t0, t1) = (e0.0 + e0.1, e1.0 + e1.1);
        assert!((t1 / t0 - 1.).abs() < 0.02, "{} -> {}", t0, t1);
    }

    #[test]
    fn test_a_solitary_wave_travels_at_its_speed() {
        // H = 0.2 in water 1 deep, 30 long: it travels at about √(g (h + H)) = 1.095 and keeps
        // its height.
        let (g, h, height) = (1., 1., 0.2);
        let (eta, phi) = solitary(height, h, 8., g);
        let mut tank = Tank::new(vec![Vector2::new(0., -h), Vector2::new(30., -h)], g, 241, eta, phi);
        let start = tank.crest();
        let mut steps = 0;
        while tank.time < 8. && steps < 2000 {
            let dt = tank.time_step(0.4).min(8. - tank.time + 1e-9);
            tank.step(dt);
            steps += 1;
            if steps % 5 == 0 { tank.smooth(); }
        }
        let end = tank.crest();
        let speed = (end.x - start.x) / tank.time;
        println!("{} steps; crest {:?} -> {:?}, speed {:.4} (theory {:.4})", steps, start, end, speed, (g * (h + height)).sqrt());
        assert!((speed / (g * (h + height)).sqrt() - 1.).abs() < 0.03, "{}", speed);
        assert!((end.y - height).abs() < 0.1 * height, "{}", end.y);
    }
}

#[cfg(test)]
mod breaking {
    use super::*;

    /// A solitary wave H/h = 0.45 shoaling up a 1:15 slope to a reef flat, run until its jet
    /// touches down, writing profiles to RAYS_TANK_PROFILES (JSON) for drawing.
    #[test]
    #[ignore]
    fn measure_a_solitary_wave_breaking_on_a_slope() {
        let started = std::time::Instant::now();
        let mut profiles = vec![];
        let mut count = 0;
        let slope = std::env::var("RAYS_TANK_SLOPE").ok().and_then(|v| v.parse().ok()).unwrap_or(15.);
        let b = break_on_slope(0.45, slope, 0.08, |tank| {
            count += 1;
            if count % 25 == 0 || (tank.overturned() && count % 5 == 0) {
                profiles.push(serde_json::json!({"t": tank.time, "points": tank.surface.iter().map(|p| [p.x, p.y]).collect::<Vec<_>>()}));
            }
        });
        println!("{} steps in {:.1?}: face vertical at t {:.3}, crest [{:.3}, {:.3}] over depth {:.3}, crest / depth {:.2}; to t {:.3}",
            count, started.elapsed(), b.overturned, b.crest.x, b.crest.y, b.depth, b.crest.y / b.depth, b.time);
        println!("{:?}", b.tube);
        profiles.push(serde_json::json!({"t": b.time, "points": b.tank.surface.iter().map(|p| [p.x, p.y]).collect::<Vec<_>>()}));
        if let Ok(path) = std::env::var("RAYS_TANK_PROFILES") {
            std::fs::write(path, serde_json::json!({"bed": b.tank.bed.iter().map(|p| [p.x, p.y]).collect::<Vec<_>>(), "profiles": profiles}).to_string()).unwrap();
        }
        assert!(b.tube.is_some());
    }

    /// The tubes of solitary waves breaking on slopes from 1:10 to 1:30, against Mead and
    /// Black's fit to 28 surfing breaks, vortex ratio = 0.065 X + 0.821 for a seabed gradient
    /// of 1 in X. Writes each breaker's final surface to RAYS_TANK_SWEEP (JSON).
    #[test]
    #[ignore]
    fn measure_tubes_against_slope() {
        use rayon::prelude::*;
        let slopes = [10., 15., 20., 25., 30.];
        let heights: Vec<f64> = std::env::var("RAYS_TANK_HEIGHT").ok().and_then(|v| v.parse().ok()).map(|h| vec![h]).unwrap_or(vec![0.45]);
        let runs: Vec<(f64, f64)> = heights.iter().flat_map(|&h| slopes.iter().map(move |&s| (h, s))).collect();
        let results: Vec<(f64, f64, Breaking)> = runs.par_iter().map(|&(h, s)| (h, s, break_on_slope(h, s, 0.08, |_| {}))).collect();
        println!("H/h  slope  crest/depth  touchdown  tube w x h  vortex ratio (Mead & Black)  tilt  roof/crest  lip/crest");
        let mut out = vec![];
        for (h, s, b) in &results {
            match &b.tube {
                Some(t) => println!("{:.2}  1:{:<4} {:.2}  t {:.2}  {:.3} x {:.3}  {:.2} ({:.2})  {:.0}°  {:.2}  {:.2}",
                    h, s, b.crest.y / b.depth, b.time, t.width, t.height, t.aspect, 0.065 * s + 0.821, t.tilt.to_degrees(), t.roof / b.crest.y, t.lip / b.crest.y),
                None => println!("{:.2}  1:{:<4} no tube (to t {:.2})", h, s, b.time),
            }
            out.push(serde_json::json!({"height": h, "slope": s, "bed": b.tank.bed.iter().map(|p| [p.x, p.y]).collect::<Vec<_>>(), "points": b.tank.surface.iter().map(|p| [p.x, p.y]).collect::<Vec<_>>()}));
        }
        if let Ok(path) = std::env::var("RAYS_TANK_SWEEP") {
            std::fs::write(path, serde_json::Value::Array(out).to_string()).unwrap();
        }
    }

    /// Writes the breaker's simulated sections, `breaker_frames(15., 33, 240)`, to
    /// src/procedural/breaker_frames.json, which `breaker::Breaker` reads (a run of a few
    /// minutes, so kept rather than run as a scene loads).
    #[test]
    #[ignore]
    fn write_breaker_frames() {
        let frames = breaker_frames(15., 33, 240);
        let round = |v: f64| (v * 1e4).round() / 1e4;
        let json = serde_json::json!({
            "source": "wave_tank::breaker_frames(15., 33, 240): a solitary wave H/h 0.45 on a 1:15 slope, from overturning less the time to touchdown, to touchdown",
            "frames": frames.iter().map(|f| f.iter().map(|p| [round(p.x), round(p.y)]).collect::<Vec<_>>()).collect::<Vec<_>>(),
        });
        std::fs::write("src/procedural/breaker_frames.json", json.to_string()).unwrap();
    }
}
