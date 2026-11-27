//! A plunging breaker close up: a wave at a shore as its lip throws forward over a tube of air,
//! as a closed body of water, so that a dielectric surface and a medium inside it (the same
//! shape, `"shape"` on a volume) render it by the scene's own light transport: sunlight through
//! the thin lip, the green of the water it crosses, the tube lit through its own walls.
//!
//! Its cross-section (in z, seaward, and y, up; the wave travelling towards -z) is drawn from the
//! measured and modelled shapes of overturning waves. The cavity under the lip is an ellipse
//! whose axes are in the ratio √3, tilted so its shoreward end is lower (D. H. New, "A class of
//! elliptical free-surface flows", J. Fluid Mech. 130, 1983, after the computed overturning
//! waves of Longuet-Higgins and Cokelet, 1976): the wave's face is the cavity's seaward wall,
//! rising, concave, to the roof, and the lip is the jet thrown forward over it, thinning to
//! its tip above the trough's water (Longuet-Higgins, "Parametric solutions for breaking
//! waves", J. Fluid Mech. 121, 1982, finds the jet's tip thinning so). Behind the crest the
//! back of the wave falls to the still water.
//!
//! How far the lip has thrown (`stage`, 0 for a steep face about to pitch to 1 for a lip about
//! to land) varies along the crest, so the wave peels: one end a full tube, the other still
//! standing.

use crate::na::{Vector2, Vector3};
use crate::shapes::mesh::IndexedMesh;
use serde_json::Value;

/// A plunging breaker, the wave travelling towards -z, its crest along x.
#[derive(Clone, Debug)]
pub struct Breaker {
    /// Where the still water's level is under the middle of its crest, and the crest's middle.
    pub center: Vector3<f64>,
    /// The wave's height, trough to crest (m).
    pub height: f64,
    /// The water's depth under the crest, still water to the bed (m).
    pub depth: f64,
    /// How long the crest is, along x (m).
    pub width: f64,
    /// How far the lip has thrown at the crest's middle (0 to 1), and how much that changes
    /// from one end to the other.
    pub stage: f64,
    pub peel: f64,
    /// How much the wave varies along its crest (0 for none, 1 by default): its height, how
    /// thick its lip is and how far it throws, and the crest's line, bending in plan.
    pub variation: f64,
    /// This section's lip's thickness and throw, as multiples of the usual.
    pub lip: f64,
    pub throw: f64,
    /// How many points across a section and sections along the crest.
    pub resolution: (usize, usize),
    pub seed: u64,
}

/// The cavity's axes' ratio (New's √3) and tilt (radians, its shoreward end lower).
const ASPECT: f64 = 1.732_050_807_568_877_2;
const TILT: f64 = 0.44;
/// The cavity's half-height as a fraction of the wave's height.
const CAVITY: f64 = 0.33;
/// The trough's level below still water and the crest's above, as fractions of the height.
const TROUGH: f64 = 0.3;
/// The lip's thickness at its tip and over the cavity's roof, as fractions of the height.
const TIP: f64 = 0.04;
const ROOF: f64 = 0.23;
/// How far the lip throws, at its fullest: the angle round the cavity its tip reaches beyond
/// the roof (radians).
const THROW: f64 = 2.1;

impl Breaker {
    /// From a scene object: `{"type": "breaker", "center": [x, y, z], "height": 2, "depth": 2,
    /// "width": 40, "stage": 0.9, "peel": 0.8, "resolution": [240, 120], "seed": 0}`.
    pub fn parse(o: &Value) -> Breaker {
        use crate::scenefile::SceneFile;
        let n = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        let resolution = match o["resolution"].as_array() {
            Some(a) if a.len() == 2 => (a[0].as_u64().unwrap_or(240) as usize, a[1].as_u64().unwrap_or(120) as usize),
            _ => (240, 120),
        };
        Breaker {
            center: SceneFile::parse_vec3_def(o, "center", Vector3::zeros()),
            height: n("height", 2.).max(0.05),
            depth: n("depth", 2.).max(0.05),
            width: n("width", 40.).max(0.1),
            stage: n("stage", 0.9).clamp(0., 1.),
            peel: n("peel", 0.8),
            variation: n("variation", 1.).max(0.),
            lip: 1.,
            throw: 1.,
            resolution: (resolution.0.max(40), resolution.1.max(2)),
            seed: SceneFile::parse_int(&o["seed"], 0) as u64,
        }
    }

    /// The cavity's ellipse: its centre, semi-axes and the point at angle `phi` on it, and the
    /// outward normal there.
    fn cavity(&self) -> (Vector2<f64>, f64, f64) {
        let b = CAVITY * self.height;
        let a = ASPECT * b;
        // Its lowest point at the trough's level.
        let reach = (a * a * TILT.sin().powi(2) + b * b * TILT.cos().powi(2)).sqrt();
        (Vector2::new(0., -TROUGH * self.height + reach), a, b)
    }

    fn on_cavity(&self, phi: f64) -> (Vector2<f64>, Vector2<f64>) {
        let (c, a, b) = self.cavity();
        let (s, k) = TILT.sin_cos();
        let rotate = |v: Vector2<f64>| Vector2::new(v.x * k - v.y * s, v.x * s + v.y * k);
        let p = c + rotate(Vector2::new(a * phi.cos(), b * phi.sin()));
        let n = rotate(Vector2::new(b * phi.cos(), a * phi.sin())).normalize();
        (p, n)
    }

    /// The angles of the cavity's lowest and highest points.
    fn extremes(&self) -> (f64, f64) {
        // y = a cos φ sin β + b sin φ cos β is extreme where tan φ = b cos β / (a sin β).
        let top = (CAVITY * TILT.cos()).atan2(CAVITY * ASPECT * TILT.sin());
        (top - std::f64::consts::PI, top)
    }

    /// A cross-section at `stage`, as a closed loop of points (z, y) about the crest, the
    /// water inside, anticlockwise, `count` points (the same for every stage, so that sections
    /// join into a mesh).
    pub fn section(&self, stage: f64, count: usize) -> Vec<Vector2<f64>> {
        let h = self.height;
        let bed = -self.depth - 0.05 * h;
        let (front, back) = (-8. * h, 8. * h);
        let (bottom, top) = self.extremes();
        let tip = top + THROW * self.throw.clamp(0.5, 1.1) * stage.clamp(0., 1.);
        // Fixed shares of the points: the face and the lip's underside, the lip's outer
        // surface, the back of the wave; the rest are the corners.
        let corners = 4;
        let n_back = (count - corners) / 5;
        let n_out = (count - corners) / 4;
        let n_in = count - corners - n_back - n_out;
        let mut loop_ = Vec::with_capacity(count);
        // The bed, front, and the trough's water running in under the lip.
        loop_.push(Vector2::new(back, bed));
        loop_.push(Vector2::new(front, bed));
        loop_.push(Vector2::new(front, -TROUGH * h));
        // Up the face (the cavity's seaward wall), over its roof and down to the lip's tip.
        for i in 0..n_in {
            let phi = bottom + (tip - bottom) * i as f64 / (n_in - 1) as f64;
            loop_.push(self.on_cavity(phi).0);
        }
        // Back over the lip's outer surface, thickening from the tip to the roof.
        let thickness = |phi: f64| {
            let u = if tip > top { ((tip - phi) / (tip - top)).clamp(0., 1.) } else { 1. };
            h * self.lip.clamp(0.5, 1.6) * (TIP + (ROOF - TIP) * u.powf(0.7))
        };
        for i in 0..n_out {
            let phi = tip - (tip - top) * i as f64 / (n_out - 1) as f64;
            let (p, n) = self.on_cavity(phi);
            loop_.push(p + n * thickness(phi));
        }
        // The crest, and the back of the wave falling to the still water (a smooth step, level
        // at both ends).
        let crest = *loop_.last().unwrap();
        let fall = crest.x + 3. * h;
        for i in 1..=n_back {
            let u = i as f64 / n_back as f64;
            let s = u * u * (3. - 2. * u);
            loop_.push(Vector2::new(crest.x + (fall - crest.x) * u, crest.y * (1. - s)));
        }
        loop_.push(Vector2::new(back, 0.));
        debug_assert_eq!(loop_.len(), count);
        // Anticlockwise.
        if signed_area(&loop_) < 0. { loop_.reverse(); }
        loop_
    }

    /// The breaker as a closed mesh: sections along the crest, each at its own stage (the
    /// wave peeling) and height (varying a little along the crest), joined, and capped at the
    /// ends.
    pub fn mesh(&self) -> IndexedMesh {
        // The sides' normals from the faces, the creases (the bed's corners, the lip's tip)
        // kept sharp where faces meet at an angle.
        crate::procedural::iceberg::creased(&self.closed(), 0.6)
    }

    /// The closed mesh, its vertices shared between faces.
    /// The wave's section at `x` along the crest: its own height, lip and throw (`variation`),
    /// its stage, and how far its crest's line is bent seaward (m).
    fn at(&self, x: f64) -> (Breaker, f64, f64) {
        let (varied, stage, bend, _) = self.at_raw(x);
        (varied, stage, bend)
    }

    /// As `at`, with the stage as it would be unclamped (beyond 1 where the lip landed earlier).
    fn at_raw(&self, x: f64) -> (Breaker, f64, f64, f64) {
        let noise = crate::noise::PerlinNoise::shuffled(self.seed.wrapping_add(0xb7ea));
        let t = x / self.width;
        // Along the crest: the stage peeling, and wandering a little; the wave higher where
        // it has thrown further (the peak, the shoulder lower); the lip thicker or thinner,
        // throwing further or less; and the crest's line bending, metres in tens.
        let v = self.variation;
        let wander = |scale: f64, seed: f64| noise.noise(x / scale, seed, 0.5);
        let raw = self.stage + self.peel * t + 0.08 * v * wander(7., 2.3);
        let stage = raw.clamp(0., 1.);
        let varied = Breaker {
            height: self.height * (1. + v * (0.25 * (stage - self.stage) + 0.1 * wander(9., 1.7))),
            lip: 1. + 0.35 * v * wander(5., 3.1),
            throw: 1. + 0.08 * v * wander(6., 4.9),
            ..self.clone()
        };
        let bend = v * (1.0 * wander(20., 6.7) + 0.15 * x * x / self.width.max(1.));
        (varied, stage, bend, raw)
    }

    /// At `x` along the crest, the scene points of the tube's middle (the cavity's centre) and
    /// of the lip's tip, and how far the lip has thrown there (beyond 1 where it landed earlier,
    /// the more so the earlier): for placing a camera in the tube or the water the lip throws up
    /// as it lands.
    pub fn tube(&self, x: f64) -> (Vector3<f64>, Vector3<f64>, f64) {
        let (varied, _, bend, stage) = self.at_raw(x);
        let stage = stage.max(0.);
        let (c, _, _) = varied.cavity();
        let (_, top) = varied.extremes();
        let (tip, _) = varied.on_cavity(top + THROW * varied.throw.clamp(0.5, 1.1) * stage.min(1.));
        let place = |p: Vector2<f64>| self.center + Vector3::new(x, p.y, p.x + bend);
        (place(c), place(tip), stage)
    }

    fn closed(&self) -> IndexedMesh {
        let (count, along) = self.resolution;
        let sections: Vec<Vec<Vector3<f64>>> = (0..=along).map(|j| {
            let x = (j as f64 / along as f64 - 0.5) * self.width;
            let (varied, stage, bend) = self.at(x);
            varied.section(stage, count).iter()
                .map(|p| self.center + Vector3::new(x, p.y, p.x + bend))
                .collect()
        }).collect();
        let mut positions: Vec<Vector3<f64>> = sections.concat();
        let at = |j: usize, i: usize| j * count + i % count;
        let mut triangles = Vec::with_capacity(2 * count * along + 2 * count);
        for j in 0..along {
            for i in 0..count {
                let (a, b, c, d) = (at(j, i), at(j, i + 1), at(j + 1, i), at(j + 1, i + 1));
                triangles.push([a, c, b]);
                triangles.push([b, c, d]);
            }
        }
        // The ends, by ear clipping each end's section (in its own plane, z and y).
        let first: Vec<Vector2<f64>> = sections[0].iter().map(|p| Vector2::new(p.z, p.y)).collect();
        for [a, b, c] in ear_clip(&first) {
            triangles.push([at(0, a), at(0, b), at(0, c)]);
        }
        let last: Vec<Vector2<f64>> = sections[along].iter().map(|p| Vector2::new(p.z, p.y)).collect();
        for [a, b, c] in ear_clip(&last) {
            triangles.push([at(along, a), at(along, c), at(along, b)]);
        }
        positions.shrink_to_fit();
        IndexedMesh { positions, normals: vec![], triangles, texcoords: vec![] }
    }

    /// The breaker as a geometry.
    pub fn geometry(&self) -> crate::shapes::mesh::SmoothMesh {
        crate::shapes::mesh::SmoothMesh::from_indexed(&[self.mesh()])
    }
}

/// The breaker's surface as an object's geometry: its mesh, through which shadow rays pass, as
/// through the sea's transparent water (`Geometry::transmits`), losing what Fresnel's
/// equations reflect at each crossing (the medium inside, the same shape, takes what the water
/// absorbs and scatters on the way). So the water inside is lit by the sun through the surface,
/// as next-event estimation can't through a refracting one; the bend at the surface is left out.
pub struct BreakerSurface {
    pub mesh: crate::shapes::mesh::SmoothMesh,
}

impl crate::shapes::geometry::Geometry for BreakerSurface {
    fn intersects(&self, r: &crate::ray::Ray) -> Option<crate::intersection::RawIntersection> { self.mesh.intersects(r) }
    fn intersects_beyond(&self, r: &crate::ray::Ray, min: f64) -> Option<crate::intersection::RawIntersection> { self.mesh.intersects_beyond(r, min) }
    fn intersects_range(&self, r: &crate::ray::Ray, min: f64, max: f64) -> Option<crate::intersection::RawIntersection> { self.mesh.intersects_range(r, min, max) }
    fn occludes(&self, r: &crate::ray::Ray, min: f64, max: f64) -> bool { self.mesh.occludes(r, min, max) }
    fn fast_intersects(&self, r: &crate::ray::Ray) -> bool { self.mesh.fast_intersects(r) }
    fn bounds(&self) -> crate::shapes::bbox::BBox { self.mesh.bounds() }
    fn primitives(&self) -> u64 { self.mesh.primitives() }
    fn kind(&self) -> &'static str { "breaking wave" }
    fn transmits(&self) -> bool { true }

    fn transmittance(&self, r: &crate::ray::Ray, min: f64, max: f64) -> f64 {
        crate::color::luminance(&self.transmittance_color(r, min, max))
    }

    fn transmittance_color(&self, r: &crate::ray::Ray, min: f64, max: f64) -> crate::color::Color {
        let rd = r.rd.normalize();
        let mut t = min;
        let mut through = 1.;
        for _ in 0..16 {
            let Some(hit) = self.mesh.intersects_range(r, t, max) else { break };
            through *= 1. - crate::ocean::fresnel(rd.dot(&hit.normal).abs());
            t = hit.dist + 1e-4 * (1. + hit.dist);
        }
        crate::color::Color::white() * through
    }
}

/// The breaker's water at its surface: a dielectric of water's refractive index, its surface
/// not glassy but rippled and streaked (`BreakerWater::relief`), whose rays refracted out of
/// the water don't count the sun's disc when they meet it (the sun was sampled from inside,
/// through the surface: `BreakerSurface`); reflections off the outside still do.
pub struct BreakerWater {
    noise: crate::noise::PerlinNoise,
    /// The trough's level and the wave's height, for where foam lies.
    trough: f64,
    height: f64,
}

impl BreakerWater {
    pub fn new(breaker: &Breaker) -> BreakerWater {
        BreakerWater {
            noise: crate::noise::PerlinNoise::shuffled(breaker.seed.wrapping_add(0x5717)),
            trough: breaker.center.y - TROUGH * breaker.height,
            height: breaker.height,
        }
    }

    /// How much of the surface at `p`, of normal `normal`, is foam, 0 or 1 but at its edges:
    /// the lace earlier waves left on the water, a third of it in patches a metre or so across
    /// and holes between, where the water lies nearly level and low on the wave (the trough in
    /// front); and up the face and over the lip, that foam drawn up by the flow and stretched
    /// into long thin lines along it (up the face and along the crest, as the wave peels: a
    /// diagonal), branching and
    /// joining, a few centimetres wide, more in some stretches of the crest than others; none
    /// on the back of the wave.
    fn foam(&self, p: &Vector3<f64>, normal: &Vector3<f64>) -> f64 {
        let n = |x: f64, y: f64, z: f64| self.noise.noise(x, y, z);
        let up = (p.y - self.trough) / self.height;
        let level = crate::math::smoothstep(0.85, 0.97, normal.y);
        let low = 1. - crate::math::smoothstep(0.1, 0.25, up);
        let mut cover = 0.;
        if level * low > 0. {
            // The lace: two octaves, its edges sharpened.
            // (Fine: patches a third of a metre across, holes between; coarser ones read as stains.)
            let lace = 0.65 * n(p.x / 0.35 + 31.7, p.y / 0.35, p.z / 0.35) + 0.35 * n(p.x / 0.12 + 5.2, p.y / 0.12, p.z / 0.12);
            cover = level * low * crate::math::smoothstep(0.08, 0.12, lace);
        }
        // The lines, where the face rises (not the level water, nor the back, which faces
        // seaward, +z): where a noise stretched along the flow crosses zero, the crest's
        // coordinate wandering so the lines wind.
        // (Little on the lip's underside, facing down into the tube: clear water, lit through.)
        // (Nor on the crest's top and the lip's upper surface, facing up: there the lines'
        // patches broke the crest's edge into tufts.)
        let face = (1. - level) * (1. - crate::math::smoothstep(-0.1, 0.3, normal.z)) * (1. - 0.85 * (1. - crate::math::smoothstep(-0.5, -0.1, normal.y)))
            * (1. - crate::math::smoothstep(0.3, 0.6, normal.y));
        if face > 0. {
            // (The flow runs along the crest as well as up the face as the wave peels: the lines
            // lie diagonally, across the crest's coordinate as it rises.)
            let x = p.x + 1.2 * (p.y - self.trough) + 0.6 * n(p.y / 0.7 + 3.9, p.z / 0.7, p.x / 1.5);
            let line = 1. - (n(x / 0.18 + 41.3, p.y / 1.3, p.z / 1.3) + 0.3 * n(x / 0.07 + 2.2, p.y / 0.5, p.z / 0.5)).abs();
            let patches = crate::math::smoothstep(0.0, 0.3, n(p.x / 4. + 13.7, p.y / 2., p.z / 2.));
            cover = cover.max(face * patches * crate::math::smoothstep(0.7, 0.73, line));
        }
        cover.clamp(0., 1.)
    }

    /// The surface's small relief at `p` (m): capillary ripples a few centimetres long, in three
    /// octaves; streaks drawn out along the flow (up the face and along the crest, as the wave
    /// peels: a diagonal, narrow across it); and a gentle
    /// undulation half a metre across.
    fn relief(&self, p: &Vector3<f64>, streaked: f64) -> f64 {
        let n = |x: f64, y: f64, z: f64| self.noise.noise(x, y, z);
        // (Small: a breaking wave's face is glassy, and stronger ripples smeared everything seen
        // through or in it.)
        let ripples = 0.00024 * n(p.x / 0.03, p.y / 0.03, p.z / 0.03)
            + 0.00012 * n(p.x / 0.015 + 7.1, p.y / 0.015, p.z / 0.015)
            + 0.0005 * n(p.x / 0.08 + 3.3, p.y / 0.08, p.z / 0.08);
        // (Diagonal, as the foam's lines: along the crest alone they ringed the tube's roof.)
        let x = p.x + 1.2 * (p.y - self.trough);
        // (Faint: stronger, they swirled over every surface like brushstrokes.)
        let streaks = streaked * 0.0005 * n(x / 0.1 + 11.9, p.y / 1.2, p.z / 1.2);
        let swell = 0.01 * n(p.x / 0.5 + 21.4, p.y / 0.5, p.z / 0.5);
        ripples + streaks + swell
    }

    /// The normal `n` at `p` tilted by the relief's slope across the surface.
    fn rippled(&self, p: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
        let e = 0.002;
        // Streaks where the water runs up a face, none where it lies level.
        let streaked = 1. - crate::math::smoothstep(0.8, 0.95, n.y);
        let h0 = self.relief(p, streaked);
        let g = Vector3::new(
            self.relief(&(p + Vector3::x() * e), streaked) - h0,
            self.relief(&(p + Vector3::y() * e), streaked) - h0,
            self.relief(&(p + Vector3::z() * e), streaked) - h0,
        ) / e;
        let along = g - n * g.dot(n);
        (n - along).normalize()
    }
}

impl crate::material::model::MaterialModel for BreakerWater {
    fn scatter(&self, r: &crate::ray::Ray, intersection: &crate::intersection::Intersection, s: &crate::scene::Scene) -> crate::material::model::ScatteredRay {
        // (Where the tilt would put the ray on the other side of the surface, at a grazing
        // angle, the surface as it is.)
        let tilted = self.rippled(&intersection.point, &intersection.normal);
        let normal = if tilted.dot(&r.rd).signum() == intersection.normal.dot(&r.rd).signum() { tilted } else { intersection.normal };
        let rippled = crate::intersection::Intersection { normal, ..*intersection };
        // Foam floating on the surface, seen from above it: a thin layer of bubbles, sending
        // back diffusely 0.4 of the light and letting 0.4 through, diffusely, into the water
        // (so it is lit from below by the water's glow as well as by the sky), one or the other
        // by half: each weighted twice.
        let foam = self.foam(&intersection.point, &intersection.normal);
        if foam > 0. && r.rd.dot(&intersection.normal) < 0. && crate::geometry::rand() < foam {
            let up = intersection.normal;
            let offset = crate::material::functions::surface_offset(intersection, s).max(1e-4);
            let albedo = crate::color::Color::new(0.8, 0.82, 0.81).as_reflectance();
            if crate::geometry::rand() < 0.5 {
                return crate::material::functions::scatter_bsdf(&crate::material::bsdf::LambertianBSDF { albedo, normal: up }, r, &(intersection.point + up * offset), s);
            }
            // Through it, a diffuse lobe on the water's side, which samples the sun through the
            // wave behind it (foam on a backlit face glows by what it lets through).
            return crate::material::functions::scatter_bsdf(&crate::material::bsdf::LambertianBSDF { albedo, normal: -up }, r, &(intersection.point - up * offset), s);
        }
        let mut out = crate::material::functions::scatter_dielectric(crate::ocean::WATER_INDEX, crate::color::Color::white(), r, &rippled,
            crate::material::functions::surface_offset(intersection, s));
        let leaving = r.rd.dot(&intersection.normal) > 0.;
        if leaving && out.ray.as_ref().is_some_and(|ray| ray.rd.dot(&intersection.normal) > 0.) {
            out.light_sampling = crate::material::model::LightSampling::Directional;
        }
        out
    }
}

/// The white water a breaker throws up where its lip lands, as a density (0 to 1) for a
/// volume: about the tip of the lip wherever it has thrown fully (stage near 1), a mass of
/// water and air churned below and about it, and the splash it throws up ahead of it (the
/// "splash-up" of a plunging breaker), thinning as it rises, broken into lumps and wisps. The
/// lip's tips along the crest are found once, every ten centimetres (`Breaker::tube`).
pub struct Whitewater {
    /// The first section's x, the spacing, and each section's tip (y, z) and stage.
    start: f64,
    step: f64,
    tips: Vec<(f64, f64, f64)>,
    height: f64,
    noise: crate::noise::PerlinNoise,
}

impl Whitewater {
    pub fn new(breaker: &Breaker) -> Whitewater {
        let step = 0.1;
        let start = breaker.center.x - 0.5 * breaker.width;
        let n = (breaker.width / step).ceil() as usize + 1;
        let tips = (0..n).map(|i| {
            let (_, tip, stage) = breaker.tube(start + i as f64 * step - breaker.center.x);
            (tip.y, tip.z, stage)
        }).collect();
        Whitewater { start, step, tips, height: breaker.height, noise: crate::noise::PerlinNoise::shuffled(breaker.seed.wrapping_add(0x3a11)) }
    }
}

impl crate::volume::DensityField for Whitewater {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        let f = (p.x - self.start) / self.step;
        if f.is_nan() || f < 0. || f >= (self.tips.len() - 1) as f64 { return 0.; }
        let (i, a) = (f.floor() as usize, f.fract());
        let (y0, z0, s0) = self.tips[i];
        let (y1, z1, s1) = self.tips[i + 1];
        let (ty, tz, stage) = (y0 + (y1 - y0) * a, z0 + (z1 - z0) * a, s0 + (s1 - s0) * a);
        let landed = crate::math::smoothstep(0.96, 1.0, stage);
        if landed <= 0. { return 0.; }
        let h = self.height;
        // How long ago it landed, by how far the stage has run beyond 1: fresh, the churned mass
        // about the tip and the splash it throws up ahead; then settling into a lower bore of
        // foam at the trough's level, thinning.
        let age = ((stage - 1.) / 0.08).max(0.);
        let fresh = (-age * age).exp();
        let settle = (age * 0.35 * h).min(0.3 * h);
        let (dy, dz) = (p.y - ty + settle, p.z - tz);
        let mass = (-(dy + 0.1 * h).powi(2) / (0.25 * h).powi(2) - dz.powi(2) / (0.4 * h * (1. + 0.5 * age.min(2.))).powi(2)).exp();
        let rise = (dy / (0.9 * h)).clamp(0., 1.);
        let ahead = (-(dz + 0.35 * h + 0.3 * h * rise).powi(2) / (0.3 * h).powi(2)).exp();
        let splash = if dy > 0. { fresh * ahead * (1. - rise).powi(2) } else { 0. };
        let thinning = 0.25 + 0.75 * (-0.5 * age).exp();
        let lumps = 0.5 + 0.5 * self.noise.noise(p.x / 0.25, p.y / 0.25, p.z / 0.25) + 0.3 * self.noise.noise(p.x / 0.08 + 9.1, p.y / 0.08, p.z / 0.08);
        (landed * (mass * thinning).max(splash) * (1.6 * lumps - 0.3)).clamp(0., 1.)
    }

    fn majorant(&self) -> f64 { 1. }
}

/// The air a breaker's lip carries, as a density (0 to 1) for a volume of the wave's shape: the
/// thrown jet is torn and entrains air, which scatters light white and makes the lip a lit,
/// tinted sheet rather than clear glass; most towards its tip, none in the body of the wave
/// below the cavity's roof. The cavity and the lip's reach along the crest are found once,
/// every ten centimetres.
pub struct Aeration {
    start: f64,
    step: f64,
    /// Each section's cavity centre (y, z), semi-axes, and the angles of its roof and the lip's tip.
    sections: Vec<(f64, f64, f64, f64, f64, f64)>,
    noise: crate::noise::PerlinNoise,
}

impl Aeration {
    pub fn new(breaker: &Breaker) -> Aeration {
        let step = 0.1;
        let start = breaker.center.x - 0.5 * breaker.width;
        let n = (breaker.width / step).ceil() as usize + 1;
        let sections = (0..n).map(|i| {
            let x = start + i as f64 * step - breaker.center.x;
            let (varied, stage, bend) = breaker.at(x);
            let (c, a, b) = varied.cavity();
            let (_, top) = varied.extremes();
            let tip = top + THROW * varied.throw.clamp(0.5, 1.1) * stage;
            (breaker.center.y + c.y, breaker.center.z + c.x + bend, a, b, top, tip)
        }).collect();
        Aeration { start, step, sections, noise: crate::noise::PerlinNoise::shuffled(breaker.seed.wrapping_add(0xae7a)) }
    }
}

impl crate::volume::DensityField for Aeration {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        let f = (p.x - self.start) / self.step;
        if f.is_nan() || f < 0. || f >= (self.sections.len() - 1) as f64 { return 0.; }
        let i = f.round() as usize;
        let (cy, cz, a, b, top, tip) = self.sections[i];
        if tip <= top + 0.05 { return 0.; }
        // Where p is about the cavity: its angle (as the section's) and its distance out in
        // units of the ellipse.
        let (s, k) = TILT.sin_cos();
        let (dz, dy) = (p.z - cz, p.y - cy);
        let (u, v) = (dz * k + dy * s, -dz * s + dy * k);
        let mut phi = (v / b).atan2(u / a);
        if phi < top - 1. { phi += std::f64::consts::TAU; }
        let out = ((u / a).powi(2) + (v / b).powi(2)).sqrt();
        if out < 1. { return 0.; }
        // (Some from the roof on, more towards the tip: the tube's roof is turbid too.)
        let thrown = (0.3 + 0.7 * crate::math::smoothstep(top, tip, phi)) * crate::math::smoothstep(top - 0.6, top, phi)
            * (1. - crate::math::smoothstep(tip - 0.05, tip + 0.15, phi));
        let near = 1. - crate::math::smoothstep(1., 1. + 1.2 * ROOF * 3., out);
        let torn = 0.6 + 0.4 * self.noise.noise(p.x / 0.2, p.y / 0.2, p.z / 0.2);
        (thrown.powf(0.7) * near * torn).clamp(0., 1.)
    }

    fn majorant(&self) -> f64 { 1. }
}

/// The spray a breaker throws off its crest, as a density (0 to 1) for a volume: droplets torn
/// from the crest and the lip, in wisps within the better part of a metre above each section's
/// crest, carried back seaward by the wind off the land; found from each section's crest, every
/// ten centimetres, so it follows the crest as it rises, falls and bends.
pub struct Spray {
    start: f64,
    step: f64,
    /// Each section's crest (y, z).
    crests: Vec<(f64, f64)>,
    height: f64,
    noise: crate::noise::PerlinNoise,
}

impl Spray {
    pub fn new(breaker: &Breaker) -> Spray {
        let step = 0.1;
        let start = breaker.center.x - 0.5 * breaker.width;
        let n = (breaker.width / step).ceil() as usize + 1;
        let crests = (0..n).map(|i| {
            let x = start + i as f64 * step - breaker.center.x;
            let (varied, stage, bend) = breaker.at(x);
            let section = varied.section(stage, 120);
            let crest = section.iter().fold(Vector2::new(0., f64::MIN), |a, p| if p.y > a.y { *p } else { a });
            (breaker.center.y + crest.y, breaker.center.z + crest.x + bend)
        }).collect();
        Spray { start, step, crests, height: breaker.height, noise: crate::noise::PerlinNoise::shuffled(breaker.seed.wrapping_add(0x5b7a)) }
    }
}

impl crate::volume::DensityField for Spray {
    fn density(&self, p: &Vector3<f64>) -> f64 {
        let f = (p.x - self.start) / self.step;
        if f.is_nan() || f < 0. || f >= (self.crests.len() - 1) as f64 { return 0.; }
        let (i, a) = (f.floor() as usize, f.fract());
        let (y0, z0) = self.crests[i];
        let (y1, z1) = self.crests[i + 1];
        let (cy, cz) = (y0 + (y1 - y0) * a, z0 + (z1 - z0) * a);
        let h = self.height;
        let up = (p.y - cy) / (0.4 * h);
        if up < -0.15 || up > 1. { return 0.; }
        // Blown back: the higher, the further seaward.
        let back = (p.z - cz - 0.4 * h * up.max(0.)) / (0.35 * h);
        let sheet = (-back * back).exp() * (1. - up.max(0.)).powi(2) * (1. - crate::math::smoothstep(-0.15, 0., -up));
        let n = |s: f64, o: f64| self.noise.noise(p.x / s + o, p.y / s, p.z / s);
        let wisps = 0.5 + 0.5 * n(0.3, 0.) + 0.35 * n(0.1, 7.7);
        (sheet * (2. * wisps - 0.6)).clamp(0., 1.)
    }

    fn majorant(&self) -> f64 { 1. }
}

/// Twice the signed area of a loop (positive anticlockwise).
fn signed_area(p: &[Vector2<f64>]) -> f64 {
    (0..p.len()).map(|i| { let (a, b) = (p[i], p[(i + 1) % p.len()]); a.x * b.y - b.x * a.y }).sum()
}

/// Triangles covering a simple anticlockwise loop, by clipping ears.
fn ear_clip(p: &[Vector2<f64>]) -> Vec<[usize; 3]> {
    let cross = |o: Vector2<f64>, a: Vector2<f64>, b: Vector2<f64>| (a - o).x * (b - o).y - (a - o).y * (b - o).x;
    let mut left: Vec<usize> = (0..p.len()).collect();
    let mut out = Vec::with_capacity(p.len());
    let mut guard = 0;
    while left.len() > 3 && guard < 4 * p.len() * p.len() {
        guard += 1;
        let n = left.len();
        let mut clipped = false;
        for k in 0..n {
            let (i0, i1, i2) = (left[(k + n - 1) % n], left[k], left[(k + 1) % n]);
            let (a, b, c) = (p[i0], p[i1], p[i2]);
            if cross(a, b, c) <= 1e-12 { continue; }
            let inside = left.iter().any(|&j| {
                if j == i0 || j == i1 || j == i2 { return false; }
                let q = p[j];
                cross(a, b, q) > 0. && cross(b, c, q) > 0. && cross(c, a, q) > 0.
            });
            if inside { continue; }
            out.push([i0, i1, i2]);
            left.remove(k);
            clipped = true;
            break;
        }
        if !clipped { break; }
    }
    if left.len() == 3 { out.push([left[0], left[1], left[2]]); }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wave() -> Breaker {
        Breaker::parse(&serde_json::json!({"height": 2, "depth": 2, "width": 10, "resolution": [200, 8]}))
    }

    /// Whether two segments cross (not merely touch at a shared end).
    fn crosses(a: Vector2<f64>, b: Vector2<f64>, c: Vector2<f64>, d: Vector2<f64>) -> bool {
        let cross = |o: Vector2<f64>, p: Vector2<f64>, q: Vector2<f64>| (p - o).x * (q - o).y - (p - o).y * (q - o).x;
        let (d1, d2, d3, d4) = (cross(c, d, a), cross(c, d, b), cross(a, b, c), cross(a, b, d));
        d1 * d2 < 0. && d3 * d4 < 0.
    }

    #[test]
    fn test_a_breakers_section_is_a_simple_loop() {
        for (stage, lip, throw) in [(0., 1., 1.), (0.3, 1., 1.), (0.7, 1., 1.), (1., 1., 1.), (1., 0.5, 1.1), (1., 1.6, 0.5), (0.6, 1.6, 1.1)] {
            let w = Breaker { lip, throw, ..wave() };
            let s = w.section(stage, 200);
            assert_eq!(s.len(), 200);
            assert!(signed_area(&s) > 0.);
            let n = s.len();
            for i in 0..n {
                for j in i + 2..n {
                    if (j + 1) % n == i { continue; }
                    assert!(!crosses(s[i], s[(i + 1) % n], s[j], s[(j + 1) % n]), "stage {}: edges {} and {} cross", stage, i, j);
                }
            }
            // Its ends are covered by triangles of the same area.
            let area: f64 = ear_clip(&s).iter().map(|t| 0.5 * ((s[t[1]] - s[t[0]]).perp(&(s[t[2]] - s[t[0]])))).sum();
            assert!((area - 0.5 * signed_area(&s)).abs() < 1e-6 * area.abs(), "{} {}", area, 0.5 * signed_area(&s));
        }
    }

    #[test]
    fn test_a_plunging_lip_hangs_over_a_tube_of_air() {
        let w = wave();
        let h = w.height;
        // The crest stands about 0.7 of the height above still water, the trough 0.3 below.
        let s = w.section(1., 400);
        let crest = s.iter().map(|p| p.y).fold(f64::MIN, f64::max);
        assert!((crest - 0.7 * h).abs() < 0.1 * h, "{}", crest);
        // Thrown forward: a vertical line through the cavity, shoreward of the face, meets the
        // water in the lip above air, and the trough's water below: three crossings or more
        // (an overhang), where a wave about to pitch has one.
        let (c, _, _) = w.cavity();
        let crossings = |s: &[Vector2<f64>], z: f64| (0..s.len()).filter(|&i| {
            let (a, b) = (s[i], s[(i + 1) % s.len()]);
            (a.x - z) * (b.x - z) < 0.
        }).count();
        assert!(crossings(&s, c.x - 0.3 * h) >= 3, "{}", crossings(&s, c.x - 0.3 * h));
        assert_eq!(crossings(&w.section(0., 400), c.x - 0.6 * h), 2);
        // The mesh is closed: every edge shared by two triangles.
        let m = w.closed();
        let mut edges = std::collections::HashMap::new();
        for t in &m.triangles {
            for k in 0..3 {
                let (a, b) = (t[k], t[(k + 1) % 3]);
                *edges.entry((a.min(b), a.max(b))).or_insert(0) += 1;
            }
        }
        assert!(edges.values().all(|&c| c == 2), "{} open edges", edges.values().filter(|&&c| c != 2).count());
        // And its triangles face out: the volume it bounds, by the divergence theorem, is
        // positive, about the section's area times the width.
        let volume: f64 = m.triangles.iter().map(|t| m.positions[t[0]].dot(&m.positions[t[1]].cross(&m.positions[t[2]])) / 6.).sum();
        let area = 0.5 * signed_area(&w.section(0.5, 200));
        assert!(volume > 0.5 * area * w.width && volume < 1.5 * area * w.width, "{} vs {}", volume, area * w.width);
    }
}
