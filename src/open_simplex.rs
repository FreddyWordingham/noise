use nalgebra::Vector2;
use rand::Rng;
use std::f32::consts::PI;

use crate::Noise;

const SKEW_FACTOR: f32 = 0.3090169943749474; // (sqrt(5) - 1) / 4;
const UNSKEW_FACTOR: f32 = 0.1381966011250105; // (5 - sqrt(5)) / 20;

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
        let s = (x + y + z + w) * SKEW_FACTOR;
        let i = (x + s).floor() as i32;
        let j = (y + s).floor() as i32;
        let k = (z + s).floor() as i32;
        let l = (w + s).floor() as i32;
        let t = (i + j + k + l) as f32 * UNSKEW_FACTOR;
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

        let x1 = x0 - i1 as f32 + UNSKEW_FACTOR;
        let y1 = y0 - j1 as f32 + UNSKEW_FACTOR;
        let z1 = z0 - k1 as f32 + UNSKEW_FACTOR;
        let w1 = w0 - l1 as f32 + UNSKEW_FACTOR;
        let x2 = x0 - i2 as f32 + 2.0 * UNSKEW_FACTOR;
        let y2 = y0 - j2 as f32 + 2.0 * UNSKEW_FACTOR;
        let z2 = z0 - k2 as f32 + 2.0 * UNSKEW_FACTOR;
        let w2 = w0 - l2 as f32 + 2.0 * UNSKEW_FACTOR;
        let x3 = x0 - i3 as f32 + 3.0 * UNSKEW_FACTOR;
        let y3 = y0 - j3 as f32 + 3.0 * UNSKEW_FACTOR;
        let z3 = z0 - k3 as f32 + 3.0 * UNSKEW_FACTOR;
        let w3 = w0 - l3 as f32 + 3.0 * UNSKEW_FACTOR;
        let x4 = x0 - 1.0 + 4.0 * UNSKEW_FACTOR;
        let y4 = y0 - 1.0 + 4.0 * UNSKEW_FACTOR;
        let z4 = z0 - 1.0 + 4.0 * UNSKEW_FACTOR;
        let w4 = w0 - 1.0 + 4.0 * UNSKEW_FACTOR;

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
        let skew_factor_4d = (5.0_f32.sqrt() - 1.0) / 4.0;
        let unskew_factor_4d = (5.0 - 5.0_f32.sqrt()) / 20.0;
        let skew_sum = (x + y + z + w) * skew_factor_4d;

        let cell_x = (x + skew_sum).floor() as i32;
        let cell_y = (y + skew_sum).floor() as i32;
        let cell_z = (z + skew_sum).floor() as i32;
        let cell_w = (w + skew_sum).floor() as i32;

        let unskew_offset = (cell_x + cell_y + cell_z + cell_w) as f32 * unskew_factor_4d;
        let origin_x = cell_x as f32 - unskew_offset;
        let origin_y = cell_y as f32 - unskew_offset;
        let origin_z = cell_z as f32 - unskew_offset;
        let origin_w = cell_w as f32 - unskew_offset;

        let local_x0 = x - origin_x;
        let local_y0 = y - origin_y;
        let local_z0 = z - origin_z;
        let local_w0 = w - origin_w;

        let mut component_rank = [0; 4];
        if local_x0 > local_y0 {
            component_rank[0] += 1;
        } else {
            component_rank[1] += 1;
        }
        if local_x0 > local_z0 {
            component_rank[0] += 1;
        } else {
            component_rank[2] += 1;
        }
        if local_x0 > local_w0 {
            component_rank[0] += 1;
        } else {
            component_rank[3] += 1;
        }
        if local_y0 > local_z0 {
            component_rank[1] += 1;
        } else {
            component_rank[2] += 1;
        }
        if local_y0 > local_w0 {
            component_rank[1] += 1;
        } else {
            component_rank[3] += 1;
        }
        if local_z0 > local_w0 {
            component_rank[2] += 1;
        } else {
            component_rank[3] += 1;
        }

        let offset1_x = if component_rank[0] >= 3 { 1 } else { 0 };
        let offset1_y = if component_rank[1] >= 3 { 1 } else { 0 };
        let offset1_z = if component_rank[2] >= 3 { 1 } else { 0 };
        let offset1_w = if component_rank[3] >= 3 { 1 } else { 0 };

        let offset2_x = if component_rank[0] >= 2 { 1 } else { 0 };
        let offset2_y = if component_rank[1] >= 2 { 1 } else { 0 };
        let offset2_z = if component_rank[2] >= 2 { 1 } else { 0 };
        let offset2_w = if component_rank[3] >= 2 { 1 } else { 0 };

        let offset3_x = if component_rank[0] >= 1 { 1 } else { 0 };
        let offset3_y = if component_rank[1] >= 1 { 1 } else { 0 };
        let offset3_z = if component_rank[2] >= 1 { 1 } else { 0 };
        let offset3_w = if component_rank[3] >= 1 { 1 } else { 0 };

        let simplex_corner1_x = local_x0 - offset1_x as f32 + unskew_factor_4d;
        let simplex_corner1_y = local_y0 - offset1_y as f32 + unskew_factor_4d;
        let simplex_corner1_z = local_z0 - offset1_z as f32 + unskew_factor_4d;
        let simplex_corner1_w = local_w0 - offset1_w as f32 + unskew_factor_4d;

        let simplex_corner2_x = local_x0 - offset2_x as f32 + 2.0 * unskew_factor_4d;
        let simplex_corner2_y = local_y0 - offset2_y as f32 + 2.0 * unskew_factor_4d;
        let simplex_corner2_z = local_z0 - offset2_z as f32 + 2.0 * unskew_factor_4d;
        let simplex_corner2_w = local_w0 - offset2_w as f32 + 2.0 * unskew_factor_4d;

        let simplex_corner3_x = local_x0 - offset3_x as f32 + 3.0 * unskew_factor_4d;
        let simplex_corner3_y = local_y0 - offset3_y as f32 + 3.0 * unskew_factor_4d;
        let simplex_corner3_z = local_z0 - offset3_z as f32 + 3.0 * unskew_factor_4d;
        let simplex_corner3_w = local_w0 - offset3_w as f32 + 3.0 * unskew_factor_4d;

        let simplex_corner4_x = local_x0 - 1.0 + 4.0 * unskew_factor_4d;
        let simplex_corner4_y = local_y0 - 1.0 + 4.0 * unskew_factor_4d;
        let simplex_corner4_z = local_z0 - 1.0 + 4.0 * unskew_factor_4d;
        let simplex_corner4_w = local_w0 - 1.0 + 4.0 * unskew_factor_4d;

        let mut noise_value = 0.0;
        let mut noise_deriv_x = 0.0;
        let mut noise_deriv_y = 0.0;
        let mut noise_deriv_z = 0.0;
        let mut noise_deriv_w = 0.0;

        // Process a simplex corner contribution.
        let mut process_simplex_corner =
            |dx: f32,
             dy: f32,
             dz: f32,
             dw: f32,
             offset_cell_x: i32,
             offset_cell_y: i32,
             offset_cell_z: i32,
             offset_cell_w: i32| {
                let attenuation = 0.6 - dx * dx - dy * dy - dz * dz - dw * dw;
                if attenuation > 0.0 {
                    let attenuation2 = attenuation * attenuation;
                    let attenuation4 = attenuation2 * attenuation2;
                    let grad_index = self.hash4(
                        cell_x + offset_cell_x,
                        cell_y + offset_cell_y,
                        cell_z + offset_cell_z,
                        cell_w + offset_cell_w,
                    );
                    const GRADIENTS_4D: [(f32, f32, f32, f32); 32] = [
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
                    let gradient = GRADIENTS_4D[grad_index as usize % 32];
                    let dot_product =
                        gradient.0 * dx + gradient.1 * dy + gradient.2 * dz + gradient.3 * dw;
                    noise_value += attenuation4 * dot_product;
                    let common_term = -8.0 * attenuation2 * attenuation * dot_product;
                    noise_deriv_x += common_term * dx + attenuation4 * gradient.0;
                    noise_deriv_y += common_term * dy + attenuation4 * gradient.1;
                    noise_deriv_z += common_term * dz + attenuation4 * gradient.2;
                    noise_deriv_w += common_term * dw + attenuation4 * gradient.3;
                }
            };

        process_simplex_corner(local_x0, local_y0, local_z0, local_w0, 0, 0, 0, 0);
        process_simplex_corner(
            simplex_corner1_x,
            simplex_corner1_y,
            simplex_corner1_z,
            simplex_corner1_w,
            offset1_x,
            offset1_y,
            offset1_z,
            offset1_w,
        );
        process_simplex_corner(
            simplex_corner2_x,
            simplex_corner2_y,
            simplex_corner2_z,
            simplex_corner2_w,
            offset2_x,
            offset2_y,
            offset2_z,
            offset2_w,
        );
        process_simplex_corner(
            simplex_corner3_x,
            simplex_corner3_y,
            simplex_corner3_z,
            simplex_corner3_w,
            offset3_x,
            offset3_y,
            offset3_z,
            offset3_w,
        );
        process_simplex_corner(
            simplex_corner4_x,
            simplex_corner4_y,
            simplex_corner4_z,
            simplex_corner4_w,
            1,
            1,
            1,
            1,
        );

        (
            27.0 * noise_value,
            (
                27.0 * noise_deriv_x,
                27.0 * noise_deriv_y,
                27.0 * noise_deriv_z,
                27.0 * noise_deriv_w,
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
