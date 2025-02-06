use nalgebra::Vector2;

pub trait Noise {
    fn sample(&self, u: f32, v: f32) -> f32;
    fn gradient(&self, u: f32, v: f32) -> Vector2<f32>;
}
