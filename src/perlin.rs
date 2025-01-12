use std::f32::consts::TAU;

use nalgebra::{Unit, Vector2};
use ndarray::Array2;
use rand::Rng;

pub struct Perlin {
    vectors: Array2<Unit<Vector2<f32>>>,
}

impl Perlin {
    pub fn new<R: Rng>(shape: (usize, usize), mut rng: R) -> Self {
        assert!(shape.0 > 0 && shape.1 > 0);

        let vectors = Array2::from_shape_fn(shape, |(_x, _y)| {
            let angle = TAU * rng.gen::<f32>();
            Unit::new_normalize(Vector2::new(angle.cos(), angle.sin()))
        });

        Self { vectors }
    }

    pub fn sample(&self, x: f32, y: f32) -> f32 {
        let (width, height) = self.vectors.dim();
        let px = x * width as f32;
        let py = y * height as f32;

        // Grid cell coordinates
        let x0 = (px.floor() as i32) % width as i32;
        let y0 = (py.floor() as i32) % height as i32;
        let x1 = (x0 + 1) % width as i32;
        let y1 = (y0 + 1) % height as i32;

        // Local coordinates within the cell
        let xf = px - px.floor();
        let yf = py - py.floor();

        // Dot products with gradient vectors
        let g00 = self.grad_dot(x0, y0, xf, yf);
        let g10 = self.grad_dot(x1, y0, xf - 1.0, yf);
        let g01 = self.grad_dot(x0, y1, xf, yf - 1.0);
        let g11 = self.grad_dot(x1, y1, xf - 1.0, yf - 1.0);

        // Interpolation
        let u = fade(xf);
        let v = fade(yf);

        let nx0 = lerp(g00, g10, u);
        let nx1 = lerp(g01, g11, u);
        lerp(nx0, nx1, v)
    }

    fn grad_dot(&self, gx: i32, gy: i32, x: f32, y: f32) -> f32 {
        self.gradient(gx, gy).dot(&Vector2::new(x, y))
    }

    fn gradient(&self, x: i32, y: i32) -> &Unit<Vector2<f32>> {
        let (rows, cols) = self.vectors.dim();
        let nx = x.rem_euclid(cols as i32) as usize;
        let ny = y.rem_euclid(rows as i32) as usize;
        &self.vectors[(ny, nx)]
    }
}

fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}
