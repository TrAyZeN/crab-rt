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
    focus_dist: f32,
    uniform_time: Uniform<f32>,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vfov: f32, aspect_ratio: f32) -> Self {
        let theta = f32::to_radians(vfov);
        let half_height = f32::tan(theta / 2.);
        let viewport_height = 2. * half_height;
        let viewport_width = aspect_ratio * viewport_height;

        let vup = Vec3::new(0., 1., 0.);
        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let focus_dist = (lookfrom - lookat).length();
        let origin = lookfrom;
        let horizontal = viewport_width * focus_dist * u;
        let vertical = viewport_height * focus_dist * v;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2. - vertical / 2. - focus_dist * w,
            u,
            v,
            w,
            lens_radius: 0.,
            focus_dist,
            uniform_time: Uniform::from(0.0..0.1),
        }
    }

    pub fn vup(self, vup: Vec3) -> Self {
        assert_ne!(vup, Vec3::zero());

        let u = vup.cross(&self.w).unit();
        let v = self.w.cross(&u);

        let horizontal = self.horizontal / self.u * u;
        let vertical = self.vertical / self.v * v;

        Self {
            horizontal,
            vertical,
            lower_left_corner: self.origin
                - horizontal / 2.
                - vertical / 2.
                - self.focus_dist * self.w,
            u,
            v,
            ..self
        }
    }

    pub fn aperture(self, aperture: f32) -> Self {
        assert!(aperture >= 0.);

        Self {
            lens_radius: aperture / 2.,
            ..self
        }
    }

    pub fn focus_dist(self, focus_dist: f32) -> Self {
        assert_ne!(focus_dist, 0.);

        let horizontal = self.horizontal / self.focus_dist * focus_dist;
        let vertical = self.vertical / self.focus_dist * focus_dist;

        Self {
            horizontal,
            vertical,
            lower_left_corner: self.origin - horizontal / 2. - vertical / 2. - focus_dist * self.w,
            focus_dist,
            ..self
        }
    }

    pub fn time_interval(self, time_interval: (f32, f32)) -> Self {
        Self {
            uniform_time: Uniform::from(time_interval.0..time_interval.1),
            ..self
        }
    }

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

impl Default for Camera {
    #[inline]
    fn default() -> Self {
        Self::new(
            Vec3::default(),
            Vec3::default(),
            f32::default(),
            f32::default(),
        )
    }
}

impl PartialEq for Camera {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin
            && self.lower_left_corner == other.lower_left_corner
            && self.horizontal == other.horizontal
            && self.vertical == other.vertical
            && self.u == other.u
            && self.v == other.v
            && self.w == other.w
            && self.lens_radius == other.lens_radius
        // TODO: Find a way to compare uniforms
    }
}
