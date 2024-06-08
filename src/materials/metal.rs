use super::Material;
use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::utils::{random_in_unit_sphere, reflect};
use crate::vec::{Color3, Vec3};

/// A material with specular reflections.
#[derive(Debug, Default, Clone)]
pub struct Metal {
    /// Albedo of the material.
    albedo: Color3,
    /// Fuzziness of the material.
    fuzziness: f32,
}

impl Metal {
    /// Constructs a new `Metal` material with the given albedo and fuzziness.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::materials::Metal;
    /// use crab_rt::vec::Color3;
    ///
    /// let material = Metal::new(Color3::new(1., 1., 1.), 0.);
    /// ```
    #[inline]
    #[must_use]
    pub fn new(albedo: Color3, fuzziness: f32) -> Self {
        Self {
            albedo,
            fuzziness: if fuzziness > 1. { 1. } else { fuzziness },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord<'_>) -> Option<(Ray, Vec3)> {
        let reflected = reflect(&ray.direction().unit(), record.normal());
        let scattered = Ray::new(
            *record.hit_point(),
            reflected + self.fuzziness * random_in_unit_sphere(),
            ray.time(),
        );
        let attenuation = self.albedo;

        if scattered.direction().dot(record.normal()) > 0. {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
