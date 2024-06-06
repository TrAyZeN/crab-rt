use core::{
    convert, iter,
    ops::{self, Add, Sub},
};

#[cfg(feature = "std")]
use image::Rgb;

#[cfg(not(feature = "std"))]
use core_maths::*;

#[cfg(feature = "uefi")]
use uefi::proto::console::gop::BltPixel;

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

/// A point in space.
///
/// **This type is an alias to `Vec3` so most methods are listed in [`Vec3`](crate::vec::Vec3)**
pub type Point3 = Vec3;

/// A RGB color represented by floats.
///
/// **This type is an alias to `Vec3` so most methods are listed in [`Vec3`](crate::vec::Vec3)**
pub type Color3 = Vec3;

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
    #[must_use]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        // Coordinates must be not be NaN as NaNs do not work well with equality, comparison
        // operations and as they infect calculations.
        // See https://doc.rust-lang.org/std/primitive.f32.html
        debug_assert!(!x.is_nan() && !y.is_nan() && !z.is_nan());

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
    #[inline(always)]
    #[must_use]
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
    #[inline(always)]
    #[must_use]
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
    #[inline(always)]
    #[must_use]
    pub fn is_near_zero(&self) -> bool {
        const THRESH: f32 = 1e-8_f32;
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
    #[inline(always)]
    #[must_use]
    pub fn length(&self) -> f32 {
        f32::sqrt(self.squared_length())
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
    #[inline(always)]
    #[must_use]
    pub fn squared_length(&self) -> f32 {
        self.dot(self)
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
    #[must_use]
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

    /// Returns a vector with the absolute value operation applied to it coordinates
    ///
    /// # Examples
    /// ```
    /// use crab_rt::vec::Vec3;
    ///
    /// let u = Vec3::new(-1., 2., -3.);
    ///
    /// assert_eq!(u.abs(), Vec3::new(1., 2., 3.));
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
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
    #[inline(always)]
    #[must_use]
    pub fn dot(&self, v: &Self) -> f32 {
        self.x.mul_add(v.x, self.y.mul_add(v.y, self.z * v.z))
    }

    /// Computes the [dot product](Vec3::dot) and apply absolute value to it.
    ///
    /// # Example
    /// ```
    /// use crab_rt::vec::Vec3;
    ///
    /// let u = Vec3::new(-1., 2., -3.);
    /// let v = Vec3::new(4., 5., -6.);
    ///
    /// assert_eq!(u.abs_dot(&v), f32::abs(-1. * 4. + 2. * 5. + (-3.) * (-6.)));
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn abs_dot(&self, v: &Self) -> f32 {
        self.dot(v).abs()
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
    #[inline(always)]
    #[must_use]
    pub fn square(&self) -> f32 {
        self.dot(self)
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
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    #[must_use]
    pub fn cross(&self, v: &Self) -> Self {
        // Here we use 64-bit float to avoid error from cancellation.
        let self_x = f64::from(self.x);
        let self_y = f64::from(self.y);
        let self_z = f64::from(self.z);
        let v_x = f64::from(v.x);
        let v_y = f64::from(v.y);
        let v_z = f64::from(v.z);

        Self::new(
            ((self_y * v_z) - (self_z * v_y)) as f32,
            ((self_z * v_x) - (self_x * v_z)) as f32,
            ((self_x * v_y) - (self_y * v_x)) as f32,
        )
    }

    /// Returns the component-wise minimum vector.
    ///
    /// # Example
    /// ```
    /// use crab_rt::vec::Vec3;
    ///
    /// let u = Vec3::new(1., 2., 3.);
    /// let v = Vec3::new(0., 4., 2.);
    ///
    /// assert_eq!(u.min(&v), Vec3::new(0., 2., 2.));
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn min(&self, v: &Self) -> Self {
        Self::new(
            f32::min(self.x, v.x),
            f32::min(self.y, v.y),
            f32::min(self.z, v.z),
        )
    }

    /// Returns the component-wise maximum vector.
    ///
    /// # Example
    /// ```
    /// use crab_rt::vec::Vec3;
    ///
    /// let u = Vec3::new(1., 2., 3.);
    /// let v = Vec3::new(0., 4., 2.);
    ///
    /// assert_eq!(u.max(&v), Vec3::new(1., 4., 3.));
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn max(&self, v: &Self) -> Self {
        Self::new(
            f32::max(self.x, v.x),
            f32::max(self.y, v.y),
            f32::max(self.z, v.z),
        )
    }

    pub fn mul_add(&self, a: &Self, b: &Self) -> Self {
        Self::new(
            self.x.mul_add(a.x, b.x),
            self.y.mul_add(a.y, b.y),
            self.z.mul_add(a.z, b.z),
        )
    }
}

// implements binary operators "&T op U", "T op &U", "&T op &U"
// based on "T op U" where T and U are expected to be `Copy`able
macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline(always)]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl<'a> $imp<&'a $u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline(always)]
            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl<'a, 'b> $imp<&'a $u> for &'b $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline(always)]
            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

forward_ref_binop! { impl Add, add for Vec3, Vec3 }

impl ops::AddAssign<Vec3> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        // Produces the same asm as without using operator overloading.
        self + (-rhs)
    }
}

forward_ref_binop! { impl Sub, sub for Vec3, Vec3 }

impl ops::SubAssign<Vec3> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<f32> for &Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
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

    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl ops::Mul<&Vec3> for f32 {
    type Output = Vec3;

    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        [&self.x, &self.y, &self.z][index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        [&mut self.x, &mut self.y, &mut self.z][index]
    }
}

#[cfg(feature = "std")]
impl convert::Into<Rgb<u8>> for Vec3 {
    #[inline(always)]
    fn into(self) -> Rgb<u8> {
        Rgb([
            (255. * self.x.clamp(0., 1.)) as u8,
            (255. * self.y.clamp(0., 1.)) as u8,
            (255. * self.z.clamp(0., 1.)) as u8,
        ])
    }
}

#[cfg(feature = "std")]
impl convert::Into<Rgb<u8>> for &Vec3 {
    #[inline(always)]
    fn into(self) -> Rgb<u8> {
        Rgb([
            (255. * self.x.clamp(0., 1.)) as u8,
            (255. * self.y.clamp(0., 1.)) as u8,
            (255. * self.z.clamp(0., 1.)) as u8,
        ])
    }
}

#[cfg(feature = "uefi")]
impl convert::Into<BltPixel> for Vec3 {
    #[inline(always)]
    fn into(self) -> BltPixel {
        BltPixel::new(
            (255. * self.x.clamp(0., 1.)) as u8,
            (255. * self.y.clamp(0., 1.)) as u8,
            (255. * self.z.clamp(0., 1.)) as u8,
        )
    }
}

#[cfg(feature = "uefi")]
impl convert::Into<BltPixel> for &Vec3 {
    #[inline(always)]
    fn into(self) -> BltPixel {
        BltPixel::new(
            (255. * self.x.clamp(0., 1.)) as u8,
            (255. * self.y.clamp(0., 1.)) as u8,
            (255. * self.z.clamp(0., 1.)) as u8,
        )
    }
}

// Needed if using rayon
impl iter::Sum<Vec3> for Vec3 {
    #[inline(always)]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck::{Arbitrary, Gen, TestResult};

    impl Arbitrary for Vec3 {
        fn arbitrary(g: &mut Gen) -> Vec3 {
            let nan_to_default = |f: f32| if f.is_nan() { f32::default() } else { f };

            Vec3::new(
                nan_to_default(f32::arbitrary(g)),
                nan_to_default(f32::arbitrary(g)),
                nan_to_default(f32::arbitrary(g)),
            )
        }
    }

    fn is_zero_subnormal_normal(v: &Vec3) -> bool {
        !v.x.is_nan()
            && !v.x.is_infinite()
            && !v.y.is_nan()
            && !v.y.is_infinite()
            && !v.z.is_nan()
            && !v.z.is_infinite()
    }

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

    #[quickcheck]
    fn vec3_squared_length_property(vec: Vec3) -> TestResult {
        TestResult::from_bool(vec.squared_length() == vec.length() * vec.length())
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

    #[quickcheck]
    fn vec3_add_property(lhs: Vec3, rhs: Vec3) -> TestResult {
        if !is_zero_subnormal_normal(&lhs) || !is_zero_subnormal_normal(&rhs) {
            return TestResult::discard();
        }

        TestResult::from_bool(
            (lhs + rhs).x == lhs.x + rhs.x
                && (lhs + rhs).y == lhs.y + rhs.y
                && (lhs + rhs).z == lhs.z + rhs.z,
        )
    }

    #[test]
    fn vec3_add_assign() {
        let mut testee = Vec3::new(1.0, 2.0, 3.0);
        testee += Vec3::new(8.0, 43.0, 7.0);

        assert_eq!(testee.x, 9.0);
        assert_eq!(testee.y, 45.0);
        assert_eq!(testee.z, 10.0);
    }

    #[quickcheck]
    fn vec3_add_assign_property(lhs: Vec3, rhs: Vec3) -> TestResult {
        if !is_zero_subnormal_normal(&lhs) || !is_zero_subnormal_normal(&rhs) {
            return TestResult::discard();
        }

        let mut testee = lhs;
        testee += rhs;

        TestResult::from_bool(testee == lhs + rhs)
    }

    #[test]
    fn vec3_neg() {
        let testee = -Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(testee.x, -1.0);
        assert_eq!(testee.y, -2.0);
        assert_eq!(testee.z, -3.0);
    }

    #[quickcheck]
    fn vec3_neg_property(rhs: Vec3) -> TestResult {
        if !is_zero_subnormal_normal(&rhs) {
            return TestResult::discard();
        }

        TestResult::from_bool((-rhs).x == -rhs.x && (-rhs).y == -rhs.y && (-rhs).z == -rhs.z)
    }

    #[test]
    fn vec3_sub() {
        let testee = Vec3::new(1.0, 2.0, 3.0) - Vec3::new(8.0, 43.0, 7.0);

        assert_eq!(testee.x, -7.0);
        assert_eq!(testee.y, -41.0);
        assert_eq!(testee.z, -4.0);
    }

    #[quickcheck]
    fn vec3_sub_property(lhs: Vec3, rhs: Vec3) -> TestResult {
        if !is_zero_subnormal_normal(&lhs) || !is_zero_subnormal_normal(&rhs) {
            return TestResult::discard();
        }

        TestResult::from_bool(
            (lhs - rhs).x == lhs.x - rhs.x
                && (lhs - rhs).y == lhs.y - rhs.y
                && (lhs - rhs).z == lhs.z - rhs.z,
        )
    }

    #[test]
    fn vec3_sub_assign() {
        let mut testee = Vec3::new(1.0, 2.0, 3.0);
        testee -= Vec3::new(8.0, 43.0, 7.0);

        assert_eq!(testee.x, -7.0);
        assert_eq!(testee.y, -41.0);
        assert_eq!(testee.z, -4.0);
    }

    #[quickcheck]
    fn vec3_sub_assign_property(lhs: Vec3, rhs: Vec3) -> TestResult {
        if !is_zero_subnormal_normal(&lhs) || !is_zero_subnormal_normal(&rhs) {
            return TestResult::discard();
        }

        let mut testee = lhs;
        testee -= rhs;

        TestResult::from_bool(testee == lhs - rhs)
    }

    #[test]
    fn vec3_mul() {
        let testee = Vec3::new(1.0, 2.0, 3.0) * 4.0;

        assert_eq!(testee.x, 4.0);
        assert_eq!(testee.y, 8.0);
        assert_eq!(testee.z, 12.0);
    }

    #[quickcheck]
    fn vec3_mul_property(lhs: Vec3, rhs: f32) -> TestResult {
        if !is_zero_subnormal_normal(&lhs) || (rhs.is_nan() || rhs.is_infinite()) {
            return TestResult::discard();
        }

        TestResult::from_bool(
            (lhs * rhs).x == lhs.x * rhs
                && (lhs * rhs).y == lhs.y * rhs
                && (lhs * rhs).z == lhs.z * rhs,
        )
    }

    #[test]
    fn vec3_mul_assign() {
        let mut testee = Vec3::new(1.0, 2.0, 3.0);
        testee *= 4.0;

        assert_eq!(testee.x, 4.0);
        assert_eq!(testee.y, 8.0);
        assert_eq!(testee.z, 12.0);
    }

    #[quickcheck]
    fn vec3_mul_assign_property(lhs: Vec3, rhs: f32) -> TestResult {
        if !is_zero_subnormal_normal(&lhs) || (rhs.is_nan() || rhs.is_infinite()) {
            return TestResult::discard();
        }

        let mut testee = lhs;
        testee *= rhs;

        TestResult::from_bool(testee == lhs * rhs)
    }

    #[test]
    fn vec3_mul_lhs() {
        let testee = 4.0 * Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(testee.x, 4.0);
        assert_eq!(testee.y, 8.0);
        assert_eq!(testee.z, 12.0);
    }

    #[quickcheck]
    fn vec3_mul_lhs_property(lhs: f32, rhs: Vec3) -> TestResult {
        if (lhs.is_nan() || lhs.is_infinite()) || !is_zero_subnormal_normal(&rhs) {
            return TestResult::discard();
        }

        TestResult::from_bool(
            (lhs * rhs).x == lhs * rhs.x
                && (lhs * rhs).y == lhs * rhs.y
                && (lhs * rhs).z == lhs * rhs.z,
        )
    }

    #[test]
    fn vec3_index() {
        let testee = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(testee[0], 1.0);
        assert_eq!(testee[1], 2.0);
        assert_eq!(testee[2], 3.0);
    }

    #[quickcheck]
    fn vec3_index_property(vec: Vec3) -> TestResult {
        TestResult::from_bool(vec[0] == vec.x && vec[1] == vec.y && vec[2] == vec.z)
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
