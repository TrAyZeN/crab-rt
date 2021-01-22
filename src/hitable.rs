use std::fmt::Debug;

use crate::aabb::Aabb;
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

pub trait Hitable: Debug + Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, time_interval: (f32, f32)) -> Option<Aabb>;
}

impl<H: Hitable> Hitable for Vec<H> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_record = None;
        let mut closest_t = t_max;
        for hitable in self {
            if let Some(record) = hitable.hit(ray, t_min, closest_t) {
                closest_t = record.t;
                closest_record = Some(record);
            }
        }

        closest_record
    }

    fn bounding_box(&self, time_interval: (f32, f32)) -> Option<Aabb> {
        if self.is_empty() {
            return None;
        }

        let first_bounding_box = self[0].bounding_box(time_interval)?;
        self[1..]
            .iter()
            .try_fold(first_bounding_box, |acc, hitable| {
                match hitable.bounding_box(time_interval) {
                    Some(bounding_box) => Some(Aabb::surrounding_box(&acc, &bounding_box)),
                    None => None,
                }
            })
    }
}

#[derive(Debug)]
pub struct HitRecord<'a> {
    t: f32,
    hit_point: Point3,
    normal: Vec3,
    texture_coordinates: (f32, f32),
    front_face: bool,
    material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    #[inline]
    pub fn new(
        t: f32,
        hit_point: Point3,
        normal: Vec3,
        texture_coordinates: (f32, f32),
        material: &'a dyn Material,
    ) -> Self {
        Self {
            t,
            hit_point,
            normal,
            texture_coordinates,
            front_face: true,
            material,
        }
    }

    #[inline]
    pub fn set_face_normal(&mut self, ray: &Ray) {
        // We want to always have the normal pointing against the ray
        self.front_face = ray.get_direction().dot(&self.normal) < 0.;
        if !self.front_face {
            self.normal = -self.normal;
        }
    }

    #[inline]
    pub fn get_t(&self) -> f32 {
        self.t
    }

    #[inline]
    pub fn get_hit_point(&self) -> &Point3 {
        &self.hit_point
    }

    #[inline]
    pub fn get_normal(&self) -> &Vec3 {
        &self.normal
    }

    #[inline]
    pub fn get_texture_coordinates(&self) -> (f32, f32) {
        self.texture_coordinates
    }

    #[inline]
    pub fn get_front_face(&self) -> bool {
        self.front_face
    }

    #[inline]
    pub fn get_material(&self) -> &dyn Material {
        self.material
    }
}
