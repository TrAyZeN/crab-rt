use crate::aabb::Aabb;
use crate::hitable::{HitRecord, Hitable};
use crate::materials::material;
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

#[derive(Debug)]
pub struct XyRect<M: Material> {
    x: (f32, f32),
    y: (f32, f32),
    k: f32,
    material: M,
}

impl<M: Material> XyRect<M> {
    #[inline]
    #[must_use]
    pub fn new(x: (f32, f32), y: (f32, f32), k: f32, material: M) -> Self {
        Self { x, y, k, material }
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
        if x < self.x.0 || x > self.x.1 || y < self.y.0 || y > self.y.1 {
            return None;
        }

        let mut record = HitRecord::new(
            t,
            ray.point(t),
            Vec3::new(0., 0., 1.),
            (
                (x - self.x.0) / (self.x.1 - self.x.0),
                (y - self.y.0) / (self.y.1 - self.y.0),
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
            Point3::new(self.x.0, self.y.0, self.k - 0.0001),
            Point3::new(self.x.1, self.y.1, self.k + 0.0001),
        ))
    }
}

#[derive(Debug)]
pub struct XzRect<M: Material> {
    x: (f32, f32),
    z: (f32, f32),
    k: f32,
    material: M,
}

impl<M: Material> XzRect<M> {
    #[inline]
    #[must_use]
    pub fn new(x: (f32, f32), z: (f32, f32), k: f32, material: M) -> Self {
        Self { x, z, k, material }
    }
}

impl<M: Material> Hitable for XzRect<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let t = (self.k - ray.get_origin().y) / ray.get_direction().y;
        if t < t_min || t > t_max {
            return None;
        }

        let x = t.mul_add(ray.get_direction().x, ray.get_origin().x);
        let z = t.mul_add(ray.get_direction().z, ray.get_origin().z);
        if x < self.x.0 || x > self.x.1 || z < self.z.0 || z > self.z.1 {
            return None;
        }

        let mut record = HitRecord::new(
            t,
            ray.point(t),
            Vec3::new(0., 1., 0.),
            (
                (x - self.x.0) / (self.x.1 - self.x.0),
                (z - self.z.0) / (self.z.1 - self.z.0),
            ),
            &self.material,
        );
        record.set_face_normal(ray);

        Some(record)
    }

    fn bounding_box(&self, _time_interval: (f32, f32)) -> Option<Aabb> {
        Some(Aabb::new(
            Point3::new(self.x.0, self.k - 0.0001, self.z.0),
            Point3::new(self.x.1, self.k + 0.0001, self.z.1),
        ))
    }
}

#[derive(Debug)]
pub struct YzRect<M: Material> {
    y: (f32, f32),
    z: (f32, f32),
    k: f32,
    material: M,
}

impl<M: Material> YzRect<M> {
    #[inline]
    #[must_use]
    pub fn new(y: (f32, f32), z: (f32, f32), k: f32, material: M) -> Self {
        Self { y, z, k, material }
    }
}

impl<M: Material> Hitable for YzRect<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let t = (self.k - ray.get_origin().x) / ray.get_direction().x;
        if t < t_min || t > t_max {
            return None;
        }

        let y = t.mul_add(ray.get_direction().y, ray.get_origin().y);
        let z = t.mul_add(ray.get_direction().z, ray.get_origin().z);
        if y < self.y.0 || y > self.y.1 || z < self.z.0 || z > self.z.1 {
            return None;
        }

        let mut record = HitRecord::new(
            t,
            ray.point(t),
            Vec3::new(0., 1., 0.),
            (
                (y - self.y.0) / (self.y.1 - self.y.0),
                (z - self.z.0) / (self.z.1 - self.z.0),
            ),
            &self.material,
        );
        record.set_face_normal(ray);

        Some(record)
    }

    fn bounding_box(&self, _time_interval: (f32, f32)) -> Option<Aabb> {
        Some(Aabb::new(
            Point3::new(self.k - 0.0001, self.y.0, self.z.0),
            Point3::new(self.k + 0.0001, self.y.1, self.z.1),
        ))
    }
}
