use vek::rgb::Rgb;
use vek::vec::Vec3;

use super::ray::Ray;

/// The different implementations for the
/// bidirectional reflectance distribution function.
#[derive(Copy, Clone)]
pub enum BRDF {
    /// A simple lambertian BRDF parametrized by reflectivity (rho)
    Lambertian(f32),
    Glossy,
    BlackBody,
}

fn random_in_hemisphere(
    rng: &mut rand::rngs::ThreadRng,
    center: Vec3<f32>,
    normal: Vec3<f32>,
) -> Vec3<f32> {
    use rand::Rng;

    // Thanks, FermatsLibrary!
    let u = rng.gen_range(0f32, 1f32);
    let v = rng.gen_range(0f32, 1f32);

    let theta = 2f32 * std::f32::consts::PI * u;
    let phi = (2f32 * v - 1f32).acos();

    let x = theta.sin() * phi.cos();
    let y = theta.sin() * phi.sin();
    let z = theta.cos();

    center + normal + Vec3::new(x, y, z)
}

impl BRDF {
    /// Returns the amount of light reflected at the given directions.
    pub fn at(&self, incoming: Vec3<f32>, outgoing: Vec3<f32>, normal: Vec3<f32>) -> f32 {
        match *self {
            BRDF::Lambertian(rho) => rho / std::f32::consts::PI,
            BRDF::Glossy => 1f32,
            BRDF::BlackBody => 0f32,
        }
    }

    /// Reflects the incoming ray according to the BRDF
    pub fn reflect(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        incoming: Vec3<f32>,
        point: Vec3<f32>,
        normal: Vec3<f32>,
    ) -> Option<Ray> {
        match *self {
            BRDF::Lambertian(_) => {
                let outgoing = (random_in_hemisphere(rng, point, normal) - point).normalized();
                Some(Ray {
                    origin: point,
                    direction: outgoing,
                })
            }
            BRDF::Glossy => Some(Ray {
                origin: point,
                direction: incoming.reflected(-normal),
            }),
            BRDF::BlackBody => None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Material {
    /// The intrinsinc color of the material.
    pub albedo: Rgb<f32>,

    /// The emitted color.
    pub emittance: Rgb<f32>,

    /// The BRDF.
    pub brdf: BRDF,
}
