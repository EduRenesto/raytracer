use super::Ray;
use vek::vec::{Vec2, Vec3, Vec4};
use vek::mat::{Mat4};

pub trait Camera {
    fn generate_ray(&self, screen_coord: Vec2<usize>) -> Ray;
}

#[derive(Copy, Clone)]
pub struct MtxCamera {
    pub vp_inverse: Mat4<f32>,

    pub width: usize,
    pub height: usize,
}

impl MtxCamera {
    pub fn new(
        origin: Vec3<f32>,
        target: Vec3<f32>,
        up: Vec3<f32>,
        fovy: f32,
        width: usize,
        height: usize
    ) -> MtxCamera {
        let aspect = (width as f32) / (height as f32);
        let view = Mat4::look_at_rh(origin, target, up);
        let proj = Mat4::perspective_rh_zo(fovy, aspect, 0.01, 100.0);

        let vp = proj * view;

        let cam = MtxCamera {
            vp_inverse: vp.inverted(),
            width,
            height,
        };

        println!("Top Left: {:?}", cam.generate_ray(Vec2::new(0, 0)));
        println!("Top Right: {:?}", cam.generate_ray(Vec2::new(width, 0)));
        println!("Bot Left: {:?}", cam.generate_ray(Vec2::new(0, height)));
        println!("Bot Right: {:?}", cam.generate_ray(Vec2::new(width, height)));

        cam
    }
}

impl Camera for MtxCamera {
    fn generate_ray(&self, coord: Vec2<usize>) -> Ray {
        let origin_eye = Vec4::new(0.0, 0.0, 0.0, 1.0);

        let u = (coord.x as f32 / self.width as f32) * 2.0 - 1.0;
        let v = (coord.y as f32 / self.height as f32) * 2.0 - 1.0;

        let target_eye = Vec4::new(u, v, 1.0, 1.0);
        
        let origin = self.vp_inverse * origin_eye;
        let origin = origin.map(|e| e / origin.w);
        let target = self.vp_inverse * target_eye;
        let target = target.map(|e| e / target.w);

        let direction = (target - origin).normalized();

        Ray {
            origin: origin.into(),
            direction: direction.into()
        }
    }
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
        println!("Rotated_up: {:?}", rotated_up);

        let w = (target - origin).normalized();
        let u = rotated_up.cross(w).normalized(); // aponta para esquerda // agora para a direita
        let v = w.cross(u).normalized(); // aponta para cima
        println!("u: {:?},\n v: {:?},\n w: {:?}", u, v, w);

        let half_height = (fov / 2.0).tan();
        let half_width = half_height * aspect;

        //let corner = origin - (u * half_width) + (v * half_height) - w;
        let corner = origin - (u * half_width) + (v * half_height);
        let horizontal = u * (2.0 * half_width);
        let vertical = v * (2.0 * half_height);

        println!("corner: {:?},\n horizontal: {:?},\n vertical: {:?}", corner, horizontal, vertical);

        let cam = PerspCamera {
            origin,
            corner,
            horizontal,
            vertical,
            width,
            height,
        };

        println!("Top Left: {:?}", cam.generate_ray(Vec2::new(0, 0)));
        println!("Top Right: {:?}", cam.generate_ray(Vec2::new(width as usize, 0)));
        println!("Bot Left: {:?}", cam.generate_ray(Vec2::new(0, height as usize)));
        println!("Bot Right: {:?}", cam.generate_ray(Vec2::new(width as usize, height as usize)));

        cam
    }
}

impl Camera for PerspCamera {
    fn generate_ray(&self, screen_coord: Vec2<usize>) -> Ray {
        let screen_coord = Vec2::new(
            screen_coord.x as f32 / self.width as f32,
            screen_coord.y as f32 / self.height as f32,
        );
        //let direction =
            //self.corner + (self.horizontal * screen_coord.x) + (self.vertical * screen_coord.y)
                //- self.origin;

        let direction =
            self.corner + (self.horizontal * screen_coord.x) - (self.vertical * screen_coord.y)
            - self.vertical.cross(self.horizontal).normalized();

        Ray {
            origin: self.origin,
            direction: (direction - self.origin).normalized(),
        }
    }
}
