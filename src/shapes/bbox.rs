use na::{Vector3};
use std::fmt;
use ray::Ray;
use intersection::RawIntersection;
use shapes::geometry::Geometry;

// Axis aligned bounding box
#[derive(Debug, Copy, Clone)]
pub struct BBox {
    pub min: Vector3<f64>, // Point closest to origin
    pub max: Vector3<f64>,
}

impl BBox {
    pub fn new(min: Vector3<f64>, max: Vector3<f64>) -> BBox {
        BBox {min: min, max: max}
    }

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

        return BBox {
            min: Vector3::new(xmin, ymin, zmin),
            max: Vector3::new(xmax, ymax, zmax),
        }
    }

    pub fn fast_intersects(&self, ro: &Vector3<f64>, invrd: &Vector3<f64>) -> bool {
        //http://tavianator.com/fast-branchless-raybounding-box-intersections/
        let t1 = (self.min.x - ro.x) * invrd.x;
        let t2 = (self.max.x - ro.x) * invrd.x;
        let t3 = (self.min.y - ro.y) * invrd.y;
        let t4 = (self.max.y - ro.y) * invrd.y;
        let t5 = (self.min.z - ro.z) * invrd.z;
        let t6 = (self.max.z - ro.z) * invrd.z;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        // if tmax < 0, ray (line) is intersecting AABB, but the whole AABB is behind us
        if tmax < 0. {
            return false;
        }

        // if tmin > tmax, ray doesn't intersect AABB
        if tmin > tmax {
            return false;
        }
        return true
    }

    pub fn mid(&self) -> Vector3<f64> {
        Vector3::new(
            &self.min.x + (&self.max.x - &self.min.x)/2f64,
            &self.min.y + (&self.max.y - &self.min.y)/2f64,
            &self.min.z + (&self.max.z - &self.min.z)/2f64,
        )
    }

    pub fn size(&self) -> Vector3<f64> {
        return self.max - self.min;
    }


    pub fn intersects_bbox(&self, b: &BBox) -> bool{
          if &self.max.x < &b.min.x { return false; } // self is left of b
          if &self.min.x > &b.max.x { return false; }// self is right of b
          if &self.max.y < &b.min.y { return false; }// self is above b
          if &self.min.y > &b.max.y { return false; }// self is below b
          if &self.max.z < &b.min.z { return false; }// self is behind b
          if &self.min.z > &b.max.z { return false; }// self is in front of b
          return true; // boxes overlap
    }

    pub fn union(self, b: &BBox) -> BBox {
        let mut o = self.clone();

        if &self.min.x > &b.min.x { o.min.x = b.min.x; } 
        if &self.max.x < &b.max.x { o.max.x = b.max.x; }
        if &self.min.y > &b.min.y { o.min.y = b.min.y; }
        if &self.max.y < &b.max.y { o.max.y = b.max.y; }
        if &self.min.z > &b.min.z { o.min.z = b.min.z; }
        if &self.max.z < &b.max.z { o.max.z = b.max.z; }

        return o;
    }

    pub fn union_point(self, p: &Vector3<f64>) -> BBox{
        let mut o = self.clone();

        if &self.min.x > &p.x { o.min.x = p.x; } 
        if &self.max.x < &p.x { o.max.x = p.x; }
        if &self.min.y > &p.y { o.min.y = p.y; }
        if &self.max.y < &p.y { o.max.y = p.y; }
        if &self.min.z > &p.z { o.min.z = p.z; }
        if &self.max.z < &p.z { o.max.z = p.z; }

        return o;
    }


    pub fn contains(self, b: &BBox) -> bool {
        if self.min.x > b.min.x  { return false; }
        if self.min.y > b.min.y  { return false; }
        if self.min.z > b.min.z  { return false; }
        if self.max.x < b.max.x  { return false; }
        if self.max.y < b.max.y  { return false; }
        if self.max.z < b.max.z  { return false; }
        return true;
    }

    pub fn contains_point(self, pt: Vector3<f64>) -> bool { 
      if pt.x < self.min.x || pt.x > self.max.x { return false; }
      if pt.y < self.min.y || pt.y > self.max.y { return false; }
      if pt.z < self.min.z || pt.z > self.max.z { return false; }
      return true;
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

fn vec3_invert(rd: Vector3<f64>) -> Vector3<f64> {
  return Vector3::new(1.0/rd.x, 1.0/rd.y, 1.0/rd.z); 
}


impl Geometry for BBox {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        let invrd = vec3_invert(r.rd);
        if !self.fast_intersects(&r.ro, &invrd) { 
            return None
        }

        let t1 = (self.min.x - r.ro.x) * invrd.x;
        let t2 = (self.max.x - r.ro.x) * invrd.x;
        let t3 = (self.min.y - r.ro.y) * invrd.y;
        let t4 = (self.max.y - r.ro.y) * invrd.y;
        let t5 = (self.min.z - r.ro.z) * invrd.z;
        let t6 = (self.max.z - r.ro.z) * invrd.z;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        // if tmax < 0, ray (line) is intersecting AABB, but the whole AABB is behind us
        if tmax < 0. {
            return None;
        }

        // if tmin > tmax, ray doesn't intersect AABB
        if tmin > tmax {
            return None;
        }
        let dist = tmin;
		let point =  r.ro + (r.rd * dist);
		let center = (self.min + self.max) * 0.5;
		let p = point - center; 
		let d = (self.min - self.max) * 0.5;
		let normal = Vector3::new(
			p.x / d.x.abs() * std::f64::EPSILON,
			p.y / d.y.abs() * std::f64::EPSILON,
			p.z / d.z.abs() * std::f64::EPSILON,
		).normalize();
		

        return Some(RawIntersection {
            point,
            dist,
            normal
        });
    }

    fn bounds(&self) -> BBox {
        return self.clone()
    }
}
