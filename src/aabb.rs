use std::mem;

use crate::ray::Ray;
use crate::vec::Vec3;

/// An Axis-Aligned Bounding Box (AABB) represented by two opposite points.
#[derive(Debug, Clone)]
pub struct Aabb {
    min: Vec3,
    max: Vec3,
}

impl Aabb {
    /// Constructs an AABB from two opposite points.
    #[inline]
    pub const fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    #[inline]
    pub fn surrounding_box(box0: &Self, box1: &Self) -> Self {
        Self::new(
            Vec3::new(
                f32::min(box0.get_min().x, box1.get_min().x),
                f32::min(box0.get_min().y, box1.get_min().y),
                f32::min(box0.get_min().z, box1.get_min().z),
            ),
            Vec3::new(
                f32::max(box0.get_max().x, box1.get_max().x),
                f32::max(box0.get_max().y, box1.get_max().y),
                f32::max(box0.get_max().z, box1.get_max().z),
            ),
        )
    }

    #[inline]
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for axis in 0..3 {
            let inv_axis_direction = ray.get_direction()[axis].recip();
            let mut t0 = (self.min[axis] - ray.get_origin()[axis]) * inv_axis_direction;
            let mut t1 = (self.max[axis] - ray.get_origin()[axis]) * inv_axis_direction;
            if inv_axis_direction < 0. {
                mem::swap(&mut t0, &mut t1);
            }

            let t_min = f32::max(t0, t_min);
            let t_max = f32::min(t1, t_max);
            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    #[inline]
    pub const fn get_min(&self) -> &Vec3 {
        &self.min
    }

    #[inline]
    pub const fn get_max(&self) -> &Vec3 {
        &self.max
    }
}
