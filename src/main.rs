use ndarray::{Array2, Array3};
use ndarray_images::Image;
use noise::{Perlin, Simplex, Worley};
use rand::prelude::*;

// #[allow(dead_code)]
// fn example_simplex_noise() {
//     let mut rng = thread_rng();

//     let noise = Simplex::new(&mut rng);

//     let resolution = (1000, 1000);
//     let mut samples = Array2::zeros(resolution);
//     let width = samples.ncols();
//     let height = samples.nrows();
//     for ((xi, yi), value) in samples.indexed_iter_mut() {
//         let x = xi as f32 / width as f32 * 2.0;
//         let y = yi as f32 / height as f32 * 2.0;
//         *value = noise.sample(x, y);
//     }

//     // Normalize the samples to the range [0, 1]
//     let min = samples.iter().fold(f32::INFINITY, |a, &b| a.min(b));
//     let max = samples.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
//     let range = max - min;
//     samples.mapv_inplace(|v| (v - min) / range);

//     // Save the samples as an image
//     samples.save("output.png").expect("Failed to save image");
// }

// #[allow(dead_code)]
// fn example_simplex_stack_noise() {
//     let mut rng = thread_rng();

//     let noise_a = Simplex::new(&mut rng);
//     let noise_b = Simplex::new(&mut rng);
//     let noise_c = Simplex::new(&mut rng);
//     let noise_d = Simplex::new(&mut rng);
//     let noise_e = Simplex::new(&mut rng);
//     let noise_f = Simplex::new(&mut rng);
//     let noise_g = Simplex::new(&mut rng);
//     let a = 2.0;
//     let b = 5.0;
//     let c = 11.0;
//     let d = 25.0;
//     let e = 41.0;
//     let f = 61.0;
//     let g = 101.0;

//     let resolution = (1000, 1000);
//     let mut samples = Array2::zeros(resolution);
//     let width = samples.ncols();
//     let height = samples.nrows();
//     for ((xi, yi), value) in samples.indexed_iter_mut() {
//         let x = xi as f32 / width as f32;
//         let y = yi as f32 / height as f32;
//         *value += noise_a.sample(x * a, y * a) * 10.0;
//         *value += noise_b.sample(x * b, y * b) * 5.0;
//         *value += noise_c.sample(x * c, y * c) * 2.5;
//         *value += noise_d.sample(x * d, y * d) * 1.25;
//         *value += noise_e.sample(x * e, y * e) * 0.625;
//         *value += noise_f.sample(x * f, y * f) * 0.3125;
//         *value += noise_g.sample(x * g, y * g) * 0.15625;
//     }

//     // Normalize the samples to the range [0, 1]
//     let min = samples.iter().fold(f32::INFINITY, |a, &b| a.min(b));
//     let max = samples.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
//     let range = max - min;
//     samples.mapv_inplace(|v| (v - min) / range);

//     // // Round values
//     // let levels = 20.0;
//     // let inv_levels = 1.0 / levels;
//     // samples.mapv_inplace(|v| (v * levels).round() * inv_levels);

//     // Save the samples as an image
//     samples.save("output.png").expect("Failed to save image");
// }

// #[allow(dead_code)]
// fn example_perlin_noise() {
//     let mut rng = thread_rng();

//     let noise = Perlin::new((5, 4), &mut rng);

//     let resolution = (1000, 1000);
//     let mut samples = Array2::zeros(resolution);
//     let width = samples.ncols();
//     let height = samples.nrows();
//     for ((xi, yi), value) in samples.indexed_iter_mut() {
//         let x = xi as f32 / width as f32 * 2.0;
//         let y = yi as f32 / height as f32 * 2.0;
//         *value = noise.sample(x, y);
//     }

//     // Normalize the samples to the range [0, 1]
//     let min = samples.iter().fold(f32::INFINITY, |a, &b| a.min(b));
//     let max = samples.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
//     let range = max - min;
//     samples.mapv_inplace(|v| (v - min) / range);

//     // Save the samples as an image
//     samples.save("output.png").expect("Failed to save image");
// }

// #[allow(dead_code)]
// fn example_perlin_stack_noise() {
//     let mut rng = thread_rng();

//     let noise = PerlinStack::new(
//         vec![((5, 4), 1.0), ((10, 8), 0.5), ((20, 16), 0.25)],
//         &mut rng,
//     );

//     let resolution = (1000, 1000);
//     let mut samples = Array2::zeros(resolution);
//     let width = samples.ncols();
//     let height = samples.nrows();
//     for ((xi, yi), value) in samples.indexed_iter_mut() {
//         let x = xi as f32 / width as f32 * 2.0;
//         let y = yi as f32 / height as f32 * 2.0;
//         *value = noise.sample(x, y);
//     }

//     // Normalize the samples to the range [0, 1]
//     let min = samples.iter().fold(f32::INFINITY, |a, &b| a.min(b));
//     let max = samples.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
//     let range = max - min;
//     samples.mapv_inplace(|v| (v - min) / range);

//     // Save the samples as an image
//     samples.save("output.png").expect("Failed to save image");
// }

// fn sample_map<R: Rng>(
//     resolution: (usize, usize),
//     layers: Vec<((usize, usize), f32)>,
//     rng: &mut R,
// ) -> Array2<f32> {
//     let noise = PerlinStack::new(layers, rng);

//     let mut samples = Array2::zeros(resolution);
//     let width = samples.ncols();
//     let height = samples.nrows();
//     for ((xi, yi), value) in samples.indexed_iter_mut() {
//         let x = xi as f32 / width as f32;
//         let y = yi as f32 / height as f32;
//         *value = noise.sample(x, y);
//     }

//     // Normalize the samples to the range [0, 1]
//     let min = samples.iter().fold(f32::INFINITY, |a, &b| a.min(b));
//     let max = samples.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
//     let range = max - min;

//     samples.mapv_inplace(|v| (v - min) / range);
//     samples
// }

// fn cutoff_map(resolution: (usize, usize)) -> Array2<f32> {
//     let mut samples = Array2::zeros(resolution);
//     let width = samples.ncols();
//     let height = samples.nrows();
//     for ((xi, yi), value) in samples.indexed_iter_mut() {
//         let x = xi as f32 / width as f32;
//         let y = yi as f32 / height as f32;
//         let dx = x - 0.5;
//         let dy = y - 0.5;
//         let dist = (dx * dx + dy * dy).sqrt();
//         *value = (1.0 - (dist * 2.0)).max(0.0);
//     }

//     samples
// }

// #[allow(dead_code)]
// fn example_terrain_generation() {
//     let resolution = (1000, 1000);
//     let mut rng = thread_rng();

//     let height_map = sample_map(
//         resolution,
//         vec![((5, 5), 1.0), ((10, 10), 0.5), ((20, 20), 0.25)],
//         &mut rng,
//     );
//     let moisture_map = sample_map(resolution, vec![((8, 8), 0.25), ((16, 16), 0.5)], &mut rng);
//     let island_map = cutoff_map(resolution);
//     // let island_map = example_worley_noise();

//     let mut map: Array3<f32> = Array3::zeros((resolution.0, resolution.1, 3));

//     for xi in 0..resolution.0 {
//         for yi in 0..resolution.1 {
//             let h = height_map[[yi, xi]] * island_map[[yi, xi]];
//             let m = moisture_map[[yi, xi]];

//             if h < 0.2 {
//                 map[[yi, xi, 0]] = 0.0;
//                 map[[yi, xi, 1]] = 0.0;
//                 map[[yi, xi, 2]] = ((h / 0.2) + 0.1).min(1.0);
//             } else if h < 0.3 {
//                 let r0 = 0.89;
//                 let g0 = 0.80;
//                 let b0 = 0.46;
//                 let r1 = 0.66;
//                 let g1 = 0.50;
//                 let b1 = 0.27;

//                 map[[yi, xi, 0]] = r0 + ((r1 - r0) * m);
//                 map[[yi, xi, 1]] = g0 + ((g1 - g0) * m);
//                 map[[yi, xi, 2]] = b0 + ((b1 - b0) * m);
//             } else if h < 0.4 {
//                 let r0 = 0.00;
//                 let g0 = 0.53;
//                 let b0 = 0.25;
//                 let r1 = 0.00;
//                 let g1 = 0.50;
//                 let b1 = 0.00;

//                 map[[yi, xi, 0]] = r0 + ((r1 - r0) * m);
//                 map[[yi, xi, 1]] = g0 + ((g1 - g0) * m);
//                 map[[yi, xi, 2]] = b0 + ((b1 - b0) * m);
//             } else if h < 0.6 {
//                 let r0 = 0.54;
//                 let g0 = 0.56;
//                 let b0 = 0.62;
//                 let r1 = 0.69;
//                 let g1 = 0.73;
//                 let b1 = 0.80;

//                 map[[yi, xi, 0]] = r0 + ((r1 - r0) * m);
//                 map[[yi, xi, 1]] = g0 + ((g1 - g0) * m);
//                 map[[yi, xi, 2]] = b0 + ((b1 - b0) * m);
//             } else {
//                 let r0 = 0.9;
//                 let g0 = 0.9;
//                 let b0 = 0.9;
//                 let r1 = 1.0;
//                 let g1 = 1.0;
//                 let b1 = 1.0;

//                 map[[yi, xi, 0]] = r0 + ((r1 - r0) * m);
//                 map[[yi, xi, 1]] = g0 + ((g1 - g0) * m);
//                 map[[yi, xi, 2]] = b0 + ((b1 - b0) * m);
//             }
//         }
//     }

//     height_map.save("height.png").expect("Failed to save image");
//     moisture_map
//         .save("moisture.png")
//         .expect("Failed to save image");
//     island_map.save("island.png").expect("Failed to save image");
//     map.save("map.png").expect("Failed to save image");
// }

// fn example_worley_noise() {
//     let mut rng = thread_rng();

//     let num_points = 7;
//     let noise = Worley::new(num_points, &mut rng);

//     let resolution = (1000, 1000);
//     let mut samples = Array2::zeros(resolution);
//     let width = samples.ncols();
//     let height = samples.nrows();
//     for ((xi, yi), value) in samples.indexed_iter_mut() {
//         let x = xi as f32 / width as f32;
//         let y = yi as f32 / height as f32;
//         *value = noise.sample(x, y);
//     }

//     // Normalize the samples to the range [0, 1]
//     let min = samples.iter().fold(f32::INFINITY, |a, &b| a.min(b));
//     let max = samples.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
//     let range = max - min;
//     samples.mapv_inplace(|v| (v - min) / range);

//     // Invert the samples
//     samples.mapv_inplace(|v| 1.0 - v);

//     // Save the samples as an image
//     samples.save("output.png").expect("Failed to save image");

//     // samples
// }

fn main() {
    // example_worley_noise();
    // example_simplex_noise();
    // example_simplex_stack_noise();
    // example_perlin_noise();
    // example_perlin_stack_noise();
    // example_terrain_generation();
}
