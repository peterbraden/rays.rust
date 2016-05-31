use na::{Vec3};
use ray::Ray;

// Axis aligned bounding box
pub struct BBox {
    min: Vec3<f64>,
    max: Vec3<f64>,
}

impl BBox {
    pub fn new(min: Vec3<f64>, max: Vec3<f64>) -> BBox {
        BBox {min: min, max: max}
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

    /*
    pub fn intersectsBBox(&self, bbox: BBox) -> bool{

    }
    */
}

/*
typedef struct BBox{
  vec3 min; // Point closest to origin
  vec3 max; // Opposing point
  BBox(vec3 l, vec3 g) : min(l), max(g) {};
} BBox;
}


bool intersectsBBox(vec3 ro, vec3 invrd, BBox b){
  //http://tavianator.com/fast-branchless-raybounding-box-intersections/

  double tx1 = (b.min.x - ro.x)*invrd.x;
  double tx2 = (b.max.x - ro.x)*invrd.x;

  double tmin = fmin(tx1, tx2);
  double tmax = fmax(tx1, tx2);

  double ty1 = (b.min.y - ro.y)*invrd.y;
  double ty2 = (b.max.y - ro.y)*invrd.y;

  tmin = fmax(tmin, fmin(ty1, ty2));
  tmax = fmin(tmax, fmax(ty1, ty2));

  return tmax >= tmin;
}
vec3 vec3_invert(vec3 rd){
  return (vec3) {1.0/rd.x, 1.0/rd.y, 1.0/rd.z}; 
};

bool intersectsBBox(BBox a, BBox b){
  if (a.max.x < b.min.x) return false; // a is left of b
  if (a.min.x > b.max.x) return false; // a is right of b
  if (a.max.y < b.min.y) return false; // a is above b
  if (a.min.y > b.max.y) return false; // a is below b
  if (a.max.z < b.min.z) return false; // a is behind b
  if (a.min.z > b.max.z) return false; // a is in front of b
  return true; // boxes overlap
};

bool contains(BBox a, vec3 pt){
  if (pt.x < a.min.x || pt.x > a.max.x) return false;
  if (pt.y < a.min.y || pt.y > a.max.y) return false;
  if (pt.z < a.min.z || pt.z > a.max.z) return false;
  return true;
}




*/
