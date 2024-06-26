use super::Texture;
use crate::perlin::Perlin;
use crate::vec::{Point3, Vec3};

#[cfg(not(feature = "std"))]
use core_maths::*;

#[derive(Debug)]
pub struct Noise {
    noise: Perlin,
    scale: f32,
}

impl Noise {
    #[inline]
    #[must_use]
    pub fn new(scale: f32) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for Noise {
    #[inline]
    fn value(&self, _texture_coordinates: (f32, f32), p: &Point3) -> Vec3 {
        Vec3::new(1., 1., 1.)
            * 0.5
            * (1. + f32::sin(self.scale.mul_add(p.z, 10. * self.noise.turbulence(p))))
    }
}
