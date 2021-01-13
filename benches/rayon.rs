use criterion::{criterion_group, criterion_main, Criterion};
use image::{ImageBuffer, Rgb, RgbImage};
use rand::{prelude::*, thread_rng};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

use crab_rt::camera::Camera;
use crab_rt::materials::{Dielectric, Lambertian, Metal};
use crab_rt::objects::{Object, Sphere};
use crab_rt::raytracer::RayTracer;
use crab_rt::scene::Scene;
use crab_rt::vec::Vec3;

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

    let pixels: Vec<Vec<Rgb<u8>>> = (0..raytracer.get_height())
        .into_par_iter()
        .map(|y_rev| {
            let y = raytracer.get_height() - 1 - y_rev;
            (0..raytracer.get_width())
                .into_par_iter()
                .map(|x| {
                    let color = (0..raytracer.get_samples())
                        .into_par_iter()
                        .map(|_| {
                            let mut rng = thread_rng();

                            let u = (x as f32 + rng.gen::<f32>()) / raytracer.get_width() as f32;
                            let v = ((raytracer.get_height() - y - 1) as f32 + rng.gen::<f32>())
                                / raytracer.get_height() as f32;

                            let r = raytracer.get_camera().get_ray(u, v);
                            raytracer.cast(&r, 0)
                        })
                        .sum::<Vec3>()
                        / raytracer.get_samples() as f32;

                    Vec3::new(f32::sqrt(color.x), f32::sqrt(color.y), f32::sqrt(color.z)).into()
                })
                .collect()
        })
        .collect();

    let image: RgbImage = ImageBuffer::from_fn(
        raytracer.get_width() as u32,
        raytracer.get_height() as u32,
        |x, y| pixels[y as usize][x as usize],
    );

    // let image: RgbImage = ImageBuffer::from_vec(
    //     raytracer.get_width() as u32,
    //     raytracer.get_height() as u32,
    //     pixels
    // ).unwrap();
}

fn raytrace_rayon_multithread() {
    let raytracer = Arc::new(sample_raytracer());
    let image: Arc<Mutex<RgbImage>> = Arc::new(Mutex::new(ImageBuffer::new(
        raytracer.get_width() as u32,
        raytracer.get_height() as u32,
    )));

    let nb_threads = 10;
    let mut workers = Vec::with_capacity(nb_threads);

    for i in 0..nb_threads {
        let raytracer = Arc::clone(&raytracer);
        let image = Arc::clone(&image);

        workers.push(thread::spawn(move || {
            let mut colors = vec![Vec3::default(); raytracer.get_width()];

            for y in ((i * raytracer.get_height() / nb_threads)
                ..((i + 1) * raytracer.get_height() / nb_threads))
                .rev()
            {
                for x in 0..raytracer.get_width() {
                    let color = (0..raytracer.get_samples())
                        .into_par_iter()
                        .map(|_| {
                            let mut rng = thread_rng();

                            let u = (x as f32 + rng.gen::<f32>()) / raytracer.get_width() as f32;
                            let v = ((raytracer.get_height() - y - 1) as f32 + rng.gen::<f32>())
                                / raytracer.get_height() as f32;

                            raytracer.cast(&raytracer.get_camera().get_ray(u, v), 0)
                        })
                        .sum::<Vec3>()
                        / raytracer.get_samples() as f32;
                    colors[x] =
                        Vec3::new(f32::sqrt(color.x), f32::sqrt(color.y), f32::sqrt(color.z));
                }

                let mut image = image.lock().unwrap();
                for x in 0..raytracer.get_width() {
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
    let lookfrom = Vec3::new(3., 3., 2.);
    let lookat = Vec3::new(0., 0., -1.);
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0., 1., 0.),
        20.,
        2.,
        2.,
        (lookfrom - lookat).length(),
        (0., 0.),
    );

    let mut scene = Scene::new();
    scene.add_object(Object::new(Box::new(Sphere::new(
        Vec3::new(0., 0., -1.),
        0.5,
        Lambertian::new(Vec3::new(0.8, 0.3, 0.3)),
    ))));
    scene.add_object(Object::new(Box::new(Sphere::new(
        Vec3::new(0., -100.5, -1.),
        100.,
        Lambertian::new(Vec3::new(0.8, 0.8, 0.)),
    ))));
    scene.add_object(Object::new(Box::new(Sphere::new(
        Vec3::new(1., 0., -1.),
        0.5,
        Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0),
    ))));
    scene.add_object(Object::new(Box::new(Sphere::new(
        Vec3::new(-1., 0., -1.),
        0.5,
        Dielectric::new(1.5),
    ))));
    scene.add_object(Object::new(Box::new(Sphere::new(
        Vec3::new(-1., 0., -1.),
        -0.45,
        Dielectric::new(1.5),
    ))));

    RayTracer::new(600, 300, 100, 50, camera, scene)
}
