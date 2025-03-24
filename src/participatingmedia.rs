use crate::color::Color;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::sceneobject::SceneObject;
use crate::material::model::{ MaterialModel, ScatteredRay };
use serde_json::{Value, Map};
use crate::shapes::bbox::BBox;
use crate::intersection::{Intersection, RawIntersection};
extern crate rand as _rand;
use crate::participatingmedia::_rand::Rng;
use std::f64;
use crate::na::{Vector3};
use crate::shapes::geometry::Geometry;
use crate::material::texture::{Solid, Medium};
use crate::geometry::random_point_on_unit_sphere;
use crate::scenefile::SceneFile;
use crate::noise::{PerlinNoise, WorleyNoise, cloud_noise};

const BIG_NUMBER:f64 = 1000.;

pub fn rand() -> f64 {
    return _rand::thread_rng().gen_range(0.,1.);
}

pub trait ParticipatingMedium: MaterialModel {}

pub struct Vacuum {}
impl ParticipatingMedium for Vacuum {}
impl MaterialModel for Vacuum {
    fn scatter(&self, _r: &Ray, _i: &Intersection, _s: &Scene) -> ScatteredRay {
        return ScatteredRay {
            ray: None,
            attenuate: Color::white(),
        }
    }
}

#[derive(Clone)]
pub struct HomogenousFog {
    pub color: Color,
    pub density: f64,
    pub scatter: f64,
}
impl ParticipatingMedium for HomogenousFog{}
impl MaterialModel for HomogenousFog {
    fn scatter(&self, r: &Ray, i: &Intersection, _s: &Scene) -> ScatteredRay {
       // let amount = i.dist * self.density;
        return ScatteredRay {
            ray: Some(Ray {
                ro: i.point,
                rd: (r.rd + (random_point_on_unit_sphere() * self.scatter * rand())).normalize(),
            }),
            attenuate: self.color,
        }
    }
}

impl Geometry for HomogenousFog {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        if rand() < self.density {
            let dist = rand().powf(3.) * BIG_NUMBER; 
            return Some(RawIntersection {
                dist: dist,
                point: r.ro + r.rd * dist,
                normal: r.rd,
            })
        }
        return None
    }

    fn bounds(&self) -> BBox {
        BBox::new(
            Vector3::new(std::f64::MIN, std::f64::MIN, std::f64::MIN),
            Vector3::new(std::f64::MAX, std::f64::MAX, std::f64::MAX),
          )
    }
}

pub struct LowAltitudeFog {
    density: f64,
    color: Color,
    max_altitude: f64,
    falloff: f64,
}
impl MaterialModel for LowAltitudeFog {
    fn scatter(&self, _r: &Ray, _i: &Intersection, _s: &Scene) -> ScatteredRay {
        //let amount = i.dist * self.density;
        // TODO
        return ScatteredRay {
            ray: None,
            attenuate: Color::white(),
        }
    }

}


#[derive(Clone)]
pub struct CloudLayer {
    /// Base color of clouds
    pub color: Color,
    /// Maximum density factor
    pub max_density: f64,
    /// Forward scattering coefficient (higher = more directional)
    pub anisotropy: f64,
    /// Cloud base height (bottom of layer)
    pub base_height: f64,
    /// Cloud layer thickness
    pub thickness: f64,
    /// Horizontal extent of cloud layer
    pub extent: f64,
    /// Noise scale factor for cloud shapes
    pub noise_scale: f64,
    /// Height-based density falloff
    pub height_falloff: f64,
    /// Perlin noise generator
    perlin: PerlinNoise,
    /// Worley noise generator
    worley: WorleyNoise,
}

impl ParticipatingMedium for CloudLayer {}

impl MaterialModel for CloudLayer {
    fn scatter(&self, r: &Ray, i: &Intersection, _s: &Scene) -> ScatteredRay {
        // Implementation of light scattering in clouds
        // Using Henyey-Greenstein phase function for anisotropic scattering
        
        // Apply phase function to determine scattering direction
        let cos_theta = r.rd.dot(&i.normal);
        let g = self.anisotropy;
        let g2 = g * g;
        
        // Probability of scattering in a particular direction
        let _phase_factor = if g.abs() < 0.001 {
            // If g is close to 0, use isotropic scattering
            1.0 / (4.0 * std::f64::consts::PI)
        } else {
            // Henyey-Greenstein phase function
            (1.0 - g2) / (4.0 * std::f64::consts::PI * (1.0 + g2 - 2.0 * g * cos_theta).powf(1.5))
        };
        
        // Compute scattering direction using phase function
        // For now we'll use a simplified approach with some randomness
        let scatter_dir = if rand() < 0.5 + self.anisotropy * 0.5 {
            // Forward scattering tendency, weighted by anisotropy
            (r.rd + random_point_on_unit_sphere() * (1.0 - self.anisotropy)).normalize()
        } else {
            // Random scattering direction
            random_point_on_unit_sphere()
        };
        
        // Calculate attenuation based on cloud density at intersection point
        let density_at_point = self.density_at(&i.point);
        
        // Higher density means more scattering and less transmission
        let attenuation = self.color * density_at_point;
        
        ScatteredRay {
            ray: Some(Ray {
                ro: i.point,
                rd: scatter_dir,
            }),
            attenuate: attenuation,
        }
    }
}

impl Geometry for CloudLayer {
    fn intersects(&self, r: &Ray) -> Option<RawIntersection> {
        // First check if ray intersects the cloud layer's bounding box
        let bounds = self.bounds();
        if !bounds.intersects(r).is_some() {
            return None;
        }
        
        // Now perform ray marching through the volume
        let step_size = 10.0; // Initial step size
        let max_steps = 100;  // Maximum ray marching steps
        let density_threshold = 0.05; // Minimum density to consider a hit
        
        let mut current_pos = r.ro;
        let mut t = 0.0;
        
        // Raymond marching loop
        for _ in 0..max_steps {
            // Check if we're outside the bounds
            if current_pos.x < bounds.min.x || current_pos.x > bounds.max.x ||
               current_pos.y < bounds.min.y || current_pos.y > bounds.max.y ||
               current_pos.z < bounds.min.z || current_pos.z > bounds.max.z {
                break;
            }
            
            // Get cloud density at current position
            let density = self.density_at(&current_pos);
            
            // If density is above threshold, consider it a hit
            if density > density_threshold {
                // Adjust hit probability based on density and step size
                let hit_probability = 1.0 - (-density * self.max_density * step_size).exp();
                
                if rand() < hit_probability {
                    return Some(RawIntersection {
                        dist: t,
                        point: current_pos,
                        normal: r.rd, // Using ray direction as normal for now
                    });
                }
            }
            
            // Move along the ray
            t += step_size * (1.0 - density).max(0.2); // Adaptive step size
            current_pos = r.ro + r.rd * t;
        }
        
        None
    }

    fn bounds(&self) -> BBox {
        // Cloud layer is a horizontal slab with defined height range and extent
        BBox::new(
            Vector3::new(-self.extent, self.base_height, -self.extent),
            Vector3::new(self.extent, self.base_height + self.thickness, self.extent),
        )
    }
}

impl CloudLayer {
    /// Calculate the cloud density at a given position
    fn density_at(&self, position: &Vector3<f64>) -> f64 {
        // Check if position is within cloud layer bounds
        let height = position.y;
        if height < self.base_height || height > self.base_height + self.thickness {
            return 0.0;
        }
        
        // Calculate normalized height within cloud layer (0.0 = base, 1.0 = top)
        let normalized_height = (height - self.base_height) / self.thickness;
        
        // Get base density from noise functions
        let density = cloud_noise::cloud_density(
            *position, 
            &self.perlin, 
            &self.worley, 
            self.noise_scale,
            self.height_falloff
        );
        
        // Apply vertical profile - more dense in the middle, less at the edges
        let vertical_profile = 4.0 * normalized_height * (1.0 - normalized_height);
        
        // Combine base density with vertical profile
        (density * vertical_profile * self.max_density).min(1.0)
    }
}

pub fn create_fog(o: &Value) -> SceneObject {
    let fog = HomogenousFog {
        color: SceneFile::parse_color_def(&o, "color", Color::new(0.1, 0.1, 0.1)),
        density: SceneFile::parse_number(&o["density"], 0.2),
        scatter: SceneFile::parse_number(&o["scatter"], 0.01),
    };
	return SceneObject {
		geometry: Box::new(fog.clone()),
		medium: Box::new(Solid { m: Box::new(fog)}),
	}
}

/// Create a cloud layer from the provided configuration
pub fn create_cloud_layer(o: &Value) -> SceneObject {
    // Parse cloud parameters from JSON config
    let cloud = CloudLayer {
        color: SceneFile::parse_color_def(&o, "color", Color::white()),
        max_density: SceneFile::parse_number(&o["density"], 0.6),
        anisotropy: SceneFile::parse_number(&o["anisotropy"], 0.2),
        base_height: SceneFile::parse_number(&o["base_height"], 800.0),
        thickness: SceneFile::parse_number(&o["thickness"], 400.0),
        extent: SceneFile::parse_number(&o["extent"], 10000.0),
        noise_scale: SceneFile::parse_number(&o["noise_scale"], 0.001),
        height_falloff: SceneFile::parse_number(&o["height_falloff"], 0.1),
        perlin: PerlinNoise::new(),
        worley: WorleyNoise::new(
            SceneFile::parse_number(&o["worley_density"], 1.0),
            SceneFile::parse_number(&o["seed"], 42.0) as u32
        ),
    };
    
    SceneObject {
        geometry: Box::new(cloud.clone()),
        medium: Box::new(Solid { m: Box::new(cloud) }),
    }
}
