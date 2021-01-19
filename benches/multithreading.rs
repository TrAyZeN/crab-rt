use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use image::{ImageBuffer, RgbImage};
use rand::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

use crab_rt::camera::Camera;
use crab_rt::materials::{Dielectric, Lambertian, Metal};
use crab_rt::objects::{Object, Sphere};
use crab_rt::raytracer::RayTracer;
use crab_rt::scene::{Background, Scene};
use crab_rt::vec::Vec3;

fn bench_singlethreaded(c: &mut Criterion) {
    let mut group = c.benchmark_group("singlethreaded");
    group.sample_size(10);

    group.bench_function("singlethread", |b| b.iter(|| singlethread()));

    group.finish();
}

fn singlethread() {
    let raytracer = sample_raytracer();
    let mut image: RgbImage =
        ImageBuffer::new(raytracer.get_width() as u32, raytracer.get_height() as u32);

    let mut rng = thread_rng();
    for y in (0..raytracer.get_height()).rev() {
        for x in 0..raytracer.get_width() {
            let mut col = Vec3::default();
            for _ in 0..raytracer.get_samples() {
                let u = (x as f32 + rng.gen::<f32>()) / raytracer.get_width() as f32;
                let v = ((raytracer.get_height() - y - 1) as f32 + rng.gen::<f32>())
                    / raytracer.get_height() as f32;

                let r = raytracer.get_camera().get_ray(u, v);
                col += raytracer.cast(&r, 0);
            }
            col /= raytracer.get_samples() as f32;

            image.put_pixel(
                x as u32,
                y as u32,
                Vec3::new(f32::sqrt(col.x), f32::sqrt(col.y), f32::sqrt(col.z)).into(),
            );
        }
    }
}

fn bench_multithreaded(c: &mut Criterion) {
    let mut group = c.benchmark_group("multithreaded");
    group.sample_size(10);

    for nb_threads in [4, 10, 100].iter() {
        group.bench_with_input(
            BenchmarkId::new("multithread-write-chunk", nb_threads),
            nb_threads,
            |b, &n| {
                b.iter(|| multithread_write_chunk(n));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("multithread-write-line", nb_threads),
            nb_threads,
            |b, &n| {
                b.iter(|| multithread_write_line(n));
            },
        );
    }

    group.finish();
}

fn multithread_write_chunk(nb_threads: usize) {
    let raytracer = Arc::new(sample_raytracer());
    let image: Arc<Mutex<RgbImage>> = Arc::new(Mutex::new(ImageBuffer::new(
        raytracer.get_width() as u32,
        raytracer.get_height() as u32,
    )));

    let mut workers = Vec::with_capacity(nb_threads);

    for i in 0..nb_threads {
        let raytracer = Arc::clone(&raytracer);
        let image = Arc::clone(&image);

        workers.push(thread::spawn(move || {
            let mut rng = thread_rng();
            let mut colors =
                vec![Vec3::default(); raytracer.get_width() * raytracer.get_height() / nb_threads];

            let mut ci = 0;
            for y in ((i * raytracer.get_height() / nb_threads)
                ..((i + 1) * raytracer.get_height() / nb_threads))
                .rev()
            {
                for x in 0..raytracer.get_width() {
                    let mut col = Vec3::default();
                    for _ in 0..raytracer.get_samples() {
                        let u = (x as f32 + rng.gen::<f32>()) / raytracer.get_width() as f32;
                        let v = ((raytracer.get_height() - y - 1) as f32 + rng.gen::<f32>())
                            / raytracer.get_height() as f32;

                        let r = raytracer.get_camera().get_ray(u, v);
                        col += raytracer.cast(&r, 0);
                    }
                    col /= raytracer.get_samples() as f32;

                    colors[ci] = Vec3::new(f32::sqrt(col.x), f32::sqrt(col.y), f32::sqrt(col.z));
                    ci += 1;
                }
            }

            let mut x = 0;
            let mut y = ((i + 1) * raytracer.get_height() / nb_threads) - 1;
            let mut image = image.lock().unwrap();
            for ci in 0..colors.len() {
                image.put_pixel(x as u32, y as u32, colors[ci].into());
                x += 1;
                if x == raytracer.get_width() {
                    x = 0;
                    y -= 1;
                }
            }
        }));
    }

    for worker in workers {
        let _ = worker.join();
    }
}

fn multithread_write_line(nb_threads: usize) {
    let raytracer = Arc::new(sample_raytracer());
    let image: Arc<Mutex<RgbImage>> = Arc::new(Mutex::new(ImageBuffer::new(
        raytracer.get_width() as u32,
        raytracer.get_height() as u32,
    )));

    let mut workers = Vec::with_capacity(nb_threads);

    for i in 0..nb_threads {
        let raytracer = Arc::clone(&raytracer);
        let image = Arc::clone(&image);

        workers.push(thread::spawn(move || {
            let mut rng = thread_rng();

            let mut colors = vec![Vec3::default(); raytracer.get_width()];

            for y in ((i * raytracer.get_height() / nb_threads)
                ..((i + 1) * raytracer.get_height() / nb_threads))
                .rev()
            {
                for x in 0..raytracer.get_width() {
                    let mut col = Vec3::default();
                    for _ in 0..raytracer.get_samples() {
                        let u = (x as f32 + rng.gen::<f32>()) / raytracer.get_width() as f32;
                        let v = ((raytracer.get_height() - y - 1) as f32 + rng.gen::<f32>())
                            / raytracer.get_height() as f32;

                        let r = raytracer.get_camera().get_ray(u, v);
                        col += raytracer.cast(&r, 0);
                    }
                    col /= raytracer.get_samples() as f32;
                    colors[x] = Vec3::new(f32::sqrt(col.x), f32::sqrt(col.y), f32::sqrt(col.z));
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

criterion_group!(benches, bench_singlethreaded, bench_multithreaded);
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
        (0., 0.1),
    );

    let mut scene = Scene::new(Background::Gradient(
        Vec3::new(0.5, 0.7, 1.),
        Vec3::new(1., 1., 1.),
    ));
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

    RayTracer::new(200, 100, 100, 50, camera, scene)
}
