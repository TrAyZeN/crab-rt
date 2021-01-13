use crate::aabb::Aabb;
use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;

pub struct Object {
    volume: Box<dyn Hitable + Send + Sync>,
}

impl Object {
    #[inline]
    pub fn new(volume: Box<dyn Hitable + Send + Sync>) -> Object {
        Object { volume }
    }
}

impl Hitable for Object {
    #[inline]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.volume.hit(ray, t_min, t_max)
    }

    #[inline]
    fn bounding_box(&self, time_interval: (f32, f32)) -> Option<Aabb> {
        self.volume.bounding_box(time_interval)
    }
}
