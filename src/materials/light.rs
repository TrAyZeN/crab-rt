use super::Material;
use crate::hitable::HitRecord;
use crate::ray::Ray;
use crate::textures::Texture;
use crate::vec::{Point3, Vec3};

#[derive(Debug)]
pub struct Light {
    emit: Box<dyn Texture>,
}

impl Light {
    #[inline]
    #[must_use]
    pub fn new(emit: Box<dyn Texture>) -> Self {
        Self { emit }
    }
}

impl Material for Light {
    fn scatter(&self, _ray: &Ray, _record: &HitRecord<'_>) -> Option<(Ray, Vec3)> {
        None
    }

    fn emitted(&self, texture_coordinates: (f32, f32), p: &Point3) -> Vec3 {
        self.emit.value(texture_coordinates, p)
    }
}
