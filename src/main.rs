use std::sync::Arc;

use clap::App;

use vek::rgb::Rgb;
use vek::vec::{Vec2, Vec3};

mod gl_shader;
mod gl_texture;
mod tracer;
mod visualizer;

use tracer::camera::PerspCamera;
use tracer::material::Material;
use tracer::render_context::RenderContext;
use tracer::shape::Shape;

fn main() {
    let yaml = clap::load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let w: usize = matches.value_of("width").unwrap_or("640").parse().unwrap();
    let h: usize = matches.value_of("height").unwrap_or("320").parse().unwrap();

    let samples: u16 = matches.value_of("samples").unwrap_or("1").parse().unwrap();

    let n_threads: u16 = matches.value_of("threads").unwrap_or("1").parse().unwrap();

    println!("Running on {} threads, with {} samples", n_threads, samples);

    let (tx, rx) = std::sync::mpsc::channel();

    let camera = PerspCamera::new(
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(-10.0, -10.0, 10.0),
        std::f32::consts::FRAC_PI_4,
        (w as f32) / (h as f32),
        (0f32).to_radians(),
        w as u32,
        h as u32,
    );

    let ctx = RenderContext {
        width: w,
        height: h,
        samples: samples,
        n_threads: n_threads,
        objects: Arc::new(vec![
            Shape::Sphere(
                Material::Lambertian(Rgb::new(0.0, 0.0, 0.6)),
                Vec3::new(-2.0, 1.0, 1.0),
                0.5,
            ),
            Shape::Sphere(
                Material::Lambertian(Rgb::new(0.0, 0.5, 0.0)),
                Vec3::new(-2.5, 1.0, 3.5),
                0.5,
            ),
            Shape::Plane(
                Material::Lambertian(Rgb::new(0.3, 0.0, 0.0)),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            //Shape::Plane(
            ////Material::Glossy,
            //Material::Lambertian(Rgb::new(0.6, 0.0, 0.0)),
            //Vec3::new(-1.0, 0.0, 0.0),
            //Vec3::new(1.0, 0.0, 0.0).normalized(),
            //),
            //Shape::Plane(
            //Material::Glossy,
            //Vec3::new(1.0, 0.0, 0.0),
            //Vec3::new(-1.0, 0.0, 0.0).normalized(),
            //),
        ]),
        camera,
    };

    tracer::render(&tx, ctx);

    let _ = visualizer::Visualizer::new(rx, w, h);
}
