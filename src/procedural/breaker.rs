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

/// A section's points, each one's share of the cross waves, and its share of the roller (how
/// far it bulges, m, and how much it is churned): `Breaker::churned_section`.
pub type ChurnedSection = (Vec<Vector2<f64>>, Vec<f64>, Vec<(f64, f64)>);

/// The wave tank's sections of a breaker (`wave_tank::breaker_frames`, kept in
/// breaker_frames.json): frames evenly in time from before its face overturns to its jet's
/// touchdown, each the same number of points from behind its crest to in front, x shoreward,
/// in units of its crest's height.
fn simulated_frames() -> &'static Vec<Vec<Vector2<f64>>> {
    static FRAMES: std::sync::OnceLock<Vec<Vec<Vector2<f64>>>> = std::sync::OnceLock::new();
    FRAMES.get_or_init(|| {
        let v: Value = serde_json::from_str(include_str!("breaker_frames.json")).unwrap();
        v["frames"].as_array().unwrap().iter().map(|f| {
            f.as_array().unwrap().iter().map(|p| Vector2::new(p[0].as_f64().unwrap(), p[1].as_f64().unwrap())).collect()
        }).collect()
    })
}

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
    /// How much foam earlier waves left on the water before it (0 to 1: none for the first of
    /// a set, glassy).
    pub lace: f64,
    /// This section's lip's thickness and throw, as multiples of the usual.
    pub lip: f64,
    pub throw: f64,
    /// How far this section's lip's very edge reaches beyond the rest of it, radians round the
    /// cavity (its fingers).
    pub finger: f64,
    /// How far this section has collapsed into a broken bore's turbulent roller (0 to 1).
    pub churn: f64,
    /// How many points across a section and sections along the crest.
    pub resolution: (usize, usize),
    /// Whether its sections are the wave tank's (`"model": "simulated"`: a breaker followed
    /// by potential flow from before its face overturns to its jet's touchdown,
    /// `wave_tank::breaker_frames`) rather than drawn (New's elliptical cavity under a thrown
    /// lip, the default).
    pub simulated: bool,
    pub seed: u64,
}

/// The cavity's axes' ratio (New's √3) and tilt (radians, its shoreward end lower).
const ASPECT: f64 = 1.732_050_807_568_877_2;
const TILT: f64 = 0.44;
/// The cavity's half-height as a fraction of the wave's height.
const CAVITY: f64 = 0.28;
/// The trough's level below still water and the crest's above, as fractions of the height.
const TROUGH: f64 = 0.3;
/// The lip's thickness at its tip and over the cavity's roof, as fractions of the height.
const TIP: f64 = 0.10;
const ROOF: f64 = 0.40;
/// How far the lip throws, at its fullest: the angle round the cavity its tip reaches beyond
/// the roof (radians).
const THROW: f64 = 1.9;

impl Breaker {
    /// From a scene object: `{"type": "breaker", "center": [x, y, z], "height": 2, "depth": 2,
    /// "width": 40, "stage": 0.9, "peel": 0.8, "resolution": [240, 120], "seed": 0}`.
    pub fn parse(o: &Value) -> Breaker {
        use crate::scenefile::SceneFile;
        let n = |k: &str, d: f64| SceneFile::parse_number(&o[k], d);
        let resolution = match o["resolution"].as_array() {
            Some(a) if a.len() == 2 => (a[0].as_u64().unwrap_or(360) as usize, a[1].as_u64().unwrap_or(600) as usize),
            _ => (360, 600),
        };
        Breaker {
            center: SceneFile::parse_vec3_def(o, "center", Vector3::zeros()),
            height: n("height", 2.).max(0.05),
            depth: n("depth", 2.).max(0.05),
            width: n("width", 40.).max(0.1),
            stage: n("stage", 0.9).clamp(0., 1.),
            peel: n("peel", 0.8),
            variation: n("variation", 1.).max(0.),
            lace: n("lace", 1.).clamp(0., 1.),
            lip: 1.,
            throw: 1.,
            finger: 0.,
            churn: 0.,
            resolution: (resolution.0.max(60), resolution.1.max(2)),
            simulated: o["model"].as_str() == Some("simulated"),
            seed: SceneFile::parse_int(&o["seed"], 0) as u64,
        }
    }

    /// How far round the cavity from its roof the lip's tip has reached at `stage` (radians):
    /// little at first (a shoulder's face steepens before its lip throws), the whole throw at 1.
    fn reach(&self, stage: f64) -> f64 {
        // (Never quite none: the lip's outer arc keeps distinct points.)
        THROW * self.throw.clamp(0.5, 1.1) * (0.02 + 0.98 * stage.clamp(0., 1.).powf(1.6))
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
        self.weighted_section(stage, count).0
    }

    /// As `section`, with each point's share of the cross waves' relief (`CrossWaves`): none
    /// on the bed and the ends, none near the lip's thin tip (where it could break through
    /// itself), all elsewhere.
    pub fn weighted_section(&self, stage: f64, count: usize) -> (Vec<Vector2<f64>>, Vec<f64>) {
        let (loop_, weight, _) = self.churned_section(stage, count);
        (loop_, weight)
    }

    /// As `weighted_section`, with each point's share of the broken bore's roller (`churn`):
    /// how far it bulges out, metres, and how much it is churned into lumps (0 to 1): the
    /// roller's rounded mass high on the bore's front, its lumps over the front, the crest and
    /// the top of the back, a little over the water before it.
    pub fn churned_section(&self, stage: f64, count: usize) -> ChurnedSection {
        if self.simulated { return self.simulated_section(stage, count); }
        let h = self.height;
        let bed = -self.depth - 0.05 * h;
        let (front, back) = (-12. * h, 8. * h);
        let (bottom, top) = self.extremes();
        let tip = top + self.reach(stage);
        // Fixed shares of the points: the trough's water in front, the face and the lip's
        // underside, the lip's outer surface, the back of the wave, the still water behind;
        // the rest are the bed's two corners.
        let corners = 2;
        let n_trough = (count - corners) / 8;
        let n_still = (count - corners) / 10;
        let n_back = (count - corners) / 6;
        let n_out = (count - corners) / 5;
        let n_in = count - corners - n_trough - n_still - n_back - n_out;
        let mut loop_ = Vec::with_capacity(count);
        let mut weight = Vec::with_capacity(count);
        let mut roller: Vec<(f64, f64)> = Vec::with_capacity(count);
        let churn = self.churn.clamp(0., 1.);
        // Near the lip's tip, by the arc length from it along the cavity.
        let (_, a, b) = self.cavity();
        let reach = 0.5 * (a + b);
        let from_tip = |phi: f64| crate::math::smoothstep(0.05, 0.35, (tip - phi).abs() * reach);
        // The bed, front, and the trough's water running in under the lip.
        loop_.push(Vector2::new(back, bed));
        loop_.push(Vector2::new(front, bed));
        weight.extend([0., 0.]);
        roller.extend([(0., 0.), (0., 0.)]);
        // How hollow the face is: a steep slope on the shoulder, before the lip throws, from a
        // toe a wave's height and a half in front of the crest; hollowing into the cavity's
        // wall as the lip throws.
        let hollow = crate::math::smoothstep(0.15, 0.7, stage);
        let lip = h * self.lip.clamp(0.5, 1.6);
        // The lip's thickness from its tip to the roof: a tip no thinner than its root on a face
        // not yet hollow (a crest, not a lip), thinning as it throws.
        let tip_thickness = TIP + (ROOF - TIP) * (1. - hollow);
        let thickness = |phi: f64| {
            let u = if tip > top { ((tip - phi) / (tip - top)).clamp(0., 1.) } else { 1. };
            // (An S-curve, level where it meets the roof: rising into it, it met the back of
            // the wave at an angle, a line along the top of the barrel.)
            lip * (tip_thickness + (ROOF - tip_thickness) * u * u * (3. - 2. * u))
        };
        // How far the lip has unfolded from the crest: later than the face hollows, so that a
        // shoulder keeps a smooth crest (unfolding with the face, a third-hollow face carried a
        // third of a hook, a row of teeth along the shoulder).
        let unfold = hollow.powi(3);
        let (roof, up) = self.on_cavity(top);
        let crest_point = roof + up * lip * ROOF;
        let toe = Vector2::new(roof.x - 1.5 * h, -TROUGH * h);
        // The face: on a face not yet hollow, a slope rising from the toe to the crest itself
        // (steepening towards it), and what lip there is folded onto the crest; hollowing into
        // the cavity's wall and the lip thrown from its roof.
        let face = |phi: f64| {
            let wall = self.on_cavity(phi).0;
            if phi >= top { return crest_point + (wall - crest_point) * unfold; }
            let u = ((phi - bottom) / (top - bottom)).clamp(0., 1.);
            // (Partly straight, so that it meets the crest at about 50 degrees, not standing
            // vertical below it: on a collapsed bore that strip stood as a palisade.)
            let along = 0.4 * u + 0.6 * (1. - (1. - u).powi(2));
            let slope = Vector2::new(toe.x + (crest_point.x - toe.x) * along, toe.y + (crest_point.y - toe.y) * u);
            slope + (wall - slope) * hollow
        };
        let start = face(bottom);
        for i in 0..n_trough {
            let u = i as f64 / n_trough as f64;
            loop_.push(Vector2::new(front + (start.x - front) * u, -TROUGH * h));
            weight.push(if i == 0 { 0. } else { 1. });
            roller.push((0., churn * 0.3 * u.powi(8)));
        }
        // Up the face (the cavity's seaward wall), over its roof and down to the lip's tip.
        // (The face and the lip, drawn smooth by the flow over them, carry a third of the cross
        // waves the level water does, less up from the face's foot.)
        const GLASSY: f64 = 0.35;
        // The edge's fingers: the last of the lip stretched or drawn in, the rest of it as it is.
        // (Over the last 0.6 radians, so that the arc never folds back for fingers within ±0.3.)
        // (Only on a lip thrown, once the face is half hollow: a lip barely there, on the
        // shoulder, showed them as a row of teeth.)
        // And in proportion to how far the lip has thrown: the same swing on a short lip made
        // each finger large beside it.
        let finger = self.finger.clamp(-0.3, 0.3) * crate::math::smoothstep(0.5, 1., hollow) * (tip - top) / THROW;
        let fingered = |phi: f64| phi + finger * crate::math::smoothstep(tip - 0.6, tip, phi);
        for i in 0..n_in {
            let phi = fingered(bottom + (tip - bottom) * i as f64 / (n_in - 1) as f64);
            loop_.push(face(phi));
            let up = ((phi - bottom) / (top - bottom)).clamp(0., 1.);
            weight.push(from_tip(phi) * (GLASSY + (1. - GLASSY) * (1. - up).powi(2)));
            // (On what is left of the lip, only as it folds away.)
            let lip_left = if phi > top { 1. - hollow } else { 1. };
            // (Growing up to the crest and over it, the roller rounding the bore's top: bulging
            // most below the crest left a notch above, broken into ledges by the lumps.)
            roller.push((churn * lip_left * 0.2 * h * up.powi(3), churn * lip_left * (0.3 + 0.7 * up)));
        }
        // Back over the lip's outer surface, thickening from the tip to the roof.
        for i in 0..n_out {
            let phi = fingered(tip - (tip - top) * i as f64 / (n_out - 1) as f64);
            let (p, n) = self.on_cavity(phi);
            // (Folded onto the crest with the lip's inside, as the face is not yet hollow or has
            // collapsed again.)
            loop_.push(crest_point + (p + n * thickness(phi) - crest_point) * unfold);
            weight.push(from_tip(phi) * GLASSY);
            roller.push((churn * (1. - hollow) * 0.2 * h, 0.5 * churn * (1. - hollow)));
        }
        // The crest, and the back of the wave falling to the still water (a smooth step, level
        // at both ends).
        let crest = *loop_.last().unwrap();
        let fall = crest.x + 3. * h;
        for i in 1..=n_back {
            let u = i as f64 / n_back as f64;
            let s = u * u * (3. - 2. * u);
            loop_.push(Vector2::new(crest.x + (fall - crest.x) * u, crest.y * (1. - s)));
            weight.push(GLASSY + (1. - GLASSY) * u);
            roller.push((churn * 0.2 * h * (1. - u).powi(2), 0.5 * churn * (1. - u).powi(2)));
        }
        // The still water behind, out to the back.
        for i in 1..=n_still {
            let u = i as f64 / n_still as f64;
            loop_.push(Vector2::new(fall + (back - fall) * u, 0.));
            weight.push(if i == n_still { 0. } else { 1. });
            roller.push((0., 0.));
        }
        debug_assert_eq!(loop_.len(), count);
        // Anticlockwise.
        if signed_area(&loop_) < 0. {
            loop_.reverse();
            weight.reverse();
            roller.reverse();
        }
        (loop_, weight, roller)
    }

    /// The wave tank's surface at `stage`, between its two nearest frames, scaled to the
    /// wave's height (the tank's crest height above the water before it as the face
    /// overturns) and turned to travel towards -z, as points from the water in front to the
    /// back; with the indices of the lip's tip (the crest itself until the face overturns)
    /// and of the crest.
    fn simulated_surface(&self, stage: f64) -> (Vec<Vector2<f64>>, usize, usize) {
        let frames = simulated_frames();
        let f = stage.clamp(0., 1.) * (frames.len() - 1) as f64;
        let i = (f as usize).min(frames.len() - 2);
        let u = f - i as f64;
        let h = self.height;
        let surface: Vec<Vector2<f64>> = frames[i].iter().zip(&frames[i + 1]).rev()
            .map(|(p, q)| { let r = p + (q - p) * u; Vector2::new(-r.x * h, r.y * h) }).collect();
        let crest = (0..surface.len()).max_by(|&a, &b| surface[a].y.total_cmp(&surface[b].y)).unwrap();
        // The tip: from the crest towards the front, the most shoreward point (least z) before
        // the surface turns back seaward by a twentieth of the height (over the jet's
        // underside); none where it never does.
        let mut tip = crest;
        for k in (0..crest).rev() {
            if surface[k].x < surface[tip].x { tip = k; }
            else if surface[k].x > surface[tip].x + 0.05 * h { break; }
            if k == 0 { tip = crest; }
        }
        (surface, tip, crest)
    }

    /// The middle of the simulated wave's cavity at `stage`: the centroid of what the lip
    /// encloses, from its tip back along its underside and down the face to near the water
    /// before it (seaward of the tip), closed by the line from there back to the tip. Below the
    /// crest, in front, where the face hasn't overturned.
    fn simulated_cavity(&self, stage: f64) -> Vector2<f64> {
        let (surface, tip, crest) = self.simulated_surface(stage);
        let h = self.height;
        let low = 0.15 * surface[crest].y;
        let foot = (0..tip).rev().find(|&k| surface[k].y < low && surface[k].x > surface[tip].x + 0.05 * h);
        let ring = match foot { Some(k) if tip > k + 2 => &surface[k..=tip], _ => return surface[crest] - Vector2::new(0.3 * h, 0.5 * h) };
        let (mut a, mut c) = (0., Vector2::zeros());
        for i in 0..ring.len() {
            let (p, q) = (ring[i], ring[(i + 1) % ring.len()]);
            let cross = p.perp(&q);
            a += cross / 2.;
            c += (p + q) * cross / 6.;
        }
        if a.abs() < 1e-9 { surface[crest] } else { c / a }
    }

    /// As `churned_section`, the wave tank's (`simulated`): its surface between the water in
    /// front, level to the section's front, and the back of the wave, falling to the still
    /// water behind; the cross waves and the roller shared out as on the drawn wave's face,
    /// lip and back.
    fn simulated_section(&self, stage: f64, count: usize) -> ChurnedSection {
        let h = self.height;
        let bed = -self.depth - 0.05 * h;
        let (front, back) = (-12. * h, 8. * h);
        let (surface, tip, crest) = self.simulated_surface(stage);
        let corners = 2;
        let n_front = (count - corners) / 10;
        let n_fall = (count - corners) / 12;
        let n_still = (count - corners) / 12;
        let n_wave = count - corners - n_front - n_fall - n_still;
        let churn = self.churn.clamp(0., 1.);
        const GLASSY: f64 = 0.35;
        // The surface resampled to n_wave points evenly in its points (which are closer where
        // it curves), keeping track of where the tip and crest fall.
        let m = surface.len();
        let at = |v: f64| {
            let k = (v as usize).min(m - 2);
            surface[k] + (surface[k + 1] - surface[k]) * (v - k as f64)
        };
        let places: Vec<f64> = (0..n_wave).map(|j| j as f64 * (m - 1) as f64 / (n_wave - 1) as f64).collect();
        let wave: Vec<Vector2<f64>> = places.iter().map(|&v| at(v)).collect();
        let (tip_at, crest_at) = (tip as f64, crest as f64);
        let mut arc = vec![0.; n_wave];
        for j in 1..n_wave { arc[j] = arc[j - 1] + (wave[j] - wave[j - 1]).norm(); }
        let tip_arc = {
            let j = places.partition_point(|&v| v < tip_at).min(n_wave - 1);
            arc[j]
        };
        let top = surface[crest].y.max(1e-6);
        let mut loop_ = Vec::with_capacity(count);
        let mut weight = Vec::with_capacity(count);
        let mut roller: Vec<(f64, f64)> = Vec::with_capacity(count);
        loop_.push(Vector2::new(back, bed));
        loop_.push(Vector2::new(front, bed));
        weight.extend([0., 0.]);
        roller.extend([(0., 0.), (0., 0.)]);
        // The water in front, level out to the section's front.
        let start = wave[0];
        for i in 0..n_front {
            let u = i as f64 / n_front as f64;
            loop_.push(Vector2::new(front + (start.x - front) * u, start.y));
            weight.push(if i == 0 { 0. } else { 1. });
            roller.push((0., churn * 0.3 * u.powi(8)));
        }
        // The tank's surface: up the face, under the lip to its tip, back over it to the crest
        // and down the back. (Smooth where the flow draws it smooth, as the drawn wave.)
        for j in 0..n_wave {
            let p = wave[j];
            let near_tip = crate::math::smoothstep(0.05, 0.35, (arc[j] - tip_arc).abs());
            let up = (p.y / top).clamp(0., 1.);
            loop_.push(p);
            if places[j] < tip_at {
                weight.push(near_tip * (GLASSY + (1. - GLASSY) * (1. - up).powi(2)));
                roller.push((churn * 0.2 * h * up.powi(3), churn * (0.3 + 0.7 * up)));
            } else if places[j] <= crest_at {
                weight.push(near_tip * GLASSY);
                roller.push((churn * 0.2 * h * up.powi(3), churn * (0.3 + 0.7 * up)));
            } else {
                let u = (places[j] - crest_at) / ((m - 1) as f64 - crest_at).max(1e-9);
                weight.push(GLASSY + (1. - GLASSY) * u);
                roller.push((churn * 0.2 * h * (1. - u).powi(2), 0.5 * churn * (1. - u).powi(2)));
            }
        }
        // The back falling to the still water (a smooth step, level at both ends), and the
        // still water out to the back.
        let end = wave[n_wave - 1];
        let fall = end.x + 3. * h;
        for i in 1..=n_fall {
            let u = i as f64 / n_fall as f64;
            let s = u * u * (3. - 2. * u);
            loop_.push(Vector2::new(end.x + (fall - end.x) * u, end.y * (1. - s)));
            weight.push(1.);
            roller.push((0., 0.));
        }
        for i in 1..=n_still {
            let u = i as f64 / n_still as f64;
            loop_.push(Vector2::new(fall + (back - fall) * u, 0.));
            weight.push(if i == n_still { 0. } else { 1. });
            roller.push((0., 0.));
        }
        debug_assert_eq!(loop_.len(), count);
        if signed_area(&loop_) < 0. {
            loop_.reverse();
            weight.reverse();
            roller.reverse();
        }
        (loop_, weight, roller)
    }

    /// The level of the water before the wave, below still water (m).
    fn trough(&self) -> f64 {
        if self.simulated { 0. } else { -TROUGH * self.height }
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
        // The shape's stage: the raw one up to the lip's landing, then running back down as
        // the tube closes and collapses into a broken bore (over another 0.3 of it).
        let stage = if raw <= 1. { raw.max(0.) } else { (1. - (raw - 1.) / 0.3).max(0.) };
        let varied = Breaker {
            // Highest at the pocket, where the lip is throwing (the stage about 1), lower along
            // the shoulder still standing and where it has landed.
            height: self.height * (1. + v * (0.45 * ((-((raw - 1.) / 0.5).powi(2)).exp() - 1.) + 0.1 * wander(9., 1.7))),
            // (Over metres and less too: the falling water isn't even.)
            // (Gently below a few metres: seen along the tube from inside, each thicker or
            // further-thrown band of the lip was a ring, and a noise of one scale spaces them
            // evenly, a tunnel of hoops.)
            lip: 1. + v * (0.35 * wander(5., 3.1) + 0.08 * wander(1.7, 8.3) + 0.04 * wander(0.7, 14.9)),
            throw: 1. + v * (0.08 * wander(6., 4.9) + 0.025 * wander(2.3, 9.7)),
            // The falling water's edge torn into fingers, decimetres across.
            finger: v * 0.15 * (wander(0.3, 16.3) + 0.6 * wander(0.12, 18.1)),
            // Past the landing, the turbulent roller of the broken bore, settling as it ages.
            // (From once the lip has mostly folded: on a lip still standing, a few centimetres
            // thick, lumps a tenth of the height drove its surfaces through each other.)
            churn: crate::math::smoothstep(1.08, 1.25, raw) * (0.4 + 0.6 * (-((raw - 1.25).max(0.)) / 0.3).exp()),
            ..self.clone()
        };
        // (A peak: its shoulders lagging seaward either side, the line wandering.)
        let bend = v * (1.5 * wander(15., 6.7) + 0.3 * x * x / self.width.max(1.));
        (varied, stage, bend, raw)
    }

    /// At `x` along the crest, the scene points of the tube's middle (the cavity's centre) and
    /// of the lip's tip, and how far the lip has thrown there (beyond 1 where it landed earlier,
    /// the more so the earlier): for placing a camera in the tube or the water the lip throws up
    /// as it lands.
    pub fn tube(&self, x: f64) -> (Vector3<f64>, Vector3<f64>, f64) {
        let (varied, shape, bend, stage) = self.at_raw(x);
        let stage = stage.max(0.);
        let c = if self.simulated { varied.simulated_cavity(shape) } else { varied.cavity().0 };
        let (_, top) = varied.extremes();
        let tip = if self.simulated {
            let (surface, tip, _) = varied.simulated_surface(shape);
            surface[tip]
        } else {
            varied.on_cavity(top + varied.reach(stage.min(1.))).0
        };
        let place = |p: Vector2<f64>| self.center + Vector3::new(x, p.y, p.x + bend);
        (place(c), place(tip), stage)
    }

    /// The section at `x` along the crest as scene points, `count` of them, moved by the cross
    /// waves and the roller (as the mesh has it).
    pub fn displaced_section(&self, x: f64, count: usize, waves: &CrossWaves, lumps: &crate::noise::PerlinNoise) -> Vec<Vector3<f64>> {
        let (varied, stage, bend) = self.at(x);
        let (points, weight, roller) = varied.churned_section(stage, count);
        let h = varied.height;
        let n = points.len();
        (0..n).map(|i| {
            // Moved along the section's outward normal by the cross waves there.
            // (The normal from the points 5 cm or more either side along the outline: where the
            // folded lip's points crowd the crest, near neighbours swing it every way.)
            let reach = |step: usize| {
                let mut j = i;
                for _ in 0..n / 4 {
                    j = (j + step) % n;
                    if (points[j] - points[i]).norm() > 0.05 { break; }
                }
                points[j]
            };
            let (prev, next) = (reach(n - 1), reach(1));
            let t = next - prev;
            let out = if t.norm() > 1e-12 { Vector2::new(t.y, -t.x).normalize() } else { Vector2::zeros() };
            let at = self.center + Vector3::new(x, points[i].y, points[i].x + bend);
            // And by the roller: its bulge, and lumps of churned water a few decimetres across.
            let (bulge, rough) = roller[i];
            let lump = if rough > 0. {
                // (Three octaves on turned, warped coordinates: at one scale the noise's near
                // repeat stood along the crest as a row of spikes.)
                let warp = 0.3 * lumps.noise(at.x / 0.8 + 3.7, at.y / 0.8, at.z / 0.8);
                let q = Vector3::new(0.8 * at.x + 0.6 * at.z + warp, at.y - warp, -0.6 * at.x + 0.8 * at.z);
                let n = |s: f64, o: f64| lumps.noise(q.x / s + o, q.y / s, q.z / s);
                0.1 * h * rough * (n(0.6, 0.) + 0.6 * n(0.25, 5.3) + 0.35 * n(0.1, 11.9))
            } else { 0. };
            let q = points[i] + out * (weight[i] * waves.height(&at) + bulge + lump);
            self.center + Vector3::new(x, q.y, q.x + bend)
        }).collect()
    }

    fn closed(&self) -> IndexedMesh {
        let (count, along) = self.resolution;
        let waves = CrossWaves::new(self.seed, self.variation);
        let lumps = crate::noise::PerlinNoise::shuffled(self.seed.wrapping_add(0xc4a5));
        // (The two end sections as they are, undisplaced, so that their caps' outlines stay
        // simple loops for clipping.)
        let sections: Vec<Vec<Vector3<f64>>> = (0..=along).map(|j| {
            let x = (j as f64 / along as f64 - 0.5) * self.width;
            if j == 0 || j == along {
                let (varied, stage, bend) = self.at(x);
                varied.section(stage, count).iter().map(|p| self.center + Vector3::new(x, p.y, p.x + bend)).collect()
            } else {
                self.displaced_section(x, count, &waves, &lumps)
            }
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
    lace: f64,
}

impl BreakerWater {
    pub fn new(breaker: &Breaker) -> BreakerWater {
        BreakerWater {
            noise: crate::noise::PerlinNoise::shuffled(breaker.seed.wrapping_add(0x5717)),
            trough: breaker.center.y + breaker.trough(),
            height: breaker.height,
            lace: breaker.lace,
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
        if level * low * self.lace > 0. {
            // The lace foam leaves on water: gathered into a network of thin lines along the
            // edges between the cells of water welling up beneath (where the second-nearest
            // of scattered points is nearly as near as the nearest), with dark water between;
            // a finer net within where the foam is thicker, filling in where it is thickest.
            // (Thresholded noise gave solid blotches, stains on the water.)
            let warp = 0.35 * n(p.x / 1.1 + 2.9, 0.3, p.z / 1.1) + 0.1 * n(p.x / 0.3 + 8.1, 0.6, p.z / 0.3);
            let (u, w) = (p.x + warp, p.z - 0.8 * warp);
            // The lines' widths vary, and they break off in places.
            let vary = 0.5 + 0.5 * n(p.x / 0.25 + 17.7, 0.4, p.z / 0.25);
            let broken = crate::math::smoothstep(-0.35, -0.1, n(p.x / 0.45 + 23.9, 0.9, p.z / 0.45));
            let thick = 0.5 + 0.5 * (0.7 * n(p.x / 3. + 41.1, 0.7, p.z / 3.) + 0.3 * n(p.x / 1. + 7.3, 0.2, p.z / 1.));
            let lines = |cell: f64, width: f64, seed: u32| {
                let gap = cell_gap(u / cell, w / cell, seed) * cell;
                1. - crate::math::smoothstep(0.3 * width, width, gap)
            };
            let net = lines(0.7, (0.015 + 0.06 * thick) * (0.3 + 1.4 * vary), 11) * broken;
            let fine = lines(0.22, (0.008 + 0.02 * thick) * (0.3 + 1.4 * vary), 23) * crate::math::smoothstep(0.45, 0.7, thick);
            let filled = crate::math::smoothstep(0.8, 0.95, thick);
            // (Less of it: the thinnest lines go first, then the net.)
            let thick = thick - 0.6 * (1. - self.lace);
            cover = level * low * net.max(fine).max(filled) * crate::math::smoothstep(0.15, 0.35, thick);
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
        // Capillary ripples, three octaves. (Cut to a fifth once, for a blur that was the
        // camera's focus; a breaking wave's face is rippled enough to glitter all over.)
        let ripples = 0.0007 * n(p.x / 0.03, p.y / 0.03, p.z / 0.03)
            + 0.00035 * n(p.x / 0.015 + 7.1, p.y / 0.015, p.z / 0.015)
            + 0.0012 * n(p.x / 0.08 + 3.3, p.y / 0.08, p.z / 0.08);
        // Streaks drawn out along the flow up the face, diagonal as it peels (along the crest
        // alone they ringed the tube's roof): narrow across, a few centimetres, long along it,
        // tens; slopes about a fifth, so each throws a glint, as the scales of a breaking
        // wave's face do.
        let x = p.x + 1.2 * (p.y - self.trough);
        let streaks = streaked * (0.0012 * n(x / 0.05 + 11.9, p.y / 0.6, p.z / 0.6)
            + 0.0005 * n(x / 0.022 + 4.4, p.y / 0.25, p.z / 0.25));
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
        // The plume: spray the landing lip throws up, to twice the wave's height, most just
        // after it lands; widening as it rises and blown back seaward (+z) over the wave by the
        // wind off the land; densest at its foot, feathered to wisps at its top; in billows.
        let tall = ((p.y - ty) / (2. * h)).clamp(0., 1.);
        let plume = if p.y > ty - 0.1 * h {
            let when = (-((age - 0.5) / 0.9).powi(2)).exp();
            let lean = 0.15 * h + 0.9 * h * tall.powf(1.5);
            let width = 0.3 * h + 0.6 * h * tall;
            let n = |s: f64, o: f64| self.noise.noise(p.x / s + o, p.y / s, p.z / s);
            let billows = 0.5 + 0.45 * n(0.45, 31.3) + 0.3 * n(0.15, 2.9) + 0.15 * n(0.05, 6.1);
            // (From a little above the tip: at its foot it stood as a grey wall in front of the
            // face, in the wave's shadow; the churned mass and splash are the foot.)
            let above = crate::math::smoothstep(0.03, 0.15, tall);
            when * above * (-((dz - lean) / width).powi(2)).exp() * (1. - tall).powf(1.3) * (2.2 * billows - 0.7)
        } else { 0. };
        (landed * ((mass * thinning).max(splash) * (1.6 * lumps - 0.3)).max(plume)).clamp(0., 1.)
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
            let tip = top + varied.reach(stage);
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
        // Torn into air near its tip, the outer part of the throw; clear water nearer the roof.
        // (Through the whole lip, from the roof on, it veiled the sky behind it in a white haze.)
        let thrown = crate::math::smoothstep(top + 0.55 * (tip - top), tip - 0.05, phi)
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
        if !(-0.15..=1.).contains(&up) { return 0.; }
        // Blown back: the higher, the further seaward.
        let back = (p.z - cz - 0.4 * h * up.max(0.)) / (0.35 * h);
        let sheet = (-back * back).exp() * (1. - up.max(0.)).powi(2) * (1. - crate::math::smoothstep(-0.15, 0., -up));
        let n = |s: f64, o: f64| self.noise.noise(p.x / s + o, p.y / s, p.z / s);
        let wisps = 0.5 + 0.5 * n(0.3, 0.) + 0.35 * n(0.1, 7.7);
        (sheet * (2. * wisps - 0.6)).clamp(0., 1.)
    }

    fn majorant(&self) -> f64 { 1. }
}

/// Smaller waves crossing the breaker: the wind's chop, the backwash, waves reflected and
/// refracted from elsewhere, running every way over the face, the lip and the water about it,
/// so that the surface is uneven, not a clean curve: a sum of 64 sinusoids of random directions
/// and phases, wavelengths from 0.3 to 3 m (evenly in their logarithm), each a few percent
/// steep and the longer ones less so (a chop, not long bands), as the relief the surface is
/// moved along its normal by.
pub struct CrossWaves {
    waves: Vec<(Vector3<f64>, f64, f64)>,
}

impl CrossWaves {
    pub fn new(seed: u64, strength: f64) -> CrossWaves {
        let mut rng = crate::geometry::Pcg32::new(seed.wrapping_add(0xc705), 7);
        let waves = (0..64).map(|_| {
            let dir = loop {
                let v = Vector3::new(rng.next_f64() - 0.5, rng.next_f64() - 0.5, rng.next_f64() - 0.5) * 2.;
                if v.norm() > 0.2 && v.norm() <= 1. { break v.normalize(); }
            };
            let length = 0.3 * 10f64.powf(rng.next_f64());
            let k = std::f64::consts::TAU / length;
            // (The longer ones less steep: a chop, not long bands.)
            (dir * k, 0.006 * length * (0.3 / length).powf(0.25) * strength, rng.next_f64() * std::f64::consts::TAU)
        }).collect();
        CrossWaves { waves }
    }

    /// The relief at `p` (m).
    pub fn height(&self, p: &Vector3<f64>) -> f64 {
        self.waves.iter().map(|(k, a, phase)| a * (k.dot(p) + phase).sin()).sum()
    }
}

/// In the plane, at (x, y) in units of the cells (one scattered point each, hashed from the
/// cell and `seed`), the gap between the distances to the second-nearest point and the
/// nearest (Worley's F2 - F1): small along the edges between cells, none on them.
fn cell_gap(x: f64, y: f64, seed: u32) -> f64 {
    let (cx, cy) = (x.floor(), y.floor());
    let (mut f1, mut f2) = (f64::MAX, f64::MAX);
    for dy in -1..=1 {
        for dx in -1..=1 {
            let (ix, iy) = (cx as i64 + dx, cy as i64 + dy);
            let h = |k: u32| crate::noise::worley_hash(ix as u32, iy as u32, k, seed);
            let (px, py) = (ix as f64 + h(0), iy as f64 + h(1));
            let d = ((px - x).powi(2) + (py - y).powi(2)).sqrt();
            if d < f1 { f2 = f1; f1 = d; } else if d < f2 { f2 = d; }
        }
    }
    f2 - f1
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
        if !clipped {
            // Only flat ears left where points coincide or lie in a line: clip one, a triangle
            // of no area, so the cap stays closed.
            match (0..n).find(|&k| cross(p[left[(k + n - 1) % n]], p[left[k]], p[left[(k + 1) % n]]).abs() <= 1e-12) {
                Some(k) => {
                    out.push([left[(k + n - 1) % n], left[k], left[(k + 1) % n]]);
                    left.remove(k);
                },
                None => break,
            }
        }
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
        for (stage, lip, throw, finger) in [(0., 1., 1., 0.), (0.3, 1., 1., 0.), (0.7, 1., 1., 0.), (1., 1., 1., 0.), (1., 0.5, 1.1, 0.), (1., 1.6, 0.5, 0.),
                (0.6, 1.6, 1.1, 0.), (1., 0.5, 1.1, 0.3), (1., 0.5, 1.1, -0.3), (0.8, 1.6, 0.5, 0.3)] {
            let w = Breaker { lip, throw, finger, ..wave() };
            let s = w.section(stage, 200);
            assert_eq!(s.len(), 200);
            assert!(signed_area(&s) > 0.);
            let n = s.len();
            for i in 0..n {
                for j in i + 2..n {
                    if (j + 1) % n == i { continue; }
                    assert!(!crosses(s[i], s[(i + 1) % n], s[j], s[(j + 1) % n]), "stage {} lip {} throw {} finger {}: edges {} and {} cross at {:?} {:?}", stage, lip, throw, finger, i, j, s[i], s[j]);
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

    #[test]
    fn test_a_simulated_breakers_sections_are_simple_loops_that_overturn() {
        let w = Breaker { simulated: true, ..wave() };
        let h = w.height;
        for k in 0..=20 {
            let stage = k as f64 / 20.;
            let s = w.section(stage, 300);
            assert_eq!(s.len(), 300);
            assert!(signed_area(&s) > 0.);
            let n = s.len();
            for i in 0..n {
                for j in i + 2..n {
                    if (j + 1) % n == i { continue; }
                    assert!(!crosses(s[i], s[(i + 1) % n], s[j], s[(j + 1) % n]), "stage {}: edges {} and {} cross at {:?} {:?}", stage, i, j, s[i], s[j]);
                }
            }
            let area: f64 = ear_clip(&s).iter().map(|t| 0.5 * ((s[t[1]] - s[t[0]]).perp(&(s[t[2]] - s[t[0]])))).sum();
            assert!((area - 0.5 * signed_area(&s)).abs() < 1e-6 * area.abs(), "stage {}: {} {}", stage, area, 0.5 * signed_area(&s));
            // Its crest the wave's height above the water before it, as the face overturns.
            let crest = s.iter().map(|p| p.y).fold(f64::MIN, f64::max);
            assert!(crest > 0.6 * h && crest < 1.2 * h, "stage {}: {}", stage, crest);
        }
        // Overturned at the end, the lip's tip shoreward of the face below it, and not at
        // the start.
        let (late, tip, crest) = w.simulated_surface(1.);
        assert!(tip < crest && late[tip].x < late[crest].x - 0.2 * h, "{:?} {:?}", late[tip], late[crest]);
        let (_, tip, crest) = w.simulated_surface(0.);
        assert_eq!(tip, crest);
        let m = w.closed();
        let mut edges = std::collections::HashMap::new();
        for t in &m.triangles {
            for k in 0..3 {
                let (a, b) = (t[k], t[(k + 1) % 3]);
                *edges.entry((a.min(b), a.max(b))).or_insert(0) += 1;
            }
        }
        assert!(edges.values().all(|&c| c == 2), "{} open edges", edges.values().filter(|&&c| c != 2).count());
    }
}

#[cfg(test)]
mod along_probe {
    use super::*;
    /// Writes the shorebreak's sections along its crest as JSON, for drawing (RAYS_SECTIONS).
    #[test]
    #[ignore]
    fn probe_along() {
        let text = std::fs::read_to_string("demo/scenes/shorebreak.json").unwrap();
        let v: serde_json::Value = serde_json::from_str(&text).unwrap();
        let o = v["objects"].as_array().unwrap().iter().find(|o| o["type"] == "breaker").unwrap();
        let w = Breaker::parse(o);
        let xs: Vec<f64> = std::env::var("RAYS_SECTIONS_AT").map(|v| v.split(',').map(|x| x.parse().unwrap()).collect())
            .unwrap_or_else(|_| vec![-25., -15., -8., -4., 0., 4., 10., 25.]);
        let (waves, lumps) = (CrossWaves::new(w.seed, w.variation), crate::noise::PerlinNoise::shuffled(w.seed.wrapping_add(0xc4a5)));
        let out: Vec<serde_json::Value> = xs.iter().map(|&x| {
            let (varied, _, _, raw) = w.at_raw(x);
            let pts: Vec<[f64; 2]> = w.displaced_section(x, w.resolution.0, &waves, &lumps).iter().map(|p| [p.z - w.center.z, p.y - w.center.y]).collect();
            let (centre, tip, _) = w.tube(x);
            serde_json::json!({"x": x, "raw": raw, "height": varied.height, "points": pts,
                "centre": [centre.x, centre.y, centre.z], "tip": [tip.x, tip.y, tip.z]})
        }).collect();
        std::fs::write(std::env::var("RAYS_SECTIONS").unwrap(), serde_json::to_string(&out).unwrap()).unwrap();
    }
}
