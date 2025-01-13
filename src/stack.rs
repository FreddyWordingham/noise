use rand::Rng;

use crate::perlin::Perlin;

pub struct PerlinStack {
    vectors: Vec<(Perlin, f32)>,
}

impl PerlinStack {
    pub fn new<R: Rng>(layers: &[((usize, usize), f32)], mut rng: R) -> Self {
        assert!(layers.len() > 0);
        assert!(layers
            .iter()
            .all(|(shape, _weight)| shape.0 > 0 && shape.1 > 0));
        assert!(layers.iter().all(|(_shape, weight)| *weight > 0.0));

        let total_weight: f32 = layers.iter().map(|(_shape, weight)| *weight).sum();
        Self {
            vectors: layers
                .iter()
                .map(|(shape, weight)| {
                    let perlin = Perlin::new(*shape, &mut rng);
                    (perlin, *weight / total_weight)
                })
                .collect(),
        }
    }

    pub fn sample(&self, x: f32, y: f32) -> f32 {
        self.vectors
            .iter()
            .map(|(perlin, weight)| perlin.sample(x, y) * weight)
            .sum()
    }
}
