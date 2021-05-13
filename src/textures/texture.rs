use std::fmt::Debug;

use crate::vec::{Point3, Vec3};
use crate::hitable::HitRecord;

pub trait Texture: Debug + Send + Sync {
    #[must_use]
    fn value(&self, texture_coordinates: (f32, f32), p: &Point3) -> Vec3;

    #[inline(always)]
    #[must_use]
    fn value_from_hit(&self, record: &HitRecord<'_>) -> Vec3 {
        self.value(record.get_texture_coordinates(), record.get_hit_point())
    }
}
