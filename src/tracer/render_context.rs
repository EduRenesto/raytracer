use std::sync::Arc;

use super::camera::Camera;
use super::light::LightSampler;
use super::shape::Shape;

/// Stores all the information needed to perform
/// the rendering.
pub struct RenderContext<C: Camera> {
    pub width: usize,
    pub height: usize,
    pub samples: u16,
    pub n_threads: u16,

    pub objects: Arc<Vec<Arc<dyn Shape>>>,

    pub camera: C,

    pub lights: Arc<Vec<Arc<dyn LightSampler>>>,
    pub ambient: vek::rgb::Rgb<f32>,
}
