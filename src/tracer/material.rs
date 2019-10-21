use vek::rgb::Rgb;

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian(Rgb<f32>),
    Glossy
}

impl Material {
}
