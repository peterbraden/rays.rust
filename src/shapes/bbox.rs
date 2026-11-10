use crate::na::{Vector3, Point3};
use crate::na::geometry::Transform;
use std::fmt;
use crate::ray::Ray;
use crate::intersection::RawIntersection;
use crate::shapes::geometry::Geometry;

#[derive(Debug, Copy, Clone)]
pub struct Box {
    pub min: Point3<f64>, // Point closest to origin
    pub max: Point3<f64>,
}

// Box but aligned with axes 
#[derive(Debug, Copy, Clone)]
pub struct BBox {
    pub min: Point3<f64>, // Point closest to origin
    pub max: Point3<f64>,
}

// As described looking along the Z-axis from 0 to positive.
#[derive(PartialEq, Eq, Debug)]
pub enum BoxFace {
    Top,
    Left,
    Right,
    Front,
    Back,
    Bottom,
}

#[derive(Debug)]
pub enum Octant {
    BottomFrontLeft = 0b000,
    BottomFrontRight = 0b001,
    TopFrontLeft = 0b010,
    TopFrontRight = 0b011,
    BottomBackLeft = 0b100,
    BottomBackRight = 0b101,
    TopBackLeft = 0b110,
    TopBackRight = 0b111,
} 

/// Entry and exit t for one axis' slab.
#[inline]
fn slab(min: f64, max: f64, ro: f64, invrd: f64) -> (f64, f64) {
    let t1 = (min - ro) * invrd;
    let t2 = (max - ro) * invrd;
    if t1.is_nan() || t2.is_nan() {
        // Ray parallel to the slab, starting on one of its faces (0 * inf): inside the closed
        // slab. Without this, f64::min/max skip the NaN and the box is wrongly culled.
        return (f64::NEG_INFINITY, f64::INFINITY);
    }
    (t1.min(t2), t1.max(t2))
}

#[inline]
fn find_min_max(min: &Vector3<f64>, max: &Vector3<f64>, ro: &Vector3<f64>, invrd: &Vector3<f64>) -> (f64, f64){
    let (x0, x1) = slab(min.x, max.x, ro.x, invrd.x);
    let (y0, y1) = slab(min.y, max.y, ro.y, invrd.y);
    let (z0, z1) = slab(min.z, max.z, ro.z, invrd.z);
    (x0.max(y0).max(z0), x1.min(y1).min(z1))
}

impl BBox {
    pub fn new(min: Vector3<f64>, max: Vector3<f64>) -> BBox {
        BBox {min: Point3::from(min), max: Point3::from(max)}
    }

    /// The identity for union: contains nothing, so the union of it with any box is that box.
    pub fn empty() -> BBox {
        BBox::new(
            Vector3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            Vector3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY)
        )
    }

    // Cast the enum here
    // Rust is kinda annoying in that it lets you go enum -> u8 but not vice versa.
    pub fn for_octant(octant: u8, bounds: &BBox) -> BBox {
        // octant is Z, Y, X
        // => 000 is aligned to z,y,x min boundaries
        // => 111 is aligned to z,y,x max
        // => 001 is aligned to z,y min, x max.
        // Calc offset from min.
        let xoffs = octant & 1;
        let yoffs = octant & 2;
        let zoffs = octant & 4;

        let xdiff = bounds.max.x - bounds.min.x;
        let ydiff = bounds.max.y - bounds.min.y;
        let zdiff = bounds.max.z - bounds.min.z;

        let xmin = bounds.min.x + (if xoffs !=0 { xdiff * 0.5 } else { 0f64 });
        let ymin = bounds.min.y + (if yoffs !=0 { ydiff * 0.5 } else { 0f64 });
        let zmin = bounds.min.z + (if zoffs !=0 { zdiff * 0.5 } else { 0f64 });

        let xmax = bounds.min.x + (if xoffs !=0 { xdiff } else { xdiff * 0.5 });
        let ymax = bounds.min.y + (if yoffs !=0 { ydiff } else { ydiff * 0.5 });
        let zmax = bounds.min.z + (if zoffs !=0 { zdiff } else { zdiff * 0.5 });

        BBox {
            min: Point3::new(xmin, ymin, zmin),
            max: Point3::new(xmax, ymax, zmax),
        }
    }

    /// Where the ray enters and leaves the box, as multiples of the ray direction; None if it
    /// misses, or the box is entirely behind the origin. Entry is negative if the origin is inside.
    pub fn entry_exit(&self, ro: &Vector3<f64>, invrd: &Vector3<f64>) -> Option<(f64, f64)> {
        let (tmin, tmax) = find_min_max(&self.min.coords, &self.max.coords, ro, invrd);
        if tmax < 0. || tmin > tmax { None } else { Some((tmin, tmax)) }
    }

    /// Where a shape solved numerically along a ray should start, given where the ray enters its
    /// bounds: a little before, so that a surface lying on the bounds is not at the very start
    /// where rounding could lose it, but not behind the ray's origin.
    pub fn solve_start(&self, entry: f64) -> f64 {
        (entry - 0.01 * (self.max - self.min).norm().min(1e6)).max(0.)
    }

    /// Where such a search should end, given where the ray leaves the bounds: a little after,
    /// for the same reason.
    pub fn solve_end(&self, exit: f64) -> f64 {
        exit + 0.01 * (self.max - self.min).norm().min(1e6)
    }

    pub fn fast_intersects(&self, ro: &Vector3<f64>, invrd: &Vector3<f64>) -> bool {
        //http://tavianator.com/fast-branchless-raybounding-box-intersections/
        let (tmin, tmax) = find_min_max(&self.min.coords, &self.max.coords, ro, invrd);

        // if tmax < 0, ray (line) is intersecting AABB, but the whole AABB is behind us
        if tmax < 0. {
            return false;
        }

        // if tmin > tmax, ray doesn't intersect AABB
        if tmin > tmax {
            return false;
        }

        true
    }

    pub fn transform(&self, transform: &na::Transform3<f64>) -> BBox {
        // Transform all vertices and then return bbox of that box.
        let mut ret = BBox::new(
            (transform * self.min).coords,
            (transform * self.min).coords
        );
        ret = ret.union_point(&(transform * Point3::new(self.max.x, self.min.y, self.min.z)).coords);
        ret = ret.union_point(&(transform * Point3::new(self.min.x, self.max.y, self.min.z)).coords);
        ret = ret.union_point(&(transform * Point3::new(self.min.x, self.min.y, self.max.z)).coords);
        ret = ret.union_point(&(transform * Point3::new(self.max.x, self.max.y, self.min.z)).coords);
        ret = ret.union_point(&(transform * Point3::new(self.max.x, self.min.y, self.max.z)).coords);
        ret = ret.union_point(&(transform * Point3::new(self.min.x, self.max.y, self.max.z)).coords);
        ret = ret.union_point(&(transform * Point3::new(self.max.x, self.max.y, self.max.z)).coords);
        ret
    }

    pub fn entry_face(&self, r: &Ray) -> Option<BoxFace> {
        let invrd = vec3_invert(&r.rd);
        if !self.fast_intersects(&r.ro, &invrd) { 
            None
        } else {
            let xmin = if r.rd.x >= 0. { self.min.x } else { self.max.x };
            let ymin = if r.rd.y >= 0. { self.min.y } else { self.max.y };
            let zmin = if r.rd.z >= 0. { self.min.z } else { self.max.z };

            let tx0 = (xmin - r.ro.x) * invrd.x;
            let ty0 = (ymin - r.ro.y) * invrd.y;
            let tz0 = (zmin - r.ro.z) * invrd.z;

            if tx0 > ty0 {
                if tx0 > tz0 {
                    // Plane X -> Left or Right depending on ray direction
                    if r.rd.x > 0. { 
                        Some(BoxFace::Left)
                    } else {
                        Some(BoxFace::Right)
                    }
                } else {
                    // tz0 is largest
                    if r.rd.z > 0. { 
                        Some(BoxFace::Front)
                    } else {
                        Some(BoxFace::Back)
                    }
                }
            } else if ty0 > tz0 {
                // Plane Y -> Top or Bottom
                if r.rd.y > 0. { 
                    Some(BoxFace::Bottom)
                } else {
                    Some(BoxFace::Top)
                }
            } else {
                // tz0 is largest
                if r.rd.z > 0. { 
                    Some(BoxFace::Front)
                } else {
                    Some(BoxFace::Back)
                }
            }
        }
    }

    pub fn exit_face(&self, r: &Ray) -> Option<BoxFace> {
        let invrd = vec3_invert(&r.rd);
        if !self.fast_intersects(&r.ro, &invrd) { 
            None 
        } else {
            let xmax = if r.rd.x >= 0. { self.max.x } else { self.min.x };
            let ymax = if r.rd.y >= 0. { self.max.y } else { self.min.y };
            let zmax = if r.rd.z >= 0. { self.max.z } else { self.min.z };

            let tx1 = (xmax - r.ro.x) * invrd.x;
            let ty1 = (ymax - r.ro.y) * invrd.y;
            let tz1 = (zmax - r.ro.z) * invrd.z;

            if tx1 < ty1 {
                if tx1 < tz1 {
                    // Plane X -> Left or Right depending on ray direction
                    if r.rd.x > 0. { 
                        Some(BoxFace::Right)
                    } else {
                        Some(BoxFace::Left)
                    }
                } else {
                    // tz1 is smallest
                    if r.rd.z > 0. { 
                        Some(BoxFace::Back)
                    } else {
                        Some(BoxFace::Front)
                    }
                }
            } else if ty1 < tz1 {
                // Plane Y -> Top or Bottom
                if r.rd.y > 0. { 
                    Some(BoxFace::Top)
                } else {
                    Some(BoxFace::Bottom)
                }
            } else {
                // tz1 is smallest
                if r.rd.z > 0. { 
                    Some(BoxFace::Back)
                } else {
                    Some(BoxFace::Front)
                }
            }
        }
    }

    pub fn mid(&self) -> Vector3<f64> {
        Vector3::new(
            self.min.x + (self.max.x - self.min.x)/2f64,
            self.min.y + (self.max.y - self.min.y)/2f64,
            self.min.z + (self.max.z - self.min.z)/2f64,
        )
    }

    pub fn size(&self) -> Vector3<f64> {
        self.max - self.min
    }


    pub fn intersects_bbox(&self, b: &BBox) -> bool {
        // Check if there's any separation along any axis
        !(self.max.x < b.min.x || // self is left of b
          self.min.x > b.max.x || // self is right of b
          self.max.y < b.min.y || // self is above b
          self.min.y > b.max.y || // self is below b
          self.max.z < b.min.z || // self is behind b
          self.min.z > b.max.z)   // self is in front of b
    }

    pub fn union(self, b: &BBox) -> BBox {
        let mut o = self;
        if self.min.x > b.min.x { o.min.x = b.min.x; } 
        if self.max.x < b.max.x { o.max.x = b.max.x; }
        if self.min.y > b.min.y { o.min.y = b.min.y; }
        if self.max.y < b.max.y { o.max.y = b.max.y; }
        if self.min.z > b.min.z { o.min.z = b.min.z; }
        if self.max.z < b.max.z { o.max.z = b.max.z; }
        o
    }

    pub fn union_point(self, p: &Vector3<f64>) -> BBox{
        let mut o = self;
        if self.min.x > p.x { o.min.x = p.x; } 
        if self.max.x < p.x { o.max.x = p.x; }
        if self.min.y > p.y { o.min.y = p.y; }
        if self.max.y < p.y { o.max.y = p.y; }
        if self.min.z > p.z { o.min.z = p.z; }
        if self.max.z < p.z { o.max.z = p.z; }
        o
    }

    pub fn contains(self, b: &BBox) -> bool {
        // Check if all min/max constraints are met
        self.min.x <= b.min.x && 
        self.min.y <= b.min.y && 
        self.min.z <= b.min.z && 
        self.max.x >= b.max.x && 
        self.max.y >= b.max.y && 
        self.max.z >= b.max.z
    }

    pub fn contains_point(self, pt: &Vector3<f64>) -> bool { 
        // Point is inside if it's within all axis bounds
        pt.x >= self.min.x && pt.x <= self.max.x &&
        pt.y >= self.min.y && pt.y <= self.max.y &&
        pt.z >= self.min.z && pt.z <= self.max.z
    }

}


impl fmt::Display for BBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(BBox {},{},{} -> {},{},{} )",
                &self.min.x,
                &self.min.y,
                &self.min.z,
                &self.max.x,
                &self.max.y,
                &self.max.z)
    }
}

fn vec3_invert(rd: &Vector3<f64>) -> Vector3<f64> {
  Vector3::new(1.0/rd.x, 1.0/rd.y, 1.0/rd.z)
}


impl Geometry for BBox {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let invrd = vec3_invert(&r.rd);
        if !self.fast_intersects(&r.ro, &invrd) { 
            None
        } else {
            let (tmin, tmax) = find_min_max(&self.min.coords, &self.max.coords, &r.ro, &invrd);
            // If the ray starts inside the box, the hit is where it exits.
            let t = if tmin >= 0. { tmin } else { tmax };
            let point =  r.ro + (r.rd * t);
            let dist = t * r.rd.norm();
            // The normal is that of the face the point lies on, ie. the nearest face.
            let (mut nearest, mut axis, mut sign) = (f64::INFINITY, 0, 1.);
            for a in 0..3 {
                for (face, s) in [(self.min[a], -1.), (self.max[a], 1.)] {
                    let d = (point[a] - face).abs();
                    if d < nearest {
                        (nearest, axis, sign) = (d, a, s);
                    }
                }
            }
            if self.min[axis] == self.max[axis] {
                // Flat box: both faces coincide, so face the incoming ray.
                sign = if r.rd[axis] > 0. { -1. } else { 1. };
            }
            let mut normal = Vector3::new(0.,0.,0.);
            normal[axis] = sign;

            Some(RawIntersection { part: 0,
                point,
                dist,
                normal
            })
        }
    }

    fn bounds(&self) -> BBox {
        *self
    }

    fn fast_intersects(&self, r: &Ray) -> bool {
        BBox::fast_intersects(self, &r.ro, &vec3_invert(&r.rd))
    }

    fn inside(&self, p: &Vector3<f64>) -> bool {
        self.contains_point(p)
    }
}

#[cfg(test)]
mod tests {
    use crate::na::{Vector3};
    use super::{BBox, BoxFace, Ray};
    use crate::shapes::geometry::Geometry;

    #[test]
    fn test_ray_on_face_plane_is_not_culled() {
        // Parallel to x and lying exactly on the box's min.x face plane.
        let b = BBox::new(Vector3::new(0., -1., -1.), Vector3::new(1., 1., 1.));
        for rdx in [0., -0.] {
            let r = Ray { ro: Vector3::new(0., 0.5, -5.), rd: Vector3::new(rdx, 0., 1.) };
            assert!(Geometry::fast_intersects(&b, &r), "rd.x = {}", rdx);
            assert!(b.intersects(&r).is_some());
        }
        // Parallel and outside the slab still misses.
        let r = Ray { ro: Vector3::new(-0.5, 0.5, -5.), rd: Vector3::new(0., 0., 1.) };
        assert!(!Geometry::fast_intersects(&b, &r));
    }

    #[test]
    fn test_flat_box_normal() {
        let b = BBox::new(Vector3::new(-1., 0., -1.), Vector3::new(1., 0., 1.));
        let down = Ray { ro: Vector3::new(0.2, 5., 0.3), rd: Vector3::new(0., -1., 0.) };
        assert_eq!(b.intersects(&down).unwrap().normal, Vector3::new(0., 1., 0.));
        let up = Ray { ro: Vector3::new(0.2, -5., 0.3), rd: Vector3::new(0., 1., 0.) };
        assert_eq!(b.intersects(&up).unwrap().normal, Vector3::new(0., -1., 0.));
    }

    #[test]
    fn test_box_normals() {
        let b = BBox::new(Vector3::new(-1., -2., -3.), Vector3::new(1., 2., 3.));
        let cases = [
            (Vector3::new(5., 0.5, 0.5), Vector3::new(-1., 0., 0.), Vector3::new(1., 0., 0.)),
            (Vector3::new(0.5, -5., 0.5), Vector3::new(0., 1., 0.), Vector3::new(0., -1., 0.)),
            (Vector3::new(0.5, 0.5, 9.), Vector3::new(0., 0., -1.), Vector3::new(0., 0., 1.)),
        ];
        for (ro, rd, n) in cases {
            assert_eq!(b.intersects(&Ray { ro, rd }).unwrap().normal, n);
        }
    }

    #[test]
    fn test_intersects_dist_is_world_space() {
        let b = BBox::new(Vector3::new(-1., -1., -1.), Vector3::new(1., 1., 1.));
        let r = Ray { ro: Vector3::new(0., 0., -5.), rd: Vector3::new(0., 0., 2.) };
        let i = b.intersects(&r).unwrap();
        assert!((i.dist - 4.).abs() < 1e-9);
        assert!((i.point - Vector3::new(0., 0., -1.)).norm() < 1e-9);
    }

    #[test]
    fn test_intersects_from_inside_finds_exit() {
        let b = BBox::new(Vector3::new(-1., -1., -1.), Vector3::new(1., 1., 1.));
        let r = Ray { ro: Vector3::new(0., 0., 0.), rd: Vector3::new(0., 0., 1.) };
        let i = b.intersects(&r).unwrap();
        assert!((i.dist - 1.).abs() < 1e-9);
        assert!((i.point - Vector3::new(0., 0., 1.)).norm() < 1e-9);
        assert_eq!(i.normal, Vector3::new(0., 0., 1.));
    }

	#[test]
	fn test_entry_face1() {
        let b = BBox::new(
            Vector3::new(1., 1., 1.),
            Vector3::new(2., 2., 2.),
        );

        let r = Ray {
            ro:  Vector3::new(0., 0., 0.),
            rd:  Vector3::new(1.4, 1., 1.1),
        };
        
		assert!((b.entry_face(&r).unwrap() == BoxFace::Bottom));
		assert!((b.exit_face(&r).unwrap() == BoxFace::Right));
	}

	#[test]
	fn test_entry_face2() {
        let b = BBox::new(
            Vector3::new(1., 1., 1.),
            Vector3::new(2., 2., 2.),
        );

        let r = Ray {
            ro:  Vector3::new(1.5, 0., 1.5),
            rd:  Vector3::new(0., 1., 0.),
        };
        
		assert!((b.entry_face(&r).unwrap() == BoxFace::Bottom));
		assert!((b.exit_face(&r).unwrap() == BoxFace::Top));
	}

	#[test]
	fn test_entry_face3() {
        let b = BBox::new(
            Vector3::new(1., 1., 1.),
            Vector3::new(2., 2., 2.),
        );

        let r = Ray {
            ro:  Vector3::new(1.5, 3.5, 1.5),
            rd:  Vector3::new(0., -1., 0.),
        };
        
		assert!((b.entry_face(&r).unwrap() == BoxFace::Top));
		assert!((b.exit_face(&r).unwrap() == BoxFace::Bottom));
	}

	#[test]
	fn test_entry_face4() {
        let b = BBox::new(
            Vector3::new(1., 1., 1.),
            Vector3::new(2., 2., 2.),
        );

        let r = Ray {
            ro:  Vector3::new(0., 3.5, 1.5),
            rd:  (Vector3::new(3.5, 0., 1.5) - Vector3::new(0., 3.5, 1.5)).normalize(),
        };
        
		assert!((b.entry_face(&r).unwrap() == BoxFace::Top));
		assert!((b.exit_face(&r).unwrap() == BoxFace::Right));

        let r2 = Ray {
            ro:  Vector3::new(1.5, 3.5, 0.),
            rd:  (Vector3::new(1.5, 0., 3.5) - Vector3::new(1.5,  3.5, 0.)).normalize(),
        };
		assert!((b.entry_face(&r2).unwrap() == BoxFace::Top));
		assert!((b.exit_face(&r2).unwrap() == BoxFace::Back));

        let r3 = Ray {
            ro:  Vector3::new(1.5, 3.5, 2.5),
            rd:  (Vector3::new(1.5, 0., 0.) - Vector3::new(1.5,  3.5, 2.5)).normalize(),
        };
		assert!((b.entry_face(&r3).unwrap() == BoxFace::Top));
		assert!((b.exit_face(&r3).unwrap() == BoxFace::Front));
	}
}
