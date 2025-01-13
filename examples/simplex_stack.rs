use ndarray::Array2;
use ndarray_images::Image;
use noise::{Noise, Simplex, Stack};
use rand::prelude::*;

const SIMPLEX_XS: (f32, f32) = (43.0, 0.0625);
const SIMPLEX_SM: (f32, f32) = (31.0, 0.125);
const SIMPLEX_MD: (f32, f32) = (21.0, 0.25);
const SIMPLEX_LG: (f32, f32) = (11.0, 0.5);
const SIMPLEX_XL: (f32, f32) = (5.0, 1.0);

const RESOLUTION: (usize, usize) = (1000, 1000);
const OUTPUT_FILE: &str = "output/simplex-stack.png";

fn main() {
    // Random number generator
    let mut rng = thread_rng();

    // Generate the noise
    let noise = Stack::new(vec![
        (Box::new(Simplex::new(SIMPLEX_XS.0, &mut rng)), SIMPLEX_XS.1),
        (Box::new(Simplex::new(SIMPLEX_SM.0, &mut rng)), SIMPLEX_SM.1),
        (Box::new(Simplex::new(SIMPLEX_MD.0, &mut rng)), SIMPLEX_MD.1),
        (Box::new(Simplex::new(SIMPLEX_LG.0, &mut rng)), SIMPLEX_LG.1),
        (Box::new(Simplex::new(SIMPLEX_XL.0, &mut rng)), SIMPLEX_XL.1),
    ]);

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
        .save(OUTPUT_FILE)
        .expect(&format!("Failed to save {}", OUTPUT_FILE));
}
