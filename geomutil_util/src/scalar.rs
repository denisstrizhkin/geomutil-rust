use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::iter::Sum;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

pub trait Float:
    Sized
    + Copy
    + Clone
    + Debug
    + Display
    + Default
    + PartialEq
    + PartialOrd
    + Add<Self, Output = Self>
    + AddAssign
    + Sub<Self, Output = Self>
    + SubAssign
    + Mul<Self, Output = Self>
    + MulAssign
    + Div<Self, Output = Self>
    + DivAssign
    + Rem<Self, Output = Self>
    + RemAssign
    + Neg
    + Sum
{
    const PI: Self;
    const E: Self;
    const NAN: Self;

    fn abs(self) -> Self;
    fn acos(self) -> Self;
    fn acosh(self) -> Self;
    fn asin(self) -> Self;
    fn asinh(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn atanh(self) -> Self;
    fn cos(self) -> Self;
    fn from(value: f64) -> Self;
    fn is_nan(self) -> bool;
    fn one() -> Self;
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
    fn mul_add(self, a: Self, b: Self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn sin(self) -> Self;
    fn sqrt(self) -> Self;
    fn total_cmp(&self, other: &Self) -> Ordering;
    fn to_degrees(self) -> Self;
    fn to_radians(self) -> Self;
    fn zero() -> Self;
}

macro_rules! impl_float {
    ($t:ident) => {
        impl Float for $t {
            const PI: Self = std::$t::consts::PI;
            const E: Self = std::$t::consts::E;
            const NAN: Self = $t::NAN;
            fn abs(self) -> Self {
                self.abs()
            }
            fn acos(self) -> Self {
                self.acos()
            }
            fn acosh(self) -> Self {
                self.acosh()
            }
            fn asin(self) -> Self {
                self.asin()
            }
            fn asinh(self) -> Self {
                self.asinh()
            }
            fn atan(self) -> Self {
                self.atan()
            }
            fn atan2(self, other: Self) -> Self {
                self.atan2(other)
            }
            fn atanh(self) -> Self {
                self.atanh()
            }
            fn cos(self) -> Self {
                self.cos()
            }
            #[allow(clippy::cast_possible_truncation)]
            fn from(value: f64) -> Self {
                value as Self
            }
            fn is_nan(self) -> bool {
                self.is_nan()
            }
            fn one() -> Self {
                1.0
            }
            fn max(self, other: Self) -> Self {
                self.max(other)
            }
            fn min(self, other: Self) -> Self {
                self.min(other)
            }
            fn mul_add(self, a: Self, b: Self) -> Self {
                self.mul_add(a, b)
            }
            fn powi(self, n: i32) -> Self {
                self.powi(n)
            }
            fn sin(self) -> Self {
                self.sin()
            }
            fn sqrt(self) -> Self {
                self.sqrt()
            }
            fn total_cmp(&self, other: &Self) -> Ordering {
                self.total_cmp(other)
            }
            fn to_degrees(self) -> Self {
                self.to_degrees()
            }
            fn to_radians(self) -> Self {
                self.to_radians()
            }
            fn zero() -> Self {
                0.0
            }
        }
    };
}

impl_float!(f32);
impl_float!(f64);
