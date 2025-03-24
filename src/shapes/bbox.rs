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

#[inline]
fn find_min_max(min: &Vector3<f64>, max: &Vector3<f64>, ro: &Vector3<f64>, invrd: &Vector3<f64>) -> (f64, f64){
    let t1 = (min.x - ro.x) * invrd.x;
    let t2 = (max.x - ro.x) * invrd.x;
    let t3 = (min.y - ro.y) * invrd.y;
    let t4 = (max.y - ro.y) * invrd.y;
    let t5 = (min.z - ro.z) * invrd.z;
    let t6 = (max.z - ro.z) * invrd.z;

    let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
    let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));
    (tmin, tmax)
}

impl BBox {
    pub fn new(min: Vector3<f64>, max: Vector3<f64>) -> BBox {
        BBox {min: Point3::from(min), max: Point3::from(max)}
    }

    pub fn min() -> BBox {
        BBox::new(
            Vector3::new(0., 0., 0.),
            Vector3::new(0., 0., 0.)
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
        ret = ret.union_point(&(transform * &Point3::new(self.max.x, self.min.y, self.min.z)).coords);
        ret = ret.union_point(&(transform * &Point3::new(self.min.x, self.max.y, self.min.z)).coords);
        ret = ret.union_point(&(transform * &Point3::new(self.min.x, self.min.y, self.max.z)).coords);
        ret = ret.union_point(&(transform * &Point3::new(self.max.x, self.max.y, self.min.z)).coords);
        ret = ret.union_point(&(transform * &Point3::new(self.max.x, self.min.y, self.max.z)).coords);
        ret = ret.union_point(&(transform * &Point3::new(self.min.x, self.max.y, self.max.z)).coords);
        ret = ret.union_point(&(transform * &Point3::new(self.max.x, self.max.y, self.max.z)).coords);
        ret
    }

    pub fn entry_face(&self, r: &Ray) -> Option<BoxFace> {
        let invrd = vec3_invert(&r.rd);
        if !self.fast_intersects(&r.ro, &invrd) { return None; }
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
                    return Some(BoxFace::Left);
                }
                return Some(BoxFace::Right);
            }
        } else if ty0 > tz0 {
            // Plane Y -> Top or Bottom
            if r.rd.y > 0. { 
                return Some(BoxFace::Bottom);
            }
            return Some(BoxFace::Top);
        } 
        // Plane Z -> Front or Back
        if r.rd.z > 0. { 
            return Some(BoxFace::Front);
        }
        Some(BoxFace::Back)
    }

    pub fn exit_face(&self, r: &Ray) -> Option<BoxFace> {
        let invrd = vec3_invert(&r.rd);
        if !self.fast_intersects(&r.ro, &invrd) { return None; }
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
                    return Some(BoxFace::Right);
                }
                return Some(BoxFace::Left);
            }
        } else if ty1 < tz1 {
            // Plane Y -> Top or Bottom
            if r.rd.y > 0. { 
                return Some(BoxFace::Top);
            }
            return Some(BoxFace::Bottom);
        } 
        // Plane Z -> Front or Back
        if r.rd.z > 0. { 
            return Some(BoxFace::Back);
        }
        Some(BoxFace::Front)
    }

    pub fn mid(&self) -> Vector3<f64> {
        Vector3::new(
            &self.min.x + (self.max.x - self.min.x)/2f64,
            &self.min.y + (self.max.y - self.min.y)/2f64,
            &self.min.z + (self.max.z - self.min.z)/2f64,
        )
    }

    pub fn size(&self) -> Vector3<f64> {
        self.max - self.min
    }


    pub fn intersects_bbox(&self, b: &BBox) -> bool{
          if self.max.x < b.min.x { return false; } // self is left of b
          if self.min.x > b.max.x { return false; }// self is right of b
          if self.max.y < b.min.y { return false; }// self is above b
          if self.min.y > b.max.y { return false; }// self is below b
          if self.max.z < b.min.z { return false; }// self is behind b
          if self.min.z > b.max.z { return false; }// self is in front of b
          true// boxes overlap
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
        if self.min.x > b.min.x  { return false; }
        if self.min.y > b.min.y  { return false; }
        if self.min.z > b.min.z  { return false; }
        if self.max.x < b.max.x  { return false; }
        if self.max.y < b.max.y  { return false; }
        if self.max.z < b.max.z  { return false; }
        true
    }

    pub fn contains_point(self, pt: &Vector3<f64>) -> bool { 
      if pt.x < self.min.x || pt.x > self.max.x { return false; }
      if pt.y < self.min.y || pt.y > self.max.y { return false; }
      if pt.z < self.min.z || pt.z > self.max.z { return false; }
      true
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
            return None
        }
        let (tmin, _tmax) = find_min_max(&self.min.coords, &self.max.coords, &r.ro, &invrd);
        let dist = tmin;
		let point =  r.ro + (r.rd * dist);
		let center = na::center(&self.min, &self.max);
		let p = (point - center.coords).normalize().component_div(&(self.max - self.min)); 
        let ndir = p.iamax();
        let mut normal = Vector3::new(0.,0.,0.);
        normal[ndir] = if p[ndir].is_sign_positive() { 1. } else { -1. };

        Some(RawIntersection {
            point,
            dist,
            normal
        })
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
