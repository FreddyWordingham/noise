use nalgebra::Vector2;
use ndarray::{Array2, Zip};
use ndarray_images::Image;
use noisette::{GradientFunction, Noise, Stack, Worley};
use rand::prelude::*;

const WORLEY_XS: (usize, f32) = (127, 0.0625);
const WORLEY_SM: (usize, f32) = (81, 0.125);
const WORLEY_MD: (usize, f32) = (51, 0.25);
const WORLEY_LG: (usize, f32) = (31, 0.5);
const WORLEY_XL: (usize, f32) = (11, 1.0);
const RESOLUTION: (usize, usize) = (256, 256);
const OUTPUT_NOISE_FILE: &str = "output/worley_stack-samples.png";
const OUTPUT_GRADIENT_FILE: &str = "output/worley_stack-gradient.png";

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

    let noise = Stack::new(
        GradientFunction::Noop,
        vec![
            (Box::new(Worley::new(WORLEY_XL.0, &mut rng)), WORLEY_XL.1),
            (Box::new(Worley::new(WORLEY_LG.0, &mut rng)), WORLEY_LG.1),
            (Box::new(Worley::new(WORLEY_MD.0, &mut rng)), WORLEY_MD.1),
            (Box::new(Worley::new(WORLEY_SM.0, &mut rng)), WORLEY_SM.1),
            (Box::new(Worley::new(WORLEY_XS.0, &mut rng)), WORLEY_XS.1),
        ],
    );
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
