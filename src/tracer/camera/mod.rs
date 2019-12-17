use super::Ray;
use vek::vec::{Vec2, Vec3};

pub trait Camera {
    fn generate_ray(&self, screen_coord: Vec2<usize>) -> Ray;
}

#[derive(Copy, Clone)]
pub struct PerspCamera {
    pub corner: Vec3<f32>,
    pub horizontal: Vec3<f32>,
    pub vertical: Vec3<f32>,
    pub origin: Vec3<f32>,
    width: u32,
    height: u32,
}

impl PerspCamera {
    pub fn new(
        origin: Vec3<f32>,
        target: Vec3<f32>,
        fov: f32,
        aspect: f32,
        roll: f32,
        width: u32,
        height: u32,
    ) -> PerspCamera {
        let rotated_up = Vec3::new(-roll.sin(), roll.cos(), 0.0);

        let w = (origin - target).normalized();
        let u = rotated_up.cross(w).normalized();
        let v = w.cross(u).normalized();

        let half_height = (fov / 2.0).tan();
        let half_width = half_height * aspect;

        let corner = origin - (u * half_width) + (v * half_height) - w;
        let horizontal = u * (2.0 * half_width);
        let vertical = v * (2.0 * half_height);

        PerspCamera {
            origin,
            corner,
            horizontal,
            vertical,
            width,
            height,
        }
    }
}

impl Camera for PerspCamera {
    fn generate_ray(&self, screen_coord: Vec2<usize>) -> Ray {
        let screen_coord = Vec2::new(
            screen_coord.x as f32 / self.width as f32,
            screen_coord.y as f32 / self.height as f32,
        );
        let direction =
            self.corner + (self.horizontal * screen_coord.x) + (self.vertical * screen_coord.y)
                - self.origin;

        Ray {
            origin: self.origin,
            direction: direction.normalized(),
        }
    }
}
