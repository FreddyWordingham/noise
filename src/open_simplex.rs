use nalgebra::Vector2;
use rand::{prelude::SliceRandom, Rng};

use crate::noise::Noise;

pub struct OpenSimplex {
    scale: f32,
    perm: Vec<u8>, // Permutation table for periodic wrapping
    period: usize, // Defines the repeat interval
}

impl OpenSimplex {
    pub fn new<R: Rng>(scale: f32, period: usize, mut rng: R) -> Self {
        let mut perm = vec![0u8; period * 2];
        for i in 0..period {
            perm[i] = i as u8;
        }

        perm.shuffle(&mut rng);

        for i in 0..period {
            perm[period + i] = perm[i];
        }

        OpenSimplex {
            scale,
            perm,
            period,
        }
    }

    fn hash(&self, x: i32, y: i32) -> usize {
        let idx = self.perm[(x.rem_euclid(self.period as i32)) as usize] as usize;
        self.perm[(idx + (y.rem_euclid(self.period as i32)) as usize) % self.period] as usize
    }
}

impl Noise for OpenSimplex {
    fn sample(&self, mut x: f32, mut y: f32) -> f32 {
        x *= self.scale;
        y *= self.scale;

        const F2: f32 = 0.3660254037844386; // Skewing factor
        const G2: f32 = 0.21132486540518713; // Unskewing factor

        let s = (x + y) * F2;
        let ix = (x + s).floor() as i32;
        let iy = (y + s).floor() as i32;
        let t = ((ix + iy) as f32) * G2;

        let x0 = x - (ix as f32 - t);
        let y0 = y - (iy as f32 - t);

        let (i1, j1) = if x0 > y0 { (1, 0) } else { (0, 1) };

        let x1 = x0 - i1 as f32 + G2;
        let y1 = y0 - j1 as f32 + G2;
        let x2 = x0 - 1.0 + 2.0 * G2;
        let y2 = y0 - 1.0 + 2.0 * G2;

        let gi0 = self.hash(ix, iy);
        let gi1 = self.hash(ix + i1, iy + j1);
        let gi2 = self.hash(ix + 1, iy + 1);

        let n0 = corner_contrib(x0, y0, gi0);
        let n1 = corner_contrib(x1, y1, gi1);
        let n2 = corner_contrib(x2, y2, gi2);

        70.0 * (n0 + n1 + n2)
    }

    fn gradient(&self, _x: f32, _y: f32) -> Vector2<f32> {
        Vector2::zeros()
    }
}

// 2D gradient table
static GRAD2: [(f32, f32); 12] = [
    (1.0, 1.0),
    (-1.0, 1.0),
    (1.0, -1.0),
    (-1.0, -1.0),
    (1.0, 0.0),
    (-1.0, 0.0),
    (1.0, 0.0),
    (-1.0, 0.0),
    (0.0, 1.0),
    (0.0, -1.0),
    (0.0, 1.0),
    (0.0, -1.0),
];

fn corner_contrib(x: f32, y: f32, gi: usize) -> f32 {
    let t = 0.5 - x * x - y * y;
    if t < 0.0 {
        0.0
    } else {
        let (gx, gy) = GRAD2[gi % 12];
        let t2 = t * t;
        t2 * t2 * (gx * x + gy * y)
    }
}
