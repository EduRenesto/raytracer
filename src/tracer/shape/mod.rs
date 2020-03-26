use vek::vec::Vec3;

use super::material::Material;
use super::ray::{Ray, RayHit};

//pub trait Shape {
//fn intersects<'a>(&self, ray: Ray) -> Option<RayHit<'a>>;
//fn normal_at<'a>(&self, hit: &RayHit<'a>) -> Vec3<f32>;
//fn material(&self) -> Material;
//}

//#[derive(Copy, Clone)]
//pub enum Shape {
    //Sphere(Material, Vec3<f32>, f32),
    //Plane(Material, Vec3<f32>, Vec3<f32>),
    //Poly(Material, Triangle),
//}

pub trait Shape {
    fn intersects<'a>(&self, ray: &'a Ray) -> Option<RayHit<'a>>;
    fn normal_at<'a>(&self, point: Vec3<f32>) -> Vec3<f32>;
    fn material(&self) -> Material;
}

#[derive(Debug)]
enum QuadraticResult {
    TwoReal(f32, f32),
    OneReal(f32),
    TwoComplex((f32, f32), (f32, f32)),
}

fn solve(a: f32, b: f32, c: f32) -> QuadraticResult {
    let disc = b * b - 4f32 * a * c;

    if disc < 0f32 {
        QuadraticResult::TwoComplex(
            (-b / (2f32 * a), disc.abs().sqrt() / (2f32 * a)),
            (-b / (2f32 * a), -disc.abs().sqrt() / (2f32 * a)),
        )
    } else if disc == 0f32 {
        QuadraticResult::OneReal(-b / (2f32 * a))
    } else {
        QuadraticResult::TwoReal(
            ((-b + disc.sqrt()) / 2f32 * a),
            ((-b - disc.sqrt()) / 2f32 * a),
        )
    }
}

impl Shape {
    pub fn intersects<'a>(&self, ray: &'a Ray) -> Option<RayHit<'a>> {
        match self {
            &Shape::Sphere(_, center, radius) => {
                let oc = ray.origin - center;
                let a = ray.direction.dot(ray.direction);
                let b = 2.0 * oc.dot(ray.direction);
                let c = oc.dot(oc) - radius * radius;

                let roots = solve(a, b, c);

                match roots {
                    QuadraticResult::OneReal(x) | QuadraticResult::TwoReal(_, x) => {
                        if x < 0f32 {
                            None
                        } else {
                            Some(RayHit {
                                ray: &ray,
                                distance: x,
                                object: *self,
                            })
                        }
                    }
                    _ => None,
                }
            }
            &Shape::Plane(_, point, normal) => {
                let nr = normal.dot(ray.direction);
                let u = ray.origin - point;
                let t = -((u.dot(normal)) / nr);

                if nr == 0f32 || t <= 0f32 {
                    None
                } else {
                    Some(RayHit {
                        ray: &ray,
                        distance: t,
                        object: *self,
                    })
                }
            }
            &Shape::Poly(_, triangle) => {
                let edge1 = triangle.vertices[1] - triangle.vertices[0];
                let edge2 = triangle.vertices[2] - triangle.vertices[0];
                let h = ray.direction.cross(edge2);
                let a = edge1.dot(h);

                if a.abs() < 0.0001 {
                    return None;
                }

                let f = 1.0/a;
                let s = ray.origin - triangle.vertices[0];
                let u = f * s.dot(h);

                if u < 0.0 || u > 1.0 {
                    return None;
                }

                let q = s.cross(edge1);
                let v = f * ray.direction.dot(q);

                if v < 0.0 || u + v > 1.0 {
                    return None;
                }

                let t = f * edge2.dot(q);

                if t > 0.0001 {
                    Some(RayHit {
                        ray: &ray,
                        distance: t,
                        object: *self,
                    })
                } else {
                    None
                }
            }
        }
    }
    pub fn normal_at<'a>(&self, hit: &RayHit<'a>) -> Vec3<f32> {
        match self {
            &Shape::Sphere(_, center, _) => {
                let normal = (hit.ray.origin + hit.ray.direction * hit.distance) - center;
                normal.normalized()
            }
            &Shape::Plane(_, _, normal) => normal,
            &Shape::Poly(_, trig) => {
                let v0 = trig.vertices[1] - trig.vertices[0];
                let v1 = trig.vertices[2] - trig.vertices[0];

                -v0.cross(v1).normalized()
            }
        }
    }
    pub fn material(&self) -> Material {
        match self {
            &Shape::Sphere(mat, _, _) => mat,
            &Shape::Plane(mat, _, _) => mat,
            &Shape::Poly(mat, _) => mat,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Triangle {
    pub vertices: [vek::Vec3<f32>; 3],
    pub normals: Option<[vek::Vec3<f32>; 3]>,
    pub tex_coords: Option<[vek::Vec2<f32>; 3]>,
}
