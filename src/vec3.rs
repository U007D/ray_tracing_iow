use crate::consts::msg;
use interpol::write;
use std::{
    fmt::{Display, Formatter},
    ops::{Add, AddAssign, DivAssign, Index, IndexMut, MulAssign, Neg, Sub},
    ops::{Div, Mul, SubAssign},
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3(f64, f64, f64);

// TODO: Add Vec3 tests
// TODO: Figure out why arithmetic operations below compiling?
impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    #[inline]
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.0
            .mul_add(self.0, self.1.mul_add(self.1, self.2.mul_add(self.2, 0.0)))
    }

    #[inline]
    pub const fn x(&self) -> f64 {
        self.0
    }

    #[inline]
    pub const fn y(&self) -> f64 {
        self.1
    }

    #[inline]
    pub const fn z(&self) -> f64 {
        self.2
    }

    #[inline]
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x()
            .mul_add(rhs.x(), self.y().mul_add(rhs.y(), self.z() * rhs.z()))
    }

    #[inline]
    pub fn cross(&self, rhs: &Self) -> Self {
        Self(
            self.y().mul_add(rhs.z(), -self.z() * rhs.y()),
            self.z().mul_add(rhs.x(), -self.x() * rhs.z()),
            self.x().mul_add(rhs.y(), -self.y() * rhs.x()),
        )
    }

    #[inline]
    pub fn unit(&self) -> Self {
        self / self.length()
    }
}

impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<T: Into<f64>> Add<T> for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl<T: Into<f64>> Add<T> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Vec3(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl AddAssign<&Self> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl<T: Into<f64>> AddAssign<T> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.0 += rhs;
        self.1 += rhs;
        self.2 += rhs;
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self.x()} {self.y()} {self.z()}")
    }
}

impl Div for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl Div for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl<T: Into<f64>> Div<T> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl<T: Into<f64>> Div<T> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl DivAssign for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}

impl DivAssign<&Self> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: &Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}

impl<T: Into<f64>> DivAssign<T> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!(msg::ERR_INDEX_OUT_OF_BOUNDS),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!(msg::ERR_INDEX_OUT_OF_BOUNDS),
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl<T: Into<f64>> Mul<T> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl<T: Into<f64>> Mul<T> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl MulAssign for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl MulAssign<&Self> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl<T: Into<f64>> MulAssign<T> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self(-self.x(), -self.y(), -self.z())
    }
}

impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<T: Into<f64>> Sub<T> for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self(self.0 - rhs, self.1 - rhs, self.2 - rhs)
    }
}

impl<T: Into<f64>> Sub<T> for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Vec3(self.0 - rhs, self.1 - rhs, self.2 - rhs)
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl SubAssign<&Self> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl<T: Into<f64>> SubAssign<T> for Vec3 {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.0 -= rhs;
        self.1 -= rhs;
        self.2 -= rhs;
    }
}
