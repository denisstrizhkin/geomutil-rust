use std::{
    array,
    cmp::Ordering,
    collections::HashSet,
    hash::{Hash, Hasher},
    iter,
    ops::{
        Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub,
        SubAssign,
    },
    ptr, slice,
};

use serde::{Deserialize, Serialize, ser::SerializeSeq};

use crate::bounding_box::BoundingBox;

macro_rules! view_impl {
    ($T: ident; $($comps: ident),*) => {
        #[repr(C)]
        #[derive(Clone, Debug, Copy)]
        pub struct $T {
            $(pub $comps: f32),*
        }
    };
}

view_impl!(ViewXY; x, y);
view_impl!(ViewXYZ; x, y, z);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<const N: usize> {
    pub coords: [f32; N],
}

pub type Point2 = Point<2>;
pub type Point3 = Point<3>;

macro_rules! deref_impl {
    ($ty: ident, $n: expr, $ty_view: ident) => {
        impl Deref for Point<$n> {
            type Target = $ty_view;

            fn deref(&self) -> &Self::Target {
                unsafe { &*ptr::from_ref(self).cast() }
            }
        }

        impl DerefMut for Point<$n> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *ptr::from_mut(self).cast() }
            }
        }
    };
}

deref_impl!(Point, 2, ViewXY);
deref_impl!(Point, 3, ViewXYZ);

impl<const N: usize> Hash for Point<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.iter().for_each(|a| a.to_bits().hash(state));
    }
}

impl<const N: usize> Eq for Point<N> {}

impl<const N: usize> Ord for Point<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        iter::zip(self, other).fold(Ordering::Equal, |ord, (a, b)| match ord {
            Ordering::Equal => a.total_cmp(b),
            ord => ord,
        })
    }
}

impl<const N: usize> PartialOrd for Point<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Default for Point<N> {
    fn default() -> Self {
        Self {
            coords: [Default::default(); N],
        }
    }
}

impl<const N: usize> From<[f32; N]> for Point<N> {
    fn from(coords: [f32; N]) -> Self {
        Self { coords }
    }
}

impl<const N: usize> Serialize for Point<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(N))?;
        self.iter().try_for_each(|a| seq.serialize_element(a))?;
        seq.end()
    }
}

impl<'de, const N: usize> Deserialize<'de> for Point<N> {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

impl<const N: usize> AddAssign<Self> for Point<N> {
    fn add_assign(&mut self, rhs: Self) {
        iter::zip(self, rhs).for_each(|(a, b)| *a += b);
    }
}

impl<const N: usize> AddAssign<f32> for Point<N> {
    fn add_assign(&mut self, rhs: f32) {
        self.iter_mut().for_each(|a| *a += rhs);
    }
}

impl<const N: usize> Add<Self> for Point<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

impl<const N: usize> Add<f32> for Point<N> {
    type Output = Self;
    fn add(self, rhs: f32) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

impl<const N: usize> SubAssign<Self> for Point<N> {
    fn sub_assign(&mut self, rhs: Self) {
        iter::zip(self, rhs).for_each(|(a, b)| *a -= b);
    }
}

impl<const N: usize> SubAssign<f32> for Point<N> {
    fn sub_assign(&mut self, rhs: f32) {
        self.iter_mut().for_each(|a| *a -= rhs);
    }
}

impl<const N: usize> Sub<Self> for Point<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result -= rhs;
        result
    }
}

impl<const N: usize> Sub<f32> for Point<N> {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self::Output {
        let mut result = self;
        result -= rhs;
        result
    }
}

impl<const N: usize> MulAssign<Self> for Point<N> {
    fn mul_assign(&mut self, rhs: Self) {
        iter::zip(self, rhs).for_each(|(a, b)| *a *= b);
    }
}

impl<const N: usize> MulAssign<f32> for Point<N> {
    fn mul_assign(&mut self, rhs: f32) {
        self.iter_mut().for_each(|a| *a *= rhs);
    }
}

impl<const N: usize> Mul<Self> for Point<N> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result *= rhs;
        result
    }
}

impl<const N: usize> Mul<f32> for Point<N> {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = self;
        result *= rhs;
        result
    }
}

impl<const N: usize> DivAssign<Self> for Point<N> {
    fn div_assign(&mut self, rhs: Self) {
        iter::zip(self, rhs).for_each(|(a, b)| *a /= b);
    }
}

impl<const N: usize> DivAssign<f32> for Point<N> {
    fn div_assign(&mut self, rhs: f32) {
        self.iter_mut().for_each(|a| *a /= rhs);
    }
}

impl<const N: usize> Div<Self> for Point<N> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result /= rhs;
        result
    }
}

impl<const N: usize> Div<f32> for Point<N> {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        let mut result = self;
        result /= rhs;
        result
    }
}

impl<'a, const N: usize> Point<N> {
    pub fn iter(&'a self) -> slice::Iter<'a, f32> {
        self.coords.iter()
    }

    pub fn iter_mut(&'a mut self) -> slice::IterMut<'a, f32> {
        self.coords.iter_mut()
    }

    pub fn length_squared(self) -> f32 {
        (self * self).iter().sum::<f32>()
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn distance_squared(self, other: Self) -> f32 {
        (self - other).length_squared()
    }

    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > 0.0 { self / len } else { self }
    }

    pub fn unique(points: impl IntoIterator<Item = Self>) -> Vec<Self> {
        points
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }

    pub fn avg(points: impl IntoIterator<Item = Self>) -> Option<Self> {
        points
            .into_iter()
            .map(|p| (p, 1.0f32))
            .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
            .map(|(p, cnt)| p / cnt)
    }

    pub fn bounding_box(points: impl IntoIterator<Item = Self>) -> Option<BoundingBox<N>> {
        points
            .into_iter()
            .map(|p| (p, p))
            .reduce(|mut a, b| {
                iter::zip(&mut a.0, b.0).for_each(|(a, b)| *a = a.min(b));
                iter::zip(&mut a.1, b.1).for_each(|(a, b)| *a = a.max(b));
                a
            })
            .map(Into::into)
    }
}

impl<const N: usize> IntoIterator for Point<N> {
    type Item = f32;
    type IntoIter = array::IntoIter<f32, N>;
    fn into_iter(self) -> Self::IntoIter {
        self.coords.into_iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a Point<N> {
    type Item = &'a f32;
    type IntoIter = slice::Iter<'a, f32>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a mut Point<N> {
    type Item = &'a mut f32;
    type IntoIter = slice::IterMut<'a, f32>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<const N: usize> Index<usize> for Point<N> {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.coords[index]
    }
}

impl<const N: usize> IndexMut<usize> for Point<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coords[index]
    }
}

impl Point2 {
    pub fn polar_angle(self) -> f32 {
        let angle = self.y.atan2(self.x);
        if angle >= 0.0 {
            angle
        } else {
            2.0f32.mul_add(std::f32::consts::PI, angle)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EPS;

    fn assert_approx_eq(a: f32, b: f32) {
        assert_approx_eq_eps(a, b, EPS);
    }

    fn assert_approx_eq_eps(a: f32, b: f32, eps: f32) {
        assert!((a - b).abs() < eps.abs(), "wanted: {b}, got: {a}");
    }

    fn assert_point_approx_eq<const N: usize>(a: Point<N>, b: Point<N>) {
        iter::zip(a, b).for_each(|(a, b)| assert_approx_eq(a, b));
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
        let a = Point3::default();
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
        assert_point_approx_eq(c, Point3::from([2.5 / 2.0, 2.0 / 3.0, 1.0 / 4.0]));
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
        assert_point_approx_eq(avg, Point::from([3.0, 4.0]));
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
        assert_point_approx_eq(bbox.lower(), Point::from([0.0, 0.0, 1.0]));
        assert_point_approx_eq(bbox.upper(), Point::from([6.0, 9.0, 15.0]));
    }

    #[test]
    fn test_length() {
        let p = Point::from([1.0, 2.0, 3.0]);
        assert_approx_eq(p.length_squared(), 14.0);
        assert_approx_eq(p.length(), (14f32).sqrt());
    }

    #[test]
    fn test_distance() {
        let a = Point::from([1.0, 2.0, 3.0]);
        let b = Point::from([4.0, 6.0, 15.0]);
        assert_approx_eq(a.distance_squared(b), 169.0);
        assert_approx_eq(a.distance(b), 169f32.sqrt());
    }

    #[test]
    fn test_normalize() {
        let p = Point::from([0.1, 0.2, 0.3]);
        let p_len = p.length();
        assert_point_approx_eq(
            p.normalize(),
            Point::from([0.1 / p_len, 0.2 / p_len, 0.3 / p_len]),
        );
    }

    #[test]
    fn test_point2_polar_angle() {
        let eps = 1e-4;
        assert_approx_eq_eps(Point::from([1.0, 0.0]).polar_angle().to_degrees(), 0.0, eps);
        assert_approx_eq_eps(
            Point::from([1.0, 1.0]).polar_angle().to_degrees(),
            45.0,
            eps,
        );
        assert_approx_eq_eps(
            Point::from([0.0, 1.0]).polar_angle().to_degrees(),
            90.0,
            eps,
        );
        assert_approx_eq_eps(
            Point::from([-1.0, 1.0]).polar_angle().to_degrees(),
            135.0,
            eps,
        );
        assert_approx_eq_eps(
            Point::from([-1.0, 0.0]).polar_angle().to_degrees(),
            180.0,
            eps,
        );
        assert_approx_eq_eps(
            Point::from([-1.0, -1.0]).polar_angle().to_degrees(),
            225.0,
            eps,
        );
        assert_approx_eq_eps(
            Point::from([0.0, -1.0]).polar_angle().to_degrees(),
            270.0,
            eps,
        );
        assert_approx_eq_eps(
            Point::from([1.0, -1.0]).polar_angle().to_degrees(),
            315.0,
            eps,
        );
    }
}
