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
        intersects_dist(self.v0, self.v1, self.v2, r).map(|x| RawIntersection { face: 0,
                dist: x.dist, 
                point: x.point,
                normal: self.normal 
            })
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
        intersects_dist(self.v0, self.v1, self.v2, r).map(|x| RawIntersection { face: 0,
                dist: x.dist, 
                point: x.point,
                normal: self.interpolate_normal(&x) 
            })
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
