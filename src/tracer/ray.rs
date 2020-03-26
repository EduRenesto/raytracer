use std::sync::Arc;

use vek::vec::{Vec2, Vec3};

use super::shape::Shape;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3<f32>,
    pub direction: Vec3<f32>,
}

#[derive(Clone)]
pub struct RayHit<'a> {
    pub ray: &'a Ray,
    pub distance: f32,
    pub object: Arc<dyn Shape>,
}
