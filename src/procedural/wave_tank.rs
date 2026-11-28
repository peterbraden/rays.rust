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
