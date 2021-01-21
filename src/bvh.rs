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

        let left_bbox = left
            .as_ref()
            .map(|a| a.bounding_box(time_interval))
            .flatten();
        let bbox = if right.is_none() {
            left_bbox
        } else {
            let right_bbox = right
                .as_ref()
                .map(|a| a.bounding_box(time_interval))
                .flatten();

            left_bbox
                .zip(right_bbox)
                .map(|(lb, rb)| Aabb::surrounding_box(&lb, &rb))
        };

        Self { bbox, left, right }
    }
}

impl Hitable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self
            .bbox
            .as_ref()
            .map_or(false, |b| b.hit(ray, t_min, t_max))
        {
            return None;
        }

        let left_record = self
            .left
            .as_ref()
            .map(|r| r.hit(ray, t_min, t_max))
            .flatten();

        let right_record = self
            .right
            .as_ref()
            .map(|r| {
                r.hit(
                    ray,
                    t_min,
                    left_record.as_ref().map_or(t_max, |r| r.get_t()),
                )
            })
            .flatten();

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
        let sphere = Sphere::new(Vec3::zero(), 1., Lambertian::default());
        let testee = BvhNode::new(vec![Object::new(sphere.clone())], (0., 0.));

        assert_eq!(testee.bounding_box((0., 0.)), sphere.bounding_box((0., 0.)));
    }

    #[test]
    fn new_with_two_object() {
        let sphere1 = Sphere::new(Vec3::zero(), 1., Lambertian::default());
        let sphere2 = Sphere::new(Vec3::new(1., 2., 3.), 1., Lambertian::default());
        let testee = BvhNode::new(
            vec![Object::new(sphere1.clone()), Object::new(sphere2.clone())],
            (0., 0.),
        );

        assert_eq!(
            testee.bounding_box((0., 0.)),
            Some(Aabb::surrounding_box(
                &sphere1.bounding_box((0., 0.)).unwrap(),
                &sphere2.bounding_box((0., 0.)).unwrap()
            ))
        );
    }
}
