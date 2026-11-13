//! Reading Tungsten scene files.
//!
//! Tungsten is Benedikt Bitterli's physically based renderer (<https://github.com/tunabrain/tungsten>,
//! zlib licence). Its scene files are JSON, and Bitterli's "Rendering Resources"
//! (<https://benedikt-bitterli.me/resources/>) publish about 30 scenes in it, each with the
//! linear EXR reference render our `--compare` diffs against. This translates a Tungsten scene
//! into our scene JSON, which then goes through the usual parser. Conventions below are from
//! Tungsten's source (file names relative to its `src/core`), not its documentation, which has
//! little on the format.
//!
//! A file has `bsdfs` (named materials), `primitives`, `camera`, `integrator` and `renderer`
//! (and `media`, which we can't represent). Our files never have the first two, which is how
//! `SceneFile::read_document` tells the formats apart.
//!
//! # Transforms (`io/JsonPtr.cpp`, `JsonPtr::get(Mat4f&)`)
//!
//! A `transform` is a 4x4 matrix: 16 numbers, row-major, or an object. From an object, the
//! matrix's columns are axes x, y, z and the translation `position`. The axes start as the
//! identity; `look_at` sets z to point from `position` at it, `up` sets y, and `x_axis`,
//! `y_axis`, `z_axis` set any directly. They are made orthonormal by Gram-Schmidt, in an order
//! depending on which were given, and right-handed by flipping one not given. Then each axis is
//! scaled by `scale`, and rotated by `rotation`: Euler angles in degrees, applied as
//! `Mat4f::rotYXZ` (`math/Mat4f.cpp`). With only `position`, `rotation` and `scale` this is
//! T R S, the order of our `transform`. `transform_matrix` follows the same steps.
//!
//! # Primitives
//!
//! - `quad` (`primitives/Quad.cpp`): the unit square in the xz plane, centred on the origin,
//!   with edges e0 = M (1, 0, 0) and e1 = M (0, 0, 1) and normal e1 x e0, the side it emits from.
//!   It becomes two triangles in world space, wound so that our normal (v1 - v0) x (v2 - v0)
//!   agrees.
//! - `cube` (`primitives/Cube.cpp`): the unit cube centred on the origin, under the transform.
//!   Twelve triangles, wound outwards.
//! - `disk` (`primitives/Disk.cpp`): a disc about M (0, 0, 0) with normal M (0, 1, 0) and radius
//!   the largest scale of the x and z axes, emitting from the normal's side. A fan of
//!   `DISK_SEGMENTS` triangles.
//! - `sphere` (`primitives/Sphere.cpp`): about M (0, 0, 0), with radius the largest axis scale.
//! - `mesh` (`primitives/TriangleMesh.cpp`): `file` (OBJ or Tungsten's `.wo3`, relative to the
//!   scene file) under the transform, flat or `smooth`.
//! - `point` (`primitives/Point.cpp`): a point light (below).
//! - `infinite_sphere` with a constant emission: the background. Image-based emission, and the
//!   other primitives (`skydome`, `cylinder`, `curves`, `instances`, ...), are skipped with a
//!   warning.
//!
//! # Emission
//!
//! A primitive emits radiance `emission`, or `power` (flux) converted to radiance by its
//! `powerToRadianceFactor`, 1/(pi area) for a surface. Our emitter is a material, so an emitting
//! primitive gets a `diffuse-light` of that radiance in place of its bsdf: exact for the usual
//! `null` bsdf of a light, but an emitter that also reflects loses its reflection (warned).
//! A `point` of power P has radiant intensity P/(4 pi); our point light of `intensity` I gives
//! irradiance I/(pi d^2) (`material::functions::diffuse`), so I = P/4 (and pi times an
//! `emission`, which is radiant intensity).
//!
//! # Materials (`bsdfs/`)
//!
//! `albedo` multiplies every bsdf (default 1); textures are replaced by grey, with a warning.
//!
//! - `lambert` -> `lambertian`; `oren_nayar` -> `lambertian` (its roughness dropped).
//! - `mirror` -> `metal` of roughness 0.
//! - `conductor`, `rough_conductor` -> `metal` whose colour is albedo times the Fresnel
//!   reflectance at normal incidence, ((eta - 1)^2 + k^2) / ((eta + 1)^2 + k^2) per channel,
//!   from `eta` and `k` or the named `material` (default "Cu") in Tungsten's table
//!   (`bsdfs/ComplexIorData.hpp`, copied into `COMPLEX_IOR`). Our metals approximate the
//!   Fresnel term from that colour by Schlick's formula, rather than from eta and k.
//! - `dielectric`, `rough_dielectric` -> `dielectric` (`ior` default 1.5), attenuated by albedo.
//! - `plastic`, `rough_plastic` -> `plastic` (`ior` default 1.5); its coat's absorption
//!   (`sigma_a`, `thickness`) is dropped.
//! - `smooth_coat`, `rough_coat` -> `plastic` over a lambertian `substrate`, or else the
//!   substrate alone.
//! - Rough variants keep `roughness`, which is GGX's alpha in both renderers (`ggx`). Tungsten's
//!   other `distribution`s (Beckmann, Phong) are approximated by GGX of the same alpha, with a
//!   warning.
//! - `null` on a primitive that doesn't emit: the primitive is skipped (in Tungsten it is
//!   invisible, typically a medium's boundary).
//! - Anything else (`mixed`, `transparency`, `thinsheet`, hair, ...): grey `lambertian`, warned.
//!
//! # Camera and settings
//!
//! `camera.fov` is the horizontal field of view in degrees (`cameras/PinholeCamera.cpp`). Our
//! `angle` has tangent equal to the image's full height at unit distance, so it is
//! atan(2 tan(fov / 2) h / w). The camera looks from the transform's position along its z axis,
//! with its y axis up, unless `look_at` or `up` are given (`cameras/Camera.cpp`). A `thinlens`
//! camera's `focus_distance` becomes the distance to our look-at point, and `aperture_size`
//! our `aperture` (both a lens radius). `resolution` (default 1000 x 563) gives the image size,
//! `renderer.spp` the samples, `integrator.max_bounces` the depth (with Russian roulette, which
//! Tungsten's path tracer also uses), and `reconstruction_filter` (default tent) the filter;
//! filters other than box and tent become tent. Tone mapping is ignored: compare our EXR with
//! Tungsten's.

use crate::na::{Matrix3, Matrix4, Vector3, Vector4};
use serde_json::{json, Map, Value};
use std::f64::consts::PI;
use std::path::Path;

/// Triangles in the fan approximating a disk.
pub const DISK_SEGMENTS: usize = 64;

/// Whether a scene file is Tungsten's, from keys ours never has.
pub fn is_tungsten(doc: &Value) -> bool {
    doc.get("primitives").is_some() && doc.get("bsdfs").is_some()
}

/// Real and imaginary parts of the index of refraction, (eta, k), at red, green and blue, from
/// Tungsten's `bsdfs/ComplexIorData.hpp` (zlib licence, Benedikt Bitterli).
const COMPLEX_IOR: [(&str, [f64; 3], [f64; 3]); 40] = [
    ("a-C", [2.9440999183, 2.2271502925, 1.9681668794], [0.8874329109, 0.7993216383, 0.8152862927]),
    ("Ag", [0.1552646489, 0.1167232965, 0.1383806959], [4.8283433224, 3.1222459278, 2.1469504455]),
    ("Al", [1.6574599595, 0.8803689579, 0.5212287346], [9.2238691996, 6.2695232477, 4.8370012281]),
    ("AlAs", [3.6051023902, 3.2329365777, 2.2175611545], [0.0006670247, -0.0004999400, 0.0074261204]),
    ("AlSb", [-0.0485225705, 4.1427547893, 4.6697691348], [-0.0363741915, 0.0937665154, 1.3007390124]),
    ("Au", [0.1431189557, 0.3749570432, 1.4424785571], [3.9831604247, 2.3857207478, 1.6032152899]),
    ("Be", [4.1850592788, 3.1850604423, 2.7840913457], [3.8354398268, 3.0101260162, 2.8690088743]),
    ("Cr", [4.3696828663, 2.9167024892, 1.6547005413], [5.2064337956, 4.2313645277, 3.7549467933]),
    ("CsI", [2.1449030413, 1.7023164587, 1.6624194173], [0.0000000000, 0.0000000000, 0.0000000000]),
    ("Cu", [0.2004376970, 0.9240334304, 1.1022119527], [3.9129485033, 2.4528477015, 2.1421879552]),
    ("Cu2O", [3.5492833755, 2.9520622449, 2.7369202137], [0.1132179294, 0.1946659670, 0.6001681264]),
    ("CuO", [3.2453822204, 2.4496293965, 2.1974114493], [0.5202739621, 0.5707372756, 0.7172250613]),
    ("d-C", [2.7112524747, 2.3185812849, 2.2288565009], [0.0000000000, 0.0000000000, 0.0000000000]),
    ("Hg", [2.3989314904, 1.4400254917, 0.9095512090], [6.3276269444, 4.3719414152, 3.4217899270]),
    ("HgTe", [4.7795267752, 3.2309984581, 2.6600252401], [1.6319827058, 1.5808189339, 1.7295753852]),
    ("Ir", [3.0864098394, 2.0821938440, 1.6178866805], [5.5921510077, 4.0671757150, 3.2672611269]),
    ("K", [0.0640493070, 0.0464100621, 0.0381842017], [2.1042155920, 1.3489364357, 0.9132113889]),
    ("Li", [0.2657871942, 0.1956102432, 0.2209198538], [3.5401743407, 2.3111306542, 1.6685930000]),
    ("MgO", [2.0895885542, 1.6507224525, 1.5948759692], [0.0000000000, -0.0000000000, 0.0000000000]),
    ("Mo", [4.4837010280, 3.5254578255, 2.7760769438], [4.1111307988, 3.4208716252, 3.1506031404]),
    ("Na", [0.0602665320, 0.0561412435, 0.0619909494], [3.1792906496, 2.1124800781, 1.5790940266]),
    ("Nb", [3.4201353595, 2.7901921379, 2.3955856658], [3.4413817900, 2.7376437930, 2.5799132708]),
    ("Ni", [2.3672753521, 1.6633583302, 1.4670554172], [4.4988329911, 3.0501643957, 2.3454274399]),
    ("Rh", [2.5857954933, 1.8601866068, 1.5544279524], [6.7822927110, 4.7029501026, 3.9760892461]),
    ("Se-e", [5.7242724833, 4.1653992967, 4.0816099264], [0.8713747439, 1.1052845009, 1.5647788766]),
    ("Se", [4.0592611085, 2.8426947380, 2.8207582835], [0.7543791750, 0.6385150558, 0.5215872029]),
    ("SiC", [3.1723450205, 2.5259677964, 2.4793623897], [0.0000007284, -0.0000006859, 0.0000100150]),
    ("SnTe", [4.5251865890, 1.9811525984, 1.2816819226], [0.0000000000, 0.0000000000, 0.0000000000]),
    ("Ta", [2.0625846607, 2.3930915569, 2.6280684948], [2.4080467973, 1.7413705864, 1.9470377016]),
    ("Te-e", [7.5090397678, 4.2964603080, 2.3698732430], [5.5842076830, 4.9476231084, 3.9975145063]),
    ("Te", [7.3908396088, 4.4821028985, 2.6370708478], [3.2561412892, 3.5273908133, 3.2921683116]),
    ("ThF4", [1.8307187117, 1.4422274283, 1.3876488528], [0.0000000000, 0.0000000000, 0.0000000000]),
    ("TiC", [3.7004673762, 2.8374356509, 2.5823030278], [3.2656905818, 2.3515586388, 2.1727857800]),
    ("TiN", [1.6484691607, 1.1504482522, 1.3797795097], [3.3684596226, 1.9434888540, 1.1020123347]),
    ("TiO2-e", [3.1065574823, 2.5131551146, 2.5823844157], [0.0000289537, -0.0000251484, 0.0001775555]),
    ("TiO2", [3.4566203131, 2.8017076558, 2.9051485020], [0.0001026662, -0.0000897534, 0.0006356902]),
    ("VC", [3.6575665991, 2.7527298065, 2.5326814570], [3.0683516659, 2.1986687713, 1.9631816252]),
    ("VN", [2.8656011588, 2.1191817791, 1.9400767149], [3.0323264950, 2.0561075580, 1.6162930914]),
    ("V", [4.2775126218, 3.5131538236, 2.7611257461], [3.4911844504, 2.8893580874, 3.1116965117]),
    ("W", [4.3707029924, 3.3002972445, 2.9982666528], [3.5006778591, 2.6048652781, 2.2731930614]),
];

type V3 = Vector3<f64>;

/// Tungsten's Gram-Schmidt (`io/JsonPtr.cpp`): a normalised; b and then c made orthogonal to
/// the earlier ones and normalised, or replaced if they were parallel.
fn gram_schmidt(a: &mut V3, b: &mut V3, c: &mut V3) {
    *a = a.normalize();
    *b -= *a * a.dot(b);
    *b = if b.norm_squared() < 1e-5 { random_ortho(a) } else { b.normalize() };
    *c -= *a * a.dot(c);
    *c -= *b * b.dot(c);
    *c = if c.norm_squared() < 1e-5 { a.cross(b) } else { c.normalize() };
}

fn random_ortho(a: &V3) -> V3 {
    let r = if a.x.abs() > a.y.abs() { V3::y() } else { V3::x() };
    a.cross(&r).normalize()
}

/// Tungsten's `Mat4f::rotYXZ`, of angles in degrees.
fn rot_yxz(r: V3) -> Matrix3<f64> {
    let r = r * PI / 180.;
    let c = [r.x.cos(), r.y.cos(), r.z.cos()];
    let s = [r.x.sin(), r.y.sin(), r.z.sin()];
    Matrix3::new(
        c[1] * c[2] - s[1] * s[0] * s[2], -c[1] * s[2] - s[1] * s[0] * c[2], -s[1] * c[0],
        c[0] * s[2], c[0] * c[2], -s[0],
        s[1] * c[2] + c[1] * s[0] * s[2], -s[1] * s[2] + c[1] * s[0] * c[2], c[1] * c[0],
    )
}

/// A number, or three: a number stands for all three, as in Tungsten.
fn vec3(v: &Value) -> Option<V3> {
    match v {
        Value::Number(n) => n.as_f64().map(|x| V3::new(x, x, x)),
        Value::Array(a) if a.len() == 3 => {
            let x: Option<Vec<f64>> = a.iter().map(Value::as_f64).collect();
            x.map(|x| V3::new(x[0], x[1], x[2]))
        },
        _ => None,
    }
}

fn field3(o: &Value, k: &str) -> Result<Option<V3>, String> {
    match o.get(k) {
        None | Some(Value::Null) => Ok(None),
        Some(v) => vec3(v).map(Some).ok_or_else(|| format!("\"{}\" must be a number or three, got {}", k, v)),
    }
}

/// A primitive's or camera's `transform` as a matrix (see the module documentation).
pub fn transform_matrix(v: &Value) -> Result<Matrix4<f64>, String> {
    match v {
        Value::Null => Ok(Matrix4::identity()),
        Value::Array(a) if a.len() == 16 => {
            let x: Option<Vec<f64>> = a.iter().map(Value::as_f64).collect();
            x.map(|x| Matrix4::from_row_slice(&x)).ok_or_else(|| format!("transform matrix must be numbers, got {}", v))
        },
        Value::Object(_) => {
            let pos = field3(v, "position")?.unwrap_or_else(V3::zeros);
            let (mut x, mut y, mut z) = (V3::x(), V3::y(), V3::z());
            let (mut ex, mut ey, mut ez) = (false, false, false);
            if let Some(l) = field3(v, "look_at")? { z = l - pos; ez = true; }
            if let Some(u) = field3(v, "up")? { y = u; ey = true; }
            if let Some(a) = field3(v, "x_axis")? { x = a; ex = true; }
            if let Some(a) = field3(v, "y_axis")? { y = a; ey = true; }
            if let Some(a) = field3(v, "z_axis")? { z = a; ez = true; }
            match (ez, ey, ex) {
                (false, false, true) => gram_schmidt(&mut x, &mut z, &mut y),
                (false, true, false) => gram_schmidt(&mut y, &mut z, &mut x),
                (false, true, true) => gram_schmidt(&mut y, &mut x, &mut z),
                (true, false, true) => gram_schmidt(&mut z, &mut x, &mut y),
                _ => gram_schmidt(&mut z, &mut y, &mut x),
            }
            if x.cross(&y).dot(&z) < 0. {
                if !ex { x = -x } else if !ey { y = -y } else { z = -z }
            }
            if let Some(s) = field3(v, "scale")? {
                x *= s.x;
                y *= s.y;
                z *= s.z;
            }
            if let Some(r) = field3(v, "rotation")? {
                let r = rot_yxz(r);
                x = r * x;
                y = r * y;
                z = r * z;
            }
            let mut m = Matrix4::identity();
            m.fixed_view_mut::<3, 1>(0, 0).copy_from(&x);
            m.fixed_view_mut::<3, 1>(0, 1).copy_from(&y);
            m.fixed_view_mut::<3, 1>(0, 2).copy_from(&z);
            m.fixed_view_mut::<3, 1>(0, 3).copy_from(&pos);
            Ok(m)
        },
        _ => Err(format!("a transform must be 16 numbers or an object, got {}", v)),
    }
}

fn point(m: &Matrix4<f64>, p: V3) -> V3 {
    (m * Vector4::new(p.x, p.y, p.z, 1.)).xyz()
}

fn linear(m: &Matrix4<f64>) -> Matrix3<f64> {
    m.fixed_view::<3, 3>(0, 0).into_owned()
}

fn triangle(v0: V3, v1: V3, v2: V3) -> Value {
    json!({ "type": "triangle", "v0": [v0.x, v0.y, v0.z], "v1": [v1.x, v1.y, v1.z], "v2": [v2.x, v2.y, v2.z] })
}

/// A quad's two triangles, wound so their normal is Tungsten's e1 x e0, and its area.
fn quad(m: &Matrix4<f64>) -> (Vec<Value>, f64) {
    let l = linear(m);
    let (e0, e1) = (l * V3::x(), l * V3::z());
    let b = point(m, V3::zeros()) - e0 * 0.5 - e1 * 0.5;
    (vec![triangle(b, b + e1, b + e0 + e1), triangle(b, b + e0 + e1, b + e0)], e1.cross(&e0).norm())
}

/// A cube's twelve triangles, wound outwards, and its area.
fn cube(m: &Matrix4<f64>) -> (Vec<Value>, f64) {
    let mirrored = linear(m).determinant() < 0.;
    let axes = [V3::x(), V3::y(), V3::z()];
    let mut tris = Vec::new();
    let mut area = 0.;
    for a in 0 .. 3 {
        for s in [-1., 1.] {
            let n = axes[a] * s;
            // u x v = n
            let (mut u, mut v) = (axes[(a + 1) % 3], axes[(a + 2) % 3]);
            if s < 0. { std::mem::swap(&mut u, &mut v) }
            let c = n * 0.5;
            let p = [c - u * 0.5 - v * 0.5, c + u * 0.5 - v * 0.5, c + u * 0.5 + v * 0.5, c - u * 0.5 + v * 0.5]
                .map(|q| point(m, q));
            for [i, j, k] in [[0, 1, 2], [0, 2, 3]] {
                area += (p[j] - p[i]).cross(&(p[k] - p[i])).norm() / 2.;
                tris.push(if mirrored { triangle(p[i], p[k], p[j]) } else { triangle(p[i], p[j], p[k]) });
            }
        }
    }
    (tris, area)
}

/// A disk's fan of triangles, wound so their normal is Tungsten's, and its area, pi r^2.
fn disk(m: &Matrix4<f64>) -> (Vec<Value>, f64) {
    let l = linear(m);
    let c = point(m, V3::zeros());
    let n = (l * V3::y()).normalize();
    let r = (l * V3::x()).norm().max((l * V3::z()).norm());
    let t = random_ortho(&n);
    let b = n.cross(&t);
    let at = |i: usize| {
        let a = 2. * PI * i as f64 / DISK_SEGMENTS as f64;
        c + (t * a.cos() + b * a.sin()) * r
    };
    ((0 .. DISK_SEGMENTS).map(|i| triangle(c, at(i), at(i + 1))).collect(), PI * r * r)
}

/// Our `transform` of `item` for a matrix T R S, or `item` itself for the identity. Shear,
/// which T R S can't hold, is dropped with a warning.
fn transformed(item: Value, m: &Matrix4<f64>, warn: &mut dyn FnMut(String)) -> Value {
    if *m == Matrix4::identity() {
        return item;
    }
    let l = linear(m);
    let mut s = V3::new(l.column(0).norm(), l.column(1).norm(), l.column(2).norm());
    let mut r = Matrix3::from_columns(&[l.column(0) / s.x, l.column(1) / s.y, l.column(2) / s.z]);
    if r.determinant() < 0. {
        s.x = -s.x;
        r.set_column(0, &-r.column(0));
    }
    if (r.transpose() * r - Matrix3::identity()).abs().max() > 1e-4 {
        warn(format!("a mesh's transform has shear, which is dropped: {}", m));
    }
    let (roll, pitch, yaw) = crate::na::Rotation3::from_matrix_unchecked(r).euler_angles();
    let t = point(m, V3::zeros());
    json!({
        "type": "transform",
        "item": item,
        "scale": [s.x, s.y, s.z],
        "rotate": { "roll": roll.to_degrees(), "pitch": pitch.to_degrees(), "yaw": yaw.to_degrees() },
        "translate": [t.x, t.y, t.z],
    })
}

/// Normal-incidence Fresnel reflectance of a conductor, per channel.
fn conductor_reflectance(eta: V3, k: V3) -> V3 {
    V3::from_fn(|i, _| ((eta[i] - 1.).powi(2) + k[i] * k[i]) / ((eta[i] + 1.).powi(2) + k[i] * k[i]))
}

fn rgb(v: V3) -> Value {
    json!([v.x, v.y, v.z])
}

struct Importer<'a> {
    /// Tungsten's named bsdfs.
    bsdfs: Map<String, Value>,
    materials: Map<String, Value>,
    objects: Vec<Value>,
    lights: Vec<Value>,
    background: V3,
    warnings: Vec<String>,
    dir: &'a Path,
}

impl<'a> Importer<'a> {
    fn warn(&mut self, w: String) {
        if !self.warnings.contains(&w) {
            self.warnings.push(w);
        }
    }

    /// An `albedo`-like colour: a number or three, or (a texture) grey.
    fn colour(&mut self, v: &Value, default: f64, context: &str) -> V3 {
        match v {
            Value::Null => V3::new(default, default, default),
            _ => vec3(v).unwrap_or_else(|| {
                self.warn(format!("{}: textures are not supported, using grey for {}", context, v));
                V3::new(0.5, 0.5, 0.5)
            }),
        }
    }

    /// Our material for a Tungsten bsdf, or None for `null`.
    fn bsdf(&mut self, b: &Value, context: &str) -> Option<Value> {
        let t = b["type"].as_str().unwrap_or("").to_string();
        let context = format!("{} ({})", context, t);
        let albedo = self.colour(&b["albedo"], 1., &context);
        let num = |k: &str, d: f64| b[k].as_f64().unwrap_or(d);
        if t.starts_with("rough_") && b["distribution"].as_str().is_some_and(|d| d != "ggx") {
            self.warn(format!("{}: the {} distribution is approximated by GGX", context, b["distribution"]));
        }
        if !b["bump"].is_null() {
            self.warn(format!("{}: bump maps are not supported", context));
        }
        Some(match t.as_str() {
            "null" => return None,
            "lambert" => json!({ "type": "lambertian", "albedo": rgb(albedo) }),
            "oren_nayar" => {
                self.warn(format!("{}: rendered as lambertian", context));
                json!({ "type": "lambertian", "albedo": rgb(albedo) })
            },
            "mirror" => json!({ "type": "metal", "reflective": rgb(albedo), "roughness": 0 }),
            "conductor" | "rough_conductor" => {
                let (eta, k) = match (vec3(&b["eta"]), vec3(&b["k"])) {
                    (Some(eta), Some(k)) if b["material"].is_null() => (eta, k),
                    _ => {
                        let name = b["material"].as_str().unwrap_or("Cu");
                        match COMPLEX_IOR.iter().find(|m| m.0 == name) {
                            Some((_, eta, k)) => (V3::from(*eta), V3::from(*k)),
                            None => {
                                self.warn(format!("{}: unknown conductor \"{}\", using copper", context, name));
                                let cu = COMPLEX_IOR.iter().find(|m| m.0 == "Cu").unwrap();
                                (V3::from(cu.1), V3::from(cu.2))
                            },
                        }
                    },
                };
                let roughness = if t == "rough_conductor" { num("roughness", 0.1) } else { 0. };
                json!({ "type": "metal", "reflective": rgb(albedo.component_mul(&conductor_reflectance(eta, k))), "roughness": roughness })
            },
            "dielectric" | "rough_dielectric" => {
                if b["enable_refraction"] == json!(false) {
                    self.warn(format!("{}: refraction can't be turned off", context));
                }
                json!({ "type": "dielectric", "refractive_index": num("ior", 1.5), "attenuate": rgb(albedo) })
            },
            "plastic" | "rough_plastic" => {
                if num("sigma_a", 0.) != 0. || vec3(&b["sigma_a"]).is_some_and(|s| s != V3::zeros()) {
                    self.warn(format!("{}: the coat's absorption (sigma_a) is dropped", context));
                }
                let roughness = if t == "rough_plastic" { num("roughness", 0.1) } else { 0. };
                json!({ "type": "plastic", "albedo": rgb(albedo), "refractive_index": num("ior", 1.5), "roughness": roughness })
            },
            "smooth_coat" | "rough_coat" => {
                let substrate = self.bsdf(&b["substrate"], &format!("{} substrate", context));
                match substrate {
                    Some(s) if s["type"] == "lambertian" => {
                        let base = vec3(&s["albedo"]).unwrap_or_else(|| V3::new(1., 1., 1.));
                        let roughness = if t == "rough_coat" { num("roughness", 0.02) } else { 0. };
                        json!({ "type": "plastic", "albedo": rgb(base.component_mul(&albedo)), "refractive_index": num("ior", 1.3), "roughness": roughness })
                    },
                    Some(s) => {
                        self.warn(format!("{}: the coat is dropped, leaving its substrate", context));
                        s
                    },
                    None => return None,
                }
            },
            _ => {
                self.warn(format!("{}: unsupported bsdf, using grey lambertian", context));
                json!({ "type": "lambertian", "albedo": [0.5, 0.5, 0.5] })
            },
        })
    }

    /// The name of our material for a primitive, or None if it is invisible. `area` converts
    /// `power` to radiance.
    fn material(&mut self, p: &Value, name: &str, area: Option<f64>) -> Result<Option<String>, String> {
        let bsdf_ref = match &p["bsdf"] {
            Value::Array(a) => {
                self.warn(format!("{}: a bsdf per mesh part is not supported, using the first", name));
                a.first().cloned().unwrap_or(Value::Null)
            },
            b => b.clone(),
        };
        let (bsdf, bsdf_name) = match &bsdf_ref {
            Value::String(s) => (self.bsdfs.get(s).cloned().ok_or_else(|| format!("{}: no bsdf named \"{}\"", name, s))?, s.clone()),
            Value::Null => (json!({ "type": "lambert" }), "default".to_string()),
            b => (b.clone(), format!("{}.bsdf", name)),
        };

        let emission = match (&p["emission"], &p["power"]) {
            (Value::Null, Value::Null) => None,
            (e, Value::Null) => Some(self.colour(e, 0., &format!("{} emission", name))),
            (_, power) => {
                let power = self.colour(power, 0., &format!("{} power", name));
                match area {
                    Some(a) => Some(power / (PI * a)),
                    None => {
                        self.warn(format!("{}: power is supported for quads, cubes, disks and spheres; it doesn't emit", name));
                        None
                    },
                }
            },
        };
        if let Some(radiance) = emission.filter(|e| *e != V3::zeros()) {
            if bsdf["type"] != "null" && vec3(&bsdf["albedo"]) != Some(V3::zeros()) {
                self.warn(format!("{}: an emitter can't also reflect, so its bsdf is dropped", name));
            }
            let m = format!("{}.emission", name);
            self.materials.insert(m.clone(), json!({ "type": "diffuse-light", "color": rgb(radiance), "intensity": 1 }));
            return Ok(Some(m));
        }

        if self.materials.contains_key(&bsdf_name) {
            return Ok(Some(bsdf_name));
        }
        Ok(match self.bsdf(&bsdf, &format!("bsdf \"{}\"", bsdf_name)) {
            Some(m) => {
                self.materials.insert(bsdf_name.clone(), m);
                Some(bsdf_name)
            },
            None => None,
        })
    }

    fn primitive(&mut self, p: &Value, index: usize) -> Result<(), String> {
        let t = p["type"].as_str().unwrap_or("").to_string();
        let name = p["name"].as_str().map_or_else(|| format!("{}{}", t, index), str::to_string);
        let m = transform_matrix(&p["transform"]).map_err(|e| format!("primitive {}: {}", name, e))?;
        if !p["int_medium"].is_null() || !p["ext_medium"].is_null() {
            self.warn(format!("{}: participating media are not supported", name));
        }

        let (geometry, area): (Vec<Value>, Option<f64>) = match t.as_str() {
            "quad" => { let (g, a) = quad(&m); (g, Some(a)) },
            "cube" => { let (g, a) = cube(&m); (g, Some(a)) },
            "disk" => { let (g, a) = disk(&m); (g, Some(a)) },
            "sphere" => {
                let c = point(&m, V3::zeros());
                let l = linear(&m);
                let r = l.column(0).norm().max(l.column(1).norm()).max(l.column(2).norm());
                (vec![json!({ "type": "sphere", "location": [c.x, c.y, c.z], "radius": r })], Some(4. * PI * r * r))
            },
            "mesh" => {
                let file = p["file"].as_str().ok_or_else(|| format!("mesh {} has no \"file\"", name))?;
                let src = self.dir.join(file).to_string_lossy().into_owned();
                let kind = if p["smooth"].as_bool().unwrap_or(false) { "smoothmesh" } else { "mesh" };
                let mut warnings = Vec::new();
                let g = transformed(json!({ "type": kind, "src": src }), &m, &mut |w| warnings.push(w));
                for w in warnings { self.warn(w) }
                (vec![g], None)
            },
            "point" => {
                let at = point(&m, V3::zeros());
                let intensity = match (&p["emission"], &p["power"]) {
                    (Value::Null, power) => self.colour(power, 0., &format!("{} power", name)) / 4.,
                    (e, _) => self.colour(e, 0., &format!("{} emission", name)) * PI,
                };
                self.lights.push(json!({ "location": [at.x, at.y, at.z], "color": rgb(intensity), "intensity": 1 }));
                return Ok(());
            },
            "infinite_sphere" => {
                let e = match (&p["emission"], &p["power"]) {
                    (Value::Null, Value::Null) => None,
                    (Value::Null, power) => vec3(power).map(|x| x / (4. * PI)),
                    (e, _) => vec3(e),
                };
                match e {
                    Some(e) => self.background = e,
                    None => self.warn(format!("{}: only a constant infinite_sphere is supported (as the background)", name)),
                }
                return Ok(());
            },
            _ => {
                self.warn(format!("{}: unsupported primitive \"{}\", skipped", name, t));
                return Ok(());
            },
        };

        match self.material(p, &name, area)? {
            Some(material) => self.objects.extend(geometry.into_iter().map(|mut g| {
                g["material"] = json!(material);
                g
            })),
            None => self.warn(format!("{}: null bsdf and no emission, skipped", name)),
        }
        Ok(())
    }
}

/// Translate a Tungsten scene into our scene file format, with the warnings for what it can't
/// represent. Mesh files are found relative to `dir`, the scene file's directory.
pub fn translate(doc: &Value, dir: &Path) -> Result<(Value, Vec<String>), String> {
    let bsdfs: Map<String, Value> = doc["bsdfs"].as_array().ok_or("\"bsdfs\" must be an array")?.iter()
        .filter_map(|b| Some((b["name"].as_str()?.to_string(), b.clone())))
        .collect();
    let mut im = Importer { bsdfs, materials: Map::new(), objects: Vec::new(), lights: Vec::new(), background: V3::zeros(), warnings: Vec::new(), dir };
    if doc["media"].as_array().is_some_and(|m| !m.is_empty()) {
        im.warn("participating media are not supported".to_string());
    }
    for (i, p) in doc["primitives"].as_array().ok_or("\"primitives\" must be an array")?.iter().enumerate() {
        im.primitive(p, i)?;
    }

    let cam = &doc["camera"];
    let m = transform_matrix(&cam["transform"]).map_err(|e| format!("camera: {}", e))?;
    let location = point(&m, V3::zeros());
    let mut lookat = location + linear(&m) * V3::z();
    let mut up = linear(&m) * V3::y();
    if cam["transform"].is_object() {
        if let Some(l) = field3(&cam["transform"], "look_at")? { lookat = l; }
        if let Some(u) = field3(&cam["transform"], "up")? { up = u; }
    }
    let (width, height) = match &cam["resolution"] {
        Value::Null => (1000, 563),
        Value::Number(n) => { let n = n.as_u64().ok_or("camera resolution must be whole")?; (n, n) },
        v => match v.as_array().map(|a| a.iter().map(Value::as_u64).collect::<Option<Vec<_>>>()) {
            Some(Some(r)) if r.len() == 2 => (r[0], r[1]),
            _ => return Err(format!("camera resolution must be a number or two, got {}", v)),
        },
    };
    let fov = cam["fov"].as_f64().unwrap_or(60.).to_radians();
    let angle = (2. * (fov / 2.).tan() * height as f64 / width as f64).atan();
    let aperture = match cam["type"].as_str() {
        Some("thinlens") => {
            if !cam["focus_pivot"].is_null() {
                im.warn("camera: focus_pivot is not supported; using focus_distance".to_string());
            }
            let focus = cam["focus_distance"].as_f64().unwrap_or(1.);
            lookat = location + (lookat - location).normalize() * focus;
            cam["aperture_size"].as_f64().unwrap_or(0.001)
        },
        None | Some("pinhole") => 0.,
        Some(t) => {
            im.warn(format!("camera: unsupported type \"{}\", using a pinhole", t));
            0.
        },
    };
    let filter = match cam["reconstruction_filter"].as_str() {
        Some("box") => "box",
        None | Some("tent") => "tent",
        Some(f) => {
            im.warn(format!("camera: reconstruction filter \"{}\" is not supported, using tent", f));
            "tent"
        },
    };

    let integrator = &doc["integrator"];
    if integrator["type"].as_str().is_some_and(|t| t != "path_tracer") {
        im.warn(format!("integrator \"{}\" is rendered with our path tracer", integrator["type"].as_str().unwrap()));
    }
    let mut scene = json!({
        "width": width,
        "height": height,
        "max_depth": integrator["max_bounces"].as_u64().unwrap_or(64),
        "path_termination": "russian_roulette",
        "sample_lights": integrator["enable_light_sampling"].as_bool().unwrap_or(true),
        "filter": filter,
        "background": rgb(im.background),
        "camera": { "location": rgb(location), "lookat": rgb(lookat), "up": rgb(up), "angle": angle, "aperture": aperture },
        "lights": im.lights,
        "materials": im.materials,
        "objects": im.objects,
    });
    if let Some(spp) = doc["renderer"]["spp"].as_u64() {
        scene["supersamples"] = json!(spp);
    }
    Ok((scene, im.warnings))
}

/// Translate a Tungsten scene, printing warnings for what it can't represent.
pub fn to_scene(doc: &Value, dir: &Path) -> Result<Value, String> {
    let (scene, warnings) = translate(doc, dir)?;
    for w in warnings {
        println!("Warning: Tungsten: {}", w);
    }
    Ok(scene)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenefile::SceneFile;
    use crate::na::Rotation3;

    fn v(o: &Value, k: &str) -> V3 {
        vec3(&o[k]).unwrap()
    }

    fn normal(t: &Value) -> V3 {
        let (a, b, c) = (v(t, "v0"), v(t, "v1"), v(t, "v2"));
        (b - a).cross(&(c - a)).normalize()
    }

    fn scene(primitives: Value, bsdfs: Value) -> (Value, Vec<String>) {
        translate(&json!({ "bsdfs": bsdfs, "primitives": primitives, "camera": {} }), Path::new("scenes")).unwrap()
    }

    /// The Cornell box's light, from Bitterli's Tungsten scene, against the corners and normal
    /// pbrt-v4's version gives it (`demo/reference/cornell-box/light.obj`).
    #[test]
    fn test_quad_matches_pbrt() {
        let (s, _) = scene(json!([{
            "type": "quad", "name": "light", "bsdf": "light", "emission": [17, 12, 4],
            "transform": { "position": [-0.005, 1.98, -0.03], "scale": [0.47, 0.1786, 0.38], "rotation": [0, 180, 180] },
        }]), json!([{ "name": "light", "type": "null" }]));
        let tris = s["objects"].as_array().unwrap();
        assert_eq!(tris.len(), 2);
        let mut corners: Vec<V3> = tris.iter().flat_map(|t| ["v0", "v1", "v2"].map(|k| v(t, k))).collect();
        for c in [V3::new(-0.24, 1.98, -0.22), V3::new(0.23, 1.98, -0.22), V3::new(0.23, 1.98, 0.16), V3::new(-0.24, 1.98, 0.16)] {
            assert!(corners.iter().any(|p| (p - c).norm() < 1e-6), "missing corner {}", c);
        }
        corners.clear();
        for t in tris {
            assert!((normal(t) - V3::new(0., -1., 0.)).norm() < 1e-6, "the light faces down");
            assert_eq!(t["material"], "light.emission");
        }
        assert_eq!(s["materials"]["light.emission"], json!({ "type": "diffuse-light", "color": [17., 12., 4.], "intensity": 1 }));
    }

    #[test]
    fn test_cube_faces_outwards() {
        for scale in [json!([1, 2, 3]), json!([-1, 2, 3])] {
            let (s, _) = scene(json!([{ "type": "cube", "transform": { "position": [5, 0, 0], "scale": scale } }]), json!([]));
            let tris = s["objects"].as_array().unwrap();
            assert_eq!(tris.len(), 12);
            for t in tris {
                let centre = (v(t, "v0") + v(t, "v1") + v(t, "v2")) / 3.;
                assert!(normal(t).dot(&(centre - V3::new(5., 0., 0.))) > 0., "inward face for scale {}", scale);
            }
            assert_eq!(s["materials"]["default"]["type"], "lambertian");
        }
    }

    #[test]
    fn test_power_becomes_radiance() {
        // A 2 x 3 quad of power 6 pi: radiance 6 pi / (pi 6) = 1.
        let (s, _) = scene(json!([{ "type": "quad", "name": "q", "power": 6. * PI, "transform": { "scale": [2, 1, 3] } }]), json!([]));
        let c = v(&s["materials"]["q.emission"], "color");
        assert!((c - V3::new(1., 1., 1.)).norm() < 1e-12, "{}", c);
    }

    #[test]
    fn test_look_at_frame() {
        let m = transform_matrix(&json!({ "position": [1, 2, 3], "look_at": [1, 2, 10], "up": [0, 1, 0] })).unwrap();
        let l = linear(&m);
        assert!((l * V3::z() - V3::z()).norm() < 1e-12);
        assert!((l * V3::y() - V3::y()).norm() < 1e-12);
        assert!((l * V3::x() - V3::x()).norm() < 1e-12, "right-handed");
        assert_eq!(point(&m, V3::zeros()), V3::new(1., 2., 3.));
        // Rows, as written.
        let m = transform_matrix(&json!([1, 0, 0, 4, 0, 1, 0, 5, 0, 0, 1, 6, 0, 0, 0, 1])).unwrap();
        assert_eq!(point(&m, V3::zeros()), V3::new(4., 5., 6.));
    }

    #[test]
    fn test_rotations_compose_as_tungsten() {
        // rotYXZ is the rotation about y, after x, after z.
        let (a, b, c) = (23., -71., 140.);
        let composed = rot_yxz(V3::new(0., b, 0.)) * rot_yxz(V3::new(a, 0., 0.)) * rot_yxz(V3::new(0., 0., c));
        assert!((rot_yxz(V3::new(a, b, c)) - composed).abs().max() < 1e-12);
        let r = rot_yxz(V3::new(a, b, c));
        assert!((r.transpose() * r - Matrix3::identity()).abs().max() < 1e-12 && (r.determinant() - 1.).abs() < 1e-12);
        // About z, x turns to y.
        assert!((rot_yxz(V3::new(0., 0., 90.)) * V3::x() - V3::y()).norm() < 1e-12);
    }

    #[test]
    fn test_mesh_transform_round_trips() {
        let t = json!({ "position": [1, 2, 3], "scale": [2, 3, 4], "rotation": [10, 20, 30] });
        let (s, w) = scene(json!([{ "type": "mesh", "file": "models/a.wo3", "smooth": true, "transform": t }]), json!([]));
        assert!(w.is_empty(), "{:?}", w);
        let o = &s["objects"][0];
        assert_eq!(o["type"], "transform");
        assert_eq!(o["item"], json!({ "type": "smoothmesh", "src": Path::new("scenes").join("models/a.wo3").to_string_lossy() }));
        let deg = |k: &str| o["rotate"][k].as_f64().unwrap().to_radians();
        let ours = Rotation3::from_euler_angles(deg("roll"), deg("pitch"), deg("yaw")).matrix() * Matrix3::from_diagonal(&v(o, "scale"));
        let theirs = linear(&transform_matrix(&t).unwrap());
        assert!((ours - theirs).abs().max() < 1e-9, "{} vs {}", ours, theirs);
        assert_eq!(v(o, "translate"), V3::new(1., 2., 3.));
    }

    #[test]
    fn test_materials() {
        let bsdfs = json!([
            { "name": "gold", "type": "conductor", "material": "Au" },
            { "name": "custom", "type": "rough_conductor", "eta": 1, "k": 0, "roughness": 0.3, "distribution": "beckmann" },
            { "name": "glass", "type": "dielectric", "ior": 1.33 },
            { "name": "plastic", "type": "rough_plastic", "albedo": [0.1, 0.2, 0.3] },
            { "name": "textured", "type": "lambert", "albedo": "wood.png" },
            { "name": "odd", "type": "hair" },
            { "name": "nothing", "type": "null" },
        ]);
        let prims: Vec<Value> = ["gold", "custom", "glass", "plastic", "textured", "odd", "nothing"].iter()
            .map(|b| json!({ "type": "sphere", "bsdf": b })).collect();
        let (s, w) = scene(json!(prims), bsdfs);
        let m = &s["materials"];
        let au = v(&m["gold"], "reflective");
        assert!(au.x > 0.9 && au.z < 0.6, "gold is yellow: {}", au);
        assert_eq!(m["custom"]["reflective"], json!([0., 0., 0.]), "eta 1, k 0 reflects nothing");
        assert_eq!(m["custom"]["roughness"], 0.3);
        assert_eq!(m["glass"], json!({ "type": "dielectric", "refractive_index": 1.33, "attenuate": [1., 1., 1.] }));
        assert_eq!(m["plastic"]["refractive_index"], 1.5);
        assert_eq!(m["textured"]["albedo"], json!([0.5, 0.5, 0.5]));
        assert_eq!(m["odd"]["type"], "lambertian");
        assert_eq!(s["objects"].as_array().unwrap().len(), 6, "the null sphere is skipped");
        for expected in ["textures are not supported", "unsupported bsdf", "null bsdf and no emission", "approximated by GGX"] {
            assert!(w.iter().any(|x| x.contains(expected)), "no warning {:?} in {:?}", expected, w);
        }
    }

    #[test]
    fn test_lights_and_background() {
        let (s, _) = scene(json!([
            { "type": "point", "power": [4, 8, 12], "transform": { "position": [0, 5, 0] } },
            { "type": "infinite_sphere", "emission": [0.1, 0.2, 0.3] },
            { "type": "curves" },
        ]), json!([]));
        assert_eq!(s["lights"], json!([{ "location": [0., 5., 0.], "color": [1., 2., 3.], "intensity": 1 }]));
        assert_eq!(s["background"], json!([0.1, 0.2, 0.3]));
        assert!(s["objects"].as_array().unwrap().is_empty());
    }

    #[test]
    fn test_camera() {
        let doc = json!({ "bsdfs": [], "primitives": [], "renderer": { "spp": 16 }, "integrator": { "max_bounces": 8 }, "camera": {
            "type": "thinlens", "fov": 90, "resolution": [200, 100], "focus_distance": 2, "aperture_size": 0.1,
            "transform": { "position": [0, 0, 5], "look_at": [0, 0, 0], "up": [0, 1, 0] },
        } });
        let (s, _) = translate(&doc, Path::new("")).unwrap();
        let c = &s["camera"];
        assert_eq!(v(c, "lookat"), V3::new(0., 0., 3.), "the look-at point is the focus");
        assert_eq!(c["aperture"], 0.1);
        // 90 degrees across 200 pixels: the 100 pixel height spans 1 at unit distance.
        assert!((c["angle"].as_f64().unwrap() - 1f64.atan()).abs() < 1e-12);
        assert_eq!((s["width"].as_u64(), s["height"].as_u64()), (Some(200), Some(100)));
        assert_eq!(s["supersamples"], 16);
        assert_eq!(s["max_depth"], 8);
        assert_eq!(s["filter"], "tent");
    }

    #[test]
    fn test_scene_files_are_recognised() {
        let dir = std::env::temp_dir().join(format!("rays-tungsten-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("models")).unwrap();
        std::fs::write(dir.join("models/tri.wo3"), crate::shapes::wo3::write(&[[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]], None, &[[0, 1, 2]])).unwrap();
        let file = dir.join("scene.json");
        std::fs::write(&file, json!({
            "bsdfs": [{ "name": "white", "type": "lambert" }],
            "primitives": [
                { "type": "mesh", "file": "models/tri.wo3", "bsdf": "white", "transform": { "rotation": [0, 45, 0] } },
                { "type": "quad", "bsdf": "white" },
            ],
            "camera": { "resolution": 8, "transform": { "position": [0, 0, 5], "look_at": [0, 0, 0] } },
        }).to_string()).unwrap();
        let doc = SceneFile::read_document(file.to_str().unwrap()).unwrap();
        assert_eq!(doc["width"], 8);
        let s = SceneFile::from_value_overridden(doc, &Default::default());
        assert_eq!(s.objects.primitives_len(), 3);
        std::fs::remove_dir_all(dir).unwrap();
    }
}
