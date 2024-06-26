use alloc::vec::Vec;
use core::fmt::Debug;

use crate::aabb::Aabb;
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

pub trait Hitable: Debug + Send + Sync {
    #[must_use]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>>;

    #[must_use]
    fn bounding_box(&self, time_interval: (f32, f32)) -> Option<Aabb>;
}

impl<H: Hitable> Hitable for Vec<H> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
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

/// A record for a surface hit.
#[derive(Debug)]
pub struct HitRecord<'material> {
    /// The distance of the hit point to the origin.
    t: f32,
    /// The hit point.
    hit_point: Point3,
    /// The surface normal.
    normal: Vec3,
    /// The texture coordinates.
    texture_coordinates: (f32, f32),
    /// Whether the ray hitted the front face.
    front_face: bool,
    /// The material of the surface.
    material: &'material dyn Material,
}

impl<'material> HitRecord<'material> {
    /// Constructs a new `HitRecord`.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::hitable::HitRecord;
    /// use crab_rt::materials::Lambertian;
    /// use crab_rt::vec::{Point3, Vec3};
    ///
    /// let material = Lambertian::default();
    /// let record = HitRecord::new(
    ///     1.,
    ///     Point3::new(1., 1., 1.),
    ///     Vec3::new(0., 1., 0.),
    ///     (0., 0.5),
    ///     &material,
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn new(
        t: f32,
        hit_point: Point3,
        normal: Vec3,
        texture_coordinates: (f32, f32),
        material: &'material dyn Material,
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
        self.front_face = ray.direction().dot(&self.normal) < 0.;
        if !self.front_face {
            self.normal = -self.normal;
        }
    }

    /// Returns the distance of the hit point to the origin.
    ///
    /// # Example
    /// ```
    /// use crab_rt::hitable::HitRecord;
    /// use crab_rt::materials::Lambertian;
    /// use crab_rt::vec::{Point3, Vec3};
    ///
    /// let material = Lambertian::default();
    /// let record = HitRecord::new(
    ///     1.,
    ///     Point3::new(1., 1., 1.),
    ///     Vec3::new(0., 1., 0.),
    ///     (0., 0.5),
    ///     &material,
    /// );
    /// assert_eq!(record.t(), 1.);
    /// ```
    #[inline]
    #[must_use]
    pub const fn t(&self) -> f32 {
        self.t
    }

    #[inline]
    pub fn set_t(&mut self, t: f32) {
        self.t = t;
    }

    /// Returns the hit point.
    ///
    /// # Example
    /// ```
    /// use crab_rt::hitable::HitRecord;
    /// use crab_rt::materials::Lambertian;
    /// use crab_rt::vec::{Point3, Vec3};
    ///
    /// let material = Lambertian::default();
    /// let record = HitRecord::new(
    ///     1.,
    ///     Point3::new(1., 1., 1.),
    ///     Vec3::new(0., 1., 0.),
    ///     (0., 0.5),
    ///     &material,
    /// );
    /// assert_eq!(record.hit_point(), &Point3::new(1., 1., 1.));
    /// ```
    #[inline]
    #[must_use]
    pub const fn hit_point(&self) -> &Point3 {
        &self.hit_point
    }

    // TODO: Remove this method ?
    #[inline]
    pub fn set_hit_point(&mut self, hit_point: Point3) {
        self.hit_point = hit_point;
    }

    /// Returns the surface normal.
    ///
    /// # Example
    /// ```
    /// use crab_rt::hitable::HitRecord;
    /// use crab_rt::materials::Lambertian;
    /// use crab_rt::vec::{Point3, Vec3};
    ///
    /// let material = Lambertian::default();
    /// let record = HitRecord::new(
    ///     1.,
    ///     Point3::new(1., 1., 1.),
    ///     Vec3::new(0., 1., 0.),
    ///     (0., 0.5),
    ///     &material,
    /// );
    /// assert_eq!(record.normal(), &Vec3::new(0., 1., 0.));
    /// ```
    #[inline]
    #[must_use]
    pub const fn normal(&self) -> &Vec3 {
        &self.normal
    }

    #[inline]
    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal;
    }

    /// Returns the texture coordinates of the point.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::hitable::HitRecord;
    /// use crab_rt::materials::Lambertian;
    /// use crab_rt::vec::{Point3, Vec3};
    ///
    /// let material = Lambertian::default();
    /// let record = HitRecord::new(
    ///     1.,
    ///     Point3::new(1., 1., 1.),
    ///     Vec3::new(0., 1., 0.),
    ///     (0., 0.5),
    ///     &material,
    /// );
    /// assert_eq!(record.texture_coordinates(), (0., 0.5));
    /// ```
    #[inline]
    #[must_use]
    pub const fn texture_coordinates(&self) -> (f32, f32) {
        self.texture_coordinates
    }

    /// Returns whether the ray hitted the front face.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::hitable::HitRecord;
    /// use crab_rt::materials::Lambertian;
    /// use crab_rt::vec::{Point3, Vec3};
    ///
    /// let material = Lambertian::default();
    /// let record = HitRecord::new(
    ///     1.,
    ///     Point3::new(1., 1., 1.),
    ///     Vec3::new(0., 1., 0.),
    ///     (0., 0.5),
    ///     &material,
    /// );
    /// assert_eq!(record.front_face(), true);
    /// ```
    #[inline]
    #[must_use]
    pub const fn front_face(&self) -> bool {
        self.front_face
    }

    /// Returns a reference to the surface material.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::hitable::HitRecord;
    /// use crab_rt::materials::{Lambertian, Material};
    /// use crab_rt::vec::{Point3, Vec3};
    ///
    /// let material = Lambertian::default();
    /// let record = HitRecord::new(
    ///     1.,
    ///     Point3::new(1., 1., 1.),
    ///     Vec3::new(0., 1., 0.),
    ///     (0., 0.5),
    ///     &material,
    /// );
    /// assert_eq!(record.material(), &material as &dyn Material);
    /// ```
    #[inline]
    #[must_use]
    pub fn material(&self) -> &dyn Material {
        self.material
    }
}
