use crate::aabb::Aabb;
use crate::hitable::{HitRecord, Hitable};
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Debug)]
pub struct XyRect<M: Material> {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    material: M,
}

impl<M: Material> XyRect<M> {
    #[inline]
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: M) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}

impl<M: Material> Hitable for XyRect<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let t = (self.k - ray.get_origin().z) / ray.get_direction().z;
        // Checks if the ray hits the plane
        if t < t_min || t > t_max {
            return None;
        }

        let x = t.mul_add(ray.get_direction().x, ray.get_origin().x);
        let y = t.mul_add(ray.get_direction().y, ray.get_origin().y);
        // Checks if the ray hits the rectangle
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let mut record = HitRecord::new(
            t,
            ray.point(t),
            Vec3::new(0., 0., 1.),
            (
                (x - self.x0) / (self.x1 - self.x0),
                (y - self.y0) / (self.y1 - self.y0),
            ),
            &self.material,
        );
        record.set_face_normal(ray);

        Some(record)
    }

    fn bounding_box(&self, _time_interval: (f32, f32)) -> Option<Aabb> {
        // The bounding box must have a non-zero width in each dimension so we
        // pad the z by a small amount
        Some(Aabb::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}
