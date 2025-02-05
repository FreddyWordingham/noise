use nalgebra::Vector2;

pub trait Noise {
    fn sample(&self, x: f32, y: f32) -> f32;
    fn gradient(&self, x: f32, y: f32) -> Vector2<f32>;
}
