use crate::camera::Camera;
use crate::hitable::Hitable;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vec::Vec3;
use image::{ImageBuffer, Rgb, RgbImage};
use rand::{prelude::*, thread_rng};
use std::sync::{Arc, Mutex};
use std::thread;

const NB_THREADS: usize = 10;

pub struct RayTracer {
    width: usize,
    height: usize,

    samples: usize,
    max_reflections: usize,

    camera: Camera,
    scene: Scene,
}

impl RayTracer {
    #[inline]
    pub const fn new(
        width: usize,
        height: usize,
        samples: usize,
        max_reflections: usize,
        camera: Camera,
        scene: Scene,
    ) -> RayTracer {
        RayTracer {
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
    pub fn raytrace(self) -> Arc<Mutex<RgbImage>> {
        let raytracer = Arc::new(self);
        // let image_pixels: Arc<Mutex<Vec<Rgb<u8>>>> = Arc::new(Mutex::new(vec![
        //     Rgb([0, 0, 0]);
        //     raytracer.get_width()
        //         * raytracer
        //             .get_height()
        // ]));
        let image_pixels = Arc::new(Mutex::new(ImageBuffer::new(
            raytracer.get_width() as u32,
            raytracer.get_height() as u32,
        )));

        // TODO: Replace that lol
        let nb_threads = NB_THREADS;
        let mut workers = Vec::with_capacity(nb_threads);

        for i in 0..nb_threads {
            let raytracer = Arc::clone(&raytracer);
            let image_pixels = Arc::clone(&image_pixels);

            workers.push(thread::spawn(move || {
                let mut rng = thread_rng();
                let mut line_pixels = vec![Vec3::default(); raytracer.get_width()];

                for y in (i * raytracer.get_height() / nb_threads)
                    ..((i + 1) * raytracer.get_height() / nb_threads)
                {
                    for x in 0..raytracer.get_width() {
                        let mut color = Vec3::default();
                        for _ in 0..raytracer.get_samples() {
                            let u = (x as f32 + rng.gen::<f32>()) / raytracer.get_width() as f32;
                            let v = ((raytracer.get_height() - y - 1) as f32 + rng.gen::<f32>())
                                / raytracer.get_height() as f32;

                            let ray = raytracer.get_camera().get_ray(u, v);

                            color += raytracer.cast(&ray, 0);
                        }
                        color /= raytracer.get_samples() as f32;

                        // We gamma correct the color
                        line_pixels[x] =
                            Vec3::new(f32::sqrt(color.x), f32::sqrt(color.y), f32::sqrt(color.z));
                    }

                    let mut image_pixels = image_pixels.lock().unwrap();
                    for x in 0..raytracer.get_width() {
                        // image_pixels[x + y * raytracer.get_width()] = line_pixels[x].into();
                        image_pixels.put_pixel(x as u32, y as u32, line_pixels[x].into());
                    }
                }
            }));
        }

        for worker in workers {
            let _ = worker.join();
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

    // pub fn raytrace_old(&self) -> RgbImage {
    //     let mut image: RgbImage = ImageBuffer::new(self.width as u32, self.height as u32);
    //     let mut rng = thread_rng();

    //     for y in (0..self.height).rev() {
    //         for x in 0..self.width {
    //             let mut col = Vec3::default();
    //             for _ in 0..self.samples {
    //                 let u = (x as f32 + rng.gen::<f32>()) / self.width as f32;
    //                 let v = ((self.height - y - 1) as f32 + rng.gen::<f32>())
    //                     / self.height as f32;

    //                 let r = self.camera.get_ray(u, v);
    //                 col += self.cast(&r, 0);
    //             }
    //             col /= self.samples as f32;

    //             image.put_pixel(
    //                 x as u32,
    //                 y as u32,
    //                 Vec3::new(f32::sqrt(col.x), f32::sqrt(col.y), f32::sqrt(col.z)).into(),
    //             );
    //         }
    //     }

    //     image
    // }

    pub fn cast(&self, ray: &Ray, depth: usize) -> Vec3 {
        if depth >= self.max_reflections {
            return Vec3::zero();
        }

        if let Some(record) = self.scene.get_objects().hit(ray, 0.001, f32::INFINITY) {
            if let Some((scattered, attenuation)) = record.get_material().scatter(ray, &record) {
                return attenuation * self.cast(&scattered, depth + 1);
            }
        } else {
            let unit_direction = ray.get_direction().unit();
            let t = 0.5 * (unit_direction.y + 1.);
            return (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.);
        }

        Vec3::zero()
    }

    /// Returns the width of the rendering window.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::raytracer::RayTracer;
    ///
    /// let raytracer = RayTracer::new(200, 100, 50, 20, camera, scene);
    /// assert_eq!(raytracer.get_width(), 200);
    /// ```
    #[inline]
    pub const fn get_width(&self) -> usize {
        self.width
    }

    /// Returns the height of the rendering window.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::raytracer::RayTracer;
    ///
    /// let raytracer = RayTracer::new(200, 100, 50, 20, camera, scene);
    /// assert_eq!(raytracer.get_height(), 100);
    /// ```
    #[inline]
    pub const fn get_height(&self) -> usize {
        self.height
    }

    /// Returns the number of samples per pixels.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::raytracer::RayTraycer;
    ///
    /// let raytracer = RayTracer::new(200, 100, 50, 20, camera, scene);
    /// assert_eq!(raytracer.get_height(), 50);
    /// ```
    #[inline]
    pub const fn get_samples(&self) -> usize {
        self.samples
    }

    #[inline]
    pub const fn get_max_reflections(&self) -> usize {
        self.max_reflections
    }

    #[inline]
    pub const fn get_camera(&self) -> &Camera {
        &self.camera
    }

    #[inline]
    pub const fn get_scene(&self) -> &Scene {
        &self.scene
    }
}

// pub struct RayTracerBuilder {
//     width: Option<usize>,
//     height: Option<usize>,

//     samples: Option<usize>,
//     max_reflections: Option<usize>,

//     camera: Option<Camera>,
//     scene: Option<Scene>,
// }
