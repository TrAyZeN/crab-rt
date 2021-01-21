use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

use crate::vec::Vec3;

/// Returns the value bounded by a minimum and a maximum.
///
/// # Panic
/// Panics in `debug` mode if `min > max`.
///
/// # Example
/// ```
/// use crab_rt::utils::clamp;
///
/// assert_eq!(clamp(0.5, 0., 1.), 0.5);
/// assert_eq!(clamp(-1., 0., 1.), 0.);
/// assert_eq!(clamp(1., 0., 1.), 1.);
/// ```
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    debug_assert!(min <= max);

    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

#[inline]
pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().unit()
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0. {
        // In the same hemisphere as the normal
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let uniform = Uniform::from(-1.0..1.0);
    let mut rng = rand::thread_rng();

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

pub fn random_in_unit_disk() -> Vec3 {
    let uniform = Uniform::from(-1.0..1.0);
    let mut rng = rand::thread_rng();

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
/// use crab_rt::vec::Vec3;
/// use crab_rt::utils::reflect;
///
/// assert_eq!(reflect(&Vec3::new(0.5, -0.5, 0.), &Vec3::new(0., 1., 0.)), Vec3::new(0.5, 0.5, 0.));
/// ```
#[inline]
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2. * v.dot(n) * n
}

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
pub fn schlick(cosine: f32, refraction_index: f32) -> f32 {
    let mut r0 = (1. - refraction_index) / (1. + refraction_index);
    r0 *= r0;
    r0 + (1. - r0) * f32::powf(1. - cosine, 5.)
}

const GAMMA: f32 = 2.2;

#[inline]
pub fn gamma_encode(x: f32) -> f32 {
    x.powf(1. / GAMMA)
}

#[inline]
pub fn gamma_decode(x: f32) -> f32 {
    x.powf(GAMMA)
}
