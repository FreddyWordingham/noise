use nalgebra::Vector2;
use rand::Rng;

use crate::noise::Noise;

const SKEW_FACTOR: f32 = 0.3660254037844386; // 0.5 * (sqrt(3) - 1)
const UNSKEW_FACTOR: f32 = 0.21132486540518713; // (3 - sqrt(3)) / 6

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

        // Skew input space to determine which simplex cell we’re in
        let s = (x + y) * SKEW_FACTOR;
        let ix = (x + s).floor() as i32;
        let iy = (y + s).floor() as i32;

        // Unskew back
        let t = ((ix + iy) as f32) * UNSKEW_FACTOR;
        let x0 = x - (ix as f32 - t);
        let y0 = y - (iy as f32 - t);

        // This determines which triangle we are in
        let (i1, j1) = if x0 > y0 { (1, 0) } else { (0, 1) };

        // Offsets for middle corner
        let x1 = x0 - i1 as f32 + UNSKEW_FACTOR;
        let y1 = y0 - j1 as f32 + UNSKEW_FACTOR;
        // Offsets for last corner
        let x2 = x0 - 1.0 + 2.0 * UNSKEW_FACTOR;
        let y2 = y0 - 1.0 + 2.0 * UNSKEW_FACTOR;

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
        // Scale the inputs.
        x *= self.scale;
        y *= self.scale;

        // Skew to determine simplex cell.
        let s = (x + y) * SKEW_FACTOR;
        let ix = (x + s).floor() as i32;
        let iy = (y + s).floor() as i32;

        // Unskew to get offsets.
        let t = ((ix + iy) as f32) * UNSKEW_FACTOR;
        let x0 = x - (ix as f32 - t);
        let y0 = y - (iy as f32 - t);

        // Determine which triangle.
        let (i1, j1) = if x0 > y0 { (1, 0) } else { (0, 1) };

        // Offsets for the three corners.
        let x1 = x0 - i1 as f32 + UNSKEW_FACTOR;
        let y1 = y0 - j1 as f32 + UNSKEW_FACTOR;
        let x2 = x0 - 1.0 + 2.0 * UNSKEW_FACTOR;
        let y2 = y0 - 1.0 + 2.0 * UNSKEW_FACTOR;

        // Get gradient indices.
        let gi0 = self.hash(ix, iy);
        let gi1 = self.hash(ix + i1, iy + j1);
        let gi2 = self.hash(ix + 1, iy + 1);

        // Compute contributions and gradients for each corner.
        let (_n0, dx0, dy0) = corner_contrib_and_grad(x0, y0, gi0);
        let (_n1, dx1, dy1) = corner_contrib_and_grad(x1, y1, gi1);
        let (_n2, dx2, dy2) = corner_contrib_and_grad(x2, y2, gi2);

        // Sum up derivatives. The sample function multiplies the sum by 70.
        let dnoise_dx = 70.0 * (dx0 + dx1 + dx2);
        let dnoise_dy = 70.0 * (dy0 + dy1 + dy2);

        // Chain rule: f(x) = g(scale*x), so df/dx = scale * g'(scale*x)
        Vector2::new(dnoise_dx * self.scale, dnoise_dy * self.scale)
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

// Helper function: computes both contribution and its gradient for a corner.
// For a given offset (dx, dy) and gradient index gi, let:
//   t = 0.5 - dx^2 - dy^2,
//   contribution = t^4 * (gx * dx + gy * dy)
// Then its partial derivatives (with respect to dx and dy) are:
fn corner_contrib_and_grad(x: f32, y: f32, gi: usize) -> (f32, f32, f32) {
    let t = 0.5 - x * x - y * y;
    if t < 0.0 {
        (0.0, 0.0, 0.0)
    } else {
        let (gx, gy) = GRAD2[gi % 12];
        let t2 = t * t;
        let t4 = t2 * t2;
        let dot = gx * x + gy * y;
        let contrib = t4 * dot;
        // d/dx:  d(contrib)/dx = 4*t^3*(dt/dx)*dot + t4*gx, where dt/dx = -2*x.
        let dcontrib_dx = -8.0 * x * t * t2 * dot + t4 * gx;
        let dcontrib_dy = -8.0 * y * t * t2 * dot + t4 * gy;
        (contrib, dcontrib_dx, dcontrib_dy)
    }
}
