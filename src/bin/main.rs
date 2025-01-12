use ndarray::Array2;
use ndarray_images::Image;
use noise::{Perlin, PerlinStack, Simplex};
use rand::prelude::*;

#[allow(dead_code)]
fn example_simplex_noise() {
    let mut rng = thread_rng();

    let noise = Simplex::new(&mut rng);

    let resolution = (1000, 1000);
    let mut samples = Array2::zeros(resolution);
    let width = samples.ncols();
    let height = samples.nrows();
    for ((xi, yi), value) in samples.indexed_iter_mut() {
        let x = xi as f32 / width as f32 * 2.0;
        let y = yi as f32 / height as f32 * 2.0;
        *value = noise.sample(x, y);
    }

    // Normalize the samples to the range [0, 1]
    let min = samples.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let max = samples.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let range = max - min;
    samples.mapv_inplace(|v| (v - min) / range);

    // Save the samples as an image
    samples.save("output.png").expect("Failed to save image");
}

#[allow(dead_code)]
fn example_perlin_noise() {
    let mut rng = thread_rng();

    let noise = Perlin::new((5, 4), &mut rng);

    let resolution = (1000, 1000);
    let mut samples = Array2::zeros(resolution);
    let width = samples.ncols();
    let height = samples.nrows();
    for ((xi, yi), value) in samples.indexed_iter_mut() {
        let x = xi as f32 / width as f32 * 2.0;
        let y = yi as f32 / height as f32 * 2.0;
        *value = noise.sample(x, y);
    }

    // Normalize the samples to the range [0, 1]
    let min = samples.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let max = samples.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let range = max - min;
    samples.mapv_inplace(|v| (v - min) / range);

    // Save the samples as an image
    samples.save("output.png").expect("Failed to save image");
}

#[allow(dead_code)]
fn example_perlin_stack_noise() {
    let mut rng = thread_rng();

    let noise = PerlinStack::new(
        vec![((5, 4), 1.0), ((10, 8), 0.5), ((20, 16), 0.25)],
        &mut rng,
    );

    let resolution = (1000, 1000);
    let mut samples = Array2::zeros(resolution);
    let width = samples.ncols();
    let height = samples.nrows();
    for ((xi, yi), value) in samples.indexed_iter_mut() {
        let x = xi as f32 / width as f32 * 2.0;
        let y = yi as f32 / height as f32 * 2.0;
        *value = noise.sample(x, y);
    }

    // Normalize the samples to the range [0, 1]
    let min = samples.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let max = samples.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let range = max - min;
    samples.mapv_inplace(|v| (v - min) / range);

    // Save the samples as an image
    samples.save("output.png").expect("Failed to save image");
}

fn main() {
    example_simplex_noise();
    // example_perlin_noise();
    // example_perlin_stack_noise();
}
