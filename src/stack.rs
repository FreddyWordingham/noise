use nalgebra::Vector2;

use crate::noise::Noise;

pub struct Stack {
    noise_weights: Vec<(Box<dyn Noise>, f32)>,
}

impl Stack {
    pub fn new(noise_weights: Vec<(Box<dyn Noise>, f32)>) -> Self {
        Self { noise_weights }
    }
}

impl Noise for Stack {
    fn sample(&self, x: f32, y: f32) -> f32 {
        self.noise_weights
            .iter()
            .map(|(noise, weight)| noise.sample(x, y) * weight)
            .sum()
    }

    fn gradient(&self, x: f32, y: f32) -> Vector2<f32> {
        self.noise_weights
            .iter()
            .map(|(noise, weight)| noise.gradient(x, y) * *weight)
            .sum()
    }
}
