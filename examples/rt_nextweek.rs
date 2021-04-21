use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

use crab_rt::camera::Camera;
use crab_rt::materials::{Dielectric, Lambertian, Light, Metal};
use crab_rt::objects::{MovingSphere, Object, Sphere, XyRect, XzRect, YzRect};
use crab_rt::raytracer::RayTracer;
use crab_rt::scene::{Background, Scene, SceneBuilder};
use crab_rt::textures::{Checker, Image, Monochrome, Noise};
use crab_rt::vec::{Color3, Point3, Vec3};

fn main() {
    let mut aspect_ratio = 16. / 9.;
    let mut image_width = 400;
    let mut samples_per_pixel = 400;
    let max_reflections = 50;

    let scene_number = 6;

    let (camera, scene) = match scene_number {
        1 => (
            Camera::new(
                Point3::new(13., 2., 3.),
                Point3::new(0., 0., 0.),
                20.,
                aspect_ratio,
            )
            .aperture(0.1)
            .focus_dist(10.)
            .time_interval((0., 1.)),
            random_scene(),
        ),
        2 => (
            Camera::new(
                Point3::new(13., 2., 13.),
                Point3::new(0., 0., 0.),
                20.,
                aspect_ratio,
            ),
            two_spheres(),
        ),
        3 => (
            Camera::new(
                Point3::new(13., 2., 13.),
                Point3::new(0., 0., 0.),
                20.,
                aspect_ratio,
            ),
            two_perlin_spheres(),
        ),
        4 => (
            Camera::new(
                Point3::new(13., 2., 13.),
                Point3::new(0., 0., 0.),
                20.,
                aspect_ratio,
            ),
            earth(),
        ),
        5 => (
            Camera::new(
                Point3::new(26., 3., 6.),
                Point3::new(0., 2., 0.),
                20.,
                aspect_ratio,
            )
            .focus_dist(10.),
            simple_light(),
        ),
        _ => {
            aspect_ratio = 1.;
            image_width = 600;
            samples_per_pixel = 200;
            (
                Camera::new(
                    Point3::new(278., 278., -800.),
                    Point3::new(278., 278., 0.),
                    40.,
                    1.,
                ),
                cornell_box(),
            )
        }
    };

    let image_height = (image_width as f32 / aspect_ratio) as u32;
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
    .save("rt_nextweek.png")
    .unwrap();
}

fn random_scene() -> Scene {
    let mut objects = Vec::new();
    let uniform1 = Uniform::from(0.0..0.9);
    let uniform2 = Uniform::from(0.5..1.0);
    let mut rng = rand::thread_rng();

    objects.push(Object::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Lambertian::from_rgb(0.5, 0.5, 0.5),
    )));

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
                    let center2 = center + Vec3::new(0., rng.gen::<f32>() * 0.5, 0.);
                    objects.push(Object::new(MovingSphere::new(
                        (center, center2),
                        (0., 1.),
                        0.2,
                        Lambertian::from_rgb(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()),
                    )))
                } else if choose_mat < 0.95 {
                    objects.push(Object::new(Sphere::new(
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
                    )))
                } else {
                    objects.push(Object::new(Sphere::new(center, 0.2, Dielectric::new(1.5))))
                }
            }
        }
    }

    objects.push(Object::new(Sphere::new(
        Vec3::new(0., 1., 0.),
        1.,
        Dielectric::new(1.5),
    )));

    objects.push(Object::new(Sphere::new(
        Vec3::new(-4., 1., 0.),
        1.,
        Lambertian::from_rgb(0.4, 0.2, 0.1),
    )));

    objects.push(Object::new(Sphere::new(
        Vec3::new(4., 1., 0.),
        1.,
        Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.),
    )));

    Scene::new(
        objects,
        Background::Gradient(Vec3::new(0.5, 0.7, 1.), Vec3::new(1., 1., 1.)),
    )
}

fn two_spheres() -> Scene {
    SceneBuilder::new(Background::Gradient(
        Vec3::new(0.5, 0.7, 1.),
        Vec3::new(1., 1., 1.),
    ))
    .add_sphere(Sphere::new(
        Point3::new(0., -10., 0.),
        10.,
        Lambertian::new(Checker::from_colors(
            Color3::new(0.2, 0.3, 0.1),
            Color3::new(0.9, 0.9, 0.9),
        )),
    ))
    .add_sphere(Sphere::new(
        Point3::new(0., 10., 0.),
        10.,
        Lambertian::new(Checker::from_colors(
            Color3::new(0.2, 0.3, 0.1),
            Color3::new(0.9, 0.9, 0.9),
        )),
    ))
    .build()
}

fn two_perlin_spheres() -> Scene {
    SceneBuilder::new(Background::Color(Color3::new(0.5, 0.7, 1.)))
        .add_sphere(Sphere::new(
            Point3::new(0., -1000., 0.),
            1000.,
            Lambertian::new(Noise::new(4.)),
        ))
        .add_sphere(Sphere::new(
            Point3::new(0., 2., 0.),
            2.,
            Lambertian::new(Noise::new(4.)),
        ))
        .build()
}

fn earth() -> Scene {
    SceneBuilder::new(Background::Color(Vec3::new(0.5, 0.7, 1.)))
        .add_sphere(Sphere::new(
            Point3::new(0., 0., 0.),
            2.,
            Lambertian::new(Image::load("resources/earthmap.jpg")),
        ))
        .build()
}

fn simple_light() -> Scene {
    SceneBuilder::new(Background::Color(Vec3::new(0., 0., 0.)))
        .add_sphere(Sphere::new(
            Point3::new(0., -1000., 0.),
            1000.,
            Lambertian::new(Noise::new(4.)),
        ))
        .add_sphere(Sphere::new(
            Point3::new(0., 2., 0.),
            2.,
            Lambertian::new(Noise::new(4.)),
        ))
        .add_object(Object::new(XyRect::new(
            (3., 5.),
            (1., 3.),
            -2.,
            Light::new(Monochrome::from_rgb(4., 4., 4.)),
        )))
        .build()
}

fn cornell_box() -> Scene {
    SceneBuilder::new(Background::Color(Color3::new(0., 0., 0.)))
        .add_object(Object::new(YzRect::new(
            (0., 555.),
            (0., 555.),
            555.,
            Lambertian::from_rgb(0.12, 0.45, 0.15),
        )))
        .add_object(Object::new(YzRect::new(
            (0., 555.),
            (0., 555.),
            0.,
            Lambertian::from_rgb(0.65, 0.05, 0.05),
        )))
        .add_object(Object::new(XzRect::new(
            (213., 343.),
            (227., 332.),
            554.,
            Light::new(Monochrome::from_rgb(15., 15., 15.)),
        )))
        .add_object(Object::new(XzRect::new(
            (0., 555.),
            (0., 555.),
            0.,
            Lambertian::from_rgb(0.73, 0.73, 0.73),
        )))
        .add_object(Object::new(XzRect::new(
            (0., 555.),
            (0., 555.),
            555.,
            Lambertian::from_rgb(0.73, 0.73, 0.73),
        )))
        .add_object(Object::new(XyRect::new(
            (0., 555.),
            (0., 555.),
            555.,
            Lambertian::from_rgb(0.73, 0.73, 0.73),
        )))
        .build()
}
