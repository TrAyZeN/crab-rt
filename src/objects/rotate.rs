use alloc::sync::Arc;

use crate::aabb::Aabb;
use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

#[cfg(not(feature = "std"))]
use core_maths::*;

#[derive(Debug)]
pub struct RotateY {
    hitable: Arc<dyn Hitable>,
    sin_theta: f32,
    cos_theta: f32,
    hasbox: bool,
    bbox: Aabb,
}

impl RotateY {
    #[must_use]
    pub fn new(hitable: Arc<dyn Hitable>, angle: f32) -> Self {
        let theta = angle.to_radians();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        let bbox = hitable.bounding_box((0., 1.));
        let hasbox = bbox.is_some();
        let bbox = bbox.unwrap_or_default();

        let mut min = Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * bbox.max().x + (1 - i) as f32 * bbox.min().x;
                    let y = j as f32 * bbox.max().y + (1 - j) as f32 * bbox.min().y;
                    let z = k as f32 * bbox.max().z + (1 - k) as f32 * bbox.min().z;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = f32::min(min[c], tester[c]);
                        max[c] = f32::max(max[c], tester[c]);
                    }
                }
            }
        }

        Self {
            hitable,
            sin_theta,
            cos_theta,
            hasbox,
            bbox: Aabb::new(min, max),
        }
    }
}

impl Hitable for RotateY {
    #[must_use]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let mut origin = *ray.origin();
        let mut direction = *ray.direction();

        origin.x = self.cos_theta * ray.origin().x - self.sin_theta * ray.origin().z;
        origin.z = self.sin_theta * ray.origin().x + self.cos_theta * ray.origin().z;

        direction.x = self.cos_theta * ray.direction().x - self.sin_theta * ray.direction().z;
        direction.z = self.sin_theta * ray.direction().x + self.cos_theta * ray.direction().z;

        let rotated_ray = Ray::new(origin, direction, ray.time());

        let mut record = self.hitable.hit(&rotated_ray, t_min, t_max)?;

        let mut p = *record.hit_point();
        let mut normal = *record.normal();

        p.x = self.cos_theta * record.hit_point().x + self.sin_theta * record.hit_point().z;
        p.z = -self.sin_theta * record.hit_point().x + self.cos_theta * record.hit_point().z;

        normal.x = self.cos_theta * record.normal().x + self.sin_theta * record.normal().z;
        normal.z = -self.sin_theta * record.normal().x + self.cos_theta * record.normal().z;

        record.set_hit_point(p);
        record.set_normal(normal);
        record.set_face_normal(&rotated_ray);

        Some(record)
    }

    #[must_use]
    fn bounding_box(&self, _time_interval: (f32, f32)) -> Option<Aabb> {
        self.hasbox.then(|| self.bbox)
    }
}
