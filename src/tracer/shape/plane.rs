use vek::vec::Vec3;

use crate::tracer::material::Material;
use crate::tracer::ray::{Ray, RayHit};

use super::Shape;

#[derive(Copy, Clone)]
pub struct Plane {
    pub point: Vec3<f32>,
    pub normal: Vec3<f32>,
    pub material: Material,
}

impl Plane {
    pub fn new(point: Vec3<f32>, normal: Vec3<f32>, material: Material) -> Plane {
        Plane {
            point,
            normal,
            material,
        }
    }
}

impl Shape for Plane {
    fn intersects<'a>(&self, ray: &'a Ray) -> Option<RayHit<'a>> {
        let nr = self.normal.dot(ray.direction);
        let u = ray.origin - self.point;
        let t = -((u.dot(self.normal)) / nr);

        if nr == 0f32 || t <= 0f32 {
            None
        } else {
            Some(RayHit {
                ray: &ray,
                distance: t,
                point: ray.origin + t * ray.direction,
            })
        }
    }

    fn normal_at(&self, point: Vec3<f32>) -> Vec3<f32> {
        self.normal
    }

    fn material(&self) -> Material {
        self.material
    }

    fn position(&self) -> Vec3<f32> {
        self.point
    }

    fn volume(&self) -> f32 {
        0.0
    }
}
