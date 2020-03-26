use std::sync::Arc;

use clap::App;

use vek::rgb::Rgb;
use vek::vec::{Vec2, Vec3};

mod gl_shader;
mod gl_texture;
mod tracer;
mod visualizer;

use tracer::camera::MtxCamera;
use tracer::material::Material;
use tracer::render_context::RenderContext;
use tracer::shape::Shape;
use tracer::shape::Triangle;
use tracer::light::Light;

fn main() {
    let yaml = clap::load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let w: usize = matches.value_of("width").unwrap_or("640").parse().unwrap();
    let h: usize = matches.value_of("height").unwrap_or("320").parse().unwrap();

    let samples: u16 = matches.value_of("samples").unwrap_or("1").parse().unwrap();

    let n_threads: u16 = matches.value_of("threads").unwrap_or("1").parse().unwrap();

    println!("Running on {} threads, with {} samples", n_threads, samples);

    let (tx, rx) = std::sync::mpsc::channel();

    //let camera = PerspCamera::new(
        //Vec3::new(0.0, 10.0, -5.0), // position of the camera
        //Vec3::new(0.0, 10.0, 1.0), // position of the target
        //std::f32::consts::FRAC_PI_4, // field of view in radians
        //(w as f32) / (h as f32), // aspect ratio (width/height)
        //(0f32).to_radians(), // camera roll
        //w as u32, // width
        //h as u32, // height
    //);

    let camera = MtxCamera::new(
        Vec3::new(10.0, 10.0, 10.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        std::f32::consts::FRAC_PI_3,
        w,
        h
    );

    let triangle = Triangle {
        vertices: [Vec3::new(-4.0, 0.0, 0.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 4.0)],
        normals: None,
        tex_coords: None,
    };

    let ctx = RenderContext {
        width: w,
        height: h,
        samples: samples,
        n_threads: n_threads,
        objects: Arc::new(vec![
            Shape::Sphere(
                Material::Glossy,
                Vec3::new(0.0, 3.0, 0.0),
                3.0
            ),
            Shape::Sphere(
                Material::Lambertian(Rgb::new(0.8, 0.0, 0.8), 0.2),
                Vec3::new(3.0, 1.0, 0.0),
                1.0
            ),
            Shape::Plane(
                Material::Lambertian(Rgb::new(1.0, 1.0, 1.0), 0.6),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0)
            ),
        ]),
        lights: Arc::new(vec![
            Light {
                intensity: 10.0,
                position: Vec3::new(0.0, 5.0, 5.0),
            }
        ]),
        ambient: Rgb::new(0.1, 0.1, 0.1),
        camera,
    };

    tracer::render(tx, ctx);

    let _ = visualizer::Visualizer::new(rx, w, h);
}
