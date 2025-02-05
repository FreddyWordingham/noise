use nalgebra::Vector2;
use ndarray::Array2;
use ndarray_images::Image;
use noisette::{Noise, Perlin};
use rand::prelude::*;

const SHAPE: (usize, usize) = (5, 7);
const RESOLUTION: (usize, usize) = (512, 512);
const OUTPUT_NOISE_FILE: &str = "output/perlin.png";
const OUTPUT_GRADIENT_FILE: &str = "output/perlin_gradient.png";

fn main() {
    // Random number generator
    let mut rng = thread_rng();

    // Generate the noise
    let noise = Perlin::new(SHAPE, &mut rng);

    // Sample the noise regularly across the unit square
    let mut samples = Array2::zeros(RESOLUTION);
    let width = samples.ncols();
    let height = samples.nrows();
    for ((xi, yi), value) in samples.indexed_iter_mut() {
        let x = xi as f32 / width as f32;
        let y = yi as f32 / height as f32;
        *value = noise.sample(x, y);
    }

    // Analyze the sample data
    let min = samples.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let max = samples.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let range = max - min;
    println!("Min: {}, Max: {}, Range: {}", min, max, range);

    // Normalize the samples to the range [0, 1]
    samples.mapv_inplace(|v| (v - min) / range);

    // Save the samples as an image
    samples
        .save(OUTPUT_NOISE_FILE)
        .expect(&format!("Failed to save {}", OUTPUT_NOISE_FILE));

    // Sample the gradient of the noise regularly across the unit square
    let mut gradients = Array2::from_elem(RESOLUTION, Vector2::new(0.0, 0.0));
    for ((xi, yi), gradient) in gradients.indexed_iter_mut() {
        let x = xi as f32 / width as f32;
        let y = yi as f32 / height as f32;
        *gradient = noise.sample_gradient(x, y);
    }

    // Find the magnitude of the gradients
    let magnitudes = gradients.mapv(|v| v.norm());

    // Analyze the magnitude data
    let min = magnitudes.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let max = magnitudes.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let range = max - min;
    println!("Min: {}, Max: {}, Range: {}", min, max, range);

    // Normalize the magnitudes to the range [0, 1]
    let magnitudes = magnitudes.mapv(|v| (v - min) / range);

    // Save the magnitudes as an image
    magnitudes
        .save(OUTPUT_GRADIENT_FILE)
        .expect(&format!("Failed to save {}", OUTPUT_GRADIENT_FILE));
}
