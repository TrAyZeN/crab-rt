use std::fmt::Debug;

use crate::aabb::Aabb;
use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;

#[derive(Debug)]
pub struct Object {
    volume: Box<dyn Hitable>,
    bbox: Option<Aabb>,
}

impl Object {
    /// Constructs a new object.
    #[inline]
    pub fn new(volume: Box<dyn Hitable>) -> Object {
        let bbox = volume.bounding_box((0., 0.1)); // TODO: Fix time interval
        Object { volume, bbox }
    }
}

impl Hitable for Object {
    #[inline]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.volume.hit(ray, t_min, t_max)
    }

    #[inline]
    fn bounding_box(&self, time_interval: (f32, f32)) -> Option<Aabb> {
        self.bbox.clone() // TODO: This clone is bad I think
    }
}
