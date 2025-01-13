use ndarray::Array2;
use ndarray_images::Image;
use noisette::{Noise, Perlin, Stack};
use rand::prelude::*;

const PERLIN_XS: ((usize, usize), f32) = ((43, 43), 0.0625);
const PERLIN_SM: ((usize, usize), f32) = ((31, 31), 0.125);
const PERLIN_MD: ((usize, usize), f32) = ((21, 21), 0.25);
const PERLIN_LG: ((usize, usize), f32) = ((11, 11), 0.5);
const PERLIN_XL: ((usize, usize), f32) = ((5, 5), 1.0);

const RESOLUTION: (usize, usize) = (512, 512);
const OUTPUT_FILE: &str = "output/perlin-stack.png";

fn main() {
    // Random number generator
    let mut rng = thread_rng();

    // Generate the noise
    let noise = Stack::new(vec![
        (Box::new(Perlin::new(PERLIN_XS.0, &mut rng)), PERLIN_XS.1),
        (Box::new(Perlin::new(PERLIN_SM.0, &mut rng)), PERLIN_SM.1),
        (Box::new(Perlin::new(PERLIN_MD.0, &mut rng)), PERLIN_MD.1),
        (Box::new(Perlin::new(PERLIN_LG.0, &mut rng)), PERLIN_LG.1),
        (Box::new(Perlin::new(PERLIN_XL.0, &mut rng)), PERLIN_XL.1),
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
