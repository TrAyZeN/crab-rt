use rand::Rng;
use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hitable::{HitRecord, Hitable};
use crate::materials::Material;
use crate::ray::Ray;
use crate::utils::rng;
use crate::vec::{Point3, Vec3};

#[derive(Debug)]
pub struct ConstantMedium<M: Material> {
    boundary: Arc<dyn Hitable>,
    neg_inv_density: f32,
    phase_function: Arc<M>,
}

impl<M: Material> ConstantMedium<M> {
    /// Constructs a new ConstantMedium with the given boundary, density and phase function.
    ///
    /// # Panic
    /// Panics if `density == 0.`.
    #[inline]
    #[must_use]
    pub fn new(boundary: Arc<dyn Hitable>, density: f32, phase_function: Arc<M>) -> Self {
        assert!(density != 0.);

        Self {
            boundary,
            neg_inv_density: -1. / density,
            phase_function,
        }
    }
}

impl<M: Material> Hitable for ConstantMedium<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let mut rec1 = self.boundary.hit(ray, f32::NEG_INFINITY, f32::INFINITY)?;
        let mut rec2 = self
            .boundary
            .hit(ray, rec1.get_t() + 0.0001, f32::INFINITY)?;

        if rec1.get_t() < t_min {
            rec1.set_t(t_min);
        }

        if rec2.get_t() > t_max {
            rec2.set_t(t_max);
        }

        if rec1.get_t() >= rec2.get_t() {
            return None;
        }

        if rec1.get_t() < 0. {
            rec1.set_t(0.);
        }

        let ray_length = ray.get_direction().length();
        let distance_inside_boundary = (rec2.get_t() - rec1.get_t()) * ray_length;
        let hit_distance = self.neg_inv_density * rng().gen::<f32>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.get_t() + hit_distance / ray_length;
        Some(HitRecord::new(
            t,
            ray.point(t),
            Vec3::new(1., 0., 0.),
            (0., 0.),
            self.phase_function.as_ref(),
        ))
    }

    fn bounding_box(&self, time_interval: (f32, f32)) -> Option<Aabb> {
        self.boundary.bounding_box(time_interval)
    }
}
