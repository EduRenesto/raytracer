use vek::vec::Vec3;

use crate::tracer::material::Material;
use crate::tracer::ray::{Ray, RayHit};

use super::Shape;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3<f32>,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3<f32>, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Shape for Sphere {
    fn intersects<'a>(&self, ray: &'a Ray) -> Option<RayHit<'a>> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let roots = super::solve(a, b, c);

        use super::QuadraticResult;

        match roots {
            QuadraticResult::OneReal(x) | QuadraticResult::TwoReal(_, x) => {
                if x < 0f32 {
                    None
                } else {
                    Some(RayHit {
                        ray: &ray,
                        distance: x,
                        point: ray.origin + x * ray.direction,
                    })
                }
            }
            _ => None,
        }
    }

    fn normal_at(&self, point: Vec3<f32>) -> Vec3<f32> {
        (point - self.center).normalized()
    }

    fn material(&self) -> Material {
        self.material
    }

    fn position(&self) -> Vec3<f32> {
        self.center
    }

    fn volume(&self) -> f32 {
        (4.0/3.0) * std::f32::consts::PI * self.radius.powi(3)
    }
}
