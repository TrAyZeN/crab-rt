use super::{Monochrome, Texture};
use crate::vec::{Point3, Vec3};

#[derive(Debug)]
pub struct Checker {
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl Checker {
    #[inline]
    pub fn new(even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
        Self { even, odd }
    }

    #[inline]
    pub fn from_colors(even: Vec3, odd: Vec3) -> Self {
        Self::new(
            Box::new(Monochrome::new(even)),
            Box::new(Monochrome::new(odd)),
        )
    }
}

impl Texture for Checker {
    fn value(&self, texture_coordinates: (f32, f32), p: &Point3) -> Vec3 {
        if f32::sin(10. * p.x) * f32::sin(10. * p.y) * f32::sin(10. * p.z) < 0. {
            self.odd.value(texture_coordinates, p)
        } else {
            self.even.value(texture_coordinates, p)
        }
    }
}
