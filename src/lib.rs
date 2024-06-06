#![no_std]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(test)]
#[macro_use]
extern crate quickcheck_macros;

pub mod aabb;
pub mod bvh;
pub mod camera;
mod core;
pub mod hitable;
pub mod materials;
pub mod objects;
pub mod perlin;
pub mod raytracer;
pub mod scene;
pub mod textures;
pub mod utils;

pub use crate::core::*;
