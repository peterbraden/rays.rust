use sceneobject::SceneObject;
use na::{Vec3, Norm, Dot};
use ray::Ray;
use intersection::Intersection;

pub struct Sphere {
    center: Vec3<f64>,
    radius: f64
}

impl Sphere{
    pub fn new(center:Vec3<f64>, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius
        }
    }
}


impl SceneObject for Sphere {
    fn intersects(&self, r: &Ray) -> Option<Intersection> {
        let dst = r.ro - self.center;
        let b = dst.dot(&r.rd.normalize());
        let c = dst.dot(&dst) - self.radius * self.radius;
        let d = b * b - c;

        if (d > 0f64) {
            let dist = -b - d.sqrt();
            let point = r.ro + (r.rd.normalize() * dist);

            return Some(
                Intersection {
                    dist: dist, point: point,  normal: (point - self.center).normalize()
                })
        }

        return None;
        /*
  float b = vec3_mul_inner(dst, vec3_norm(rd)); // dot product.
  float c = vec3_mul_inner(dst, dst) - radius * radius;
  float d = b*b-c;
  float dist;

  if (d > 0){
    dist = -b - sqrt(d);
  } else {
    dist = -1;
  }

  // Project point along ray
  pt = vec3_add(ro, vec3_scale(vec3_norm(rd), dist));
  return (Intersection) {dist, pt, normal(pt), this};
  */
    }
}
