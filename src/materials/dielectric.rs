use super::material::Material;
use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::utils::{reflect, refract, schlick};
use crate::vec::Vec3;
use rand::prelude::*;

// use rand::{prelude::*, Rng};

const WATER_REFRACTIVE_INDEX: f32 = 1.333;
const DIAMOND_REFRACTIVE_INDEX: f32 = 2.417;

#[derive(Default)]
pub struct Dielectric {
    /// https://en.wikipedia.org/wiki/List_of_refractive_indices
    refractive_index: f32,
}

impl Dielectric {
    #[inline]
    pub const fn new(refractive_index: f32) -> Self {
        Dielectric { refractive_index }
    }

    #[inline]
    pub const fn water() -> Self {
        Self::new(WATER_REFRACTIVE_INDEX)
    }

    #[inline]
    pub const fn diamond() -> Self {
        Self::new(DIAMOND_REFRACTIVE_INDEX)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut rng = rand::thread_rng();
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
        let scattered = Ray::new(record.get_hit_point().clone(), direction, ray.get_time());
        Some((scattered, attenuation))
    }
}
