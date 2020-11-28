use crate::tracer::Shape;
use crate::tracer::material::{ Material, BRDF };
use crate::tracer::ray::{Ray, RayHit};

use rand_distr::{ Poisson, UnitSphere, Distribution };
use vek::vec::{ Vec3, Rgb };

pub struct Volume<S: Shape> {
    carrier: S,
    density: f32,
    dist: Poisson<f32>,
}

impl<S: Shape> Volume<S> {
    pub fn new(carrier: S, density: f32) -> Volume<S> {
        let dist = Poisson::new(density).unwrap();
        Volume {
            carrier,
            density,
            dist,
        }
    }
}

impl<S: Shape> Shape for Volume<S> {
    fn intersects<'a>(&self, ray: &'a Ray) -> Option<RayHit<'a>> {
        let mut carrier_hit = self.carrier.intersects(ray)?;

        let op = self.carrier.position() - carrier_hit.point;

        let ray_volume = std::f32::consts::TAU * 0.0001 * (2.0 * op.magnitude());
        let n_particles = self.dist.sample(&mut rand::thread_rng());

        // wat
        let k = ( n_particles * ray_volume ) / self.density;

        carrier_hit.point += 2.0 * k * op;

        Some(carrier_hit)
    }

    fn normal_at(&self, _point: Vec3<f32>) -> Vec3<f32> {
        let mut r = rand::thread_rng();
        let nrml = UnitSphere.sample(&mut r);
        Vec3::from(nrml)
    }

    fn material(&self) -> Material {
        Material {
            albedo: Rgb::new(0.0, 0.0, 0.0),
            emittance: Rgb::new(0.0, 0.0, 0.0),
            brdf: BRDF::Glossy,
        }
    }

    fn position(&self) -> Vec3<f32> {
        self.carrier.position()
    }

    fn volume(&self) -> f32 {
        self.carrier.volume()
    }
}
