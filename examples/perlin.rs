use nalgebra::Vector2;
use ndarray::{Array2, Zip};
use ndarray_images::Image;
use noisette::{Noise, Perlin};
use rand::prelude::*;

const SHAPE: (usize, usize) = (5, 7);
const RESOLUTION: (usize, usize) = (1024, 1024);
const OUTPUT_NOISE_FILE: &str = "output/perlin-samples.png";
const OUTPUT_GRADIENT_FILE: &str = "output/perlin-gradient.png";

fn sample_noise<N: Noise>(
    resolution: (usize, usize),
    noise: &N,
) -> (Array2<f32>, Array2<Vector2<f32>>) {
    let width = resolution.1;
    let height = resolution.0;

    let mut samples = Array2::zeros(RESOLUTION);
    let mut gradients = Array2::from_elem(RESOLUTION, Vector2::zeros());

    Zip::indexed(&mut samples)
        .and(&mut gradients)
        .for_each(|(xi, yi), sample, gradient| {
            let x = xi as f32 / width as f32;
            let y = yi as f32 / height as f32;
            *sample = noise.sample(x, y);
            *gradient = noise.gradient(x, y);
        });

    (samples, gradients)
}

fn find_min_max(data: &Array2<f32>) -> (f32, f32) {
    let min = data.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let max = data.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    (min, max)
}

fn magnitudes(data: &Array2<Vector2<f32>>) -> Array2<f32> {
    data.mapv(|v| v.norm())
}

fn normalize(data: &mut Array2<f32>) {
    let min = data.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let max = data.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let range = max - min;
    data.mapv_inplace(|v| (v - min) / range);
}

fn save(data: &Array2<f32>, filename: &str) {
    data.save(filename)
        .expect(&format!("Failed to save {}", filename));
}

fn main() {
    let mut rng = thread_rng();

    let noise = Perlin::new(SHAPE, &mut rng);
    let (mut samples, gradients) = sample_noise(RESOLUTION, &noise);

    let (min, max) = find_min_max(&samples);
    println!("Samples min: {}, max: {}", min, max);
    normalize(&mut samples);
    save(&samples, OUTPUT_NOISE_FILE);

    let mut magnitudes = magnitudes(&gradients);
    let (min, max) = find_min_max(&magnitudes);
    println!("Magnitudes min: {}, max: {}", min, max);
    normalize(&mut magnitudes);
    save(&magnitudes, OUTPUT_GRADIENT_FILE);
}
