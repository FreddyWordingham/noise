use ndarray::Array2;
use ndarray_images::Image;
use noise::{Noise, Worley};
use rand::prelude::*;

const NUM_POINTS: usize = 17;
const RESOLUTION: (usize, usize) = (1000, 1000);
const OUTPUT_FILE: &str = "output/worley.png";

fn main() {
    // Random number generator
    let mut rng = thread_rng();

    // Generate the noise
    let noise = Worley::new(NUM_POINTS, &mut rng);

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
