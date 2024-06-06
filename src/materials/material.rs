use core::fmt::Debug;

use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

pub trait Material: Debug + Send + Sync {
    #[must_use]
    fn scatter(&self, ray: &Ray, record: &HitRecord<'_>) -> Option<(Ray, Vec3)>;

    #[allow(unused_variables)]
    #[must_use]
    fn emitted(&self, texture_coordinates: (f32, f32), p: &Point3) -> Vec3 {
        Vec3::new(0., 0., 0.)
    }
}
