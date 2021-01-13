use rand::distributions::{Distribution, Uniform};

use crate::ray::Ray;
use crate::utils::random_in_unit_disk;
use crate::vec::Vec3;

#[derive(Debug, Clone)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
    uniform_time: Uniform<f32>,
}

impl Camera {
    #[inline]
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
        time_interval: (f32, f32),
    ) -> Self {
        let theta = f32::to_radians(vfov);
        let half_height = f32::tan(theta / 2.);
        let viewport_height = 2. * half_height;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = viewport_width * focus_dist * u;
        let vertical = viewport_height * focus_dist * v;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2. - vertical / 2. - focus_dist * w,
            u,
            v,
            w,
            lens_radius: aperture / 2.,
            uniform_time: Uniform::from(time_interval.0..time_interval.1),
        }
    }

    #[inline]
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let mut rng = rand::thread_rng();
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            self.uniform_time.sample(&mut rng),
        )
    }
}
