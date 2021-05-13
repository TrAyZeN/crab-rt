use crate::hitable::HitRecord;
use crate::materials::Material;
use crate::ray::Ray;
use crate::textures::Texture;
use crate::utils::random_in_unit_sphere;
use crate::vec::Vec3;

#[derive(Debug)]
pub struct Isotropic {
    albedo: Box<dyn Texture>,
}

impl Isotropic {
    #[inline]
    #[must_use]
    pub fn new<T: 'static + Texture>(texture: T) -> Self {
        Self {
            albedo: Box::new(texture),
        }
    }
}

impl Material for Isotropic {
    #[inline]
    fn scatter(&self, ray: &Ray, record: &HitRecord<'_>) -> Option<(Ray, Vec3)> {
        Some((
            Ray::new(
                *record.get_hit_point(),
                random_in_unit_sphere(),
                ray.get_time(),
            ),
            self.albedo.value_from_hit(record),
        ))
    }
}
