use std::fmt::Debug;

use crate::vec::{Point3, Vec3};

pub trait Texture: Debug + Send + Sync {
    #[must_use]
    fn value(&self, texture_coordinates: (f32, f32), p: &Point3) -> Vec3;
}
