use crate::shapes::infinite::Infinite;
use crate::shapes::sphere::Sphere;
use crate::na::{Vector3};
use crate::sceneobject::SceneObject;
use crate::material::texture::{Solid, Medium};
use crate::color::Color;
use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::scene::Scene;
use crate::geometry::{random_point_on_unit_sphere, rand};
use crate::material::model::{MaterialModel, ScatteredRay};
use crate::material::functions::{scatter_lambertian, scatter_dielectric, diffuse};
use crate::shapes::geometry::Geometry;
use serde_json::{Value, Map};
use crate::scenefile::SceneFile;

pub struct SkyMaterial {
	atmosphere: Sphere,
	earth: Sphere,
	sun_direction: Vector3<f64>, // Normalised
	rayleigh_thickness: f64,
	mie_thickness: f64,
    brightness: f64,
}

impl MaterialModel for SkyMaterial {
    fn scatter(&self, r: &Ray, _intersection: &Intersection, _s: &Scene) -> ScatteredRay{

		let beta_r: Vector3<f64> = Vector3::new(3.8e-6_f64, 13.5e-6_f64, 33.1e-6_f64); 
		let beta_m: Vector3<f64> = Vector3::new(21e-6_f64, 21e-6_f64, 21e-6_f64);
		let atmos_intersection = self.atmosphere.intersects(r);
		if atmos_intersection.is_none(){
			return ScatteredRay { attenuate: Color::black(), ray: None };
		}
		let mut ray_max = atmos_intersection.unwrap().dist; 

		let earth_intersection = self.earth.intersects(r);
		if r.ro.y > self.earth.radius && earth_intersection.is_some(){
			ray_max = earth_intersection.unwrap().dist; 
		}
    	let num_samples = 16; 
    	let num_samples_light = 8; 
    	let segment_length = ray_max / num_samples as f64; 

		let mut rayleigh_sum: Vector3<f64> = Vector3::new(0f64, 0f64, 0f64);
		let mut mie_sum:Vector3<f64> = Vector3::new(0f64, 0f64, 0f64);

		let mut optical_depth_r = 0.;
		let mut optical_depth_m = 0.;

    	let mu = r.rd.dot(&self.sun_direction); // mu in the paper which is the cosine of the angle between the sun direction and the ray direction 
    	let phase_r = 3. / (16. * std::f64::consts::PI) * (1. + mu * mu); 
    	let g = 0.76; 
    	let phase_m = 3. / (8. * std::f64::consts::PI) * ((1. - g * g) * (1. + mu * mu)) / ((2. + g * g) * (1. + g * g - 2. * g * mu).powf(1.5)); 
    	
		for i in 0..num_samples {
        	let sample_position = r.ro + (i as f64 * segment_length) * r.rd; 
        	//let height = sample_position.norm();  // TODO y proportion?
			let height = (sample_position - self.atmosphere.center).norm() - self.earth.radius;
			//println!("1 - {} {}", height, sample_position);
			// compute optical depth for light
        	let rayleigh: f64 = (-height / self.rayleigh_thickness).exp() * segment_length; 
        	let mie:f64 = (-height / self.mie_thickness).exp() * segment_length; 
			//println!("2 - r{} m{} {}", rayleigh, mie, (-height / self.mie_thickness));
			optical_depth_r += rayleigh; 
			optical_depth_m += mie; 

			// light optical depth
			let atmosphere_intersection = self.atmosphere.intersects(&Ray {ro: sample_position, rd:self.sun_direction}); 
			if atmosphere_intersection.is_none(){
				// Does not intersect atmosphere - must be outside ...
				continue;
			}

			let light_len = atmosphere_intersection.unwrap().dist;
			let segment_length_light = light_len / num_samples_light as f64;
			let mut optical_depth_light_r = 0.;
			let mut optical_depth_light_m = 0.; 
			for j in 0 .. num_samples_light {
				let sample_position_light = sample_position + (j as f64 * segment_length_light ) * self.sun_direction;
				let height_light = (sample_position_light - self.atmosphere.center).norm() - self.earth.radius; 
				//if height_light < 0. { break }; 
				optical_depth_light_r = optical_depth_light_r + (-height_light / self.rayleigh_thickness).exp() * segment_length_light; 
				optical_depth_light_m = optical_depth_light_m + (-height_light / self.mie_thickness).exp() * segment_length_light; 
			} 
			//if (j == numSamplesLight) { 
			let tau: Vector3<f64>= beta_r * (optical_depth_r + optical_depth_light_r) + beta_m * 1.1 * (optical_depth_m + optical_depth_light_m); 
			let attenuation: Vector3<f64> = Vector3::new((-tau.x).exp(), (-tau.y).exp(), (-tau.z).exp()); 
			//println!("-\n r{} a{} r{} o{} t{} l{}", rayleigh_sum, attenuation, rayleigh, optical_depth_light_r, tau, (-tau.x as f64).exp(), );
			rayleigh_sum = rayleigh_sum + attenuation * rayleigh; 
			mie_sum = mie_sum + attenuation * mie; 
			//} 
		} 
		let attenuate_vec: Vector3<f64> = (rayleigh_sum.component_mul(&beta_r) * phase_r + mie_sum .component_mul(&beta_m) * phase_m) * self.brightness; 
		
		// Apply tone mapping function
		let attenuate = Color::new(
            if attenuate_vec.x < 1.413f64 { (attenuate_vec.x * 0.38317f64).powf(1.0f64 / 2.2f64) } else { 1.0f64 - (-attenuate_vec.x).exp() }, 
            if attenuate_vec.y < 1.413f64 { (attenuate_vec.y * 0.38317f64).powf(1.0f64 / 2.2f64) } else { 1.0f64 - (-attenuate_vec.y).exp() },
            if attenuate_vec.z < 1.413f64 { (attenuate_vec.z * 0.38317f64).powf(1.0f64 / 2.2f64) } else { 1.0f64 - (-attenuate_vec.z).exp() },
		);
        /*
        let attenuate = Color::new(
            attenuate_vec.x,
            attenuate_vec.y,
            attenuate_vec.z,
        );*/

        return ScatteredRay { attenuate, ray: None }
    }
}


pub fn create_sky_sphere(o: &Value) -> SceneObject {
    let earth_location = SceneFile::parse_vec3_def(&o, "earth_location", Vector3::new(0., -6360e3, 0.));
    return SceneObject {
        geometry: Box::new(Infinite {}),
        medium: Box::new(Solid { m: Box::new(SkyMaterial {
			earth: Sphere::new(
                       earth_location,
                       SceneFile::parse_number(&o["earth_radius"], 6360e3),
                       ),
			atmosphere: Sphere::new(earth_location,
                            SceneFile::parse_number(&o["atmosphere_radius"], 6420e3)),
			rayleigh_thickness: SceneFile::parse_number(&o["rayleigh_thickness"], 7994.),
			mie_thickness: SceneFile::parse_number(&o["mie_thickness"], 1200.),
			sun_direction: SceneFile::parse_vec3_def(&o, "sun_direction", Vector3::new(0., 0.5, 2.)).normalize(),
            brightness: SceneFile::parse_number(&o["brightness"], 20.)
		}) })
    }
}
