use criterion::{criterion_group, criterion_main, Criterion};

use crab_rt::camera::Camera;
use crab_rt::materials::{Dielectric, Lambertian, Metal};
use crab_rt::objects::{Object, Sphere};
use crab_rt::raytracer::RayTracer;
use crab_rt::scene::{Background, Scene};
use crab_rt::vec::Vec3;

fn raytrace() {
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

    let raytracer = RayTracer::new(200, 100, 100, 50, camera, scene);

    raytracer.raytrace();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("benchmark-group");
    group.sample_size(10);

    group.bench_function("raytrace", |b| b.iter(|| raytrace()));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
