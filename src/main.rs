use crab_rt::camera::Camera;
use crab_rt::materials::{Dielectric, Lambertian, Metal};
use crab_rt::objects::Sphere;
use crab_rt::raytracer::RayTracer;
use crab_rt::scene::{Background, SceneBuilder};
use crab_rt::textures::{Checker, Monochrome};
use crab_rt::vec::{Point3, Vec3};

const WIDTH: usize = 600;
const HEIGHT: usize = 300;

fn main() {
    // TODO: Remove that
    let start = std::time::Instant::now();

    let camera = Camera::new(
        Point3::new(3., 3., 2.),
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
        Lambertian::new(Box::new(Checker::new(
            Box::new(Monochrome::from_rgb(1., 1., 1.)),
            Box::new(Monochrome::from_rgb(0.5, 0.1, 0.8)),
        ))),
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

    let raytracer = RayTracer::new(WIDTH, HEIGHT, 200, 50, camera, scene);

    raytracer
        .raytrace()
        .lock()
        .unwrap()
        .save("out.png")
        .unwrap();

    println!("Done in {:?}", start.elapsed());
}
