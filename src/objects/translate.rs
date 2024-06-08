use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Debug)]
pub struct Translate {
    hitable: Arc<dyn Hitable>,
    offset: Vec3,
}

impl Translate {
    #[must_use]
    pub fn new<H: Hitable + 'static>(hitable: Arc<H>, offset: Vec3) -> Self {
        Self { hitable, offset }
    }
}

impl Hitable for Translate {
    #[must_use]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let translated_ray = Ray::new(ray.origin() - self.offset, *ray.direction(), ray.time());

        self.hitable
            .hit(&translated_ray, t_min, t_max)
            .map(|mut r| {
                r.set_hit_point(r.hit_point() + self.offset);
                r.set_face_normal(&translated_ray);
                r
            })
    }

    #[must_use]
    fn bounding_box(&self, time_interval: (f32, f32)) -> Option<Aabb> {
        self.hitable
            .bounding_box(time_interval)
            .map(|aabb| Aabb::new(aabb.min() + self.offset, aabb.max() + self.offset))
    }
}
