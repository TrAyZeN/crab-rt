use alloc::rc::Rc;
use core::cell::UnsafeCell;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::SmallRng;
use rand::{Error, Rng, RngCore, SeedableRng};

use crate::vec::Vec3;

#[cfg(feature = "std")]
use std::thread_local;

#[cfg(not(feature = "std"))]
use core_maths::*;

pub struct SmallThreadRng {
    rng: Rc<UnsafeCell<SmallRng>>,
}

#[cfg(feature = "std")]
thread_local! {
    pub static SMALL_THREAD_RNG_KEY: Rc<UnsafeCell<SmallRng>> =
        //Rc::new(UnsafeCell::new(SmallRng::from_entropy()));
        Rc::new(UnsafeCell::new(SmallRng::from_seed([0; 32])));
}

#[cfg(feature = "std")]
#[must_use]
pub fn small_thread_rng() -> SmallThreadRng {
    let rng = SMALL_THREAD_RNG_KEY.with(Clone::clone);
    SmallThreadRng { rng }
}

impl RngCore for SmallThreadRng {
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        // SAFETY: SmallThreadRng is !Send and !Sync so it can be in only used
        // by one thread and we only create reference to rng for the scope of
        // a function so we are sure that no one else holds a
        // reference to it
        let rng = unsafe { &mut *self.rng.get() };
        rng.next_u32()
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        // SAFETY: SmallThreadRng is !Send and !Sync so it can be in only used
        // by one thread and we only create reference to rng for the scope of
        // a function so we are sure that no one else holds a
        // reference to it
        let rng = unsafe { &mut *self.rng.get() };
        rng.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        // SAFETY: SmallThreadRng is !Send and !Sync so it can be in only used
        // by one thread and we only create reference to rng for the scope of
        // a function so we are sure that no one else holds a
        // reference to it
        let rng = unsafe { &mut *self.rng.get() };
        rng.fill_bytes(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        // SAFETY: SmallThreadRng is !Send and !Sync so it can be in only used
        // by one thread and we only create reference to rng for the scope of
        // a function so we are sure that no one else holds a
        // reference to it
        let rng = unsafe { &mut *self.rng.get() };
        rng.try_fill_bytes(dest)
    }
}

#[cfg(feature = "std")]
#[inline(always)]
#[must_use]
pub fn rng() -> impl Rng {
    rand::thread_rng()
    // small_thread_rng()
}

#[cfg(not(feature = "std"))]
#[inline]
#[must_use]
#[inline(always)]
#[must_use]
pub fn rng() -> impl Rng {
    SmallRng::from_seed([0; 32])
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit()
}

#[must_use]
pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0. {
        // In the same hemisphere as the normal
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

#[must_use]
pub fn random_in_unit_sphere() -> Vec3 {
    let uniform = Uniform::from(-1.0..1.0);
    let mut rng = rng();

    let mut p = Vec3::new(
        uniform.sample(&mut rng),
        uniform.sample(&mut rng),
        uniform.sample(&mut rng),
    );
    while p.squared_length() >= 1. {
        p *= rng.gen::<f32>();
    }

    p
}

#[must_use]
pub fn random_in_unit_disk() -> Vec3 {
    let uniform = Uniform::from(-1.0..1.0);
    let mut rng = rng();

    let mut p = Vec3::new(uniform.sample(&mut rng), uniform.sample(&mut rng), 0.);
    while p.squared_length() >= 1. {
        p *= rng.gen::<f32>();
    }

    p
}

/// Computes the outcoming reflection vector with the given incoming vector and normal.
/// We now that the angle between the incoming vector and the normal is equal to the angle
/// between the outcoming vector and the normal.
///
/// # Examples
/// ```
/// use crab_rt::utils::reflect;
/// use crab_rt::vec::Vec3;
///
/// assert_eq!(
///     reflect(&Vec3::new(0.5, -0.5, 0.), &Vec3::new(0., 1., 0.)),
///     Vec3::new(0.5, 0.5, 0.)
/// );
/// ```
#[inline]
#[must_use]
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2. * v.dot(n) * n
}

#[must_use]
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = f32::min((-uv).dot(n), 1.);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f32::sqrt(f32::abs(1. - r_out_perp.squared_length())) * n;
    r_out_perp + r_out_parallel
}

/// Computes the specular reflection coefficient by approximating the Fresnel equations.
/// https://en.wikipedia.org/wiki/Schlick%27s_approximation
/// R(theta) = R_0 + (1 - R_0)(1 - cos(theta))^5
/// where
/// R_0 = \frac{n_1 - n_2}{n_1 + n_2}^2
/// Schlick-approximation is used to efficiently calculate vacuum-medium type of interactions.
#[inline]
#[must_use]
pub fn schlick(cosine: f32, refraction_index: f32) -> f32 {
    let mut r0 = (1. - refraction_index) / (1. + refraction_index);
    r0 *= r0;
    (1. - r0).mul_add(f32::powf(1. - cosine, 5.), r0)
}

const GAMMA: f32 = 2.2;

// The human visual system is approximately logarithmically sensitive to power over a large range
// so common image formats use gamma encoding to optimize the usage of bits when encoding an image
// so we have to revert that equation to get a correct encoding
#[inline]
#[must_use]
pub fn gamma_encode(x: f32) -> f32 {
    x.powf(1. / GAMMA)
}

#[inline]
#[must_use]
pub fn gamma_decode(x: f32) -> f32 {
    x.powf(GAMMA)
}
