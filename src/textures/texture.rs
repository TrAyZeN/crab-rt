use core::fmt::Debug;

use crate::hitable::HitRecord;
use crate::vec::{Point3, Vec3};

pub trait Texture: Debug + Send + Sync {
    #[must_use]
    fn value(&self, texture_coordinates: (f32, f32), p: &Point3) -> Vec3;

    #[inline(always)]
    #[must_use]
    fn value_from_hit(&self, record: &HitRecord<'_>) -> Vec3 {
        self.value(record.texture_coordinates(), record.hit_point())
    }
}
