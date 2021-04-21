use std::marker::PhantomData;
use std::sync::Arc;

use super::{Object, XyRect, XzRect, YzRect};
use crate::aabb::Aabb;
use crate::hitable::{HitRecord, Hitable};
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec::Point3;

/// An Axis-aligned box
#[derive(Debug)]
pub struct AaBox<M: Material> {
    min: Point3,
    max: Point3,
    faces: [Object; 6],
    material: PhantomData<M>,
}

impl<M> AaBox<M>
where
    M: 'static + Material,
{
    /// Creates a new axis-aligned box of the given material with the given vertices.
    #[must_use]
    pub fn new(min: Point3, max: Point3, material: Arc<M>) -> Self {
        Self {
            min,
            max,
            faces: [
                // TODO: Use Arc for material
                Object::new(XyRect::new(
                    (min.x, max.x),
                    (min.y, max.y),
                    min.z,
                    material.clone(),
                )),
                Object::new(XyRect::new(
                    (min.x, max.x),
                    (min.y, max.y),
                    max.z,
                    material.clone(),
                )),
                Object::new(XzRect::new(
                    (min.x, max.x),
                    (min.z, max.z),
                    min.y,
                    material.clone(),
                )),
                Object::new(XzRect::new(
                    (min.x, max.x),
                    (min.z, max.z),
                    min.y,
                    material.clone(),
                )),
                Object::new(YzRect::new(
                    (min.y, max.y),
                    (min.z, max.z),
                    max.x,
                    material.clone(),
                )),
                Object::new(YzRect::new((min.y, max.y), (min.z, max.z), max.x, material)),
            ],
            material: PhantomData,
        }
    }
}

impl<M: Material> Hitable for AaBox<M> {
    #[must_use]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        let mut closest_record = None;
        let mut closest_t = t_max;
        for i in 0..6 {
            if let Some(record) = self.faces[i].hit(ray, t_min, closest_t) {
                closest_t = record.get_t();
                closest_record = Some(record);
            }
        }

        closest_record
    }

    #[must_use]
    #[inline]
    fn bounding_box(&self, _time_interval: (f32, f32)) -> Option<Aabb> {
        Some(Aabb::new(self.min, self.max))
    }
}
