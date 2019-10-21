use vek::vec::{Vec3, Vec2};
use vek::mat::Mat4;

use super::ray::Ray;

pub enum Camera {
    Hardcoded(usize, usize), // Just for testing
    /// Width, Height, Position, Target, Fovy
    Perspective(usize, usize, Vec3<f32>, Vec3<f32>, f32)
}

impl Camera {
    pub fn generate_ray(&self, screen_coord: Vec2<usize>) -> Ray {
        match self {
            &Camera::Hardcoded(w, h) => {
                let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
                let horizontal = Vec3::new(4.0, 0.0, 0.0);
                let vertical = Vec3::new(0.0, 2.0, 0.0);
                let origin = Vec3::new(0.0, 0.0, 0.0);

                let i = screen_coord.x as f32;
                let j = screen_coord.y as f32;

                let u = i/w as f32;
                let v = j/h as f32;

                Ray {
                    origin,
                    direction: lower_left_corner + (u*horizontal) + (v*vertical)
                }
            }, 
            &Camera::Perspective(w, h, pos, target, fovy) => {
                let ndc = Vec2::new(screen_coord.x as f32 + 0.5f32, screen_coord.y as f32 + 0.5f32)
                            / Vec2::new(w as f32, h as f32);

                let screen = Vec2::new(
                    2f32 * ndc.x - 1f32,
                    1f32 - 2f32 * ndc.y
                );

                let aspect = w as f32 / h as f32;

                let fov_over_two = fovy / 2f32;

                //let pixel_camera = Vec2::new(
                    //(2f32 * screen.x - 1f32) * aspect * fov_over_two.tan(),
                    //(1f32 - 2f32 * screen.y) * fov_over_two.tan()
                //);

                //let pos_camera_space = Vec3::new(pixel_camera.x, pixel_camera.y, -1f32);

                //let direction = (pos_camera_space - pos).normalized();

//float Px = (2 * ((x + 0.5) / imageWidth) - 1) * tan(fov / 2 * M_PI / 180) * imageAspectRatio; 
//float Py = (1 - 2 * ((y + 0.5) / imageHeight) * tan(fov / 2 * M_PI / 180); 

                let x = screen_coord.x as f32;
                let y = screen_coord.y as f32;

                let Px = (2.0 * ((x + 0.5) / w as f32) - 1.0) * fov_over_two.tan() * aspect;
                let Py = (1.0 - 2.0 * ((y + 0.5) / h as f32)) * fov_over_two.tan();

                let cam_to_world: Mat4<f32> = Mat4::look_at_lh(pos, target, Vec3::new(0.0, -1.0, 0.0));

                let origin = cam_to_world.mul_point(Vec3::zero());
                let ray_p_world = cam_to_world.mul_point(Vec3::new(Px, Py, -1.0));

                let direction = ray_p_world - origin;

                Ray {
                    origin,
                    direction
                }
            }
        }
    }
}
