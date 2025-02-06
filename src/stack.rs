use nalgebra::Vector2;

use crate::noise::Noise;

pub enum GradientFunction {
    Noop,
    Inverse { factor: f32 },   // 1/(1 + factor * x)
    Exp { scale: f32 },        // exp(-(scale * x)^2)
    Sigmoid { factor: f32 },   // 1/(1 + exp(-factor * x))
    Tanh { factor: f32 },      // (tanh(factor * x) + 1) / 2
    Cosine { frequency: f32 }, // 0.5 * (cos(frequency * x) + 1)
    Quadratic { factor: f32 }, // 1/(1 + factor * x^2)
    Arctan { factor: f32 },    // (atan(factor * x) / pi + 0.5)
}

impl GradientFunction {
    fn scale(&self, x: f32) -> f32 {
        match self {
            GradientFunction::Noop => 1.0,
            GradientFunction::Inverse { factor } => 1.0 / (1.0 + factor * x),
            GradientFunction::Exp { scale } => (-(scale * x).powi(2)).exp(),
            GradientFunction::Sigmoid { factor } => 1.0 / (1.0 + (-factor * x).exp()),
            GradientFunction::Tanh { factor } => (f32::tanh(factor * x) + 1.0) / 2.0,
            GradientFunction::Cosine { frequency } => 0.5 * ((frequency * x).cos() + 1.0),
            GradientFunction::Quadratic { factor } => 1.0 / (1.0 + factor * x.powi(2)),
            GradientFunction::Arctan { factor } => {
                f32::atan(factor * x) / std::f32::consts::PI + 0.5
            }
        }
    }
}

pub struct Stack {
    gradient_function: GradientFunction,
    noise_weights: Vec<(Box<dyn Noise>, f32)>,
}

impl Stack {
    pub fn new(
        gradient_function: GradientFunction,
        noise_weights: Vec<(Box<dyn Noise>, f32)>,
    ) -> Self {
        debug_assert!(noise_weights.iter().all(|(_, weight)| *weight >= 0.0));

        Self {
            gradient_function,
            noise_weights,
        }
    }
}

impl Noise for Stack {
    fn sample(&self, x: f32, y: f32) -> f32 {
        let mut total_sample = 0.0;
        let mut total_gradient = Vector2::new(0.0, 0.0);
        for (noise, weight) in &self.noise_weights {
            let factor = self.gradient_function.scale(total_gradient.norm());
            let sample_value = noise.sample(x, y) * weight;
            let grad_value = *weight * noise.gradient(x, y);
            total_sample += sample_value * factor;
            total_gradient += grad_value * factor;
        }
        total_sample
    }

    fn gradient(&self, x: f32, y: f32) -> Vector2<f32> {
        let mut total_gradient = Vector2::new(0.0, 0.0);
        for (noise, weight) in &self.noise_weights {
            let factor = self.gradient_function.scale(total_gradient.norm());
            let grad_value = *weight * noise.gradient(x, y);
            total_gradient += grad_value * factor;
        }
        total_gradient
    }
}
