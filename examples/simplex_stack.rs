use nalgebra::Vector2;
use ndarray::{Array2, Zip};
use ndarray_images::Image;
use noisette::{GradientFunction, Noise, Simplex, Stack};
use rand::rng;

const SIMPLEX_XS: (f32, f32) = (43.0, 0.0625);
const SIMPLEX_SM: (f32, f32) = (31.0, 0.125);
const SIMPLEX_MD: (f32, f32) = (21.0, 0.25);
const SIMPLEX_LG: (f32, f32) = (11.0, 0.5);
const SIMPLEX_XL: (f32, f32) = (5.0, 1.0);
const RESOLUTION: (usize, usize) = (256, 256);
const OUTPUT_NOISE_FILE: &str = "output/simplex_stack-samples.png";
const OUTPUT_GRADIENT_FILE: &str = "output/simplex_stack-gradient.png";

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
    let mut rng = rng();

    let noise = Stack::new(
        GradientFunction::Noop,
        vec![
            (Box::new(Simplex::new(SIMPLEX_XL.0, &mut rng)), SIMPLEX_XL.1),
            (Box::new(Simplex::new(SIMPLEX_LG.0, &mut rng)), SIMPLEX_LG.1),
            (Box::new(Simplex::new(SIMPLEX_MD.0, &mut rng)), SIMPLEX_MD.1),
            (Box::new(Simplex::new(SIMPLEX_SM.0, &mut rng)), SIMPLEX_SM.1),
            (Box::new(Simplex::new(SIMPLEX_XS.0, &mut rng)), SIMPLEX_XS.1),
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
