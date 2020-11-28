use vek::vec::Vec3;
use vek::rgb::Rgb;

use crate::tracer::ray::Ray;

pub struct LightSample {
    pub distance: f32,
    pub ray: Ray,
}

pub trait LightSampler: Send + Sync {
    fn sample(&self, rng: &mut rand::rngs::ThreadRng, point: Vec3<f32>) -> LightSample;
}

pub struct PointLight {
    pub position: vek::vec::Vec3<f32>,
}

impl LightSampler for PointLight {
    fn sample(&self, _: &mut rand::rngs::ThreadRng, point: Vec3<f32>) -> LightSample {
        let dir = self.position - point;
        let distance = dir.magnitude();
        let dir = dir.normalized();
        let ray = Ray {
            origin: point,
            direction: dir.normalized()
        };
        LightSample {
            distance,
            ray,
        }
    }
}

pub struct AreaLight {
    pub position: vek::vec::Vec3<f32>,
    pub radius: f32,
}

impl LightSampler for AreaLight {
    fn sample(&self, rng: &mut rand::rngs::ThreadRng, point: Vec3<f32>) -> LightSample {
        let light_point = (crate::tracer::random_in_hemisphere(rng, Vec3::zero(), Vec3::zero()) * self.radius)
            + self.position;
        let dir = light_point - point;
        let distance = dir.magnitude();
        let dir = dir.normalized();
        let ray = Ray {
            origin: point,
            direction: dir.normalized()
        };
        LightSample {
            distance,
            ray,
        }
    }
}
