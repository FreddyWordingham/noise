use std::f32;

use nalgebra::Vector2;
use rand::Rng;

use crate::noise::Noise;

pub struct Worley {
    points: Vec<Vector2<f32>>,
}

impl Worley {
    /// Creates a new Worley noise generator with the specified number of feature points.
    pub fn new<R: Rng>(num_points: usize, mut rng: R) -> Self {
        let points = (0..num_points)
            .map(|_| Vector2::new(rng.gen::<f32>(), rng.gen::<f32>()))
            .collect();
        Self { points }
    }
}

impl Noise for Worley {
    /// Samples the Worley noise at a given (x, y) coordinate.
    /// Ensures the noise is tilable within the unit square.
    fn sample(&self, x: f32, y: f32) -> f32 {
        let sample_point = Vector2::new(x, y);
        self.points
            .iter()
            .flat_map(|&p| {
                // Wrap points to make the noise tilable
                [
                    p,
                    p + Vector2::new(1.0, 0.0),
                    p + Vector2::new(0.0, 1.0),
                    p + Vector2::new(-1.0, 0.0),
                    p + Vector2::new(0.0, -1.0),
                    p + Vector2::new(1.0, 1.0),
                    p + Vector2::new(-1.0, 1.0),
                    p + Vector2::new(1.0, -1.0),
                    p + Vector2::new(-1.0, -1.0),
                ]
            })
            .map(|p| (p - sample_point).norm())
            .fold(f32::MAX, f32::min)
    }
}
