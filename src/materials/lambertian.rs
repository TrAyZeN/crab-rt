use super::material::Material;
use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::utils::random_unit_vector;
use crate::vec::Vec3;

#[derive(Debug, Default)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    #[inline]
    pub const fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut scatter_direction = record.get_normal() + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = record.get_normal().clone();
        }

        Some((
            Ray::new(
                record.get_hit_point().clone(),
                scatter_direction,
                ray.get_time(),
            ),
            self.albedo,
        ))
    }
}
