use super::Material;
use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::utils::{random_in_unit_sphere, reflect};
use crate::vec::Vec3;

#[derive(Default)]
pub struct Metal {
    albedo: Vec3,
    fuzziness: f32,
}

impl Metal {
    #[inline]
    pub fn new(albedo: Vec3, fuzziness: f32) -> Self {
        Metal {
            albedo,
            fuzziness: if fuzziness > 1. { 1. } else { fuzziness },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)> {
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
