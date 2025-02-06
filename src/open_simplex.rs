use nalgebra::Vector2;
use rand::Rng;
use std::f32::consts::PI;

use crate::Noise;

pub struct OpenSimplex {
    scale: f32,
    perm: [u8; 512],
}

impl OpenSimplex {
    pub fn new<R: Rng>(scale: f32, mut rng: R) -> Self {
        // Create and shuffle a 256-element permutation table.
        let mut p = [0u8; 256];
        for (i, v) in p.iter_mut().enumerate() {
            *v = i as u8;
        }
        for i in (1..256).rev() {
            let j = rng.gen_range(0..=i);
            p.swap(i, j);
        }
        // Duplicate to avoid wrapping.
        let mut perm = [0u8; 512];
        for i in 0..512 {
            perm[i] = p[i & 255];
        }
        Self { scale, perm }
    }

    // 4D Simplex noise.
    fn simplex4d(&self, x: f32, y: f32, z: f32, w: f32) -> f32 {
        let F4 = (5.0_f32.sqrt() - 1.0) / 4.0;
        let G4 = (5.0 - 5.0_f32.sqrt()) / 20.0;
        let s = (x + y + z + w) * F4;
        let i = (x + s).floor() as i32;
        let j = (y + s).floor() as i32;
        let k = (z + s).floor() as i32;
        let l = (w + s).floor() as i32;
        let t = (i + j + k + l) as f32 * G4;
        let x0 = x - (i as f32 - t);
        let y0 = y - (j as f32 - t);
        let z0 = z - (k as f32 - t);
        let w0 = w - (l as f32 - t);

        // Determine simplex ordering via ranking.
        let mut rank = [0; 4];
        if x0 > y0 {
            rank[0] += 1
        } else {
            rank[1] += 1;
        }
        if x0 > z0 {
            rank[0] += 1
        } else {
            rank[2] += 1;
        }
        if x0 > w0 {
            rank[0] += 1
        } else {
            rank[3] += 1;
        }
        if y0 > z0 {
            rank[1] += 1
        } else {
            rank[2] += 1;
        }
        if y0 > w0 {
            rank[1] += 1
        } else {
            rank[3] += 1;
        }
        if z0 > w0 {
            rank[2] += 1
        } else {
            rank[3] += 1;
        }

        let i1 = if rank[0] >= 3 { 1 } else { 0 };
        let j1 = if rank[1] >= 3 { 1 } else { 0 };
        let k1 = if rank[2] >= 3 { 1 } else { 0 };
        let l1 = if rank[3] >= 3 { 1 } else { 0 };

        let i2 = if rank[0] >= 2 { 1 } else { 0 };
        let j2 = if rank[1] >= 2 { 1 } else { 0 };
        let k2 = if rank[2] >= 2 { 1 } else { 0 };
        let l2 = if rank[3] >= 2 { 1 } else { 0 };

        let i3 = if rank[0] >= 1 { 1 } else { 0 };
        let j3 = if rank[1] >= 1 { 1 } else { 0 };
        let k3 = if rank[2] >= 1 { 1 } else { 0 };
        let l3 = if rank[3] >= 1 { 1 } else { 0 };

        let x1 = x0 - i1 as f32 + G4;
        let y1 = y0 - j1 as f32 + G4;
        let z1 = z0 - k1 as f32 + G4;
        let w1 = w0 - l1 as f32 + G4;
        let x2 = x0 - i2 as f32 + 2.0 * G4;
        let y2 = y0 - j2 as f32 + 2.0 * G4;
        let z2 = z0 - k2 as f32 + 2.0 * G4;
        let w2 = w0 - l2 as f32 + 2.0 * G4;
        let x3 = x0 - i3 as f32 + 3.0 * G4;
        let y3 = y0 - j3 as f32 + 3.0 * G4;
        let z3 = z0 - k3 as f32 + 3.0 * G4;
        let w3 = w0 - l3 as f32 + 3.0 * G4;
        let x4 = x0 - 1.0 + 4.0 * G4;
        let y4 = y0 - 1.0 + 4.0 * G4;
        let z4 = z0 - 1.0 + 4.0 * G4;
        let w4 = w0 - 1.0 + 4.0 * G4;

        let mut n0 = 0.0;
        let mut n1 = 0.0;
        let mut n2 = 0.0;
        let mut n3 = 0.0;
        let mut n4 = 0.0;

        let t0 = 0.6 - x0 * x0 - y0 * y0 - z0 * z0 - w0 * w0;
        if t0 > 0.0 {
            let t0_sq = t0 * t0;
            n0 = t0_sq * t0_sq * self.dot4(self.hash4(i, j, k, l), x0, y0, z0, w0);
        }
        let t1 = 0.6 - x1 * x1 - y1 * y1 - z1 * z1 - w1 * w1;
        if t1 > 0.0 {
            let t1_sq = t1 * t1;
            n1 = t1_sq
                * t1_sq
                * self.dot4(self.hash4(i + i1, j + j1, k + k1, l + l1), x1, y1, z1, w1);
        }
        let t2 = 0.6 - x2 * x2 - y2 * y2 - z2 * z2 - w2 * w2;
        if t2 > 0.0 {
            let t2_sq = t2 * t2;
            n2 = t2_sq
                * t2_sq
                * self.dot4(self.hash4(i + i2, j + j2, k + k2, l + l2), x2, y2, z2, w2);
        }
        let t3 = 0.6 - x3 * x3 - y3 * y3 - z3 * z3 - w3 * w3;
        if t3 > 0.0 {
            let t3_sq = t3 * t3;
            n3 = t3_sq
                * t3_sq
                * self.dot4(self.hash4(i + i3, j + j3, k + k3, l + l3), x3, y3, z3, w3);
        }
        let t4 = 0.6 - x4 * x4 - y4 * y4 - z4 * z4 - w4 * w4;
        if t4 > 0.0 {
            let t4_sq = t4 * t4;
            n4 = t4_sq * t4_sq * self.dot4(self.hash4(i + 1, j + 1, k + 1, l + 1), x4, y4, z4, w4);
        }
        27.0 * (n0 + n1 + n2 + n3 + n4)
    }

    // 4D Simplex noise with analytic gradient.
    fn simplex4d_with_grad(&self, x: f32, y: f32, z: f32, w: f32) -> (f32, (f32, f32, f32, f32)) {
        let F4 = (5.0_f32.sqrt() - 1.0) / 4.0;
        let G4 = (5.0 - 5.0_f32.sqrt()) / 20.0;
        let s = (x + y + z + w) * F4;
        let i = (x + s).floor() as i32;
        let j = (y + s).floor() as i32;
        let k = (z + s).floor() as i32;
        let l = (w + s).floor() as i32;
        let t = (i + j + k + l) as f32 * G4;
        let X0 = i as f32 - t;
        let Y0 = j as f32 - t;
        let Z0 = k as f32 - t;
        let W0 = l as f32 - t;
        let x0 = x - X0;
        let y0 = y - Y0;
        let z0 = z - Z0;
        let w0 = w - W0;

        let mut rank = [0; 4];
        if x0 > y0 {
            rank[0] += 1
        } else {
            rank[1] += 1;
        }
        if x0 > z0 {
            rank[0] += 1
        } else {
            rank[2] += 1;
        }
        if x0 > w0 {
            rank[0] += 1
        } else {
            rank[3] += 1;
        }
        if y0 > z0 {
            rank[1] += 1
        } else {
            rank[2] += 1;
        }
        if y0 > w0 {
            rank[1] += 1
        } else {
            rank[3] += 1;
        }
        if z0 > w0 {
            rank[2] += 1
        } else {
            rank[3] += 1;
        }

        let i1 = if rank[0] >= 3 { 1 } else { 0 };
        let j1 = if rank[1] >= 3 { 1 } else { 0 };
        let k1 = if rank[2] >= 3 { 1 } else { 0 };
        let l1 = if rank[3] >= 3 { 1 } else { 0 };

        let i2 = if rank[0] >= 2 { 1 } else { 0 };
        let j2 = if rank[1] >= 2 { 1 } else { 0 };
        let k2 = if rank[2] >= 2 { 1 } else { 0 };
        let l2 = if rank[3] >= 2 { 1 } else { 0 };

        let i3 = if rank[0] >= 1 { 1 } else { 0 };
        let j3 = if rank[1] >= 1 { 1 } else { 0 };
        let k3 = if rank[2] >= 1 { 1 } else { 0 };
        let l3 = if rank[3] >= 1 { 1 } else { 0 };

        let x1 = x0 - i1 as f32 + G4;
        let y1 = y0 - j1 as f32 + G4;
        let z1 = z0 - k1 as f32 + G4;
        let w1 = w0 - l1 as f32 + G4;
        let x2 = x0 - i2 as f32 + 2.0 * G4;
        let y2 = y0 - j2 as f32 + 2.0 * G4;
        let z2 = z0 - k2 as f32 + 2.0 * G4;
        let w2 = w0 - l2 as f32 + 2.0 * G4;
        let x3 = x0 - i3 as f32 + 3.0 * G4;
        let y3 = y0 - j3 as f32 + 3.0 * G4;
        let z3 = z0 - k3 as f32 + 3.0 * G4;
        let w3 = w0 - l3 as f32 + 3.0 * G4;
        let x4 = x0 - 1.0 + 4.0 * G4;
        let y4 = y0 - 1.0 + 4.0 * G4;
        let z4 = z0 - 1.0 + 4.0 * G4;
        let w4 = w0 - 1.0 + 4.0 * G4;

        let mut noise = 0.0;
        let mut dnoise_dx = 0.0;
        let mut dnoise_dy = 0.0;
        let mut dnoise_dz = 0.0;
        let mut dnoise_dw = 0.0;

        // Helper: process one simplex corner.
        let mut process_corner =
            |xi: f32, yi: f32, zi: f32, wi: f32, ii: i32, jj: i32, kk: i32, ll: i32| {
                let t = 0.6 - xi * xi - yi * yi - zi * zi - wi * wi;
                if t > 0.0 {
                    let t2 = t * t;
                    let t4 = t2 * t2;
                    let gi = self.hash4(ii, jj, kk, ll);
                    const GRAD4: [(f32, f32, f32, f32); 32] = [
                        (0.0, 1.0, 1.0, 1.0),
                        (0.0, 1.0, 1.0, -1.0),
                        (0.0, 1.0, -1.0, 1.0),
                        (0.0, 1.0, -1.0, -1.0),
                        (0.0, -1.0, 1.0, 1.0),
                        (0.0, -1.0, 1.0, -1.0),
                        (0.0, -1.0, -1.0, 1.0),
                        (0.0, -1.0, -1.0, -1.0),
                        (1.0, 0.0, 1.0, 1.0),
                        (1.0, 0.0, 1.0, -1.0),
                        (1.0, 0.0, -1.0, 1.0),
                        (1.0, 0.0, -1.0, -1.0),
                        (-1.0, 0.0, 1.0, 1.0),
                        (-1.0, 0.0, 1.0, -1.0),
                        (-1.0, 0.0, -1.0, 1.0),
                        (-1.0, 0.0, -1.0, -1.0),
                        (1.0, 1.0, 0.0, 1.0),
                        (1.0, 1.0, 0.0, -1.0),
                        (1.0, -1.0, 0.0, 1.0),
                        (1.0, -1.0, 0.0, -1.0),
                        (-1.0, 1.0, 0.0, 1.0),
                        (-1.0, 1.0, 0.0, -1.0),
                        (-1.0, -1.0, 0.0, 1.0),
                        (-1.0, -1.0, 0.0, -1.0),
                        (1.0, 1.0, 1.0, 0.0),
                        (1.0, 1.0, -1.0, 0.0),
                        (1.0, -1.0, 1.0, 0.0),
                        (1.0, -1.0, -1.0, 0.0),
                        (-1.0, 1.0, 1.0, 0.0),
                        (-1.0, 1.0, -1.0, 0.0),
                        (-1.0, -1.0, 1.0, 0.0),
                        (-1.0, -1.0, -1.0, 0.0),
                    ];
                    let grad = GRAD4[gi as usize % 32];
                    let dot = grad.0 * xi + grad.1 * yi + grad.2 * zi + grad.3 * wi;
                    noise += t4 * dot;
                    let common = -8.0 * t2 * t * dot;
                    dnoise_dx += common * xi + t4 * grad.0;
                    dnoise_dy += common * yi + t4 * grad.1;
                    dnoise_dz += common * zi + t4 * grad.2;
                    dnoise_dw += common * wi + t4 * grad.3;
                }
            };

        process_corner(x0, y0, z0, w0, i, j, k, l);
        process_corner(x1, y1, z1, w1, i + i1, j + j1, k + k1, l + l1);
        process_corner(x2, y2, z2, w2, i + i2, j + j2, k + k2, l + l2);
        process_corner(x3, y3, z3, w3, i + i3, j + j3, k + k3, l + l3);
        process_corner(x4, y4, z4, w4, i + 1, j + 1, k + 1, l + 1);

        (
            27.0 * noise,
            (
                27.0 * dnoise_dx,
                27.0 * dnoise_dy,
                27.0 * dnoise_dz,
                27.0 * dnoise_dw,
            ),
        )
    }

    // Dot product for 4D using a fixed gradient table.
    fn dot4(&self, gi: usize, x: f32, y: f32, z: f32, w: f32) -> f32 {
        const GRAD4: [(f32, f32, f32, f32); 32] = [
            (0.0, 1.0, 1.0, 1.0),
            (0.0, 1.0, 1.0, -1.0),
            (0.0, 1.0, -1.0, 1.0),
            (0.0, 1.0, -1.0, -1.0),
            (0.0, -1.0, 1.0, 1.0),
            (0.0, -1.0, 1.0, -1.0),
            (0.0, -1.0, -1.0, 1.0),
            (0.0, -1.0, -1.0, -1.0),
            (1.0, 0.0, 1.0, 1.0),
            (1.0, 0.0, 1.0, -1.0),
            (1.0, 0.0, -1.0, 1.0),
            (1.0, 0.0, -1.0, -1.0),
            (-1.0, 0.0, 1.0, 1.0),
            (-1.0, 0.0, 1.0, -1.0),
            (-1.0, 0.0, -1.0, 1.0),
            (-1.0, 0.0, -1.0, -1.0),
            (1.0, 1.0, 0.0, 1.0),
            (1.0, 1.0, 0.0, -1.0),
            (1.0, -1.0, 0.0, 1.0),
            (1.0, -1.0, 0.0, -1.0),
            (-1.0, 1.0, 0.0, 1.0),
            (-1.0, 1.0, 0.0, -1.0),
            (-1.0, -1.0, 0.0, 1.0),
            (-1.0, -1.0, 0.0, -1.0),
            (1.0, 1.0, 1.0, 0.0),
            (1.0, 1.0, -1.0, 0.0),
            (1.0, -1.0, 1.0, 0.0),
            (1.0, -1.0, -1.0, 0.0),
            (-1.0, 1.0, 1.0, 0.0),
            (-1.0, 1.0, -1.0, 0.0),
            (-1.0, -1.0, 1.0, 0.0),
            (-1.0, -1.0, -1.0, 0.0),
        ];
        let g = GRAD4[gi % 32];
        g.0 * x + g.1 * y + g.2 * z + g.3 * w
    }

    // 4D hash function using the permutation table.
    fn hash4(&self, i: i32, j: i32, k: i32, l: i32) -> usize {
        let idx = self.perm[(i & 255) as usize] as usize;
        let idx = self.perm[(idx + (j & 255) as usize) & 511] as usize;
        let idx = self.perm[(idx + (k & 255) as usize) & 511] as usize;
        self.perm[(idx + (l & 255) as usize) & 511] as usize
    }
}

impl Noise for OpenSimplex {
    fn sample(&self, u: f32, v: f32) -> f32 {
        // Map (u,v) ∈ [0,1] onto two circles (a torus) for tiling.
        let angle_u = 2.0 * PI * u;
        let angle_v = 2.0 * PI * v;
        let r = 1.0;
        let a = r * angle_u.sin();
        let b = r * angle_u.cos();
        let c = r * angle_v.sin();
        let d = r * angle_v.cos();
        self.simplex4d(
            123.0 + a * self.scale,
            231.0 + b * self.scale,
            312.0 + c * self.scale,
            273.0 + d * self.scale,
        )
    }

    fn gradient(&self, u: f32, v: f32) -> Vector2<f32> {
        // Map (u,v) as above.
        let angle_u = 2.0 * PI * u;
        let angle_v = 2.0 * PI * v;
        let r = 1.0;
        let a = r * angle_u.sin();
        let b = r * angle_u.cos();
        let c = r * angle_v.sin();
        let d = r * angle_v.cos();
        let ox = 123.0;
        let oy = 231.0;
        let oz = 312.0;
        let ow = 273.0;
        let x = ox + a * self.scale;
        let y = oy + b * self.scale;
        let z = oz + c * self.scale;
        let w = ow + d * self.scale;
        let (_noise, grad4) = self.simplex4d_with_grad(x, y, z, w);
        // Derivatives of the mapping (u,v)→(x,y,z,w):
        let dangle = 2.0 * PI;
        let dx_du = self.scale * r * dangle * angle_u.cos();
        let dy_du = -self.scale * r * dangle * angle_u.sin();
        let dz_dv = self.scale * r * dangle * angle_v.cos();
        let dw_dv = -self.scale * r * dangle * angle_v.sin();
        let dnoise_du = grad4.0 * dx_du + grad4.1 * dy_du;
        let dnoise_dv = grad4.2 * dz_dv + grad4.3 * dw_dv;
        Vector2::new(dnoise_du, dnoise_dv)
    }
}
