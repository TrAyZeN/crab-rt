use std::fmt::Debug;

use crate::aabb::Aabb;
use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;

#[derive(Debug)]
pub struct Object {
    volume: Box<dyn Hitable>,
    bbox: Option<Aabb>,
}

impl Object {
    /// Constructs a new object with the given volume.
    ///
    /// # Examples
    /// ```
    /// use std::sync::Arc;
    ///
    /// use crab_rt::materials::Lambertian;
    /// use crab_rt::objects::{Object, Sphere};
    /// use crab_rt::vec::Vec3;
    ///
    /// let object = Object::new(Sphere::new(
    ///     Vec3::zero(),
    ///     1.,
    ///     Arc::new(Lambertian::default()),
    /// ));
    /// ```
    #[inline]
    pub fn new<H: 'static + Hitable>(volume: H) -> Self {
        let bbox = volume.bounding_box((0., 0.1)); // TODO: Fix time interval
        Self {
            volume: Box::new(volume),
            bbox,
        }
    }

    // #[inline]
    // fn get_volume(&self) -> Box<dyn Hitable> {
    //     self.volume
    // }
}

impl Hitable for Object {
    #[inline]
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        self.volume.hit(ray, t_min, t_max)
    }

    #[inline]
    fn bounding_box(&self, _time_interval: (f32, f32)) -> Option<Aabb> {
        self.bbox // TODO: We could maybe use a Cow
    }
}

// impl PartialEq for Object {
//     fn eq(&self, other: &Self) -> bool {
//         self.get_volume() == other.get_volume()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::materials::Lambertian;
    use crate::objects::Sphere;
    use crate::vec::Vec3;
    use std::sync::Arc;

    #[test]
    fn object_bounding_box() {
        let time_interval = (0., 0.);
        let sphere = Sphere::new(Vec3::zero(), 1., Arc::new(Lambertian::default()));
        let sphere_bbox = sphere.bounding_box(time_interval);

        let testee = Object::new(sphere);
        assert_eq!(testee.bounding_box(time_interval), sphere_bbox);
    }
}
