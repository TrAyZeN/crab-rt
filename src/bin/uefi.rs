#![no_main]
#![no_std]

extern crate alloc;

use alloc::{sync::Arc, vec, vec::Vec};
use core::ffi::c_void;
use core::iter::zip;
use core::ptr::NonNull;

use crab_rt::camera::Camera;
use crab_rt::materials::{Dielectric, Lambertian, Metal};
use crab_rt::objects::Sphere;
use crab_rt::raytracer::RayTracer;
use crab_rt::scene::{Background, SceneBuilder};
use crab_rt::textures::Checker;
use crab_rt::utils::{gamma_encode, partial_row_views_mut, rng, PartialRowViewMut};
use crab_rt::vec::{Color3, Point3, Vec3};
use rand::Rng;
use uefi::data_types::Event;
use uefi::prelude::*;
use uefi::proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput};
use uefi::proto::pi::mp::MpServices;
use uefi::table::boot::{EventType, Tpl};

struct WorkerArg<'a> {
    proc_id: usize,
    num_procs: usize,
    width: usize,
    height: usize,
    worker_pixels_view: PartialRowViewMut<'a, Color3>,
    worker_framebuffer_view: PartialRowViewMut<'a, BltPixel>,
    raytracer: &'a RayTracer,
}

extern "efiapi" fn worker(arg: *mut c_void) {
    // SAFETY: We assume that arg is a mutable reference to `WorkerArg`.
    let arg = unsafe { &mut *(arg as *mut WorkerArg) };

    let WorkerArg {
        proc_id,
        num_procs,
        width,
        height,
        ref mut worker_pixels_view,
        ref mut worker_framebuffer_view,
        raytracer,
    } = *arg;

    let mut rng = rng();

    let mut num_samples = 1;
    loop {
        for y_inv in (proc_id..height).step_by(num_procs) {
            let y = height - 1 - y_inv;
            let pixels_row = worker_pixels_view.row(y_inv).unwrap();
            let framebuffer_row = worker_framebuffer_view.row(y_inv).unwrap();

            for x in 0..width {
                let u = (x as f32 + rng.gen::<f32>()) / width as f32;
                let v = (y as f32 + rng.gen::<f32>()) / height as f32;

                let ray = raytracer.camera().ray(u, v);

                let color = raytracer.cast(&ray, 0);
                pixels_row[x] =
                    (pixels_row[x] * (num_samples - 1) as f32 + color) / num_samples as f32;

                framebuffer_row[x] = Color3::new(
                    gamma_encode(pixels_row[x].x),
                    gamma_encode(pixels_row[x].y),
                    gamma_encode(pixels_row[x].z),
                )
                .into();
            }
        }

        num_samples += 1;
    }
}

unsafe extern "efiapi" fn notify_fn(_event: Event, _context: Option<NonNull<c_void>>) {
    todo!()
}

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut system_table).unwrap();

    let boot_services = system_table.boot_services();

    // Disable the watchdog timer.
    boot_services.set_watchdog_timer(0, 0x10000, None).unwrap();

    let mp_services_handle = boot_services
        .get_handle_for_protocol::<MpServices>()
        .unwrap();
    let mp_services = boot_services
        .open_protocol_exclusive::<MpServices>(mp_services_handle)
        .unwrap();
    let num_procs = mp_services.get_number_of_processors().unwrap().total;
    let bsp_proc_id = mp_services.who_am_i().unwrap();

    let gop_handle = boot_services
        .get_handle_for_protocol::<GraphicsOutput>()
        .unwrap();
    let mut gop = boot_services
        .open_protocol_exclusive::<GraphicsOutput>(gop_handle)
        .unwrap();
    let (width, height) = gop.current_mode_info().resolution();

    let raytracer = raytracer1(width as u32, height as u32);

    let mut pixels = vec![Color3::zero(); width * height];
    let mut framebuffer = vec![BltPixel::new(0, 0, 0); width * height];

    // This reference is used for rendering. Artifacts from race conditions on the framebuffer when
    // rendering are accepted.
    // TODO: safety
    let framebuffer_ref = unsafe { &*(&framebuffer as *const Vec<BltPixel>) };

    let pixels_views = partial_row_views_mut(&mut pixels, width, num_procs);
    let framebuffer_views = partial_row_views_mut(&mut framebuffer, width, num_procs);

    let mut proc_args = Vec::new();

    for (proc_id, (worker_pixels_view, worker_framebuffer_view)) in
        zip(0..num_procs, zip(pixels_views, framebuffer_views))
    {
        proc_args.push(WorkerArg {
            proc_id,
            num_procs,
            width,
            height,
            worker_pixels_view,
            worker_framebuffer_view,
            raytracer: &raytracer,
        });
    }

    for (proc_id, proc_arg) in zip(0..num_procs, proc_args.iter_mut()) {
        if proc_id == bsp_proc_id {
            continue;
        }

        // This event is used to run code on APs without blocking the BSP.
        // WARN: This call is unsafe as `create_event` safety relies on `notify_fn` handling exit
        // from boot services correctly, which is currently not the case.
        let event = unsafe {
            boot_services.create_event(EventType::NOTIFY_WAIT, Tpl::NOTIFY, Some(notify_fn), None)
        }
        .unwrap();

        mp_services
            .startup_this_ap(
                proc_id,
                worker,
                proc_arg as *mut _ as *mut c_void,
                Some(event),
                None,
            )
            .unwrap();
    }

    let mut rng = rng();

    let WorkerArg {
        ref mut worker_pixels_view,
        ref mut worker_framebuffer_view,
        ..
    } = proc_args[bsp_proc_id];

    let mut num_samples = 1;
    loop {
        for y_inv in (bsp_proc_id..height).step_by(num_procs) {
            let y = height - 1 - y_inv;
            let pixels_row = worker_pixels_view.row(y_inv).unwrap();
            let framebuffer_row = worker_framebuffer_view.row(y_inv).unwrap();

            for x in 0..width {
                let u = (x as f32 + rng.gen::<f32>()) / width as f32;
                let v = (y as f32 + rng.gen::<f32>()) / height as f32;

                let ray = raytracer.camera().ray(u, v);

                let color = raytracer.cast(&ray, 0);
                pixels_row[x] =
                    (pixels_row[x] * (num_samples - 1) as f32 + color) / num_samples as f32;

                framebuffer_row[x] = Color3::new(
                    gamma_encode(pixels_row[x].x),
                    gamma_encode(pixels_row[x].y),
                    gamma_encode(pixels_row[x].z),
                )
                .into();
            }

            gop.blt(BltOp::BufferToVideo {
                buffer: &framebuffer_ref,
                src: BltRegion::Full,
                dest: (0, 0),
                dims: (width, height),
            })
            .unwrap();
        }

        num_samples += 1;
    }
}

fn raytracer1(width: u32, height: u32) -> RayTracer {
    let camera = Camera::new(
        Point3::new(4., 2., 4.),
        Point3::new(0., 0., -1.),
        20.,
        width as f32 / height as f32,
    );
    // .aperture(1.);

    let scene = SceneBuilder::new(Background::Gradient(
        Vec3::new(0.5, 0.7, 1.),
        Vec3::new(1., 1., 1.),
    ))
    .add_sphere(Sphere::new(
        Vec3::new(0., 0., -1.),
        0.5,
        Arc::new(Lambertian::from_rgb(0.1, 0.2, 0.5)),
    ))
    .add_sphere(Sphere::new(
        Vec3::new(0., -100.5, -1.),
        100.,
        Arc::new(Lambertian::new(Checker::from_colors(
            Color3::new(1., 1., 1.),
            Color3::new(0.5, 0.1, 0.8),
        ))),
    ))
    .add_sphere(Sphere::new(
        Vec3::new(1., 0., -1.),
        0.5,
        Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.)),
    ))
    .add_sphere(Sphere::new(
        Vec3::new(-1., 0., -1.),
        0.5,
        Arc::new(Dielectric::new(1.5)),
    ))
    // .add_sphere(Sphere::new(
    //     Vec3::new(-1., 0., -1.),
    //     -0.45,
    //     Dielectric::new(1.5),
    // ))
    .build();

    RayTracer::new(width, height, 200, 50, camera, scene)
}
