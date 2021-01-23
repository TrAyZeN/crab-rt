use std::fmt::Debug;

use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

pub trait Material: Debug + Send + Sync {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)>;

    #[allow(unused_variables)]
    fn emitted(&self, texture_coordinates: (f32, f32), p: &Point3) -> Vec3 {
        Vec3::new(0., 0., 0.)
    }
}
