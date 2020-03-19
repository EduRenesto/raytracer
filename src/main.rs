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
use tracer::shape::Triangle;

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
        Vec3::new(0.0, 10.0, -5.0), // position of the camera
        Vec3::new(0.0, 10.0, 1.0), // position of the target
        std::f32::consts::FRAC_PI_4, // field of view in radians
        (w as f32) / (h as f32), // aspect ratio (width/height)
        (0f32).to_radians(), // camera roll
        w as u32, // width
        h as u32, // height
    );

    let ctx = RenderContext {
        width: w,
        height: h,
        samples: samples,
        n_threads: n_threads,
        objects: Arc::new(vec![
            Shape::Sphere(
                Material::Lambertian(Rgb::new(0.0, 0.0, 0.6)),
                Vec3::new(1.0, 10.0, 2.0),
                0.8,
            ),
            Shape::Sphere(
                Material::Lambertian(Rgb::new(0.0, 0.5, 0.0)),
                Vec3::new(-1.0, 10.0, 2.0),
                0.5,
            ),
            Shape::Sphere(
                Material::Glossy,
                Vec3::new(0.0, 11.0, 2.0),
                0.3,
            ),
            //Shape::Plane(
            ////Material::Glossy,
            //Material::Lambertian(Rgb::new(0.6, 0.0, 0.0)),
            //Vec3::new(-1.0, 0.0, 0.0),
            //Vec3::new(1.0, 0.0, 0.0).normalized(),
            //),
            //Shape::Plane(
            //    //Material::Lambertian(Rgb::new(0.2, 0.0, 0.2)),
            //    Material::Glossy,
            //    Vec3::new(0.0, 8.0, 0.0),
            //    Vec3::new(0.0, 1.0, 0.0).normalized(),
            //),
            Shape::Poly(
                Material::Lambertian(Rgb::new(0.0, 0.0, 0.0)),
                Triangle {
                    vertices: [Vec3::new(-10.0, 8.0, -10.0),
                               Vec3::new(10.0, 8.0, -10.0),
                               Vec3::new(-10.0, 8.0, 10.0)],
                    normals: None,
                    tex_coords: None,
                }
            )
        ]),
        camera,
    };

    tracer::render(tx, ctx);

    let _ = visualizer::Visualizer::new(rx, w, h);
}
