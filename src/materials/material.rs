use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::vec::Vec3;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)>;
}
