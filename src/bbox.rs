use na::{Vec3};
use std::fmt;
//use ray::Ray;

// Axis aligned bounding box
#[derive(Debug, Copy, Clone)]
pub struct BBox {
    pub min: Vec3<f64>, // Point closest to origin
    pub max: Vec3<f64>,
}

impl BBox {
    pub fn new(min: Vec3<f64>, max: Vec3<f64>) -> BBox {
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
        let zoffs = octant % 4;

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
            min: Vec3::new(xmin, ymin, zmin),
            max: Vec3::new(xmax, ymax, zmax),
        }
    }

    pub fn intersects(&self, ro: &Vec3<f64>, invrd: &Vec3<f64>) -> bool {
        //http://tavianator.com/fast-branchless-raybounding-box-intersections/

        let tx1 = (self.min.x - ro.x) * invrd.x;
        let tx2 = (self.max.x - ro.x) * invrd.x;

        let tmin = f64::min(tx1, tx2);
        let tmax = f64::max(tx1, tx2);

        let ty1 = (self.min.y - ro.y) * invrd.y;
        let ty2 = (self.max.y - ro.y) * invrd.y;

        let tmin2 = f64::max(tmin, f64::min(ty1, ty2));
        let tmax2 = f64::min(tmax, f64::max(ty1, ty2));

        return tmax2 >= tmin2;
    }

    pub fn mid(&self) -> Vec3<f64> {
        Vec3::new(
            &self.min.x + (&self.max.x - &self.min.x)/2f64,
            &self.min.y + (&self.max.y - &self.min.y)/2f64,
            &self.min.z + (&self.max.z - &self.min.z)/2f64,
        )
    }

    pub fn size(&self) -> Vec3<f64> {
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

    pub fn union_point(self, p: &Vec3<f64>) -> BBox{
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
/*

vec3 vec3_invert(vec3 rd){
  return (vec3) {1.0/rd.x, 1.0/rd.y, 1.0/rd.z}; 
};

bool contains(BBox a, vec3 pt){
  if (pt.x < a.min.x || pt.x > a.max.x) return false;
  if (pt.y < a.min.y || pt.y > a.max.y) return false;
  if (pt.z < a.min.z || pt.z > a.max.z) return false;
  return true;
}




*/
