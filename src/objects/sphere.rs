use crate::aabb::Aabb;
use crate::hitable::{HitRecord, Hitable};
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec::Vec3;

/// A sphere.
#[derive(Debug, Clone, PartialEq)]
pub struct Sphere<M: Material> {
    /// Center of the sphere.
    center: Vec3,
    /// Radius of the sphere.
    radius: f32,
    /// Material of the sphere.
    material: M,
}

impl<M: Material> Sphere<M> {
    /// Constructs a sphere from the given center, radius and material.
    ///
    /// # Panic
    /// Panics if `radius == 0.`.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::materials::Lambertian;
    /// use crab_rt::objects::Sphere;
    /// use crab_rt::vec::Vec3;
    ///
    /// let sphere = Sphere::new(Vec3::zero(), 1., Lambertian::default());
    /// ```
    #[inline]
    pub fn new(center: Vec3, radius: f32, material: M) -> Self {
        assert_ne!(radius, 0.);

        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Hitable for Sphere<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.get_origin() - self.center;
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
            (hit_point - self.center) / self.radius,
            &self.material,
        );
        record.set_face_normal(ray);
        Some(record)
    }

    fn bounding_box(&self, _time_interval: (f32, f32)) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::materials::Lambertian;

    #[test]
    fn sphere_bounding_box() {
        let testee = Sphere::new(
            Vec3::new(1., 2., 3.),
            1.,
            Lambertian::new(Vec3::new(0., 0., 0.)),
        );
        let bounding_box = testee.bounding_box((0., 0.));
        assert!(bounding_box.is_some());

        let bounding_box = bounding_box.unwrap();
        assert_eq!(
            bounding_box.get_min(),
            &Vec3::new(1. - 1., 2. - 1., 3. - 1.)
        );
        assert_eq!(
            bounding_box.get_max(),
            &Vec3::new(1. + 1., 2. + 1., 3. + 1.)
        );
    }
}
