use ndarray::Array2;
use ndarray_images::Image;
use noise::Perlin;
use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();

    let perlin = Perlin::new((5, 4), &mut rng);

    let resolution = (1000, 1000);
    let mut samples = Array2::zeros(resolution);
    let width = samples.ncols();
    let height = samples.nrows();
    for ((xi, yi), value) in samples.indexed_iter_mut() {
        let x = xi as f32 / width as f32 * 2.0;
        let y = yi as f32 / height as f32 * 2.0;
        *value = perlin.sample(x, y);
    }

    let min = samples.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let max = samples.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let range = max - min;
    samples.mapv_inplace(|v| (v - min) / range);

    samples.save("output.png").expect("Failed to save image");
}
