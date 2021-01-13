use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

use crab_rt::camera::Camera;
use crab_rt::materials::{Dielectric, Lambertian, Metal};
use crab_rt::objects::{Object, Sphere};
use crab_rt::raytracer::RayTracer;
use crab_rt::scene::Scene;
use crab_rt::vec::Vec3;

fn main() {
    let aspect_ratio = 3. / 2.;
    let image_width = 1200;
    let image_height = (image_width as f32 / aspect_ratio) as usize;
    let samples_per_pixel = 500;
    let max_reflections = 50;

    let scene = random_scene();

    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.,
        aspect_ratio,
        aperture,
        dist_to_focus,
        (0., 1.),
    );

    RayTracer::new(
        image_width,
        image_height,
        samples_per_pixel,
        max_reflections,
        camera,
        scene,
    )
    .raytrace()
    .lock()
    .unwrap()
    .save("rt_weekend.png")
    .unwrap();
}

fn random_scene() -> Scene {
    let mut scene = Scene::new();
    let uniform1 = Uniform::from(0.0..0.9);
    let uniform2 = Uniform::from(0.5..1.0);
    let mut rng = rand::thread_rng();

    scene.add_object(Object::new(Box::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Lambertian::new(Vec3::new(0.5, 0.5, 0.5)),
    ))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Vec3::new(
                a as f32 + uniform1.sample(&mut rng),
                0.2,
                b as f32 + uniform1.sample(&mut rng),
            );

            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    scene.add_object(Object::new(Box::new(Sphere::new(
                        center,
                        0.2,
                        Lambertian::new(Vec3::new(
                            rng.gen::<f32>(),
                            rng.gen::<f32>(),
                            rng.gen::<f32>(),
                        )),
                    ))))
                } else if choose_mat < 0.95 {
                    scene.add_object(Object::new(Box::new(Sphere::new(
                        center,
                        0.2,
                        Metal::new(
                            Vec3::new(
                                uniform2.sample(&mut rng),
                                uniform2.sample(&mut rng),
                                uniform2.sample(&mut rng),
                            ),
                            rng.gen::<f32>() * 0.5,
                        ),
                    ))))
                } else {
                    scene.add_object(Object::new(Box::new(Sphere::new(
                        center,
                        0.2,
                        Dielectric::new(1.5),
                    ))))
                }
            }
        }
    }

    scene.add_object(Object::new(Box::new(Sphere::new(
        Vec3::new(0., 1., 0.),
        1.,
        Dielectric::new(1.5),
    ))));

    scene.add_object(Object::new(Box::new(Sphere::new(
        Vec3::new(-4., 1., 0.),
        1.,
        Lambertian::new(Vec3::new(0.4, 0.2, 0.1)),
    ))));

    scene.add_object(Object::new(Box::new(Sphere::new(
        Vec3::new(4., 1., 0.),
        1.,
        Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.),
    ))));

    scene
}
