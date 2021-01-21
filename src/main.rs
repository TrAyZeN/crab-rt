use crab_rt::camera::Camera;
use crab_rt::materials::{Dielectric, Lambertian, Metal};
use crab_rt::objects::{Object, Sphere};
use crab_rt::raytracer::RayTracer;
use crab_rt::scene::{Background, SceneBuilder};
use crab_rt::vec::Vec3;

const WIDTH: usize = 600;
const HEIGHT: usize = 300;

fn main() {
    // TODO: Remove that
    let start = std::time::Instant::now();

    let camera = Camera::new(
        Vec3::new(3., 3., 2.),
        Vec3::new(0., 0., -1.),
        20.,
        WIDTH as f32 / HEIGHT as f32,
    )
    .aperture(1.);

    let scene = SceneBuilder::new(Background::Gradient(
        Vec3::new(0.5, 0.7, 1.),
        Vec3::new(1., 1., 1.),
    ))
    .add_sphere(Sphere::new(
        Vec3::new(0., 0., -1.),
        0.5,
        Lambertian::new(Vec3::new(0.1, 0.2, 0.5)),
    ))
    .add_sphere(Sphere::new(
        Vec3::new(0., -100.5, -1.),
        100.,
        Lambertian::new(Vec3::new(0.8, 0.8, 0.)),
    ))
    .add_sphere(Sphere::new(
        Vec3::new(1., 0., -1.),
        0.5,
        Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.),
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

    println!("{:#?}", scene);

    let raytracer = RayTracer::new(WIDTH, HEIGHT, 100, 50, camera, scene);

    raytracer
        .raytrace()
        .lock()
        .unwrap()
        .save("out.png")
        .unwrap();

    println!("Done in {:?}", start.elapsed());
}
