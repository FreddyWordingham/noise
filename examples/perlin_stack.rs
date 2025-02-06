use nalgebra::Vector2;
use ndarray::{Array2, Zip};
use ndarray_images::Image;
use noisette::{Noise, Perlin, Stack};
use rand::prelude::*;

const PERLIN_XS: ((usize, usize), f32) = ((43, 43), 0.0625);
const PERLIN_SM: ((usize, usize), f32) = ((31, 31), 0.125);
const PERLIN_MD: ((usize, usize), f32) = ((21, 21), 0.25);
const PERLIN_LG: ((usize, usize), f32) = ((11, 11), 0.5);
const PERLIN_XL: ((usize, usize), f32) = ((5, 5), 1.0);
const RESOLUTION: (usize, usize) = (256, 256);
const OUTPUT_NOISE_FILE: &str = "output/perlin_stack-samples.png";
const OUTPUT_GRADIENT_FILE: &str = "output/perlin_stack-gradient.png";

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

    let noise = Stack::new(vec![
        (Box::new(Perlin::new(PERLIN_XS.0, &mut rng)), PERLIN_XS.1),
        (Box::new(Perlin::new(PERLIN_SM.0, &mut rng)), PERLIN_SM.1),
        (Box::new(Perlin::new(PERLIN_MD.0, &mut rng)), PERLIN_MD.1),
        (Box::new(Perlin::new(PERLIN_LG.0, &mut rng)), PERLIN_LG.1),
        (Box::new(Perlin::new(PERLIN_XL.0, &mut rng)), PERLIN_XL.1),
    ]);
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
