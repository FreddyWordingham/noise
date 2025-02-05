use nalgebra::Vector2;
use rand::Rng;

use crate::noise::Noise;

pub struct Simplex {
    scale: f32,
    perm: [u8; 512], // Permutation table repeated twice
}

impl Simplex {
    pub fn new<R: Rng>(scale: f32, mut rng: R) -> Self {
        // Generate a random permutation of 0..255
        let mut p = [0u8; 256];
        for (i, val) in p.iter_mut().enumerate() {
            *val = i as u8;
        }
        // Shuffle the permutation
        for i in (1..256).rev() {
            let j = rng.gen_range(0..=i);
            p.swap(i, j);
        }

        // Duplicate it to avoid having to wrap indices
        let mut perm = [0u8; 512];
        for i in 0..512 {
            perm[i] = p[i & 255];
        }

        Simplex { scale, perm }
    }

    // Hash corner coords -> gradient index
    fn hash(&self, x: i32, y: i32) -> usize {
        let idx = self.perm[(x & 255) as usize] as usize;
        self.perm[(idx + (y & 255) as usize) & 511] as usize
    }
}

impl Noise for Simplex {
    fn sample(&self, mut x: f32, mut y: f32) -> f32 {
        x *= self.scale;
        y *= self.scale;

        // Skewing/Unskewing factors for 2D
        const F2: f32 = 0.3660254037844386; // 0.5 * (sqrt(3.0) - 1.0)
        const G2: f32 = 0.21132486540518713; // (3.0 - sqrt(3.0)) / 6.0

        // Skew input space to determine which simplex cell we’re in
        let s = (x + y) * F2;
        let ix = (x + s).floor() as i32;
        let iy = (y + s).floor() as i32;

        // Unskew back
        let t = ((ix + iy) as f32) * G2;
        let x0 = x - (ix as f32 - t);
        let y0 = y - (iy as f32 - t);

        // This determines which triangle we are in
        let (i1, j1) = if x0 > y0 { (1, 0) } else { (0, 1) };

        // Offsets for middle corner
        let x1 = x0 - i1 as f32 + G2;
        let y1 = y0 - j1 as f32 + G2;
        // Offsets for last corner
        let x2 = x0 - 1.0 + 2.0 * G2;
        let y2 = y0 - 1.0 + 2.0 * G2;

        // Calculate the hashed gradient indices of the three corners
        let gi0 = self.hash(ix, iy);
        let gi1 = self.hash(ix + i1, iy + j1);
        let gi2 = self.hash(ix + 1, iy + 1);

        // Calculate the contribution from the three corners
        let n0 = corner_contrib(x0, y0, gi0);
        let n1 = corner_contrib(x1, y1, gi1);
        let n2 = corner_contrib(x2, y2, gi2);

        // Scale the result
        70.0 * (n0 + n1 + n2)
    }

    fn gradient(&self, mut x: f32, mut y: f32) -> Vector2<f32> {
        x *= self.scale;
        y *= self.scale;

        const F2: f32 = 0.3660254037844386;
        const G2: f32 = 0.21132486540518713;

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

        let (dx0, dy0) = gradient_contrib(x0, y0, gi0);
        let (dx1, dy1) = gradient_contrib(x1, y1, gi1);
        let (dx2, dy2) = gradient_contrib(x2, y2, gi2);

        let dx = 70.0 * (dx0 + dx1 + dx2);
        let dy = 70.0 * (dy0 + dy1 + dy2);

        Vector2::new(dx, dy)
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

// Contribution from a corner
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

// Computes gradient contribution for a corner
fn gradient_contrib(x: f32, y: f32, gi: usize) -> (f32, f32) {
    let t = 0.5 - x * x - y * y;
    if t < 0.0 {
        (0.0, 0.0)
    } else {
        let (gx, gy) = GRAD2[gi % 12];
        let t2 = t * t;
        let t3 = t2 * t;

        let common = -8.0 * t3 * (gx * x + gy * y);
        let dx = common * x + t2 * t2 * gx;
        let dy = common * y + t2 * t2 * gy;

        (dx, dy)
    }
}
