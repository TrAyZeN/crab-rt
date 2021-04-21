use rand::Rng;

use super::material::Material;
use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::utils::{reflect, refract, rng, schlick};
use crate::vec::Vec3;

// use rand::{prelude::*, Rng};

const WATER_REFRACTIVE_INDEX: f32 = 1.333;
const DIAMOND_REFRACTIVE_INDEX: f32 = 2.417;

/// A material with refractions and specular reflections.
#[derive(Debug, Default, Clone)]
pub struct Dielectric {
    /// [Refractive index](https://en.wikipedia.org/wiki/List_of_refractive_indices) of the material.
    refractive_index: f32,
}

impl Dielectric {
    /// Constructs a new `Dielectric` material with the given refractive index.
    ///
    /// # Panic
    /// Panics if `refractive_index < 1.`.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::materials::Dielectric;
    ///
    /// let material = Dielectric::new(1.1);
    /// ```
    #[inline]
    #[must_use]
    pub fn new(refractive_index: f32) -> Self {
        assert!(refractive_index >= 1.);

        Self { refractive_index }
    }

    /// Constructs a new `Dielecric` material with the water's refractive index.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::materials::Dielectric;
    ///
    /// let water_material = Dielectric::water();
    /// ```
    #[inline]
    #[must_use]
    pub fn water() -> Self {
        Self::new(WATER_REFRACTIVE_INDEX)
    }

    /// Constructs a new `Dielectric` material with the diamond's refractive index.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::materials::Dielectric;
    ///
    /// let diamond_material = Dielectric::diamond();
    /// ```
    #[inline]
    #[must_use]
    pub fn diamond() -> Self {
        Self::new(DIAMOND_REFRACTIVE_INDEX)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord<'_>) -> Option<(Ray, Vec3)> {
        let mut rng = rng();
        let refraction_ratio = if record.get_front_face() {
            1. / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = ray.get_direction().unit();
        let cos_theta = f32::min((-unit_direction).dot(record.get_normal()), 1.);
        let sin_theta = f32::sqrt(1. - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.;

        let direction = if cannot_refract || schlick(cos_theta, refraction_ratio) > rng.gen::<f32>()
        {
            reflect(&unit_direction, record.get_normal())
        } else {
            refract(&unit_direction, record.get_normal(), refraction_ratio)
        };

        let attenuation = Vec3::new(1., 1., 1.);
        let scattered = Ray::new(*record.get_hit_point(), direction, ray.get_time());
        Some((scattered, attenuation))
    }
}
