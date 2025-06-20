use std::{
    cmp::Ordering,
    collections::HashSet,
    hash::{Hash, Hasher},
    iter::zip,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    ptr,
};

use serde::{Deserialize, Serialize, ser::SerializeSeq};

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
    coords: [f32; N],
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
        self.coords.iter().for_each(|a| a.to_bits().hash(state));
    }
}

impl<const N: usize> Eq for Point<N> {}

impl<const N: usize> Ord for Point<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        zip(self.coords, other.coords).fold(Ordering::Equal, |ord, (a, b)| match ord {
            Ordering::Equal => a.total_cmp(&b),
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
        Self { coords: [0.0; N] }
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
        self.coords
            .iter()
            .try_for_each(|a| seq.serialize_element(a))?;
        seq.end()
    }
}

impl<'de, const N: usize> Deserialize<'de> for Point<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

impl<const N: usize> AddAssign<Point<N>> for Point<N> {
    fn add_assign(&mut self, rhs: Point<N>) {
        zip(&mut self.coords, rhs.coords).for_each(|(a, b)| *a += b);
    }
}

impl<const N: usize> AddAssign<f32> for Point<N> {
    fn add_assign(&mut self, rhs: f32) {
        self.coords.iter_mut().for_each(|a| *a += rhs);
    }
}

impl<const N: usize> Add<Point<N>> for Point<N> {
    type Output = Point<N>;
    fn add(self, rhs: Point<N>) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

impl<const N: usize> Add<f32> for Point<N> {
    type Output = Point<N>;
    fn add(self, rhs: f32) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

impl<const N: usize> SubAssign<Point<N>> for Point<N> {
    fn sub_assign(&mut self, rhs: Point<N>) {
        zip(&mut self.coords, rhs.coords).for_each(|(a, b)| *a -= b);
    }
}

impl<const N: usize> SubAssign<f32> for Point<N> {
    fn sub_assign(&mut self, rhs: f32) {
        self.coords.iter_mut().for_each(|a| *a -= rhs);
    }
}

impl<const N: usize> Sub<Point<N>> for Point<N> {
    type Output = Point<N>;
    fn sub(self, rhs: Point<N>) -> Self::Output {
        let mut result = self;
        result -= rhs;
        result
    }
}

impl<const N: usize> Sub<f32> for Point<N> {
    type Output = Point<N>;
    fn sub(self, rhs: f32) -> Self::Output {
        let mut result = self;
        result -= rhs;
        result
    }
}

impl<const N: usize> MulAssign<Point<N>> for Point<N> {
    fn mul_assign(&mut self, rhs: Point<N>) {
        zip(&mut self.coords, rhs.coords).for_each(|(a, b)| *a *= b);
    }
}

impl<const N: usize> MulAssign<f32> for Point<N> {
    fn mul_assign(&mut self, rhs: f32) {
        self.coords.iter_mut().for_each(|a| *a *= rhs);
    }
}

impl<const N: usize> Mul<Point<N>> for Point<N> {
    type Output = Point<N>;
    fn mul(self, rhs: Point<N>) -> Self::Output {
        let mut result = self;
        result *= rhs;
        result
    }
}

impl<const N: usize> Mul<f32> for Point<N> {
    type Output = Point<N>;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = self;
        result *= rhs;
        result
    }
}

impl<const N: usize> DivAssign<Point<N>> for Point<N> {
    fn div_assign(&mut self, rhs: Point<N>) {
        zip(&mut self.coords, rhs.coords).for_each(|(a, b)| *a /= b)
    }
}

impl<const N: usize> DivAssign<f32> for Point<N> {
    fn div_assign(&mut self, rhs: f32) {
        self.coords.iter_mut().for_each(|a| *a /= rhs);
    }
}

impl<const N: usize> Div<Point<N>> for Point<N> {
    type Output = Point<N>;
    fn div(self, rhs: Point<N>) -> Self::Output {
        let mut result = self;
        result /= rhs;
        result
    }
}

impl<const N: usize> Div<f32> for Point<N> {
    type Output = Point<N>;
    fn div(self, rhs: f32) -> Self::Output {
        let mut result = self;
        result /= rhs;
        result
    }
}

impl<const N: usize> Point<N> {
    pub fn length_squared(self) -> f32 {
        (self * self).coords.iter().sum::<f32>()
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
        let mut points = points.into_iter();
        let first = points.next()?;
        let (sum, cnt) = points.fold((first, 1), |(sum, cnt), p| (sum + p, cnt + 1));
        Some(sum / cnt as f32)
    }

    pub fn bounding_box(points: impl IntoIterator<Item = Self>) -> Option<(Self, Self)> {
        let mut points = points.into_iter();
        let first = points.next()?;
        Some(points.fold((first, first), |(mut low, mut high), p| {
            zip(&mut low.coords, p.coords).for_each(|(a, b)| *a = a.min(b));
            zip(&mut high.coords, p.coords).for_each(|(a, b)| *a = a.max(b));
            (low, high)
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EPS;

    fn assert_approx_eq(a: f32, b: f32) {
        assert!((a - b).abs() < EPS,);
    }

    fn assert_point_approx_eq<const N: usize>(a: Point<N>, b: Point<N>) {
        zip(a.coords, b.coords).for_each(|(a, b)| assert_approx_eq(a, b));
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
        let bounds = Point::bounding_box(points);
        assert!(bounds.is_some());
        let (low, high) = bounds.unwrap();
        assert_point_approx_eq(low, Point::from([0.0, 0.0, 1.0]));
        assert_point_approx_eq(high, Point::from([6.0, 9.0, 15.0]));
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
}
