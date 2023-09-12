use super::Texture;
use crate::vec::{Color3, Point3, Vec3};

#[derive(Debug)]
pub struct Monochrome {
    color: Color3,
}

impl Monochrome {
    #[inline]
    #[must_use]
    pub const fn new(color: Color3) -> Self {
        Self { color }
    }

    #[inline]
    #[must_use]
    pub fn from_rgb(red: f32, green: f32, blue: f32) -> Self {
        Self::new(Color3::new(red, green, blue))
    }
}

impl Texture for Monochrome {
    #[inline]
    fn value(&self, _texture_coordinates: (f32, f32), _p: &Point3) -> Vec3 {
        self.color
    }
}
