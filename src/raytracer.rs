use core_affinity;
use image::{ImageBuffer, RgbImage};
use rand::Rng;

use std::sync::{Arc, Mutex};
use std::thread;

use crate::camera::Camera;
use crate::hitable::Hitable;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::utils::{gamma_encode, rng};
use crate::vec::{Color3, Vec3};

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

    // Seems faster when returning Arc<Mutex
    // pub fn raytrace(self) -> RgbImage {
    #[must_use]
    pub fn raytrace(self) -> Arc<Mutex<RgbImage>> {
        let core_ids = core_affinity::get_core_ids();
        if core_ids.is_none() {
            println!("Failed to get core ids");
        }

        let raytracer = Arc::new(self);
        // let image_pixels: Arc<Mutex<Vec<Rgb<u8>>>> = Arc::new(Mutex::new(vec![
        //     Rgb([0, 0, 0]);
        //     raytracer.get_width()
        //         * raytracer
        //             .get_height()
        // ]));
        let image_pixels = Arc::new(Mutex::new(ImageBuffer::new(
            raytracer.get_width(),
            raytracer.get_height(),
        )));

        let mut workers = Vec::with_capacity(NB_THREADS);

        for i in 0..NB_THREADS {
            let raytracer = Arc::clone(&raytracer);
            let image_pixels = Arc::clone(&image_pixels);
            let core_id = core_ids.as_ref().map(|ids| ids[i]);

            workers.push(thread::spawn(move || {
                if let Some(id) = core_id {
                    core_affinity::set_for_current(id);
                }

                let mut line_pixels = vec![Vec3::default(); raytracer.get_width() as usize];

                for y in (i * raytracer.get_height() as usize / NB_THREADS)
                    ..((i + 1) * raytracer.get_height() as usize / NB_THREADS)
                {
                    for (x, pixel) in line_pixels.iter_mut().enumerate() {
                        *pixel = raytracer.pixel(x, y);
                    }

                    let mut image_pixels = image_pixels.lock().unwrap();
                    for (x, pixel) in line_pixels.iter().enumerate() {
                        // image_pixels[x + y * raytracer.get_width()] = line_pixels[x].into();
                        image_pixels.put_pixel(x as u32, y as u32, pixel.into());
                    }
                }
            }));
        }

        for worker in workers {
            worker.join().expect("Failed to join thread.");
        }

        // Maybe perf lost :thinking:
        // let image_pixels = image_pixels.lock().unwrap();
        // ImageBuffer::from_fn(
        //     raytracer.get_width() as u32,
        //     raytracer.get_height() as u32,
        //     move |x, y| image_pixels[x as usize + y as usize * raytracer.get_width()],
        // )
        image_pixels
    }

    #[inline(always)]
    #[must_use]
    fn pixel(&self, x: usize, y: usize) -> Color3 {
        let mut rng = rng();
        let y = self.height as usize - y - 1;

        let color = (0..self.samples)
            .into_iter()
            .map(|_| {
                let u = (x as f32 + rng.gen::<f32>()) / self.width as f32;
                let v = (y as f32 + rng.gen::<f32>()) / self.height as f32;

                let ray = self.camera.get_ray(u, v);

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

        let record = self.scene.get_bvh().hit(ray, 0.001, f32::INFINITY);
        if record.is_none() {
            let unit_direction = ray.get_direction().unit();
            let t = 0.5 * (unit_direction.y + 1.);
            return self.scene.get_background().color(t);
        }

        let record = record.unwrap();
        let emitted = record
            .get_material()
            .emitted(record.get_texture_coordinates(), record.get_hit_point());

        let record = record.get_material().scatter(ray, &record);
        if record.is_none() {
            return emitted;
        }

        let (scattered, attenuation) = record.unwrap();
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
    /// assert_eq!(raytracer.get_width(), 200);
    /// ```
    #[inline]
    #[must_use]
    pub const fn get_width(&self) -> u32 {
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
    /// assert_eq!(raytracer.get_height(), 100);
    /// ```
    #[inline]
    #[must_use]
    pub const fn get_height(&self) -> u32 {
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
    /// assert_eq!(raytracer.get_samples(), 50);
    /// ```
    #[inline]
    #[must_use]
    pub const fn get_samples(&self) -> usize {
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
    /// assert_eq!(raytracer.get_max_reflections(), 20);
    /// ```
    #[inline]
    #[must_use]
    pub const fn get_max_reflections(&self) -> usize {
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
    /// assert_eq!(raytracer.get_camera(), &Camera::default());
    /// ```
    #[inline]
    #[must_use]
    pub const fn get_camera(&self) -> &Camera {
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
    /// assert_eq!(raytracer.get_scene(), &Scene::default());
    /// ```
    #[inline]
    #[must_use]
    pub const fn get_scene(&self) -> &Scene {
        &self.scene
    }
}
