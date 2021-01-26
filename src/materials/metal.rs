use super::Material;
use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::utils::{random_in_unit_sphere, reflect};
use crate::vec::Vec3;

/// A material with specular reflections.
#[derive(Debug, Default, Clone)]
pub struct Metal {
    /// Albedo of the material.
    albedo: Vec3,
    /// Fuzziness of the material.
    fuzziness: f32,
}

impl Metal {
    /// Constructs a new `Metal` material with the given albedo and fuzziness.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::materials::Metal;
    /// use crab_rt::vec::Vec3;
    ///
    /// let material = Metal::new(Vec3::new(1., 1., 1.), 0.);
    /// ```
    #[inline]
    pub fn new(albedo: Vec3, fuzziness: f32) -> Self {
        Metal {
            albedo,
            fuzziness: if fuzziness > 1. { 1. } else { fuzziness },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord<'_>) -> Option<(Ray, Vec3)> {
        let reflected = reflect(&ray.get_direction().unit(), record.get_normal());
        let scattered = Ray::new(
            record.get_hit_point().clone(),
            reflected + self.fuzziness * random_in_unit_sphere(),
            ray.get_time(),
        );
        let attenuation = self.albedo;

        if scattered.get_direction().dot(record.get_normal()) > 0. {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
