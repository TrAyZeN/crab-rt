use crab_rt::camera::Camera;
use crab_rt::materials::{Dielectric, Lambertian, Metal};
use crab_rt::objects::Sphere;
use crab_rt::raytracer::RayTracer;
use crab_rt::scene::{Background, SceneBuilder};
use crab_rt::textures::Checker;
use crab_rt::vec::{Color3, Point3, Vec3};

const WIDTH: u32 = 600;
const HEIGHT: u32 = 300;

fn main() {
    // TODO: Remove that
    let start = std::time::Instant::now();

    let raytracer = raytracer1();

    raytracer
        .raytrace()
        .lock()
        .unwrap()
        .save("out.png")
        .unwrap();

    println!("Done in {:?}", start.elapsed());
}

fn raytracer1() -> RayTracer {
    let camera = Camera::new(
        Point3::new(4., 2., 4.),
        Point3::new(0., 0., -1.),
        20.,
        WIDTH as f32 / HEIGHT as f32,
    );
    // .aperture(1.);

    let scene = SceneBuilder::new(Background::Gradient(
        Vec3::new(0.5, 0.7, 1.),
        Vec3::new(1., 1., 1.),
    ))
    .add_sphere(Sphere::new(
        Vec3::new(0., 0., -1.),
        0.5,
        Lambertian::from_rgb(0.1, 0.2, 0.5),
    ))
    .add_sphere(Sphere::new(
        Vec3::new(0., -100.5, -1.),
        100.,
        Lambertian::new(Checker::from_colors(
            Color3::new(1., 1., 1.),
            Color3::new(0.5, 0.1, 0.8),
        )),
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
    // .add_sphere(Sphere::new(
    //     Vec3::new(-1., 0., -1.),
    //     -0.45,
    //     Dielectric::new(1.5),
    // ))
    .build();

    RayTracer::new(WIDTH, HEIGHT, 200, 50, camera, scene)
}
