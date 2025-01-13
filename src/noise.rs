pub trait Noise {
    fn sample(&self, x: f32, y: f32) -> f32;
}
