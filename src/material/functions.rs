use na::{Vector3};

pub fn refract(v: Vector3<f64>, n: Vector3<f64>, ni_over_nt:f64) -> Option<Vector3<f64>> {
    let uv = v.normalize();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt*dt);
    if discriminant > 0.0 {
        return Some( (uv - n * dt) * ni_over_nt - n * discriminant.sqrt())
    }
    None
}
