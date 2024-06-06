use alloc::boxed::Box;

use super::{Monochrome, Texture};
use crate::vec::{Color3, Point3, Vec3};

#[cfg(not(feature = "std"))]
use core_maths::*;

#[derive(Debug)]
pub struct Checker {
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl Checker {
    #[inline]
    #[must_use]
    pub fn new<T1, T2>(even: T1, odd: T2) -> Self
    where
        T1: 'static + Texture,
        T2: 'static + Texture,
    {
        Self {
            even: Box::new(even),
            odd: Box::new(odd),
        }
    }

    #[inline]
    #[must_use]
    pub fn from_colors(even: Color3, odd: Color3) -> Self {
        Self::new(Monochrome::new(even), Monochrome::new(odd))
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
