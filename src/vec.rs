use image::Rgb;
use std::{
    convert, iter,
    ops::{self, Add, Sub},
};

use crate::utils::clamp;

/// A 3D mathematical vector.
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Vec3 {
    /// Coordinate along the x-axis.
    pub x: f32,
    /// Coordinate along the y-axis.
    pub y: f32,
    /// Coordinate along the z-axis.
    pub z: f32,
}

impl Vec3 {
    /// Constructs a 3D mathematical vector with the given coordinates.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::vec::Vec3;
    ///
    /// let u = Vec3::new(1., 2., 3.);
    ///
    /// assert_eq!(u.x, 1.);
    /// assert_eq!(u.y, 2.);
    /// assert_eq!(u.z, 3.);
    /// ```
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Contructs the zero vector.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::vec::Vec3;
    ///
    /// let u = Vec3::zero();
    ///
    /// assert_eq!(u, Vec3::new(0., 0., 0.));
    /// ```
    #[inline]
    pub const fn zero() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    /// Checks if the given vector is the zero vector ie all the coordinates of the vector are zero.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::vec::Vec3;
    ///
    /// assert_eq!(Vec3::new(0., 1., 2.).is_zero(), false);
    /// assert_eq!(Vec3::new(0., 0., 0.).is_zero(), true);
    /// ```
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.x == 0. && self.y == 0. && self.z == 0.
    }

    /// Checks if all the vector coordinates are below a threshold close to zero.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::vec::Vec3;
    ///
    /// assert_eq!(Vec3::new(0., 1., 2.).is_near_zero(), false);
    /// assert_eq!(Vec3::new(1e-9, 1e-10, 1e-11).is_near_zero(), true);
    /// ```
    #[inline]
    pub fn is_near_zero(&self) -> bool {
        const THRESH: f32 = 1e-8f32;
        self.x.abs() < THRESH && self.y.abs() < THRESH && self.z.abs() < THRESH
    }

    /// Returns the [length](https://en.wikipedia.org/wiki/Euclidean_vector#Length) of the vector.
    /// The length of a vector (x, y, z) is sqrt(x^2 + y^2 + z^2).
    ///
    /// # Examples
    /// ```
    /// use crab_rt::vec::Vec3;
    ///
    /// let u = Vec3::new(1., 2., 3.);
    ///
    /// assert_eq!(u.length(), f32::sqrt(1. * 1. + 2. * 2. + 3. * 3.));
    /// ```
    #[inline]
    pub fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    /// Returns the squared [length](https://en.wikipedia.org/wiki/Euclidean_vector#Length) of the vector.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::vec::Vec3;
    ///
    /// let u = Vec3::new(1., 2., 3.);
    ///
    /// assert_eq!(u.squared_length(), 1. * 1. + 2. * 2. + 3. * 3.);
    /// ```
    #[inline]
    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Consumes the vector and returns the unit vector with the same direction.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::vec::Vec3;
    ///
    /// let u = Vec3::new(1., 2., 3.).unit();
    ///
    /// let length = f32::sqrt(1. * 1. + 2. * 2. + 3. * 3.);
    /// assert_eq!(u.x, 1. / length);
    /// assert_eq!(u.y, 2. / length);
    /// assert_eq!(u.z, 3. / length);
    /// ```
    pub fn unit(self) -> Self {
        if self.is_zero() {
            self
        } else {
            self / self.length()
        }
    }

    /// Normalizes the vector.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::vec::Vec3;
    ///
    /// let mut u = Vec3::new(1., 2., 3.);
    /// u.normalize();
    ///
    /// let length = f32::sqrt(1. * 1. + 2. * 2. + 3. * 3.);
    /// assert_eq!(u.x, 1. / length);
    /// assert_eq!(u.y, 2. / length);
    /// assert_eq!(u.z, 3. / length);
    /// ```
    pub fn normalize(&mut self) {
        if !self.is_zero() {
            let inv_length = 1. / self.length();

            self.x *= inv_length;
            self.y *= inv_length;
            self.z *= inv_length;
        }
    }

    /// Computes the [dot product](https://en.wikipedia.org/wiki/Dot_product) with the given vector.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::vec::Vec3;
    ///
    /// let u = Vec3::new(1., 2., 3.);
    /// let v = Vec3::new(4., 5., 6.);
    ///
    /// assert_eq!(u.dot(&v), 1. * 4. + 2. * 5. + 3. * 6.);
    /// ```
    #[inline]
    pub fn dot(&self, v: &Self) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    /// Computes the dot product of the vector with itself.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::vec::Vec3;
    ///
    /// let u = Vec3::new(1., 2., 3.);
    ///
    /// assert_eq!(u.square(), u.dot(&u));
    /// ```
    #[inline]
    pub fn square(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Computes the [cross product](https://en.wikipedia.org/wiki/Cross_product) with the given vector.
    ///
    /// # Examples
    /// ```
    /// use crab_rt::vec::Vec3;
    ///
    /// let u = Vec3::new(1., 2., 3.);
    /// let v = Vec3::new(4., 2., 5.);
    ///
    /// let w = u.cross(&v);
    /// assert_eq!(w.x, 2. * 5. - 3. * 2.);
    /// assert_eq!(w.y, 3. * 4. - 1. * 5.);
    /// assert_eq!(w.z, 1. * 2. - 2. * 4.);
    /// ```
    #[inline]
    pub fn cross(&self, v: &Self) -> Self {
        Self {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }
}

// implements binary operators "&T op U", "T op &U", "&T op &U"
// based on "T op U" where T and U are expected to be `Copy`able
macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl<'a> $imp<&'a $u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl<'a, 'b> $imp<&'a $u> for &'b $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

forward_ref_binop! { impl Add, add for Vec3, Vec3 }

impl ops::AddAssign<Vec3> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        // Produces the same asm as without using operator overloading.
        self + (-rhs)
    }
}

forward_ref_binop! { impl Sub, sub for Vec3, Vec3 }

impl ops::SubAssign<Vec3> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<f32> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Mul<&Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        // Produces the same asm as without using operator overloading.
        self * rhs.recip()
    }
}

impl ops::DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        let inv_rhs = rhs.recip();

        self.x *= inv_rhs;
        self.y *= inv_rhs;
        self.z *= inv_rhs;
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        [&self.x, &self.y, &self.z][index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        [&mut self.x, &mut self.y, &mut self.z][index]
    }
}

impl convert::Into<Rgb<u8>> for Vec3 {
    #[inline]
    fn into(self) -> Rgb<u8> {
        Rgb([
            (255. * clamp(self.x, 0., 1.)) as u8,
            (255. * clamp(self.y, 0., 1.)) as u8,
            (255. * clamp(self.z, 0., 1.)) as u8,
        ])
    }
}

impl convert::Into<Rgb<u8>> for &Vec3 {
    #[inline]
    fn into(self) -> Rgb<u8> {
        Rgb([
            (255. * clamp(self.x, 0., 1.)) as u8,
            (255. * clamp(self.y, 0., 1.)) as u8,
            (255. * clamp(self.z, 0., 1.)) as u8,
        ])
    }
}

// Needed if using rayon
impl iter::Sum<Vec3> for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3::zero(), Add::add)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_is_zero() {
        assert_eq!(Vec3::new(0., 0., 0.).is_zero(), true);
        assert_eq!(Vec3::new(0., 1., 0.).is_zero(), false);
    }

    #[test]
    fn vec3_length() {
        let testee = Vec3::new(3.0, 4.0, 12.0);

        assert_eq!(testee.length(), 13.0);
    }

    #[test]
    fn vec3_squared_length() {
        let testee = Vec3::new(3.0, 4.0, 12.0);

        assert_eq!(testee.squared_length(), 169.0);
    }

    #[test]
    fn vec3_cross() {
        let u = Vec3::new(0., 1., 0.);
        let v = Vec3::new(0., 0., 1.);

        let w = u.cross(&v);
        assert_eq!(w.x, 1.);
        assert_eq!(w.y, 0.);
        assert_eq!(w.z, 0.);
    }

    #[test]
    fn vec3_add() {
        let testee = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(8.0, 43.0, 7.0);

        assert_eq!(testee.x, 9.0);
        assert_eq!(testee.y, 45.0);
        assert_eq!(testee.z, 10.0);
    }

    #[test]
    fn vec3_add_assign() {
        let mut testee = Vec3::new(1.0, 2.0, 3.0);
        testee += Vec3::new(8.0, 43.0, 7.0);

        assert_eq!(testee.x, 9.0);
        assert_eq!(testee.y, 45.0);
        assert_eq!(testee.z, 10.0);
    }

    #[test]
    fn vec3_neg() {
        let testee = -Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(testee.x, -1.0);
        assert_eq!(testee.y, -2.0);
        assert_eq!(testee.z, -3.0);
    }

    #[test]
    fn vec3_sub() {
        let testee = Vec3::new(1.0, 2.0, 3.0) - Vec3::new(8.0, 43.0, 7.0);

        assert_eq!(testee.x, -7.0);
        assert_eq!(testee.y, -41.0);
        assert_eq!(testee.z, -4.0);
    }

    #[test]
    fn vec3_sub_assign() {
        let mut testee = Vec3::new(1.0, 2.0, 3.0);
        testee -= Vec3::new(8.0, 43.0, 7.0);

        assert_eq!(testee.x, -7.0);
        assert_eq!(testee.y, -41.0);
        assert_eq!(testee.z, -4.0);
    }

    #[test]
    fn vec3_mul() {
        let testee = Vec3::new(1.0, 2.0, 3.0) * 4.0;

        assert_eq!(testee.x, 4.0);
        assert_eq!(testee.y, 8.0);
        assert_eq!(testee.z, 12.0);
    }

    #[test]
    fn vec3_mul_assign() {
        let mut testee = Vec3::new(1.0, 2.0, 3.0);
        testee *= 4.0;

        assert_eq!(testee.x, 4.0);
        assert_eq!(testee.y, 8.0);
        assert_eq!(testee.z, 12.0);
    }

    #[test]
    fn vec3_mul_lhs() {
        let testee = 4.0 * Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(testee.x, 4.0);
        assert_eq!(testee.y, 8.0);
        assert_eq!(testee.z, 12.0);
    }

    #[test]
    fn vec3_index() {
        let testee = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(testee[0], 1.0);
        assert_eq!(testee[1], 2.0);
        assert_eq!(testee[2], 3.0);
    }

    #[test]
    fn vec3_index_mut() {
        let mut testee = Vec3::new(1.0, 2.0, 3.0);
        testee[0] += 3.0;
        testee[1] *= 4.0;
        testee[2] -= 12.0;

        assert_eq!(testee.x, 4.0);
        assert_eq!(testee.y, 8.0);
        assert_eq!(testee.z, -9.0);
    }

    #[test]
    fn vec3_into_rgb() {
        let testee: Rgb<u8> = Vec3::new(0.5, 0.2, 0.3).into();

        assert_eq!(testee, Rgb([127, 51, 76]));
    }
}
