use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};
use std::sync::Arc;

use crab_rt::camera::Camera;
use crab_rt::materials::{Dielectric, Isotropic, Lambertian, Light, Metal};
use crab_rt::objects::{
    AaBox, ConstantMedium, MovingSphere, Object, RotateY, Sphere, Translate, XyRect, XzRect, YzRect,
};
use crab_rt::raytracer::RayTracer;
use crab_rt::scene::{Background, Scene, SceneBuilder};
use crab_rt::textures::{Checker, Image, Monochrome, Noise};
use crab_rt::vec::{Color3, Point3, Vec3};

fn main() {
    let start = std::time::Instant::now();

    let mut aspect_ratio = 16. / 9.;
    let mut image_width = 400;
    let mut samples_per_pixel = 400;
    let max_reflections = 50;

    let scene_number = 7;

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
        6 => {
            aspect_ratio = 1.;
            image_width = 600;
            samples_per_pixel = 200;
            (
                Camera::new(
                    Point3::new(278., 278., -800.),
                    Point3::new(278., 278., 0.),
                    40.,
                    aspect_ratio,
                ),
                cornell_box(),
            )
        }
        _ => {
            aspect_ratio = 1.;
            image_width = 600;
            samples_per_pixel = 200;
            (
                Camera::new(
                    Point3::new(278., 278., -800.),
                    Point3::new(278., 278., 0.),
                    40.,
                    aspect_ratio,
                ),
                cornell_smoke(),
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
    .save("rt_nextweek.jpg")
    .unwrap();

    println!("Done in {:?}", start.elapsed());
}

fn random_scene() -> Scene {
    let mut objects = Vec::new();
    let uniform1 = Uniform::from(0.0..0.9);
    let uniform2 = Uniform::from(0.5..1.0);
    let mut rng = rand::thread_rng();

    let dielectric_material = Arc::new(Dielectric::new(1.5));

    objects.push(Object::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::from_rgb(0.5, 0.5, 0.5)),
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
                        Arc::new(Lambertian::from_rgb(
                            rng.gen::<f32>(),
                            rng.gen::<f32>(),
                            rng.gen::<f32>(),
                        )),
                    )))
                } else if choose_mat < 0.95 {
                    objects.push(Object::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(
                            Vec3::new(
                                uniform2.sample(&mut rng),
                                uniform2.sample(&mut rng),
                                uniform2.sample(&mut rng),
                            ),
                            rng.gen::<f32>() * 0.5,
                        )),
                    )))
                } else {
                    objects.push(Object::new(Sphere::new(
                        center,
                        0.2,
                        dielectric_material.clone(),
                    )))
                }
            }
        }
    }

    objects.push(Object::new(Sphere::new(
        Vec3::new(0., 1., 0.),
        1.,
        dielectric_material,
    )));

    objects.push(Object::new(Sphere::new(
        Vec3::new(-4., 1., 0.),
        1.,
        Arc::new(Lambertian::from_rgb(0.4, 0.2, 0.1)),
    )));

    objects.push(Object::new(Sphere::new(
        Vec3::new(4., 1., 0.),
        1.,
        Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.)),
    )));

    Scene::new(
        objects,
        Background::Gradient(Vec3::new(0.5, 0.7, 1.), Vec3::new(1., 1., 1.)),
    )
}

fn two_spheres() -> Scene {
    let checker_material = Arc::new(Lambertian::new(Checker::from_colors(
        Color3::new(0.2, 0.3, 0.1),
        Color3::new(0.9, 0.9, 0.9),
    )));
    SceneBuilder::new(Background::Gradient(
        Vec3::new(0.5, 0.7, 1.),
        Vec3::new(1., 1., 1.),
    ))
    .add_sphere(Sphere::new(
        Point3::new(0., -10., 0.),
        10.,
        checker_material.clone(),
    ))
    .add_sphere(Sphere::new(Point3::new(0., 10., 0.), 10., checker_material))
    .build()
}

fn two_perlin_spheres() -> Scene {
    let perlin_material = Arc::new(Lambertian::new(Noise::new(4.)));

    SceneBuilder::new(Background::Color(Color3::new(0.5, 0.7, 1.)))
        .add_sphere(Sphere::new(
            Point3::new(0., -1000., 0.),
            1000.,
            perlin_material.clone(),
        ))
        .add_sphere(Sphere::new(Point3::new(0., 2., 0.), 2., perlin_material))
        .build()
}

fn earth() -> Scene {
    SceneBuilder::new(Background::Color(Vec3::new(0.5, 0.7, 1.)))
        .add_sphere(Sphere::new(
            Point3::new(0., 0., 0.),
            2.,
            Arc::new(Lambertian::new(Image::load("resources/earthmap.jpg"))),
        ))
        .build()
}

fn simple_light() -> Scene {
    let perlin_material = Arc::new(Lambertian::new(Noise::new(4.)));

    SceneBuilder::new(Background::Color(Vec3::new(0., 0., 0.)))
        .add_sphere(Sphere::new(
            Point3::new(0., -1000., 0.),
            1000.,
            perlin_material.clone(),
        ))
        .add_sphere(Sphere::new(Point3::new(0., 2., 0.), 2., perlin_material))
        .add_object(Object::new(XyRect::new(
            (3., 5.),
            (1., 3.),
            -2.,
            Arc::new(Light::new(Monochrome::from_rgb(4., 4., 4.))),
        )))
        .build()
}

fn cornell_box() -> Scene {
    let white = Arc::new(Lambertian::from_rgb(0.73, 0.73, 0.73));

    let box1 = AaBox::new(Point3::zero(), Point3::new(165., 330., 165.), white.clone());
    let box1 = RotateY::new(Arc::new(box1), 15.);
    let box1 = Translate::new(Arc::new(box1), Vec3::new(265., 0., 295.));

    let box2 = AaBox::new(Point3::zero(), Point3::new(165., 165., 165.), white.clone());
    let box2 = RotateY::new(Arc::new(box2), -18.);
    let box2 = Translate::new(Arc::new(box2), Vec3::new(130., 0., 65.));
    SceneBuilder::new(Background::Color(Color3::new(0., 0., 0.)))
        .add_object(Object::new(YzRect::new(
            (0., 555.),
            (0., 555.),
            555.,
            Arc::new(Lambertian::from_rgb(0.12, 0.45, 0.15)),
        )))
        .add_object(Object::new(YzRect::new(
            (0., 555.),
            (0., 555.),
            0.,
            Arc::new(Lambertian::from_rgb(0.65, 0.05, 0.05)),
        )))
        .add_object(Object::new(XzRect::new(
            (213., 343.),
            (227., 332.),
            554.,
            Arc::new(Light::new(Monochrome::from_rgb(15., 15., 15.))),
        )))
        .add_object(Object::new(XzRect::new(
            (0., 555.),
            (0., 555.),
            0.,
            white.clone(),
        )))
        .add_object(Object::new(XzRect::new(
            (0., 555.),
            (0., 555.),
            555.,
            white.clone(),
        )))
        .add_object(Object::new(XyRect::new(
            (0., 555.),
            (0., 555.),
            555.,
            white,
        )))
        .add_object(Object::new(box1))
        .add_object(Object::new(box2))
        .build()
}

fn cornell_smoke() -> Scene {
    let white = Arc::new(Lambertian::from_rgb(0.73, 0.73, 0.73));

    let box1 = AaBox::new(Point3::zero(), Point3::new(165., 330., 165.), white.clone());
    let box1 = RotateY::new(Arc::new(box1), 15.);
    let box1 = Translate::new(Arc::new(box1), Vec3::new(265., 0., 295.));

    let box2 = AaBox::new(Point3::zero(), Point3::new(165., 165., 165.), white.clone());
    let box2 = RotateY::new(Arc::new(box2), -18.);
    let box2 = Translate::new(Arc::new(box2), Vec3::new(130., 0., 65.));
    SceneBuilder::new(Background::Color(Color3::new(0., 0., 0.)))
        .add_object(Object::new(YzRect::new(
            (0., 555.),
            (0., 555.),
            555.,
            Arc::new(Lambertian::from_rgb(0.12, 0.45, 0.15)),
        )))
        .add_object(Object::new(YzRect::new(
            (0., 555.),
            (0., 555.),
            0.,
            Arc::new(Lambertian::from_rgb(0.65, 0.05, 0.05)),
        )))
        .add_object(Object::new(XzRect::new(
            (213., 343.),
            (227., 332.),
            554.,
            Arc::new(Light::new(Monochrome::from_rgb(15., 15., 15.))),
        )))
        .add_object(Object::new(XzRect::new(
            (0., 555.),
            (0., 555.),
            0.,
            white.clone(),
        )))
        .add_object(Object::new(XzRect::new(
            (0., 555.),
            (0., 555.),
            555.,
            white.clone(),
        )))
        .add_object(Object::new(XyRect::new(
            (0., 555.),
            (0., 555.),
            555.,
            white,
        )))
        .add_object(Object::new(ConstantMedium::new(
            Arc::new(box1),
            0.01,
            Arc::new(Isotropic::new(Monochrome::from_rgb(0., 0., 0.))),
        )))
        .add_object(Object::new(ConstantMedium::new(
            Arc::new(box2),
            0.01,
            Arc::new(Isotropic::new(Monochrome::from_rgb(1., 1., 1.))),
        )))
        .build()
}
