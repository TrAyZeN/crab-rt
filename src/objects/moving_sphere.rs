use crate::aabb::Aabb;
use crate::hitable::{HitRecord, Hitable};
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct MovingSphere<M: Material> {
    center_interval: (Vec3, Vec3),
    time_interval: (f32, f32),
    radius: f32,
    material: M,
}

impl<M: Material> MovingSphere<M> {
    #[inline]
    pub fn new(
        center_interval: (Vec3, Vec3),
        time_interval: (f32, f32),
        radius: f32,
        material: M,
    ) -> Self {
        MovingSphere {
            center_interval,
            time_interval,
            radius,
            material,
        }
    }

    #[inline]
    fn center(&self, time: f32) -> Vec3 {
        self.center_interval.0
            + ((time - self.time_interval.0) / (self.time_interval.1 - self.time_interval.1))
                * (self.center_interval.1 - self.center_interval.0)
    }
}

impl<M: Material> Hitable for MovingSphere<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let center = self.center(ray.get_time());
        let oc = ray.get_origin() - center;
        let a = ray.get_direction().square();
        let half_b = oc.dot(ray.get_direction()); // We use b/2 to avoid useless divisions and mutliplications by 2
        let c = oc.square() - self.radius * self.radius;
        let discriminant_over_4 = half_b * half_b - a * c;

        // No solution ie the ray didn't hit the sphere
        if discriminant_over_4 < 0. {
            return None;
        }

        let inv_a = 1. / a;
        let half_sqrt_discriminitant = f32::sqrt(discriminant_over_4);

        let mut root = (-half_b - half_sqrt_discriminitant) * inv_a;
        if root < t_min || t_max < root {
            root = (-half_b + half_sqrt_discriminitant) * inv_a;

            // Root not in the interval
            if root < t_min || t_max < root {
                return None;
            }
        }

        let hit_point = ray.point(root);
        let mut record = HitRecord::new(
            root,
            hit_point,
            (hit_point - center) / self.radius,
            &self.material,
        );
        record.set_face_normal(ray);
        Some(record)
    }

    fn bounding_box(&self, time_interval: (f32, f32)) -> Option<Aabb> {
        let initial_bounding_box = Aabb::new(
            self.center(time_interval.0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time_interval.0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let final_bounding_box = Aabb::new(
            self.center(time_interval.1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time_interval.1) + Vec3::new(self.radius, self.radius, self.radius),
        );

        Some(Aabb::surrounding_box(
            &initial_bounding_box,
            &final_bounding_box,
        ))
    }
}
