use std::mem;

use crate::ray::Ray;
use crate::vec::Vec3;

/// An Axis-Aligned Bounding Box (AABB) represented by two opposite points.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Aabb {
    /// Vertex with minimal coordinates on all axis.
    min: Vec3,
    /// Vertex with maximal coordinates on all axis.
    max: Vec3,
}

impl Aabb {
    /// Constructs an AABB from two opposite points.
    ///
    /// # Panic
    /// Panics in `debug` mode if `min.x > max.x || min.y > max.y || min.z > max.z`.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::aabb::Aabb;
    /// use crab_rt::vec::Vec3;
    ///
    /// let bbox = Aabb::new(Vec3::new(1., 2., 3.), Vec3::new(4., 5., 6.));
    /// ```
    #[inline]
    #[must_use]
    pub fn new(min: Vec3, max: Vec3) -> Self {
        debug_assert!(min.x <= max.x && min.y <= max.y && min.z <= max.z); // Should be < but for now let's say <=

        Self { min, max }
    }

    /// Constructs an AABB that surrounds the two given AABB.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::aabb::Aabb;
    /// use crab_rt::vec::Vec3;
    ///
    /// let bbox0 = Aabb::new(Vec3::new(1., 2., 3.), Vec3::new(4., 5., 6.));
    /// let bbox1 = Aabb::new(Vec3::new(7., 8., 9.), Vec3::new(10., 11., 12.));
    ///
    /// let surrounding_bbox = Aabb::surrounding_box(&bbox0, &bbox1);
    /// assert_eq!(surrounding_bbox.min(), &Vec3::new(1., 2., 3.));
    /// assert_eq!(surrounding_bbox.max(), &Vec3::new(10., 11., 12.));
    /// ```
    #[inline]
    #[must_use]
    pub fn surrounding_box(bbox0: &Self, bbox1: &Self) -> Self {
        Self::new(
            Vec3::new(
                f32::min(bbox0.min.x, bbox1.min.x),
                f32::min(bbox0.min.y, bbox1.min.y),
                f32::min(bbox0.min.z, bbox1.min.z),
            ),
            Vec3::new(
                f32::max(bbox0.max.x, bbox1.max.x),
                f32::max(bbox0.max.y, bbox1.max.y),
                f32::max(bbox0.max.z, bbox1.max.z),
            ),
        )
    }

    /// Tests if the given ray hits the AABB.
    #[must_use]
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for axis in 0..3 {
            let inv_axis_direction = ray.direction()[axis].recip();
            let mut t0 = (self.min[axis] - ray.origin()[axis]) * inv_axis_direction;
            let mut t1 = (self.max[axis] - ray.origin()[axis]) * inv_axis_direction;
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

    /// Returns the vertex with minimal coordinates on all axis of the AABB.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::aabb::Aabb;
    /// use crab_rt::vec::Vec3;
    ///
    /// let bbox = Aabb::new(Vec3::new(1., 2., 3.), Vec3::new(4., 5., 6.));
    /// assert_eq!(bbox.min(), &Vec3::new(1., 2., 3.));
    /// ```
    #[inline]
    #[must_use]
    pub const fn min(&self) -> &Vec3 {
        &self.min
    }

    /// Returns the vertex with maximal coordinates on all axis of the AABB.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::aabb::Aabb;
    /// use crab_rt::vec::Vec3;
    ///
    /// let bbox = Aabb::new(Vec3::new(1., 2., 3.), Vec3::new(4., 5., 6.));
    /// assert_eq!(bbox.max(), &Vec3::new(4., 5., 6.));
    /// ```
    #[inline]
    #[must_use]
    pub const fn max(&self) -> &Vec3 {
        &self.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aabb_new() {
        let testee = Aabb::new(Vec3::new(1., 2., 3.), Vec3::new(4., 5., 6.));

        assert_eq!(testee.min(), &Vec3::new(1., 2., 3.));
        assert_eq!(testee.max(), &Vec3::new(4., 5., 6.));
    }

    #[test]
    fn surrounding_box_not_intersecting() {
        let bbox0 = Aabb::new(Vec3::new(1., 2., 3.), Vec3::new(4., 5., 6.));
        let bbox1 = Aabb::new(Vec3::new(-4., -5., -6.), Vec3::new(-1., -2., -3.));

        let testee = Aabb::surrounding_box(&bbox0, &bbox1);

        assert_eq!(testee.min(), &Vec3::new(-4., -5., -6.));
        assert_eq!(testee.max(), &Vec3::new(4., 5., 6.));
    }

    #[test]
    fn surrounding_box_overlapping() {
        let bbox0 = Aabb::new(Vec3::new(1., 2., 3.), Vec3::new(4., 5., 6.));
        let bbox1 = Aabb::new(Vec3::new(0., 3., 3.), Vec3::new(3., 6., 6.));

        let testee = Aabb::surrounding_box(&bbox0, &bbox1);

        assert_eq!(testee.min(), &Vec3::new(0., 2., 3.));
        assert_eq!(testee.max(), &Vec3::new(4., 6., 6.));
    }

    #[test]
    fn surrounding_box_containing() {
        let bbox0 = Aabb::new(Vec3::new(1., 2., 3.), Vec3::new(4., 5., 6.));
        let bbox1 = Aabb::new(Vec3::new(2., 3., 4.), Vec3::new(3., 4., 5.));

        let testee = Aabb::surrounding_box(&bbox0, &bbox1);

        assert_eq!(testee.min(), &Vec3::new(1., 2., 3.));
        assert_eq!(testee.max(), &Vec3::new(4., 5., 6.));
    }
}
