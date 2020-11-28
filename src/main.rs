use std::sync::Arc;

use clap::App;

use vek::rgb::Rgb;
use vek::vec::{Vec2, Vec3};

mod gl_shader;
mod gl_texture;
mod tracer;
mod visualizer;

use tracer::camera::MtxCamera;
use tracer::light::{AreaLight, PointLight};
use tracer::material::{Material, BRDF};
use tracer::render_context::RenderContext;
use tracer::shape::Triangle;
use tracer::shape::{Plane, Shape, Sphere};
use tracer::volume::Volume;

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
        h,
    );

    let triangle = Triangle {
        vertices: [
            Vec3::new(-4.0, 0.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 4.0),
        ],
        normals: None,
        tex_coords: None,
    };

    let red = Material {
        emittance: Rgb::zero(),
        brdf: BRDF::Lambertian(0.9),
        albedo: Rgb::new(1.0, 0.0, 0.0),
    };

    let green = Material {
        emittance: Rgb::zero(),
        brdf: BRDF::Lambertian(0.6),
        albedo: Rgb::new(0.0, 1.0, 0.0),
    };

    let blue = Material {
        emittance: Rgb::zero(),
        brdf: BRDF::Lambertian(0.8),
        albedo: Rgb::new(0.0, 0.0, 1.0),
    };

    let purple = Material {
        emittance: Rgb::new(1.0, 1.0, 1.0),
        brdf: BRDF::BlackBody,
        albedo: Rgb::new(0.0, 0.0, 0.0),
    };

    let white = Material {
        emittance: Rgb::new(0.0, 0.0, 0.0),
        brdf: BRDF::Lambertian(1.0),
        albedo: Rgb::new(0.4, 0.0, 0.4),
    };

    // Volume
    //let k_b = 1.38e-23;
    //let k_b = 1.38e-10;
    let mist_radius = 1.0f32;
    let mist_volume = (4.0/3.0) * std::f32::consts::PI * mist_radius.powi(2);
    let mist_pressure = 101325.0;
    let mist_temperature = 273.0;
    //let mist_density = mist_pressure / (k_b * mist_temperature);
    let mist_density = 10.0;
    let mist_carrier = Sphere::new(Vec3::zero(), 1.0, white);

    let ctx = RenderContext {
        width: w,
        height: h,
        samples: samples,
        n_threads: n_threads,
        objects: Arc::new(vec![
            Arc::new(Sphere::new(
                Vec3::new(2.0, 2.0, 0.0),
                1.0,
                blue,
            )),
            Arc::new(Sphere::new(
                Vec3::new(-2.0, 2.0, 0.0),
                1.0,
                blue,
            )),
            Arc::new(Sphere::new(
                Vec3::new(0.0, 2.0, 2.0),
                1.0,
                blue,
            )),
            Arc::new(Plane::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                white,
            )),
            Arc::new(Volume::new(mist_carrier, mist_density)),
        ]),
        lights: Arc::new(vec![
            Arc::new(AreaLight {
                position: Vec3::new(-5.0, 5.0, -5.0),
                radius: 5.0,
            }),
            //Arc::new(PointLight {
                //position: Vec3::new(-5.0, 5.0, -5.0),
            //}),
        ]),
        ambient: Rgb::new(0.1, 0.1, 0.1),
        camera,
    };

    tracer::render(tx, ctx);

    let _ = visualizer::Visualizer::new(rx, w, h);
}
