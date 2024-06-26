use criterion::{criterion_group, criterion_main, Criterion};
use image::{ImageBuffer, Rgb, RgbImage};
use rand::{prelude::*, thread_rng};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

use crab_rt::camera::Camera;
use crab_rt::materials::{Dielectric, Lambertian, Metal};
use crab_rt::objects::Sphere;
use crab_rt::raytracer::RayTracer;
use crab_rt::scene::{Background, SceneBuilder};
use crab_rt::vec::{Point3, Vec3};

fn bench_default(c: &mut Criterion) {
    let mut group = c.benchmark_group("default");
    group.sample_size(10);

    group.bench_function("default", |b| b.iter(|| raytrace_default()));

    group.finish();
}

fn raytrace_default() {
    sample_raytracer().raytrace();
}

fn bench_rayon(c: &mut Criterion) {
    let mut group = c.benchmark_group("rayon");
    group.sample_size(10);

    group.bench_function("singlethread", |b| b.iter(|| raytrace_rayon_singlethread()));
    group.bench_function("multithread", |b| b.iter(|| raytrace_rayon_multithread()));

    group.finish();
}

fn raytrace_rayon_singlethread() {
    let raytracer = sample_raytracer();

    let pixels: Vec<Vec<Rgb<u8>>> = (0..raytracer.height())
        .into_par_iter()
        .map(|y_rev| {
            let y = raytracer.height() - 1 - y_rev;
            (0..raytracer.width())
                .into_par_iter()
                .map(|x| {
                    let color = (0..raytracer.samples())
                        .into_par_iter()
                        .map(|_| {
                            let mut rng = thread_rng();

                            let u = (x as f32 + rng.gen::<f32>()) / raytracer.width() as f32;
                            let v = ((raytracer.height() - y - 1) as f32 + rng.gen::<f32>())
                                / raytracer.height() as f32;

                            let r = raytracer.camera().ray(u, v);
                            raytracer.cast(&r, 0)
                        })
                        .sum::<Vec3>()
                        / raytracer.samples() as f32;

                    Vec3::new(f32::sqrt(color.x), f32::sqrt(color.y), f32::sqrt(color.z)).into()
                })
                .collect()
        })
        .collect();

    let _image: RgbImage = ImageBuffer::from_fn(
        raytracer.width() as u32,
        raytracer.height() as u32,
        |x, y| pixels[y as usize][x as usize],
    );

    // let image: RgbImage = ImageBuffer::from_vec(
    //     raytracer.width() as u32,
    //     raytracer.height() as u32,
    //     pixels
    // ).unwrap();
}

fn raytrace_rayon_multithread() {
    let raytracer = Arc::new(sample_raytracer());
    let image: Arc<Mutex<RgbImage>> = Arc::new(Mutex::new(ImageBuffer::new(
        raytracer.width() as u32,
        raytracer.height() as u32,
    )));

    let nb_threads = 10;
    let mut workers = Vec::with_capacity(nb_threads);

    for i in 0..nb_threads {
        let raytracer = Arc::clone(&raytracer);
        let image = Arc::clone(&image);

        workers.push(thread::spawn(move || {
            let mut colors = vec![Vec3::default(); raytracer.width()];

            for y in ((i * raytracer.height() / nb_threads)
                ..((i + 1) * raytracer.height() / nb_threads))
                .rev()
            {
                for x in 0..raytracer.width() {
                    let color = (0..raytracer.samples())
                        .into_par_iter()
                        .map(|_| {
                            let mut rng = thread_rng();

                            let u = (x as f32 + rng.gen::<f32>()) / raytracer.width() as f32;
                            let v = ((raytracer.height() - y - 1) as f32 + rng.gen::<f32>())
                                / raytracer.height() as f32;

                            raytracer.cast(&raytracer.camera().ray(u, v), 0)
                        })
                        .sum::<Vec3>()
                        / raytracer.samples() as f32;
                    colors[x] =
                        Vec3::new(f32::sqrt(color.x), f32::sqrt(color.y), f32::sqrt(color.z));
                }

                let mut image = image.lock().unwrap();
                for x in 0..raytracer.width() {
                    image.put_pixel(x as u32, y as u32, colors[x].into());
                }
            }
        }));
    }

    for worker in workers {
        let _ = worker.join();
    }
}

criterion_group!(benches, bench_default, bench_rayon);
criterion_main!(benches);

fn sample_raytracer() -> RayTracer {
    let camera =
        Camera::new(Point3::new(3., 3., 2.), Point3::new(0., 0., -1.), 20., 2.).aperture(2.);

    let scene = SceneBuilder::new(Background::Gradient(
        Vec3::new(0.5, 0.7, 1.),
        Vec3::new(1., 1., 1.),
    ))
    .add_sphere(Sphere::new(
        Vec3::new(0., 0., -1.),
        0.5,
        Lambertian::from_rgb(0.8, 0.3, 0.3),
    ))
    .add_sphere(Sphere::new(
        Vec3::new(0., -100.5, -1.),
        100.,
        Lambertian::from_rgb(0.8, 0.8, 0.),
    ))
    .add_sphere(Sphere::new(
        Vec3::new(1., 0., -1.),
        0.5,
        Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0),
    ))
    .add_sphere(Sphere::new(
        Vec3::new(-1., 0., -1.),
        0.5,
        Dielectric::new(1.5),
    ))
    .add_sphere(Sphere::new(
        Vec3::new(-1., 0., -1.),
        -0.45,
        Dielectric::new(1.5),
    ))
    .build();

    RayTracer::new(600, 300, 100, 50, camera, scene)
}
