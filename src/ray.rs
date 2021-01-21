use crate::vec::Vec3;

/// A mathematical ray.
#[derive(Debug)]
pub struct Ray {
    /// Origin point of the ray.
    origin: Vec3, // We could try to use a Cow here :thinking:
    /// Direction vector of the ray.
    direction: Vec3,
    /// The time when the ray was casted.
    time: f32,
}

impl Ray {
    /// Constructs a new `Ray` from the given origin, direction and time.
    ///
    /// # Panic
    /// Panics in `debug` mode if `direction == Vec3::new(0., 0., 0.)`.
    /// 
    /// # Examples
    /// ```
    /// use crab_rt::ray::Ray;
    /// use crab_rt::vec::Vec3;
    ///
    /// let ray = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(1., 2., 3.), 0.);
    /// ```
    #[inline]
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Self {
        debug_assert!(!direction.is_zero());

        // Should we force direction vector to be unit ?
        Ray {
            origin,
            direction,
            time,
        }
    }

    /// Returns the origin of the `Ray`.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::ray::Ray;
    /// use crab_rt::vec::Vec3;
    ///
    /// let ray = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(1., 2., 3.), 0.);
    /// assert_eq!(ray.get_origin(), &Vec3::new(0., 0., 0.));
    /// ```
    #[inline]
    pub const fn get_origin(&self) -> &Vec3 {
        &self.origin
    }

    /// Returns the direction of the `Ray`.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::ray::Ray;
    /// use crab_rt::vec::Vec3;
    ///
    /// let ray = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(1., 2., 3.), 0.);
    /// assert_eq!(ray.get_direction(), &Vec3::new(1., 2., 3.));
    /// ```
    #[inline]
    pub const fn get_direction(&self) -> &Vec3 {
        &self.direction
    }

    /// Returns the time when the `Ray` was casted.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::ray::Ray;
    /// use crab_rt::vec::Vec3;
    ///
    /// let ray = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(1., 2., 3.), 0.);
    /// assert_eq!(ray.get_time(), 0.);
    /// ```
    #[inline]
    pub const fn get_time(&self) -> f32 {
        self.time
    }

    /// Returns the point on the `Ray` at distance t.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::ray::Ray;
    /// use crab_rt::vec::Vec3;
    ///
    /// let ray = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(1., 2., 3.), 0.);
    /// assert_eq!(ray.point(2.), Vec3::new(2., 4., 6.));
    /// ```
    #[inline]
    pub fn point(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

// use core::ops;
// impl ops::Fn<f32> for Ray {
//     type Output = Vec3;
//     fn call(self, t: f32) -> Self::Output {
//         self.origin + self.direction * t
//     }
// }
