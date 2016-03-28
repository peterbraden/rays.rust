use color::Color;
use ray::Ray;
use scene::Scene;
use std::f64;

pub fn trace (r: Ray, depth: i32, s: &Scene) -> Color {
    
    let closest = s.objects.nearestIntersection(r, f64::INFINITY, 0f64);
    Color::new(0,0,0)
}
