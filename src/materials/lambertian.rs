use super::material::Material;
use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::textures::{Monochrome, Texture};
use crate::utils::random_unit_vector;
use crate::vec::Vec3;

/// A diffuse material that follows the Lambertian reflectance model.
#[derive(Debug)]
pub struct Lambertian {
    /// Albedo of the material.
    albedo: Box<dyn Texture>,
}

impl Lambertian {
    /// Constructs a new `Lambertian` material with the given texture.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::materials::Lambertian;
    /// use crab_rt::textures::Monochrome;
    ///
    /// // Creates a red diffuse material
    /// let material = Lambertian::new(Box::new(Monochrome::from_rgb(1., 0., 0.)));
    /// ```
    #[inline]
    pub fn new(texture: Box<dyn Texture>) -> Self {
        Lambertian { albedo: texture }
    }

    /// Constructs a new monochrome `Lambertian` with the given color.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::materials::Lambertian;
    ///
    /// // Creates a red diffuse material
    /// let material = Lambertian::from_rgb(1., 0., 0.);
    /// ```
    #[inline]
    pub fn from_rgb(red: f32, green: f32, blue: f32) -> Self {
        Self::new(Box::new(Monochrome::from_rgb(red, green, blue)))
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
            self.albedo
                .value(record.get_texture_coordinates(), record.get_hit_point()),
        ))
    }
}

impl Default for Lambertian {
    /// The default lambertian material is a black monochrome lambertian material.
    #[inline]
    fn default() -> Self {
        Lambertian::from_rgb(0., 0., 0.)
    }
}
