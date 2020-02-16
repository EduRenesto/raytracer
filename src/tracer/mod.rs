pub mod camera;
pub mod material;
pub mod ray;
pub mod render_context;
pub mod shape;

use rand::Rng;
use vek::rgb::Rgb;
use vek::vec::{Vec2, Vec3};
use rayon::prelude::*;

use camera::Camera;
use material::Material;
use ray::{Ray, RayHit};
use render_context::RenderContext;
use shape::Shape;

use std::sync::Arc;

static MAX_DEPTH: u16 = 20;

type Sender = std::sync::mpsc::Sender<(Vec2<usize>, Rgb<f32>)>;

fn spawn_thread<'a, C>(sender: &Sender, ctx: Arc<RenderContext<C>>, idx: u16)
where
    C: Camera + Send + Sync + 'static,
{
    let tx = sender.clone();

    let idx = idx as usize;

    let dx = 2 * ctx.width / ctx.n_threads as usize;
    let dy = 2 * ctx.height / ctx.n_threads as usize;

    let threads_per_row = ctx.n_threads as usize / 2;

    // FIXME!
    let offset = Vec2::new(idx % threads_per_row, idx / threads_per_row);
    let offset = offset * Vec2::new(dx, dy);

    std::thread::spawn(move || {
        let mut rng = rand::thread_rng();
        for j in 0..dy {
            for i in 0..dx {
                let coord = Vec2::new(i, j);
                let coord = coord + offset;

                let mut acc = Rgb::<f32>::zero();

                for _ in 0..ctx.samples {
                    let ray = ctx.camera.generate_ray(coord);
                    //println!("{:?}", ray);
                    acc += trace(ctx.clone(), &mut rng, ray, 0);
                }

                let color = acc / ctx.samples as f32;

                tx.send((coord, color)).unwrap();
            }
        }
    });
}

fn trace<C: Camera>(
    ctx: Arc<RenderContext<C>>,
    rng: &mut rand::ThreadRng,
    ray: Ray,
    depth: u16,
) -> Rgb<f32> {
    if depth > MAX_DEPTH {
        Rgb::zero()
    } else {
        let mut nearest_hit: Option<RayHit> = None;

        for obj in ctx.objects.iter() {
            if let Some(hit) = obj.intersects(&ray) {
                if let Some(nearest) = nearest_hit {
                    if hit.distance < nearest.distance {
                        nearest_hit = Some(hit);
                    }
                } else {
                    nearest_hit = Some(hit);
                }
            }
        }

        if let Some(hit) = nearest_hit {
            let normal = hit.object.normal_at(&hit);

            //if depth == 0 {
                //println!(
                    //"Primary hit at {:?}. Origin: {:?}",
                    //hit.ray.origin + hit.distance * hit.ray.direction,
                    //hit.ray.origin
                //);
            //}

            match hit.object.material() {
                Material::Lambertian(color) => {
                    let new_origin = ray.origin + ray.direction * hit.distance;
                    let new_direction =
                        (random_in_hemisphere(rng, new_origin, normal) - new_origin).normalized();

                    let cos_theta = new_direction.dot(normal);

                    let p = 1f32 / (2f32 * std::f32::consts::PI);

                    let new_ray = Ray {
                        origin: new_origin,
                        direction: new_direction,
                    };

                    let brdf = 0.5 / std::f32::consts::PI;

                    let incoming = trace(ctx.clone(), rng, new_ray, depth + 1);

                    color + brdf * incoming * cos_theta / p
                    //brdf * incoming * cos_theta / p
                }
                Material::Glossy => {
                    let reflected = ray.direction.reflected(-normal);
                    let new_origin = ray.origin + ray.direction * hit.distance;
                    let new_ray = Ray {
                        origin: new_origin,
                        direction: reflected.normalized(),
                    };
                    0.8f32 * trace(ctx, rng, new_ray, depth + 1)
                }
            }
        } else {
            let t = 0.5 * (ray.direction.normalized().y + 1.0);
            (1.0 - t) * Rgb::new(1.0, 1.0, 1.0) + t * Rgb::new(0.5, 0.7, 1.0)
        }
    }
}

fn random_in_hemisphere(
    rng: &mut rand::ThreadRng,
    center: Vec3<f32>,
    normal: Vec3<f32>,
) -> Vec3<f32> {
    let theta = rng.gen_range(0f32, std::f32::consts::PI);
    let phi = rng.gen_range(0f32, 2f32 * std::f32::consts::PI);

    let x = theta.sin() * phi.cos();
    let y = theta.sin() * phi.sin();
    let z = theta.cos();

    center + normal + Vec3::new(x, y, z)
}

pub fn render<'a, C>(sender: &Sender, ctx: RenderContext<C>)
where
    C: Camera + Send + Sync + 'static,
{
    let pxls = 0..(ctx.width * ctx.height);

    let ctx = Arc::new(ctx);
    //for i in 0..ctx.n_threads {
        //spawn_thread(sender, ctx.clone(), i);
    //}
    
    let sender = sender.clone();
    
    std::thread::spawn(move || {
        pxls
            .into_par_iter()
            .for_each_with(sender.clone(), |s, idx| { 
                let u = idx / ctx.width;
                let v = idx % ctx.width;

                let coord = Vec2::new(u, v);

                let mut acc = Rgb::<f32>::zero();
                let mut rng = rand::thread_rng();

                for _ in 0..ctx.samples {
                    let ray = ctx.camera.generate_ray(coord);
                    //println!("{:?}", ray);
                    acc += trace(ctx.clone(), &mut rng, ray, 0);
                }

                let color = acc / ctx.samples as f32;

                s.send((coord, color)).unwrap();
        });
    });
}
