use rand::Rng;

use crate::utils::rng;
use crate::vec::Point3;

#[derive(Debug)]
pub struct Perlin {
    random_floats: Vec<f32>,

    x_permutation: Vec<usize>,
    y_permutation: Vec<usize>,
    z_permutation: Vec<usize>,
}

impl Perlin {
    #[must_use]
    pub fn new() -> Self {
        const POINT_COUNT: usize = 256;
        let mut rng = rng();

        let mut random_floats = vec![0.; POINT_COUNT];
        for f in &mut random_floats {
            *f = rng.gen();
        }

        Self {
            random_floats,
            x_permutation: Self::permutation(POINT_COUNT),
            y_permutation: Self::permutation(POINT_COUNT),
            z_permutation: Self::permutation(POINT_COUNT),
        }
    }

    #[must_use]
    pub fn noise(&self, p: &Point3) -> f32 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();
        u = u * u * (3. - 2. * u);
        v = v * v * (3. - 2. * v);
        w = w * w * (3. - 2. * w);

        let i = p.x.floor() as isize;
        let j = p.y.floor() as isize;
        let k = p.z.floor() as isize;

        let mut c = [[[0.; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_floats[self.x_permutation
                        [((i + di as isize) & 255) as usize]
                        ^ self.y_permutation[((j + dj as isize) & 255) as usize]
                        ^ self.z_permutation[((k + dk as isize) & 255) as usize]]
                }
            }
        }

        Self::trilinear_interpolation(c, (u, v, w))
    }

    #[must_use]
    pub fn turbulence(&self, p: &Point3) -> f32 {
        const DEPTH: usize = 7;
        let mut acc = 0.;
        let mut temp_p = *p;
        let mut weight = 1.;

        for _ in 0..DEPTH {
            acc += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.;
        }

        acc.abs()
    }

    fn permutation(n: usize) -> Vec<usize> {
        let mut rng = rng();

        let mut perm: Vec<usize> = (0..n).collect();
        for i in (1..n).rev() {
            perm.swap(i, rng.gen_range(0..i));
        }

        perm
    }

    fn trilinear_interpolation(c: [[[f32; 2]; 2]; 2], uvw: (f32, f32, f32)) -> f32 {
        let mut acc = 0.;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    acc += (i as f32).mul_add(uvw.0, (1. - i as f32) * (1. - uvw.0))
                        * (j as f32).mul_add(uvw.1, (1. - j as f32) * (1. - uvw.1))
                        * (k as f32).mul_add(uvw.2, (1. - k as f32) * (1. - uvw.2))
                        * c[i][j][k];
                }
            }
        }

        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation() {
        let size = 256;
        let permutation = Perlin::permutation(size);
        assert_eq!(permutation.len(), size);

        let mut is_in_permutation = vec![false; size];
        for i in 0..size {
            is_in_permutation[permutation[i]] = true;
        }

        // Every value in is_in_permutation should be true
        assert!(is_in_permutation.into_iter().fold(true, |acc, e| acc && e));
    }
}
