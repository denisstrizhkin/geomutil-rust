use serde::{Deserialize, Serialize};
use std::{
    cmp,
    collections::HashSet,
    fs::File,
    hash::{Hash, Hasher},
    io::{self, BufReader},
    iter::Sum,
    ops,
    path::Path,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Hash for Point2D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

impl Eq for Point2D {}

impl Ord for Point2D {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.x.total_cmp(&other.x) {
            cmp::Ordering::Equal => self.y.total_cmp(&other.y),
            ord => ord,
        }
    }
}

impl PartialOrd for Point2D {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ops::Add<Point2D> for Point2D {
    type Output = Point2D;
    fn add(self, rhs: Point2D) -> Self::Output {
        Point2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Add<f32> for Point2D {
    type Output = Point2D;
    fn add(self, rhs: f32) -> Self::Output {
        Point2D::new(self.x + rhs, self.y + rhs)
    }
}

impl ops::Sub<Point2D> for Point2D {
    type Output = Point2D;
    fn sub(self, rhs: Point2D) -> Self::Output {
        Point2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Sub<f32> for Point2D {
    type Output = Point2D;
    fn sub(self, rhs: f32) -> Self::Output {
        Point2D::new(self.x - rhs, self.y - rhs)
    }
}

impl ops::Mul<Point2D> for Point2D {
    type Output = Point2D;
    fn mul(self, rhs: Point2D) -> Self::Output {
        Point2D::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl ops::Mul<f32> for Point2D {
    type Output = Point2D;
    fn mul(self, rhs: f32) -> Self::Output {
        Point2D::new(self.x * rhs, self.y * rhs)
    }
}

impl ops::Div<Point2D> for Point2D {
    type Output = Point2D;
    fn div(self, rhs: Point2D) -> Self::Output {
        Point2D::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl ops::Div<f32> for Point2D {
    type Output = Point2D;
    fn div(self, rhs: f32) -> Self::Output {
        Point2D::new(self.x / rhs, self.y / rhs)
    }
}

impl Sum for Point2D {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Point2D::default(), |r, p| r + p)
    }
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Point2D { x, y }
    }

    pub fn length_squared(self) -> f32 {
        let m = self * self;
        m.x + m.y
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn distance(self, other: Point2D) -> f32 {
        (self - other).length()
    }

    pub fn distance_squared(self, other: Point2D) -> f32 {
        (self - other).length_squared()
    }

    pub fn rotate(&self, angle: f32) -> Point2D {
        let sin_a = angle.sin();
        let cos_a = angle.cos();
        Point2D::new(
            cos_a * self.x - sin_a * self.y,
            sin_a * self.x + cos_a * self.y,
        )
    }

    pub fn normalize(self) -> Point2D {
        let len = self.length();
        if len > 0.0 { self / len } else { self }
    }
}

pub fn points_from_file(path: &Path) -> Result<Vec<Point2D>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let points = serde_json::from_reader(reader)?;
    Ok(points)
}

pub fn points_unique(points: &[Point2D]) -> Vec<Point2D> {
    points
        .iter()
        .copied()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

pub fn points_average(points: &[Point2D]) -> Option<Point2D> {
    if points.is_empty() {
        return None;
    }
    let sum = points.iter().copied().sum::<Point2D>();
    Some(sum / (points.len() as f32))
}

pub fn points_min(points: &[Point2D]) -> Option<Point2D> {
    points.iter().copied().min()
}

pub fn points_max(points: &[Point2D]) -> Option<Point2D> {
    points.iter().copied().max()
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module

    // Helper for float comparison due to precision issues
    const EPSILON: f32 = 0.00001;

    fn assert_approx_eq(a: f32, b: f32) {
        assert!((a - b).abs() < EPSILON,);
    }

    fn assert_point2d_approx_eq(p1: Point2D, p2: Point2D) {
        assert_approx_eq(p1.x, p2.x);
        assert_approx_eq(p1.y, p2.y);
    }

    #[test]
    fn test_point2d_new() {
        let p = Point2D::new(1.0, 2.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
    }

    #[test]
    fn test_point2d_default() {
        let p = Point2D::default();
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
    }

    #[test]
    fn test_point2d_add_point2d() {
        let p1 = Point2D::new(1.0, 2.0);
        let p2 = Point2D::new(3.0, 4.0);
        let result = p1 + p2;
        assert_point2d_approx_eq(result, Point2D::new(4.0, 6.0));
    }

    #[test]
    fn test_point2d_add_f32() {
        let p = Point2D::new(1.0, 2.0);
        let scalar = 5.0;
        let result = p + scalar;
        assert_point2d_approx_eq(result, Point2D::new(6.0, 7.0));
    }

    #[test]
    fn test_point2d_sub_point2d() {
        let p1 = Point2D::new(5.0, 7.0);
        let p2 = Point2D::new(1.0, 2.0);
        let result = p1 - p2;
        assert_point2d_approx_eq(result, Point2D::new(4.0, 5.0));
    }

    #[test]
    fn test_point2d_sub_f32() {
        let p = Point2D::new(10.0, 8.0);
        let scalar = 3.0;
        let result = p - scalar;
        assert_point2d_approx_eq(result, Point2D::new(7.0, 5.0));
    }

    #[test]
    fn test_point2d_mul_point2d() {
        let p1 = Point2D::new(2.0, 3.0);
        let p2 = Point2D::new(4.0, 5.0);
        let result = p1 * p2;
        assert_point2d_approx_eq(result, Point2D::new(8.0, 15.0));
    }

    #[test]
    fn test_point2d_mul_f32() {
        let p = Point2D::new(2.0, 3.0);
        let scalar = 2.5;
        let result = p * scalar;
        assert_point2d_approx_eq(result, Point2D::new(5.0, 7.5));
    }

    #[test]
    fn test_point2d_div_point2d() {
        let p1 = Point2D::new(10.0, 20.0);
        let p2 = Point2D::new(2.0, 4.0);
        let result = p1 / p2;
        assert_point2d_approx_eq(result, Point2D::new(5.0, 5.0));
    }

    #[test]
    fn test_point2d_div_f32() {
        let p = Point2D::new(10.0, 20.0);
        let scalar = 2.0;
        let result = p / scalar;
        assert_point2d_approx_eq(result, Point2D::new(5.0, 10.0));
    }

    #[test]
    fn test_point2d_hash_eq() {
        let p1 = Point2D::new(1.0, 2.0);
        let p2 = Point2D::new(1.0, 2.0);
        let p3 = Point2D::new(3.0, 4.0);

        let mut set = HashSet::new();
        set.insert(p1);
        assert!(set.contains(&p2));
        assert!(!set.contains(&p3));
        assert_eq!(set.len(), 1);

        set.insert(p3);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_point2d_ord() {
        let p1 = Point2D::new(1.0, 2.0);
        let p2 = Point2D::new(1.0, 3.0);
        let p3 = Point2D::new(2.0, 1.0);
        let p4 = Point2D::new(1.0, 2.0);

        assert!(p1 < p2);
        assert!(p1 < p3);
        assert!(p2 > p1);
        assert!(p3 > p1);
        assert_eq!(p1, p4);
        assert!(p1 <= p4);
        assert!(p1 >= p4);
    }

    #[test]
    fn test_point2d_length() {
        let p = Point2D::new(3.0, 4.0);
        assert_approx_eq(p.length(), 5.0);
        let p_zero = Point2D::new(0.0, 0.0);
        assert_approx_eq(p_zero.length(), 0.0);
        let p_neg = Point2D::new(-3.0, -4.0);
        assert_approx_eq(p_neg.length(), 5.0);
    }

    #[test]
    fn test_point2d_distance() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(3.0, 4.0);
        assert_approx_eq(p1.distance(p2), 5.0);
        assert_approx_eq(p2.distance(p1), 5.0);
        assert_approx_eq(p1.distance(p1), 0.0);
    }

    #[test]
    fn test_point2d_distance_squared() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(3.0, 4.0);
        assert_approx_eq(p1.distance_squared(p2), 25.0);
    }

    #[test]
    fn test_point2d_rotate_90_degrees() {
        let p = Point2D::new(1.0, 0.0);
        let rotated = p.rotate(std::f32::consts::FRAC_PI_2);
        assert_point2d_approx_eq(rotated, Point2D::new(0.0, 1.0));
    }

    #[test]
    fn test_point2d_rotate_180_degrees() {
        let p = Point2D::new(1.0, 0.0);
        let rotated = p.rotate(std::f32::consts::PI); // 180 degrees (pi radians)
        assert_point2d_approx_eq(rotated, Point2D::new(-1.0, 0.0));
    }

    #[test]
    fn test_point2d_rotate_origin() {
        let p = Point2D::new(0.0, 0.0);
        let rotated = p.rotate(std::f32::consts::FRAC_PI_2);
        assert_point2d_approx_eq(rotated, Point2D::new(0.0, 0.0));
    }

    #[test]
    fn test_point2d_normalize() {
        let p = Point2D::new(3.0, 4.0);
        let normalized = p.normalize();
        assert_approx_eq(normalized.x, 0.6); // 3/5
        assert_approx_eq(normalized.y, 0.8); // 4/5
        assert_approx_eq(normalized.length(), 1.0);

        let p_zero = Point2D::new(0.0, 0.0);
        let normalized_zero = p_zero.normalize();
        assert_point2d_approx_eq(normalized_zero, Point2D::new(0.0, 0.0));
    }

    #[test]
    fn test_sum_iterator() {
        let points = vec![
            Point2D::new(1.0, 1.0),
            Point2D::new(2.0, 2.0),
            Point2D::new(3.0, 3.0),
        ];
        let total_sum: Point2D = points.iter().copied().sum();
        assert_point2d_approx_eq(total_sum, Point2D::new(6.0, 6.0));
    }

    #[test]
    fn test_points_unique() {
        let points = vec![
            Point2D::new(1.0, 1.0),
            Point2D::new(2.0, 2.0),
            Point2D::new(1.0, 1.0),
            Point2D::new(3.0, 3.0),
        ];
        let unique_points = points_unique(&points);
        assert_eq!(unique_points.len(), 3);
        assert!(unique_points.contains(&Point2D::new(1.0, 1.0)));
        assert!(unique_points.contains(&Point2D::new(2.0, 2.0)));
        assert!(unique_points.contains(&Point2D::new(3.0, 3.0)));
    }

    #[test]
    fn test_points_average_non_empty() {
        let points = vec![
            Point2D::new(1.0, 1.0),
            Point2D::new(2.0, 2.0),
            Point2D::new(3.0, 3.0),
        ];
        let avg = points_average(&points);
        assert!(avg.is_some());
        assert_point2d_approx_eq(avg.unwrap(), Point2D::new(2.0, 2.0));
    }

    #[test]
    fn test_points_average_empty() {
        let points: Vec<Point2D> = Vec::new();
        let avg = points_average(&points);
        assert!(avg.is_none());
    }

    #[test]
    fn test_points_min_non_empty() {
        let points = vec![
            Point2D::new(5.0, 5.0),
            Point2D::new(1.0, 10.0),
            Point2D::new(1.0, 5.0),
            Point2D::new(10.0, 1.0),
        ];
        let min_point = points_min(&points);
        assert_eq!(min_point, Some(Point2D::new(1.0, 5.0)));
    }

    #[test]
    fn test_points_min_empty() {
        let points: Vec<Point2D> = Vec::new();
        let min_point = points_min(&points);
        assert!(min_point.is_none(),);
    }

    #[test]
    fn test_points_max_non_empty() {
        let points = vec![
            Point2D::new(5.0, 5.0),
            Point2D::new(1.0, 10.0),
            Point2D::new(10.0, 1.0),
            Point2D::new(9.0, 100.0),
        ];
        let max_point = points_max(&points);
        assert_eq!(max_point, Some(Point2D::new(10.0, 1.0)));
    }

    #[test]
    fn test_points_max_empty() {
        let points: Vec<Point2D> = Vec::new();
        let max_point = points_max(&points);
        assert!(max_point.is_none(),);
    }
}
