#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod hitable;
pub mod materials;
pub mod objects;
pub mod perlin;
pub mod ray;
pub mod raytracer;
pub mod scene;
pub mod textures;
pub mod utils;
pub mod vec;
