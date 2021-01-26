use rand::distributions::{Distribution, Uniform};
use std::cmp::Ordering;

use crate::aabb::Aabb;
use crate::hitable::{HitRecord, Hitable};
use crate::objects::Object;
use crate::ray::Ray;

#[derive(Debug, Default)]
pub struct BvhNode {
    bbox: Option<Aabb>,

    left: Option<Box<dyn Hitable>>,
    right: Option<Box<dyn Hitable>>,
}

impl BvhNode {
    #[must_use]
    pub fn new(mut objects: Vec<Object>, time_interval: (f32, f32)) -> Self {
        let uniform = Uniform::from(0..3);
        let mut rng = rand::thread_rng();
        let axis = uniform.sample(&mut rng);
        let comparator = |object1: &Object, object2: &Object| {
            let bbox_1 = object1.bounding_box((0., 0.));
            let bbox_2 = object2.bounding_box((0., 0.));

            if bbox_1.is_none() || bbox_2.is_none() {
                return Ordering::Less;
            }

            bbox_1.unwrap().get_min()[axis]
                .partial_cmp(&bbox_2.unwrap().get_min()[axis])
                .unwrap()
        };

        let (left, right): (Option<Box<dyn Hitable>>, Option<Box<dyn Hitable>>) =
            match objects.len() {
                1 => (Some(Box::new(objects.remove(0))), None),
                2 => {
                    let first = objects.remove(0);
                    let second = objects.remove(0);

                    if comparator(&first, &second) == Ordering::Less {
                        (Some(Box::new(first)), Some(Box::new(second)))
                    } else {
                        (Some(Box::new(second)), Some(Box::new(first)))
                    }
                }
                n => {
                    objects.sort_by(comparator);
                    let second_half = objects.split_off(n / 2);
                    (
                        Some(Box::new(Self::new(objects, time_interval))),
                        Some(Box::new(Self::new(second_half, time_interval))),
                    )
                }
            };

        let left_bbox = left.as_ref().and_then(|a| a.bounding_box(time_interval));
        let bbox = if right.is_none() {
            left_bbox
        } else {
            let right_bbox = right.as_ref().and_then(|a| a.bounding_box(time_interval));

            left_bbox
                .zip(right_bbox)
                .map(|(lb, rb)| Aabb::surrounding_box(&lb, &rb))
        };

        Self { bbox, left, right }
    }
}

impl Hitable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>> {
        if !self
            .bbox
            .as_ref()
            .map_or(false, |b| b.hit(ray, t_min, t_max))
        {
            return None;
        }

        let left_record = self.left.as_ref().and_then(|r| r.hit(ray, t_min, t_max));

        let right_record = self.right.as_ref().and_then(|r| {
            r.hit(
                ray,
                t_min,
                left_record.as_ref().map_or(t_max, |r| r.get_t()),
            )
        });

        right_record.or(left_record)
    }

    #[inline]
    fn bounding_box(&self, _time_interval: (f32, f32)) -> Option<Aabb> {
        self.bbox
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::materials::Lambertian;
    use crate::objects::Sphere;
    use crate::vec::Vec3;

    #[test]
    fn new_with_one_object() {
        let time_interval = (0., 0.);
        let sphere = Sphere::new(Vec3::zero(), 1., Lambertian::default());
        let sphere_bbox = sphere.bounding_box(time_interval);

        let testee = BvhNode::new(vec![Object::new(sphere)], time_interval);
        assert_eq!(testee.bounding_box(time_interval), sphere_bbox);
    }

    #[test]
    fn new_with_two_object() {
        let time_interval = (0., 0.);
        let sphere1 = Sphere::new(Vec3::zero(), 1., Lambertian::default());
        let sphere2 = Sphere::new(Vec3::new(1., 2., 3.), 1., Lambertian::default());

        let sphere1_bbox = sphere1.bounding_box(time_interval);
        let sphere2_bbox = sphere2.bounding_box(time_interval);

        let testee = BvhNode::new(
            vec![Object::new(sphere1), Object::new(sphere2)],
            time_interval,
        );
        assert_eq!(
            testee.bounding_box(time_interval),
            Some(Aabb::surrounding_box(
                &sphere1_bbox.unwrap(),
                &sphere2_bbox.unwrap(),
            ))
        );
    }
}
