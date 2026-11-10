/// 
/// ## References
///
/// 1. https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/barycentric-coordinates
use crate::shapes::geometry::Geometry;
use crate::na::{Vector3};
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::bbox::BBox;

#[derive(Clone, Debug)]
pub struct Triangle {
    pub v0: Vector3<f64>,
    pub v1: Vector3<f64>,
    pub v2: Vector3<f64>,
    pub normal: Vector3<f64>,
}

fn panic_if_nan(v: Vector3<f64>) {
    if v.y.is_nan() {
        panic!("NaN in vector");
    }
}

impl Triangle {

    pub fn new(v0: Vector3<f64>, v1: Vector3<f64>, v2: Vector3<f64>) -> Triangle{
        let v0v1 = v1 - v0; 
        let v0v2 = v2 - v0; 

        //let area2 = normal.length(); // Before norm
        let normal = v0v1.cross(&v0v2).normalize();  
        //panic_if_nan(normal);
        Triangle {
            v0,
            v1,
            v2,
            normal,
        }
    }
    pub fn new_with_normal(v0: Vector3<f64>, v1: Vector3<f64>, v2: Vector3<f64>, normal: Vector3<f64>) -> Triangle{
        Triangle {
            v0,
            v1,
            v2,
            normal,
        }
    }

    pub fn translate_vec3(&self, v: Vector3<f64>) -> Triangle {
        Triangle {
            v0: self.v0 - v,
            v1: self.v1 - v,
            v2: self.v2 - v,
            normal: self.normal,
        }
    }
}

struct IntersectionPoint {
    dist: f64, 
    point: Vector3<f64>,
}

/// Möller and Trumbore's test, from Tomas Möller and Ben Trumbore, "Fast, Minimum Storage
/// Ray-Triangle Intersection", Journal of Graphics Tools 2(1), 21-28, 1997,
/// doi:10.1080/10867651.1997.10487468 (code: <https://fileadmin.cs.lth.se/cs/Personal/Tomas_Akenine-Moller/raytri/>).
/// It solves O + tD = (1 - u - v) V0 + u V1 + v V2 for (t, u, v) by Cramer's rule, where the
/// determinant is det = D . (E1 x E2), with edges E1 = V1 - V0 and E2 = V2 - V0.
///
/// det is zero when the ray is parallel to the triangle's plane. The paper's code treats
/// |det| < EPSILON = 1e-6 as parallel, but det = |D| |E1 x E2| cos(theta): its size depends on
/// the ray's length and the triangle's area as much as on the angle. A fixed threshold therefore
/// rejects every small triangle (the Stanford dragon's triangles have |E1 x E2| near 1e-7 at
/// their native scale) and every triangle met by a shortened ray, as under a scaling transform.
/// We reject only det = 0, as PBR 4th ed. section 6.5.3 does with Woop, Benthin and Wald's
/// watertight test (JCGT 2(1), 2013, <https://jcgt.org/published/0002/01/05/>): a near-parallel
/// ray gives a large 1/det, and the u, v bounds then reject it unless it really grazes the triangle.
fn intersects_dist(v0: Vector3<f64>, v1: Vector3<f64>, v2: Vector3<f64>, r: &Ray) -> Option<IntersectionPoint> {
        let v0v1 = v1 - v0; 
        let v0v2 = v2 - v0; 
        let pvec = r.rd.cross(&v0v2); 
        panic_if_nan(pvec);
        let det = v0v1.dot(&pvec); 
                     
        if det == 0. { return None };

        let inv_det = 1. / det; 
        let tvec = r.ro - v0; 
        let u = tvec.dot(&pvec) * inv_det; 

        if !(0. ..=1.).contains(&u) { return None }; 

        let qvec = tvec.cross(&v0v1); 
        panic_if_nan(qvec);
        let v = r.rd.dot(&qvec) * inv_det; 

        if v < 0. || u + v > 1. { return None }; 
                
        // t is in multiples of rd; scale to a world-space distance.
        let t = v0v2.dot(&qvec) * inv_det; 
                     
        if t > 0. {
            let point = r.ro + (r.rd * t);
            return Some(IntersectionPoint { dist: t * r.rd.norm(), point })
        }
        None
}


/// How many triangles `may_hit` tests at once. Four f64 lanes: two SSE2 or NEON registers, or one
/// AVX register.
pub const LANES: usize = 4;

/// A triangle as `intersects_dist` uses it: a corner and the edges from it. Containers keep these
/// in one contiguous array, to test several triangles at once with `may_hit`.
#[derive(Clone, Copy, Debug, Default)]
pub struct EdgeTriangle {
    v0: [f64; 3],
    e1: [f64; 3],
    e2: [f64; 3],
}

impl EdgeTriangle {
    pub fn new(v: &[Vector3<f64>; 3]) -> EdgeTriangle {
        // The same subtractions as `intersects_dist`, so the same edges to the last bit.
        let (e1, e2) = (v[1] - v[0], v[2] - v[0]);
        EdgeTriangle { v0: [v[0].x, v[0].y, v[0].z], e1: [e1.x, e1.y, e1.z], e2: [e2.x, e2.y, e2.z] }
    }
}

/// Slack in `may_hit`'s tests, so that it never rejects a triangle `intersects_dist` would hit.
const MAY_HIT_SLACK: f64 = 1e-9;

/// Which of up to `LANES` triangles (`tris[i]` for each `i` in `indices`) the ray may hit at a
/// world-space distance below `limit`: bit n is set for `indices[n]`.
///
/// This is Möller and Trumbore's test as in `intersects_dist`, for several triangles at once:
/// arranged lane by lane, with the products and sums in the same order, and no branches, the
/// compiler turns each line into vector instructions. It is a filter, run before the exact test
/// rather than in place of it: each triangle it passes is tested again by its own `intersects`,
/// which supplies the hit's normal (interpolated for a `SmoothTriangle`), and its bounds have
/// some slack, so that a filtered render matches an unfiltered one exactly. Most triangles in a
/// leaf are missed, so most are rejected here.
// The comparisons are combined with `&`, not `&&` (as `contains` would), so that no lane branches.
#[allow(clippy::manual_range_contains)]
pub fn may_hit(tris: &[EdgeTriangle], indices: &[usize], r: &Ray, len: f64, limit: f64) -> u32 {
    // Gather the triangles into one array per coordinate. Unused lanes keep a degenerate
    // triangle: det = 0, which fails every test below.
    let (mut v0x, mut v0y, mut v0z) = ([0.; LANES], [0.; LANES], [0.; LANES]);
    let (mut e1x, mut e1y, mut e1z) = ([0.; LANES], [0.; LANES], [0.; LANES]);
    let (mut e2x, mut e2y, mut e2z) = ([0.; LANES], [0.; LANES], [0.; LANES]);
    for (l, &i) in indices.iter().enumerate() {
        let t = &tris[i];
        (v0x[l], v0y[l], v0z[l]) = (t.v0[0], t.v0[1], t.v0[2]);
        (e1x[l], e1y[l], e1z[l]) = (t.e1[0], t.e1[1], t.e1[2]);
        (e2x[l], e2y[l], e2z[l]) = (t.e2[0], t.e2[1], t.e2[2]);
    }
    let (rdx, rdy, rdz) = (r.rd.x, r.rd.y, r.rd.z);
    let (rox, roy, roz) = (r.ro.x, r.ro.y, r.ro.z);

    let mut hits = 0;
    for l in 0..LANES {
        // pvec = rd x e2
        let px = rdy * e2z[l] - rdz * e2y[l];
        let py = rdz * e2x[l] - rdx * e2z[l];
        let pz = rdx * e2y[l] - rdy * e2x[l];
        let det = e1x[l] * px + e1y[l] * py + e1z[l] * pz;
        let inv_det = 1. / det;
        // tvec = ro - v0
        let tx = rox - v0x[l];
        let ty = roy - v0y[l];
        let tz = roz - v0z[l];
        let u = (tx * px + ty * py + tz * pz) * inv_det;
        // qvec = tvec x e1
        let qx = ty * e1z[l] - tz * e1y[l];
        let qy = tz * e1x[l] - tx * e1z[l];
        let qz = tx * e1y[l] - ty * e1x[l];
        let v = (rdx * qx + rdy * qy + rdz * qz) * inv_det;
        let dist = (e2x[l] * qx + e2y[l] * qy + e2z[l] * qz) * inv_det * len;
        // Comparisons with NaN (det = 0 gives inf * 0) are false, rejecting the lane.
        let hit = (det != 0.)
            & (u >= -MAY_HIT_SLACK) & (u <= 1. + MAY_HIT_SLACK)
            & (v >= -MAY_HIT_SLACK) & (u + v <= 1. + MAY_HIT_SLACK)
            & (dist > -MAY_HIT_SLACK) & (dist < limit * (1. + MAY_HIT_SLACK) + MAY_HIT_SLACK);
        hits |= (hit as u32) << l;
    }
    hits
}

/// A point uniformly distributed over a triangle's area, following Osada, Funkhouser, Chazelle
/// and Dobkin, "Shape Distributions", ACM Transactions on Graphics 21(4), 2002, section 4.2,
/// eq. (1): P = (1 - sqrt(r1)) A + sqrt(r1) (1 - r2) B + sqrt(r1) r2 C. sqrt(r1) sets how far
/// the point lies from A towards the opposite edge, and r2 where along it; the square root makes
/// the distribution uniform by area, since the triangle widens linearly away from A.
pub fn sample_triangle(a: Vector3<f64>, b: Vector3<f64>, c: Vector3<f64>, r1: f64, r2: f64) -> Vector3<f64> {
    let s = r1.sqrt();
    a * (1. - s) + b * (s * (1. - r2)) + c * (s * r2)
}

impl Geometry for Triangle {
    fn surface_area(&self) -> f64 {
        0.5 * (self.v1 - self.v0).cross(&(self.v2 - self.v0)).norm()
    }

    fn sample_surface(&self, u: [f64; 3]) -> Option<(Vector3<f64>, Vector3<f64>)> {
        Some((sample_triangle(self.v0, self.v1, self.v2, u[0], u[1]), self.normal))
    }

    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        intersects_dist(self.v0, self.v1, self.v2, r).map(|x| RawIntersection {
                dist: x.dist, 
                point: x.point,
                normal: self.normal 
            })
    }

    fn vertices(&self) -> Option<[Vector3<f64>; 3]> {
        Some([self.v0, self.v1, self.v2])
    }

    fn bounds(&self) -> BBox {
        BBox::new(
            Vector3::new(
                self.v0.x.min(self.v1.x).min(self.v2.x),
                self.v0.y.min(self.v1.y).min(self.v2.y),
                self.v0.z.min(self.v1.z).min(self.v2.z),
            ),
            Vector3::new(
                self.v0.x.max(self.v1.x).max(self.v2.x),
                self.v0.y.max(self.v1.y).max(self.v2.y),
                self.v0.z.max(self.v1.z).max(self.v2.z),
            )
        )
    }
}


fn triangle_area(v0: Vector3<f64>, v1: Vector3<f64>, v2: Vector3<f64>) -> f64 {
    let v0v1 = v1 - v0; 
    let v0v2 = v2 - v0; 
    // Magnitude of the cross product can be interpreted as the area of the parallelogram. See [1]
    v0v1.cross(&v0v2).norm() / 2.
}


#[derive(Clone, Debug)]
pub struct SmoothTriangle {
    pub v0: Vector3<f64>,
    pub v1: Vector3<f64>,
    pub v2: Vector3<f64>,
    pub normalv0: Vector3<f64>,
    pub normalv1: Vector3<f64>,
    pub normalv2: Vector3<f64>,
}


impl SmoothTriangle {
    pub fn new(
        v0: Vector3<f64>,
        v1: Vector3<f64>,
        v2: Vector3<f64>,
        normalv0: Vector3<f64>,
        normalv1: Vector3<f64>,
        normalv2: Vector3<f64>
    ) -> SmoothTriangle{
        SmoothTriangle { v0, v1, v2, normalv0, normalv1, normalv2 }
    }

    /*
    pub fn translate_vec3(&self, v: Vector3<f64>) -> Triangle {
        return Triangle {
            v0: self.v0 - v,
            v1: self.v1 - v,
            v2: self.v2 - v,
            normal: self.normal,
        }
    }
    */

    fn interpolate_normal(&self, p: &IntersectionPoint) -> Vector3<f64>{
        // Calculate barycentric coordinates of the intersection point
        // Barycentric coordinate components correspond to the proportional
        // area of the triangle between the internal point and each edge, and the
        // entire triangle. Thus:
        let a = triangle_area(self.v0, self.v1, p.point) / triangle_area(self.v0, self.v1, self.v2);
        let b = triangle_area(self.v0, self.v2, p.point) / triangle_area(self.v0, self.v1, self.v2);
        // We can skip c as a + b + c = 1
        (self.normalv2 * a  + self.normalv1 * b + (1. - a - b) * self.normalv0).normalize()
    }
}

impl Geometry for SmoothTriangle {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        intersects_dist(self.v0, self.v1, self.v2, r).map(|x| RawIntersection {
                dist: x.dist, 
                point: x.point,
                normal: self.interpolate_normal(&x) 
            })
    }

    fn vertices(&self) -> Option<[Vector3<f64>; 3]> {
        Some([self.v0, self.v1, self.v2])
    }

    // Copy of Triangle
    fn bounds(&self) -> BBox {
        BBox::new(
            Vector3::new(
                self.v0.x.min(self.v1.x).min(self.v2.x),
                self.v0.y.min(self.v1.y).min(self.v2.y),
                self.v0.z.min(self.v1.z).min(self.v2.z),
            ),
            Vector3::new(
                self.v0.x.max(self.v1.x).max(self.v2.x),
                self.v0.y.max(self.v1.y).max(self.v2.y),
                self.v0.z.max(self.v1.z).max(self.v2.z),
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tri_verts() -> (Vector3<f64>, Vector3<f64>, Vector3<f64>) {
        (Vector3::new(-1., -1., 0.), Vector3::new(1., -1., 0.), Vector3::new(0., 1., 0.))
    }

    #[test]
    fn test_triangle_samples_are_uniform() {
        let (a, b, c) = (Vector3::new(0., 0., 0.), Vector3::new(4., 0., 0.), Vector3::new(0., 2., 0.));
        crate::geometry::seed_rand(5, 0);
        let n = 20000;
        let mut mean = Vector3::zeros();
        let mut near_a = 0;
        for _ in 0..n {
            let p = sample_triangle(a, b, c, crate::geometry::rand(), crate::geometry::rand());
            assert!(p.x >= -1e-12 && p.y >= -1e-12 && p.x / 4. + p.y / 2. <= 1. + 1e-12, "outside: {}", p);
            mean += p / n as f64;
            // The quarter of the area nearest A: the triangle scaled by 1/2 about A.
            if p.x / 4. + p.y / 2. <= 0.5 { near_a += 1; }
        }
        // Uniform by area: the mean is the centroid, and a quarter of the points fall in the
        // quarter-area corner (without the square root, half would).
        assert!((mean - Vector3::new(4. / 3., 2. / 3., 0.)).norm() < 0.03, "{}", mean);
        assert!((near_a as f64 / n as f64 - 0.25).abs() < 0.01, "{}", near_a);
    }

    #[test]
    fn test_tiny_triangles_are_hit() {
        // A triangle 1e-4 across, as in a scanned mesh at native scale: det = 2e-8 here, which
        // a fixed threshold of 1e-7 rejected as parallel. And the same triangle 50x larger,
        // met by a ray shortened 50x as a scaling transform does.
        let s = 1e-4;
        let t = Triangle::new(Vector3::new(-s, -s, 0.), Vector3::new(s, -s, 0.), Vector3::new(0., s, 0.));
        let r = Ray { ro: Vector3::new(0., 0., -1.), rd: Vector3::new(0., 0., 1.) };
        assert!((t.intersects(&r).expect("hit").dist - 1.).abs() < 1e-9);

        let big = Triangle::new(t.v0 * 50., t.v1 * 50., t.v2 * 50.);
        let short = Ray { ro: Vector3::new(0., 0., -50.), rd: Vector3::new(0., 0., 1. / 50.) };
        assert!((big.intersects(&short).expect("hit").dist - 50.).abs() < 1e-9);
    }

    #[test]
    fn test_may_hit_passes_every_hit() {
        // Triangles from 1e-4 to 10 across, and rays aimed at them, some grazing their plane:
        // the filter must pass every triangle the exact test hits within the limit, and
        // should reject most it misses.
        use rand::{Rng, SeedableRng};
        use rand::rngs::StdRng;
        let mut rng: StdRng = SeedableRng::from_seed([7; 32]);
        let mut u = |a: f64, b: f64| a + (b - a) * rng.gen_range(0., 1.);
        let (mut hits, mut passed_misses, mut misses) = (0, 0, 0);
        for _ in 0..2000 {
            let scale = 10f64.powf(u(-4., 1.));
            let tris: Vec<Triangle> = (0..LANES).map(|_| {
                let c = Vector3::new(u(-1., 1.), u(-1., 1.), u(-1., 1.));
                let mut corner = || c + Vector3::new(u(-1., 1.), u(-1., 1.), u(-1., 1.)) * scale;
                Triangle::new(corner(), corner(), corner())
            }).collect();
            let edges: Vec<EdgeTriangle> = tris.iter().map(|t| EdgeTriangle::new(&t.vertices().unwrap())).collect();
            for _ in 0..20 {
                let target = sample_triangle(tris[0].v0, tris[0].v1, tris[0].v2, u(0., 1.), u(0., 1.));
                // Some rays nearly in the first triangle's plane.
                let along = (tris[0].v1 - tris[0].v0).normalize();
                let rd = if u(0., 1.) < 0.3 {
                    along + tris[0].normal * u(-1e-6, 1e-6)
                } else {
                    Vector3::new(u(-1., 1.), u(-1., 1.), u(-1., 1.))
                } * u(0.1, 10.);
                let r = Ray { ro: target - rd * u(0.5, 2.), rd };
                let limit = if u(0., 1.) < 0.5 { f64::INFINITY } else { u(0., 3.) };
                // Groups of one to four triangles.
                let n = 1 + (u(0., 1.) * LANES as f64) as usize % LANES;
                let indices: Vec<usize> = (0..n).collect();
                let mask = may_hit(&edges, &indices, &r, r.rd.norm(), limit);
                assert_eq!(mask >> n, 0, "unused lanes passed");
                for (lane, t) in tris[..n].iter().enumerate() {
                    let passed = mask & 1 << lane != 0;
                    match t.intersects(&r) {
                        Some(x) if x.dist < limit => {
                            hits += 1;
                            assert!(passed, "lane {} hit at {} < {} but rejected", lane, x.dist, limit);
                        },
                        _ => {
                            misses += 1;
                            if passed { passed_misses += 1 }
                        },
                    }
                }
            }
        }
        assert!(hits > 5000, "{}", hits);
        assert!(passed_misses * 100 < misses, "{} of {} misses passed", passed_misses, misses);
    }

    #[test]
    fn test_parallel_ray_misses() {
        let (v0, v1, v2) = tri_verts();
        let t = Triangle::new(v0, v1, v2);
        let r = Ray { ro: Vector3::new(-5., 0., 0.), rd: Vector3::new(1., 0., 0.) };
        assert!(t.intersects(&r).is_none());
    }

    #[test]
    fn test_intersects_dist_is_world_space() {
        let (v0, v1, v2) = tri_verts();
        let t = Triangle::new(v0, v1, v2);
        let r = Ray { ro: Vector3::new(0., 0., -5.), rd: Vector3::new(0., 0., 2.) };
        let i = t.intersects(&r).unwrap();
        assert!((i.dist - 5.).abs() < 1e-9);
        assert!((i.point - Vector3::new(0., 0., 0.)).norm() < 1e-9);
    }

    #[test]
    fn test_smooth_normal_is_unit_length() {
        let (v0, v1, v2) = tri_verts();
        let t = SmoothTriangle::new(
            v0, v1, v2,
            Vector3::new(-1., 0., -1.).normalize(),
            Vector3::new(1., 0., -1.).normalize(),
            Vector3::new(0., 1., -1.).normalize(),
        );
        let r = Ray { ro: Vector3::new(0., -0.3, -5.), rd: Vector3::new(0., 0., 1.) };
        let i = t.intersects(&r).unwrap();
        assert!((i.normal.norm() - 1.).abs() < 1e-9);
    }
}
