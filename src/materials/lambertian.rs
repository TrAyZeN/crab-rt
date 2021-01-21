use super::material::Material;
use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::utils::random_unit_vector;
use crate::vec::Vec3;

/// A diffuse material that follows the Lambertian reflectance model.
#[derive(Debug, Default, Clone)]
pub struct Lambertian {
    /// Albedo of the material.
    albedo: Vec3,
}

impl Lambertian {
    /// Constructs a new `Lambertian` material with the given albedo.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::materials::Lambertian;
    /// use crab_rt::vec::Vec3;
    ///
    /// // Creates a red diffuse material
    /// let material = Lambertian::new(Vec3::new(1., 0., 0.));
    /// ```
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
