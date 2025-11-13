use std::{
    array,
    cmp::Ordering,
    iter::{self, Sum},
    ops::{
        Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub,
        SubAssign,
    },
    ptr, slice,
};

use serde::{Deserialize, Serialize, ser::SerializeSeq};

use crate::{bounding_box::BoundingBox, scalar::Float};

macro_rules! view_impl {
    ($T: ident; $($comps: ident),*) => {
        #[repr(C)]
        #[derive(Clone, Debug, Copy)]
        pub struct $T<T: Float> {
            $(pub $comps: T),*
        }
    };
}

view_impl!(ViewXY; x, y);
view_impl!(ViewXYZ; x, y, z);

#[allow(clippy::derive_partial_eq_without_eq)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point<const N: usize, T: Float> {
    pub coords: [T; N],
}

pub type Point2<T> = Point<2, T>;
pub type Point3<T> = Point<3, T>;

macro_rules! deref_impl {
    ($ty: ident, $n: expr, $ty_view: ident) => {
        impl<T: Float> Deref for Point<$n, T> {
            type Target = $ty_view<T>;

            fn deref(&self) -> &Self::Target {
                unsafe { &*ptr::from_ref(self).cast() }
            }
        }

        impl<T: Float> DerefMut for Point<$n, T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *ptr::from_mut(self).cast() }
            }
        }
    };
}

deref_impl!(Point, 2, ViewXY);
deref_impl!(Point, 3, ViewXYZ);

impl<const N: usize, T: Float> Default for Point<N, T> {
    fn default() -> Self {
        Self {
            coords: [Default::default(); N],
        }
    }
}

impl<const N: usize, T: Float> From<[T; N]> for Point<N, T> {
    fn from(coords: [T; N]) -> Self {
        Self { coords }
    }
}

impl<const N: usize, T: Float + Serialize> Serialize for Point<N, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(N))?;
        self.iter().try_for_each(|a| seq.serialize_element(a))?;
        seq.end()
    }
}

impl<'de, const N: usize, T: Float> Deserialize<'de> for Point<N, T> {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

impl<const N: usize, T: Float> AddAssign<Self> for Point<N, T> {
    fn add_assign(&mut self, rhs: Self) {
        iter::zip(self, rhs).for_each(|(a, b)| *a += b);
    }
}

impl<const N: usize, T: Float> AddAssign<T> for Point<N, T> {
    fn add_assign(&mut self, rhs: T) {
        self.iter_mut().for_each(|a| *a += rhs);
    }
}

impl<const N: usize, T: Float> Add<Self> for Point<N, T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

impl<const N: usize, T: Float> Add<T> for Point<N, T> {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

impl<const N: usize, T: Float> SubAssign<Self> for Point<N, T> {
    fn sub_assign(&mut self, rhs: Self) {
        iter::zip(self, rhs).for_each(|(a, b)| *a -= b);
    }
}

impl<const N: usize, T: Float> SubAssign<T> for Point<N, T> {
    fn sub_assign(&mut self, rhs: T) {
        self.iter_mut().for_each(|a| *a -= rhs);
    }
}

impl<const N: usize, T: Float> Sub<Self> for Point<N, T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result -= rhs;
        result
    }
}

impl<const N: usize, T: Float> Sub<T> for Point<N, T> {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        let mut result = self;
        result -= rhs;
        result
    }
}

impl<const N: usize, T: Float> MulAssign<Self> for Point<N, T> {
    fn mul_assign(&mut self, rhs: Self) {
        iter::zip(self, rhs).for_each(|(a, b)| *a *= b);
    }
}

impl<const N: usize, T: Float> MulAssign<T> for Point<N, T> {
    fn mul_assign(&mut self, rhs: T) {
        self.iter_mut().for_each(|a| *a *= rhs);
    }
}

impl<const N: usize, T: Float> Mul<Self> for Point<N, T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result *= rhs;
        result
    }
}

impl<const N: usize, T: Float> Mul<T> for Point<N, T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let mut result = self;
        result *= rhs;
        result
    }
}

impl<const N: usize, T: Float> DivAssign<Self> for Point<N, T> {
    fn div_assign(&mut self, rhs: Self) {
        iter::zip(self, rhs).for_each(|(a, b)| *a /= b);
    }
}

impl<const N: usize, T: Float> DivAssign<T> for Point<N, T> {
    fn div_assign(&mut self, rhs: T) {
        self.iter_mut().for_each(|a| *a /= rhs);
    }
}

impl<const N: usize, T: Float> Div<Self> for Point<N, T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result /= rhs;
        result
    }
}

impl<const N: usize, T: Float> Div<T> for Point<N, T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let mut result = self;
        result /= rhs;
        result
    }
}

impl<const N: usize, T: Float> Point<N, T> {
    pub fn zero() -> Self {
        Self::from(array::from_fn(|_| T::zero()))
    }

    pub fn one() -> Self {
        Self::from(array::from_fn(|_| T::one()))
    }

    pub fn dot(self, other: Self) -> T {
        (self * other).into_iter().sum()
    }

    pub fn total_cmp(self, other: &Self) -> Ordering {
        iter::zip(self, other).fold(Ordering::Equal, |ord, (a, b)| match ord {
            Ordering::Equal => a.total_cmp(b),
            _ => ord,
        })
    }

    pub fn min(self, other: Self) -> Self {
        match self.total_cmp(&other) {
            Ordering::Less | Ordering::Equal => self,
            Ordering::Greater => other,
        }
    }

    pub fn max(self, other: Self) -> Self {
        match self.total_cmp(&other) {
            Ordering::Greater | Ordering::Equal => self,
            Ordering::Less => other,
        }
    }

    pub fn has_nan(self) -> bool {
        self.iter().any(|x| x.is_nan())
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.coords.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.coords.iter_mut()
    }

    pub fn length_squared(self) -> T {
        (self * self).iter().copied().sum()
    }

    pub fn length(self) -> T {
        self.length_squared().sqrt()
    }

    pub fn distance_squared(self, other: Self) -> T {
        (self - other).length_squared()
    }

    pub fn distance(self, other: Self) -> T {
        self.distance_squared(other).sqrt()
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > T::zero() { self / len } else { self }
    }

    pub fn unique(points: impl IntoIterator<Item = Self>) -> Vec<Self> {
        let mut points = points.into_iter().collect::<Vec<_>>();
        points.sort_by(|a, b| a.total_cmp(b));
        let mut unique = Vec::new();
        for point in points {
            if !unique.last().is_some_and(|previous| point.eq(previous)) {
                unique.push(point);
            }
        }
        unique
    }

    pub fn avg(points: impl IntoIterator<Item = Self>) -> Option<Self> {
        points
            .into_iter()
            .map(|p| (p, 1))
            .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
            .map(|(p, cnt)| p / T::from(Into::<f64>::into(cnt)))
    }

    pub fn bounding_box(points: impl IntoIterator<Item = Self>) -> Option<BoundingBox<N, T>> {
        points
            .into_iter()
            .map(|p| (p, p))
            .reduce(|mut a, b| {
                iter::zip(&mut a.0, b.0).for_each(|(a, b)| *a = a.min(b));
                iter::zip(&mut a.1, b.1).for_each(|(a, b)| *a = a.max(b));
                a
            })
            .map(|(lower, upper)| BoundingBox { lower, upper })
    }
}

impl<const N: usize, T: Float> Sum for Point<N, T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |sum, p| sum + p)
    }
}

impl<'a, const N: usize, T: Float> Sum<&'a Self> for Point<N, T> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl<const N: usize, T: Float> IntoIterator for Point<N, T> {
    type Item = T;
    type IntoIter = array::IntoIter<T, N>;
    fn into_iter(self) -> Self::IntoIter {
        self.coords.into_iter()
    }
}

impl<'a, const N: usize, T: Float> IntoIterator for &'a Point<N, T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, const N: usize, T: Float> IntoIterator for &'a mut Point<N, T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<const N: usize, T: Float> Index<usize> for Point<N, T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.coords[index]
    }
}

impl<const N: usize, T: Float> IndexMut<usize> for Point<N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coords[index]
    }
}

impl<T: Float> Point2<T> {
    pub fn polar_angle(self) -> T {
        let angle = self.y.atan2(self.x);
        if angle >= T::zero() {
            angle
        } else {
            T::from(2.0).mul_add(T::PI, angle)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EPS: f64 = 1e-10;

    fn assert_approx_eq_eps<T: Float>(a: T, b: T, eps: T) {
        assert!((a - b).abs() < eps.abs(), "wanted: {b}, got: {a}");
    }

    fn assert_point_approx_eq<const N: usize, T: Float>(a: Point<N, T>, b: Point<N, T>, eps: T) {
        iter::zip(a, b).for_each(|(a, b)| assert_approx_eq_eps(a, b, eps));
    }

    #[test]
    fn test_xy_fields() {
        let p = Point2::from([1.0, 2.0]);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
    }

    #[test]
    fn test_xyz_fields() {
        let p = Point3::from([1.0, 2.0, 3.0]);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
    }

    #[test]
    fn test_default() {
        let a = Point3::<f64>::default();
        assert_eq!(a.coords, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_add() {
        let mut a = Point3::from([1.0, 2.0, 3.0]);
        let b = Point3::from([2.0, 3.0, 4.0]);
        a += b;
        assert_eq!(a.coords, [3.0, 5.0, 7.0]);
        a += 0.5;
        assert_eq!(a.coords, [3.5, 5.5, 7.5]);
        let c = a + 0.5;
        assert_eq!(c.coords, [4.0, 6.0, 8.0]);
        let c = a + b;
        assert_eq!(c.coords, [5.5, 8.5, 11.5]);
    }

    #[test]
    fn test_dot() {
        let a = Point3::from([2.0, 7.0, 1.0]);
        let b = Point3::from([8.0, 2.0, 8.0]);
        assert_eq!(a.dot(b), 38.0);
    }

    #[test]
    fn test_sub() {
        let mut a = Point3::from([3.0, 5.0, 7.0]);
        let b = Point3::from([1.0, 2.0, 3.0]);
        a -= b;
        assert_eq!(a.coords, [2.0, 3.0, 4.0]);
        a -= 0.5;
        assert_eq!(a.coords, [1.5, 2.5, 3.5]);
        let c = a - 0.5;
        assert_eq!(c.coords, [1.0, 2.0, 3.0]);
        let c = a - b;
        assert_eq!(c.coords, [0.5, 0.5, 0.5]);
    }

    #[test]
    fn test_mul() {
        let mut a = Point3::from([1.0, 2.0, 3.0]);
        let b = Point3::from([2.0, 3.0, 4.0]);
        a *= b;
        assert_eq!(a.coords, [2.0, 6.0, 12.0]);
        a *= 0.5;
        assert_eq!(a.coords, [1.0, 3.0, 6.0]);
        let c = a * 0.5;
        assert_eq!(c.coords, [0.5, 1.5, 3.0]);
        let c = a * b;
        assert_eq!(c.coords, [2.0, 9.0, 24.0]);
    }

    #[test]
    fn test_div() {
        let mut a = Point3::from([10.0, 12.0, 8.0]);
        let b = Point3::from([2.0, 3.0, 4.0]);
        a /= b;
        assert_eq!(a.coords, [5.0, 4.0, 2.0]);
        a /= 2.0;
        assert_eq!(a.coords, [2.5, 2.0, 1.0]);
        let c = a / 2.0;
        assert_eq!(c.coords, [1.25, 1.0, 0.5]);
        let c = a / b;
        assert_point_approx_eq(c, Point3::from([2.5 / 2.0, 2.0 / 3.0, 1.0 / 4.0]), EPS);
    }

    #[test]
    fn test_unique() {
        let points = vec![
            Point2::from([1.0, 2.0]),
            Point2::from([3.0, 2.0]),
            Point2::from([1.0, 2.0]),
            Point2::from([4.0, 5.0]),
        ];
        let unique = Point2::unique(points);
        assert_eq!(unique.len(), 3);
        assert!(unique.contains(&Point2::from([1.0, 2.0])));
        assert!(unique.contains(&Point2::from([3.0, 2.0])));
        assert!(unique.contains(&Point2::from([4.0, 5.0])));
    }

    #[test]
    fn test_avg() {
        let points = vec![
            Point2::from([1.0, 2.0]),
            Point2::from([3.0, 4.0]),
            Point2::from([5.0, 6.0]),
        ];
        let avg = Point::avg(points);
        assert!(avg.is_some());
        let avg = avg.unwrap();
        assert_point_approx_eq(avg, Point::from([3.0, 4.0]), EPS);
    }

    #[test]
    fn test_bounding_box() {
        let points = vec![
            Point::from([1.0, 5.0, 10.0]),
            Point::from([4.0, 2.0, 1.0]),
            Point::from([2.0, 8.0, 5.0]),
            Point::from([0.0, 0.0, 15.0]),
            Point::from([6.0, 1.0, 7.0]),
            Point::from([3.0, 9.0, 4.0]),
        ];
        let bbox = Point::bounding_box(points);
        assert!(bbox.is_some());
        let bbox = bbox.unwrap();
        assert_point_approx_eq(bbox.lower, Point::from([0.0, 0.0, 1.0]), EPS);
        assert_point_approx_eq(bbox.upper, Point::from([6.0, 9.0, 15.0]), EPS);
    }

    #[test]
    fn test_length() {
        let p = Point::from([1.0, 2.0, 3.0]);
        assert_approx_eq_eps(p.length_squared(), 14.0, EPS);
        assert_approx_eq_eps(p.length(), 14.0.sqrt(), EPS);
    }

    #[test]
    fn test_distance() {
        let a = Point::from([1.0, 2.0, 3.0]);
        let b = Point::from([4.0, 6.0, 15.0]);
        assert_approx_eq_eps(a.distance_squared(b), 169.0, EPS);
        assert_approx_eq_eps(a.distance(b), 169.0.sqrt(), EPS);
    }

    #[test]
    fn test_normalize() {
        let p = Point::from([0.1, 0.2, 0.3]);
        let p_len = p.length();
        assert_point_approx_eq(
            p.normalize(),
            Point::from([0.1 / p_len, 0.2 / p_len, 0.3 / p_len]),
            EPS,
        );
    }

    #[test]
    fn test_point2_polar_angle() {
        assert_approx_eq_eps(Point::from([1.0, 0.0]).polar_angle().to_degrees(), 0.0, EPS);
        assert_approx_eq_eps(
            Point::from([1.0, 1.0]).polar_angle().to_degrees(),
            45.0,
            EPS,
        );
        assert_approx_eq_eps(
            Point::from([0.0, 1.0]).polar_angle().to_degrees(),
            90.0,
            EPS,
        );
        assert_approx_eq_eps(
            Point::from([-1.0, 1.0]).polar_angle().to_degrees(),
            135.0,
            EPS,
        );
        assert_approx_eq_eps(
            Point::from([-1.0, 0.0]).polar_angle().to_degrees(),
            180.0,
            EPS,
        );
        assert_approx_eq_eps(
            Point::from([-1.0, -1.0]).polar_angle().to_degrees(),
            225.0,
            EPS,
        );
        assert_approx_eq_eps(
            Point::from([0.0, -1.0]).polar_angle().to_degrees(),
            270.0,
            EPS,
        );
        assert_approx_eq_eps(
            Point::from([1.0, -1.0]).polar_angle().to_degrees(),
            315.0,
            EPS,
        );
    }
}
