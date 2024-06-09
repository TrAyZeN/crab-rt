use rand::Rng;

use crate::camera::Camera;
use crate::hitable::Hitable;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::utils::{gamma_encode, rng};
use crate::vec::{Color3, Vec3};

#[cfg(feature = "std")]
use {
    alloc::{vec, vec::Vec},
    core_affinity,
    image::{ImageBuffer, Rgb, RgbImage},
    std::println,
    std::sync::{Arc, Mutex},
    std::thread,
};

const NB_THREADS: usize = 8;

/// A renderer using raytracing to produce images.
#[derive(Debug)]
pub struct RayTracer {
    width: u32,
    height: u32,

    samples: usize,
    max_reflections: usize,

    camera: Camera,
    scene: Scene,
}

impl RayTracer {
    #[inline]
    #[must_use]
    pub const fn new(
        width: u32,
        height: u32,
        samples: usize,
        max_reflections: usize,
        camera: Camera,
        scene: Scene,
    ) -> Self {
        Self {
            width,
            height,
            samples,
            max_reflections,
            camera,
            scene,
        }
    }

    #[cfg(feature = "std")]
    #[must_use]
    pub fn raytrace(self) -> RgbImage {
        let core_ids = core_affinity::get_core_ids();
        if core_ids.is_none() {
            println!("Failed to get core ids");
        }

        let raytracer = Arc::new(self);
        let image_buffer = Arc::new(Mutex::new(vec![
            0u8;
            raytracer.width() as usize
                * raytracer.height() as usize
                * 3
        ]));

        let mut workers = Vec::with_capacity(NB_THREADS);

        for i in 0..NB_THREADS {
            let raytracer = Arc::clone(&raytracer);
            let image_buffer = Arc::clone(&image_buffer);
            let core_id = core_ids.as_ref().map(|ids| ids[i]);

            workers.push(thread::spawn(move || {
                if let Some(id) = core_id {
                    core_affinity::set_for_current(id);
                }

                let mut line_pixels = vec![Vec3::default(); raytracer.width() as usize];

                for y in (i * raytracer.height() as usize / NB_THREADS)
                    ..((i + 1) * raytracer.height() as usize / NB_THREADS)
                {
                    for (x, pixel) in line_pixels.iter_mut().enumerate() {
                        *pixel = raytracer.pixel(x, y);
                    }

                    let mut image_buffer = image_buffer.lock().unwrap();
                    for (x, pixel) in line_pixels.iter().enumerate() {
                        let pixel = Rgb::from(pixel);
                        image_buffer[(x + y * raytracer.width() as usize) * 3] = pixel[0];
                        image_buffer[(x + y * raytracer.width() as usize) * 3 + 1] = pixel[1];
                        image_buffer[(x + y * raytracer.width() as usize) * 3 + 2] = pixel[2];
                    }
                }
            }));
        }

        for worker in workers {
            worker.join().expect("Failed to join thread.");
        }

        ImageBuffer::from_vec(
            raytracer.width(),
            raytracer.height(),
            Arc::try_unwrap(image_buffer).unwrap().into_inner().unwrap(),
        )
        .unwrap()
    }

    #[inline(always)]
    #[must_use]
    fn pixel(&self, x: usize, y: usize) -> Color3 {
        let mut rng = rng();
        let y = self.height as usize - y - 1;

        let color = (0..self.samples)
            .map(|_| {
                let u = (x as f32 + rng.gen::<f32>()) / self.width as f32;
                let v = (y as f32 + rng.gen::<f32>()) / self.height as f32;

                let ray = self.camera.ray(u, v);

                self.cast(&ray, 0)
            })
            .sum::<Vec3>()
            / self.samples as f32;

        // We gamma correct the color
        Color3::new(
            gamma_encode(color.x),
            gamma_encode(color.y),
            gamma_encode(color.z),
        )
    }

    #[must_use]
    pub fn cast(&self, ray: &Ray, depth: usize) -> Color3 {
        if depth >= self.max_reflections {
            return Color3::zero();
        }

        let record = self.scene.bvh().hit(ray, 0.001, f32::INFINITY);
        let Some(record) = record else {
            let unit_direction = ray.direction().unit();
            let t = 0.5 * (unit_direction.y + 1.);
            return self.scene.background().color(t);
        };

        let emitted = record
            .material()
            .emitted(record.texture_coordinates(), record.hit_point());

        let record = record.material().scatter(ray, &record);
        let Some((scattered, attenuation)) = record else {
            return emitted;
        };

        emitted + attenuation * self.cast(&scattered, depth + 1)
    }

    /// Returns the width of the rendering window.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::camera::Camera;
    /// use crab_rt::raytracer::RayTracer;
    /// use crab_rt::scene::Scene;
    ///
    /// let raytracer = RayTracer::new(200, 100, 50, 20, Camera::default(), Scene::default());
    /// assert_eq!(raytracer.width(), 200);
    /// ```
    #[inline]
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the rendering window.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::camera::Camera;
    /// use crab_rt::raytracer::RayTracer;
    /// use crab_rt::scene::Scene;
    ///
    /// let raytracer = RayTracer::new(200, 100, 50, 20, Camera::default(), Scene::default());
    /// assert_eq!(raytracer.height(), 100);
    /// ```
    #[inline]
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Returns the number of samples per pixels.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::camera::Camera;
    /// use crab_rt::raytracer::RayTracer;
    /// use crab_rt::scene::Scene;
    ///
    /// let raytracer = RayTracer::new(200, 100, 50, 20, Camera::default(), Scene::default());
    /// assert_eq!(raytracer.samples(), 50);
    /// ```
    #[inline]
    #[must_use]
    pub const fn samples(&self) -> usize {
        self.samples
    }

    /// Returns the maximum number of reflections of a ray.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::camera::Camera;
    /// use crab_rt::raytracer::RayTracer;
    /// use crab_rt::scene::Scene;
    ///
    /// let raytracer = RayTracer::new(200, 100, 50, 20, Camera::default(), Scene::default());
    /// assert_eq!(raytracer.max_reflections(), 20);
    /// ```
    #[inline]
    #[must_use]
    pub const fn max_reflections(&self) -> usize {
        self.max_reflections
    }

    /// Returns the camera of the raytracer.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::camera::Camera;
    /// use crab_rt::raytracer::RayTracer;
    /// use crab_rt::scene::Scene;
    ///
    /// let raytracer = RayTracer::new(200, 100, 50, 20, Camera::default(), Scene::default());
    /// assert_eq!(raytracer.camera(), &Camera::default());
    /// ```
    #[inline]
    #[must_use]
    pub const fn camera(&self) -> &Camera {
        &self.camera
    }

    /// Returns the scene of the raytracer.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::camera::Camera;
    /// use crab_rt::raytracer::RayTracer;
    /// use crab_rt::scene::Scene;
    ///
    /// let raytracer = RayTracer::new(200, 100, 50, 20, Camera::default(), Scene::default());
    /// assert_eq!(raytracer.scene(), &Scene::default());
    /// ```
    #[inline]
    #[must_use]
    pub const fn scene(&self) -> &Scene {
        &self.scene
    }
}
