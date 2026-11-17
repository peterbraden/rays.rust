//! A giant planet's cloud tops: belts and zones, grown from its winds.
//!
//! A giant's visible face is the top of its clouds, painted by its winds. Its winds are zonal
//! jets, alternating east and west with latitude (Jupiter has a dozen or so in each
//! hemisphere; Saturn fewer and broader, under a great equatorial jet; Uranus and Neptune one
//! broad retrograde equatorial current and a prograde jet near each pole). Between the jets
//! the shear gives each band a sense of rotation, and that decides its colour: where the flow
//! turns anticyclonically, gas rises and fresh ammonia ice forms a bright *zone*; where it
//! turns cyclonically, gas sinks, the clouds clear and the deeper, coloured layers show as a
//! dark *belt* (Ingersoll et al., "Dynamics of Jupiter's atmosphere", in Bagenal et al.,
//! "Jupiter", Cambridge 2004). So here the bands come from the wind profile, not the other way
//! round: the relative vorticity ζ = -∂u/∂φ + u tan φ (on a unit sphere), anticyclonic where
//! ζ has the opposite sign to the latitude.
//!
//! The pattern of the clouds is made by letting the flow carry them. Every point of the map is
//! traced backwards along the velocity field for a while (a flow map, as in semi-Lagrangian
//! advection, but exact for each texel rather than resampled each step, so nothing blurs), and
//! takes its colour from where it came from: the band of the latitude it started at, with
//! some mottling. The velocity is the jets, plus turbulence (the curl of a noise stream
//! function, so it neither converges nor diverges), strongest where the jets shear, plus the
//! vortices: long-lived storms like the Great Red Spot and the white ovals, anticyclones in the
//! zones' shear, each with its own colour carried round in its closed core. The jets' shear
//! draws the mottling into the long streaks and filaments of the real thing, and the vortices
//! wind the bands' edges round themselves.
//!
//! Time is in units such that the fastest jet crosses one radian in one unit.

use crate::color::Color;
use crate::na::Vector3;
use crate::noise::SeededNoise;
use crate::procedural::space::{mix, smoothstep, tangent_basis};
use rayon::prelude::*;
use std::f64::consts::PI;

/// A zonal jet: Gaussian in latitude.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Jet {
    /// Latitude of its core, radians.
    pub latitude: f64,
    /// Eastward speed at its core, relative (the fastest jet is 1 after normalising).
    pub speed: f64,
    /// Its half-width, radians.
    pub width: f64,
}

/// A long-lived vortex.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Storm {
    /// Its centre, a unit vector in the body's frame (y the pole).
    pub centre: Vector3<f64>,
    /// The radius of its core, radians.
    pub radius: f64,
    /// Its east-west elongation (the Great Red Spot is twice as long as it is wide).
    pub elongation: f64,
    /// Turns per unit time, anticlockwise seen from outside: anticyclones are clockwise in
    /// the north.
    pub spin: f64,
    /// Its colour (normal albedo), and how strongly it shows.
    pub colour: Color,
    pub strength: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Weather {
    pub seed: u64,
    pub jets: Vec<Jet>,
    pub storms: Vec<Storm>,
    /// The turbulence, against the fastest jet.
    pub turbulence: f64,
    /// Normal albedos of the zones and belts.
    pub zone: Color,
    pub belt: Color,
    /// The colour of the polar haze, and the latitude (radians) it starts at.
    pub polar: Color,
    pub polar_latitude: f64,
    /// How much the bands' colour varies, from place to place (0 none).
    pub mottling: f64,
    /// A second colour the belts are stained with in patches (Jupiter's browns and oranges).
    pub stain: Color,
    /// Bright high clouds (methane ice on the ice giants), 0 to 1.
    pub high_clouds: f64,
    /// How long the flow has run.
    pub time: f64,
    /// Map width in texels (its height is half).
    pub resolution: usize,
}

/// An equirectangular map over the sphere: longitude across, latitude down from the north
/// pole, in the body's frame (y the pole, longitude 0 along x, increasing towards z).
#[derive(Debug, Clone)]
pub struct Map<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

/// The unit vector at longitude and latitude (radians).
pub fn direction(lon: f64, lat: f64) -> Vector3<f64> {
    Vector3::new(lat.cos() * lon.cos(), lat.sin(), lat.cos() * lon.sin())
}

/// Longitude and latitude of unit `p`.
pub fn lon_lat(p: &Vector3<f64>) -> (f64, f64) {
    (p.z.atan2(p.x), p.y.clamp(-1., 1.).asin())
}

impl<T: Copy + Default + std::ops::Add<Output = T> + std::ops::Mul<f64, Output = T>> Map<T> {
    /// A map whose texel centres hold `f` there.
    pub fn build(width: usize, height: usize, f: impl Fn(Vector3<f64>) -> T + Sync) -> Map<T> where T: Send {
        let mut data = vec![T::default(); width * height];
        data.par_chunks_mut(width).enumerate().for_each(|(j, row)| {
            let lat = PI / 2. - (j as f64 + 0.5) / height as f64 * PI;
            for (i, v) in row.iter_mut().enumerate() {
                let lon = (i as f64 + 0.5) / width as f64 * 2. * PI - PI;
                *v = f(direction(lon, lat));
            }
        });
        Map { width, height, data }
    }

    /// Bilinearly interpolated at unit `p`.
    pub fn at(&self, p: &Vector3<f64>) -> T {
        let (lon, lat) = lon_lat(p);
        let x = (lon + PI) / (2. * PI) * self.width as f64 - 0.5;
        let y = ((PI / 2. - lat) / PI * self.height as f64 - 0.5).clamp(0., (self.height - 1) as f64);
        let (x0, y0) = (x.floor(), y.floor());
        let (fx, fy) = (x - x0, y - y0);
        let i0 = (x0 as isize).rem_euclid(self.width as isize) as usize;
        let i1 = (i0 + 1) % self.width;
        let j0 = y0 as usize;
        let j1 = (j0 + 1).min(self.height - 1);
        let g = |i: usize, j: usize| self.data[j * self.width + i];
        (g(i0, j0) * (1. - fx) + g(i1, j0) * fx) * (1. - fy) + (g(i0, j1) * (1. - fx) + g(i1, j1) * fx) * fy
    }
}

/// A unit number from a hash.
fn unit(h: u64) -> f64 {
    (h >> 11) as f64 / (1u64 << 53) as f64
}

impl Weather {
    /// The eastward wind at latitude `lat`.
    pub fn wind(&self, lat: f64) -> f64 {
        self.jets.iter().map(|j| j.speed * (-((lat - j.latitude) / j.width).powi(2)).exp()).sum::<f64>()
            // Nothing flows across the poles.
            * smoothstep(0., 0.12, PI / 2. - lat.abs())
    }

    /// The relative vorticity of the zonal flow at `lat`: -∂u/∂φ + u tan φ.
    pub fn vorticity(&self, lat: f64) -> f64 {
        let h = 1e-3;
        let du = (self.wind(lat + h) - self.wind(lat - h)) / (2. * h);
        -du + self.wind(lat) * lat.tan().clamp(-20., 20.)
    }

    /// How anticyclonic the flow is at `lat`: positive in zones, negative in belts.
    pub fn anticyclonic(&self, lat: f64) -> f64 {
        -self.vorticity(lat) * lat.sin() / (lat.sin().abs() + 0.02)
    }

    /// The velocity at unit `p`: tangent, in radians per unit time.
    fn velocity(&self, p: &Vector3<f64>, noise: &SeededNoise, shear_scale: f64) -> Vector3<f64> {
        let (_, lat) = lon_lat(p);
        let east = Vector3::y().cross(p);
        let east = if east.norm() > 1e-9 { east.normalize() } else { Vector3::x() };
        // Turbulence: the surface curl of a stream function, p × ∇ψ, stronger where the jets
        // shear.
        let h = 1e-3;
        let psi = |q: Vector3<f64>| noise.fbm(q * 7., 4) + 0.3 * noise.fbm(q * 19. + Vector3::new(5.1, 1.3, 7.7), 2);
        let grad = Vector3::new(
            psi(p + Vector3::x() * h) - psi(p - Vector3::x() * h),
            psi(p + Vector3::y() * h) - psi(p - Vector3::y() * h),
            psi(p + Vector3::z() * h) - psi(p - Vector3::z() * h),
        ) / (2. * h);
        let shear = ((self.wind(lat + h) - self.wind(lat - h)) / (2. * h)).abs() / shear_scale;
        // Near the poles the jets weaken and turbulence (and the polar cyclones) takes over.
        let polar = smoothstep(self.polar_latitude - 0.15, self.polar_latitude + 0.1, lat.abs());
        let turbulence = p.cross(&grad) * (self.turbulence * (0.6 + shear.min(1.) + 1.5 * polar) / 7.);
        let mut v = east * self.wind(lat) + turbulence;
        for s in &self.storms {
            let cos = p.dot(&s.centre);
            if cos < (s.radius * s.elongation * 3.).cos() {
                continue;
            }
            // Distance from the centre in the storm's own (stretched) frame.
            let (se, sn) = storm_axes(&s.centre);
            let x = p.dot(&se) / s.elongation;
            let y = p.dot(&sn);
            let d = (x * x + y * y).sqrt() / s.radius;
            // The storm sits in the jets and is not torn by them: it holds its place and the
            // flow goes round it.
            let hold = (-(d / 1.3).powi(2)).exp();
            v *= 1. - hold;
            // Its swirl: solid-body at the centre, falling away outside the core.
            let around = s.centre.cross(p);
            if around.norm() > 1e-12 {
                v += around.normalize() * (2. * PI * s.spin * d * s.radius * (-0.6 * d * d).exp());
            }
        }
        v
    }

    /// The flow's velocity, sampled on a coarse map (it is smooth), for the tracing.
    fn velocity_map(&self, noise: &SeededNoise) -> Map<Vector3<f64>> {
        let shear_scale = (0..400).map(|i| {
            let lat = -PI / 2. + PI * (i as f64 + 0.5) / 400.;
            ((self.wind(lat + 1e-3) - self.wind(lat - 1e-3)) / 2e-3).abs()
        }).fold(0., f64::max).max(1e-6) * 0.5;
        let w = (self.resolution / 2).clamp(64, 1024);
        Map::build(w, w / 2, |p| self.velocity(&p, noise, shear_scale))
    }

    /// The scale of `anticyclonic`, for normalising the bands' contrast.
    fn band_scale(&self) -> f64 {
        let n = 400;
        let sum: f64 = (0..n).map(|i| {
            let lat = -PI / 2. + PI * (i as f64 + 0.5) / n as f64;
            self.anticyclonic(lat).powi(2) * lat.cos()
        }).sum();
        (sum / n as f64).sqrt().max(1e-9)
    }

    /// The cloud tops' normal albedo, as a map.
    pub fn generate(&self) -> Map<Vector3<f64>> {
        let noise = SeededNoise::new(mix(self.seed ^ 0x6a75_7069));
        let mottle = SeededNoise::new(mix(self.seed ^ 0x6d6f_7474));
        let flow = self.velocity_map(&noise);
        let scale = self.band_scale();
        let steps = 56;
        let dt = self.time / steps as f64;
        let w = self.resolution.max(16);
        // Two samples a texel, half a texel apart, where the shear would otherwise alias.
        let offset = PI / w as f64 / 2.;
        Map::build(w, w / 2, |p| {
            let (lon, lat) = lon_lat(&p);
            let mut sum = Vector3::zeros();
            for k in [-1., 1.] {
                // Back along the flow, by the midpoint rule.
                let mut q = crate::celestial::bands::direction(lon + k * offset, (lat + k * offset).clamp(-PI / 2., PI / 2.));
                for _ in 0..steps {
                    let mid = (q - flow.at(&q) * (dt / 2.)).normalize();
                    q = (q - flow.at(&mid) * dt).normalize();
                }
                sum += self.colour_at(&q, &p, &mottle, scale).rgb;
            }
            sum / 2.
        })
    }

    /// The colour the clouds started with at `q` (now carried to `p`).
    fn colour_at(&self, q: &Vector3<f64>, p: &Vector3<f64>, n: &SeededNoise, scale: f64) -> Color {
        let (_, lat) = lon_lat(q);
        // Zones and belts from the flow's rotation, their edges ragged.
        let ragged = n.fbm(q * 14., 4) * 0.8 + n.fbm(q * 40., 2) * 0.12;
        let band = 0.5 + 0.5 * ((self.anticyclonic(lat) / scale) * 1.6 + ragged).tanh();
        let mut c = self.belt * (1. - band) + self.zone * band;
        // Mottling: lighter and darker patches, and stains in the belts.
        let m = n.fbm(q * 6. + Vector3::new(2.3, 7.1, 0.4), 5);
        c = c * (1. + self.mottling * m);
        let stain = smoothstep(-0.1, 0.5, n.fbm(q * 4. + Vector3::new(9.3, 3.7, 1.2), 4)) * (1. - band) * self.mottling * 2.;
        c = c * (1. - stain.min(1.)) + self.stain * stain.min(1.);
        // The storms' own colours, carried round in their cores.
        for s in &self.storms {
            let (se, sn) = storm_axes(&s.centre);
            if q.dot(&s.centre) < 0. {
                continue;
            }
            let x = q.dot(&se) / s.elongation;
            let y = q.dot(&sn);
            let d = (x * x + y * y).sqrt() / s.radius;
            let core = (1. - smoothstep(0.5, 1., d + 0.15 * n.fbm(q * 40., 3))) * s.strength;
            c = c * (1. - core) + s.colour * core;
        }
        // Bright high clouds, drawn out along the flow.
        if self.high_clouds > 0. {
            let h = smoothstep(0.25, 0.6, n.fbm(q * 10. + Vector3::new(4.4, 0.2, 8.8), 5)) * self.high_clouds;
            c = c * (1. - h) + Color::new(0.9, 0.9, 0.9) * h;
        }
        // Haze over the poles (where it is now, not where it came from).
        let polar = smoothstep(self.polar_latitude - 0.12, self.polar_latitude + 0.2, p.y.abs().asin());
        c * (1. - polar) + self.polar * polar
    }
}

/// East and north at unit `c`.
fn storm_axes(c: &Vector3<f64>) -> (Vector3<f64>, Vector3<f64>) {
    let e = Vector3::y().cross(c);
    if e.norm() < 1e-9 {
        return tangent_basis(c);
    }
    let e = e.normalize();
    (e, c.cross(&e))
}

/// Jets in degrees and m/s, normalised to the fastest.
fn jets(list: &[(f64, f64, f64)]) -> Vec<Jet> {
    let max = list.iter().map(|j| j.1.abs()).fold(0., f64::max);
    list.iter().map(|&(lat, u, w)| Jet { latitude: lat.to_radians(), speed: u / max, width: w.to_radians() }).collect()
}

/// A storm at latitude and longitude (degrees), radius (degrees), elongation, anticyclonic
/// (true) or cyclonic, colour.
fn storm(lat: f64, lon: f64, radius: f64, elongation: f64, anticyclone: bool, colour: Color, strength: f64) -> Storm {
    let latr = lat.to_radians();
    // Anticyclones turn clockwise in the north, anticlockwise in the south.
    let sense = if anticyclone { -latr.signum() } else { latr.signum() };
    Storm { centre: direction(lon.to_radians(), latr), radius: radius.to_radians(), elongation, spin: 0.5 * sense, colour, strength }
}

impl Weather {
    fn base(seed: u64) -> Weather {
        Weather {
            seed, jets: vec![], storms: vec![], turbulence: 0.05,
            zone: Color::new(0.8, 0.75, 0.65), belt: Color::new(0.5, 0.38, 0.27),
            polar: Color::new(0.45, 0.44, 0.44), polar_latitude: 60f64.to_radians(),
            mottling: 0.15, stain: Color::new(0.55, 0.32, 0.18), high_clouds: 0., time: 3., resolution: 2048,
        }
    }

    /// Jupiter: its jets after Porco et al. (Cassini imaging, Science 299, 2003), the Great
    /// Red Spot at 22°S, the white ovals at 33°S, smaller ovals and a chaotic, hazy pole.
    pub fn jupiter(seed: u64) -> Weather {
        let mut w = Weather::base(seed);
        w.jets = jets(&[
            (7., 100., 3.5), (-7., 100., 3.5), (0., 40., 3.),
            (17., -30., 2.5), (23.7, 150., 2.), (27., -25., 2.), (31., 40., 2.), (35.5, -15., 2.), (39.5, 30., 2.),
            (43., -15., 2.), (47., 25., 2.), (51., -10., 2.), (55., 20., 2.5),
            (-18., -60., 2.5), (-26., 50., 2.5), (-30., -15., 2.), (-36., 40., 2.), (-40., -15., 2.), (-43.5, 30., 2.),
            (-47., -10., 2.), (-51., 25., 2.), (-55., -10., 2.), (-58., 20., 2.5),
        ]);
        w.zone = Color::new(0.80, 0.75, 0.64);
        w.belt = Color::new(0.47, 0.34, 0.24);
        w.stain = Color::new(0.52, 0.30, 0.17);
        w.mottling = 0.22;
        w.turbulence = 0.07;
        w.storms.push(storm(-22., 40., 5.5, 1.9, true, Color::new(0.66, 0.36, 0.23), 0.95));
        w.storms.push(storm(-33.5, -60., 2.2, 1.4, true, Color::new(0.88, 0.86, 0.82), 0.9));
        w.storms.push(storm(-33.5, 100., 1.8, 1.4, true, Color::new(0.86, 0.84, 0.8), 0.8));
        w.storms.push(storm(-41., -20., 1.3, 1.3, true, Color::new(0.85, 0.83, 0.8), 0.7));
        w.storms.push(storm(-41., 150., 1.2, 1.3, true, Color::new(0.85, 0.83, 0.8), 0.7));
        w.storms.push(storm(41., 70., 1.3, 1.3, true, Color::new(0.82, 0.78, 0.74), 0.6));
        w.storms.push(storm(20., -110., 1.5, 1.6, true, Color::new(0.65, 0.42, 0.33), 0.6));
        w.add_polar_cyclones(seed, 12);
        w
    }

    /// Saturn: a broad, fast equatorial jet (Sánchez-Lavega et al., Icarus 147, 2000), muted
    /// butterscotch bands under a thicker haze, the hexagonal jet's latitude at 76°N.
    pub fn saturn(seed: u64) -> Weather {
        let mut w = Weather::base(seed);
        w.jets = jets(&[
            (0., 450., 16.), (-5., 50., 8.), (5., 50., 8.),
            (31., -40., 4.), (36.5, 110., 3.), (42., -20., 3.), (47., 120., 3.), (55., -25., 3.), (61.5, 100., 3.), (69., -20., 3.), (76., 140., 2.5),
            (-34., -30., 4.), (-40., 90., 3.), (-48., -20., 3.), (-55., 90., 3.), (-63., -20., 3.), (-70., 100., 3.), (-77., 60., 3.),
        ]);
        w.zone = Color::new(0.80, 0.70, 0.52);
        w.belt = Color::new(0.66, 0.55, 0.40);
        w.stain = Color::new(0.68, 0.53, 0.36);
        w.polar = Color::new(0.6, 0.6, 0.6);
        w.polar_latitude = 70f64.to_radians();
        w.mottling = 0.08;
        w.turbulence = 0.03;
        w.storms.push(storm(42., 30., 1.2, 1.5, true, Color::new(0.82, 0.76, 0.62), 0.5));
        w
    }

    /// Uranus: nearly featureless, one broad retrograde equatorial current and prograde jets
    /// near 60° (Sromovsky et al., Icarus 2015), its blue-green the red light methane absorbs,
    /// with a bright polar cap and faint bands.
    pub fn uranus(seed: u64) -> Weather {
        let mut w = Weather::base(seed);
        w.jets = jets(&[(0., -80., 18.), (58., 250., 16.), (-58., 250., 16.)]);
        w.zone = Color::new(0.44, 0.64, 0.68);
        w.belt = Color::new(0.40, 0.60, 0.65);
        w.stain = w.belt;
        w.polar = Color::new(0.52, 0.70, 0.73);
        w.polar_latitude = 55f64.to_radians();
        w.mottling = 0.02;
        w.turbulence = 0.01;
        w.high_clouds = 0.03;
        w
    }

    /// Neptune: the fastest winds in the solar system, retrograde at 400 m/s on the equator
    /// and prograde at 250 m/s near 70° (Sromovsky et al., Icarus 105, 1993), a deep blue, a
    /// Great Dark Spot as Voyager 2 saw and bright methane-ice clouds beside it.
    pub fn neptune(seed: u64) -> Weather {
        let mut w = Weather::base(seed);
        w.jets = jets(&[(0., -400., 22.), (70., 250., 12.), (-70., 250., 12.)]);
        w.zone = Color::new(0.31, 0.45, 0.70);
        w.belt = Color::new(0.27, 0.40, 0.66);
        w.stain = Color::new(0.24, 0.36, 0.62);
        w.polar = Color::new(0.30, 0.42, 0.64);
        w.polar_latitude = 70f64.to_radians();
        w.mottling = 0.04;
        w.turbulence = 0.02;
        w.high_clouds = 0.12;
        w.storms.push(storm(-20., 30., 5., 1.6, true, Color::new(0.17, 0.27, 0.55), 0.8));
        w.storms.push(storm(-54., -80., 1.5, 1.3, true, Color::new(0.2, 0.3, 0.58), 0.6));
        w
    }

    /// Small cyclones crowding round the poles, as Juno found at Jupiter's.
    fn add_polar_cyclones(&mut self, seed: u64, n: usize) {
        for i in 0..n {
            let h = mix(seed ^ mix(0x706f_6c65 + i as u64));
            let north = i % 2 == 0;
            let lat = (self.polar_latitude.to_degrees() + 5. + unit(mix(h ^ 1)) * (85. - self.polar_latitude.to_degrees())) * if north { 1. } else { -1. };
            self.storms.push(storm(lat, unit(mix(h ^ 2)) * 360. - 180., 0.8 + unit(mix(h ^ 3)) * 1.5, 1., false,
                self.polar * 0.85, 0.3));
        }
    }

    /// A giant of its own from `seed`: hot or cold (`ice`: an ice giant, its colour from
    /// methane, its bands faint and broad), its jets as many as its size and spin allow
    /// (spaced by the Rhines scale), its storms where the zones' shear holds them.
    pub fn generated(seed: u64, ice: bool) -> Weather {
        let r = |k: u64| unit(mix(mix(seed) ^ mix(k)));
        let mut w = Weather::base(seed);
        if ice {
            w.jets = jets(&[(0., -(100. + 300. * r(1)), 16. + 10. * r(2)),
                (55. + 15. * r(3), 200. + 100. * r(4), 10. + 8. * r(5)), (-(55. + 15. * r(3)), 200. + 100. * r(4), 10. + 8. * r(5))]);
            // Methane's absorption: more of it, bluer and darker in the red.
            let methane = r(6);
            w.zone = Color::new(0.48 - 0.2 * methane, 0.62 - 0.14 * methane, 0.68);
            w.belt = w.zone * (0.92 + 0.05 * r(7));
            w.stain = w.belt * 0.95;
            w.polar = w.zone * (1. + 0.15 * r(8));
            w.polar_latitude = (50. + 20. * r(9)).to_radians();
            w.mottling = 0.01 + 0.04 * r(10);
            w.turbulence = 0.01 + 0.02 * r(11);
            w.high_clouds = 0.15 * r(12).powi(2);
            if r(13) < 0.5 {
                let lat = (r(14) * 2. - 1.) * 40.;
                w.storms.push(storm(lat, r(15) * 360. - 180., 3. + 3. * r(16), 1.5, true, w.belt * 0.6, 0.7));
            }
            return w;
        }
        // Jets alternate in latitude with a spacing near the Rhines scale; faster spinning
        // giants have more, narrower jets.
        let spacing = 6. + 10. * r(20);
        let equator = 100. + 400. * r(21);
        let mut list = vec![(0., equator, spacing * (0.6 + r(22)))];
        let mut lat = spacing * (0.8 + 0.4 * r(23));
        let mut k = 0;
        while lat < 80. {
            let sign = if k % 2 == 0 { -1. } else { 1. };
            let speed = (if sign > 0. { 60. + 120. * r(30 + k) } else { 15. + 40. * r(30 + k) }) * (1. - lat / 110.);
            for (hemisphere, salt) in [(1., 0u64), (-1., 1000u64)] {
                let jitter = (r(100 + k + salt) - 0.5) * spacing * 0.3;
                list.push((hemisphere * (lat + jitter), sign * speed, spacing * (0.28 + 0.1 * r(200 + k + salt))));
            }
            lat += spacing * (0.8 + 0.4 * r(40 + k));
            k += 1;
        }
        w.jets = jets(&list);
        // Colours: from pale cream to deep ochre; belts darker and redder.
        let warmth = r(50);
        let zone = Color::new(0.82, 0.76 - 0.08 * warmth, 0.64 - 0.18 * warmth);
        w.zone = zone * (0.92 + 0.1 * r(51));
        let depth = 0.55 + 0.35 * r(52);
        w.belt = Color::new(zone.rgb.x * depth, zone.rgb.y * depth * (0.85 + 0.1 * r(53)), zone.rgb.z * depth * (0.7 + 0.2 * r(54)));
        w.stain = Color::new(0.55 + 0.15 * r(55), 0.32 + 0.12 * r(56), 0.2 + 0.1 * r(57));
        w.polar = Color::new(0.42, 0.43, 0.46) * (0.9 + 0.3 * r(58));
        w.polar_latitude = (55. + 15. * r(59)).to_radians();
        w.mottling = 0.04 + 0.16 * r(60);
        w.turbulence = 0.02 + 0.05 * r(61);
        // Storms: anticyclones in the zones, where the shear is anticyclonic.
        let n = (r(62) * 8.) as u64;
        for i in 0..n {
            let mut lat = 0.;
            for attempt in 0..20 {
                let l = (r(300 + i * 31 + attempt) * 2. - 1.) * 55.;
                if w.anticyclonic(l.to_radians()) > 0. && l.abs() > 8. {
                    lat = l;
                    break;
                }
            }
            if lat == 0. {
                continue;
            }
            let big = i == 0 && r(63) < 0.4;
            let radius = if big { 4. + 3. * r(64) } else { 0.8 + 1.8 * r(400 + i) };
            let colour = if big { Color::new(0.7, 0.4 + 0.1 * r(65), 0.28) } else { zone * 1.08 };
            w.storms.push(storm(lat, r(500 + i) * 360. - 180., radius, 1.2 + 0.7 * r(600 + i), true, colour, 0.6 + 0.3 * r(700 + i)));
        }
        w.add_polar_cyclones(seed, 6 + (r(66) * 10.) as usize);
        w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jupiter_bands_follow_the_jets() {
        // The equatorial zone, the north and south equatorial belts, the north tropical zone.
        let w = Weather::jupiter(1);
        let a = |deg: f64| w.anticyclonic(deg.to_radians());
        assert!(a(3.) > 0. && a(-3.) > 0., "EZ {} {}", a(3.), a(-3.));
        assert!(a(12.) < 0. && a(-12.) < 0., "NEB/SEB {} {}", a(12.), a(-12.));
        assert!(a(20.) > 0., "NTrZ {}", a(20.));
    }

    #[test]
    fn test_map_interpolates_and_wraps() {
        let m = Map::build(64, 32, |p| p.x);
        // Across the seam at longitude ±180°.
        let v = m.at(&direction(PI - 1e-6, 0.1));
        assert!((v - (-0.1f64.cos())).abs() < 0.02, "{}", v);
        assert!((m.at(&direction(0.3, -0.4)) - 0.3f64.cos() * 0.4f64.cos()).abs() < 0.01);
    }

    #[test]
    fn test_generated_bands_are_banded() {
        // The map varies far more across latitude than along it.
        let mut w = Weather::generated(7, false);
        w.resolution = 128;
        let m = w.generate();
        let lum = |p: Vector3<f64>| crate::color::luminance(&Color { rgb: m.at(&p) });
        let (mut along, mut across) = (0., 0.);
        for i in 0..200 {
            let lon = i as f64 * 0.031;
            let lat = (i as f64 * 0.37).sin() * 0.9;
            along += (lum(direction(lon + 0.02, lat)) - lum(direction(lon, lat))).abs();
            across += (lum(direction(lon, lat + 0.02)) - lum(direction(lon, lat))).abs();
        }
        assert!(across > 1.5 * along, "{} {}", across, along);
    }
}
