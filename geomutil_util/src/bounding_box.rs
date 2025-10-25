use std::iter;

use crate::{point::Point, scalar::Float};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox<const N: usize, T: Float> {
    pub lower: Point<N, T>,
    pub upper: Point<N, T>,
}

pub type BoundingBox2<T> = BoundingBox<2, T>;
pub type BoundingBox3<T> = BoundingBox<3, T>;

impl<const N: usize, T: Float> BoundingBox<N, T> {
    #[must_use]
    pub fn new(a: Point<N, T>, b: Point<N, T>) -> Self {
        Point::bounding_box([a, b]).unwrap()
    }

    #[must_use]
    pub fn dimensions(&self) -> Point<N, T> {
        self.upper - self.lower
    }

    #[must_use]
    pub fn center(&self) -> Point<N, T> {
        Point::avg([self.lower, self.upper]).unwrap()
    }

    #[must_use]
    pub fn volume(&self) -> T {
        self.dimensions()
            .into_iter()
            .fold(T::from(1.0), |vol, x| vol * x)
    }

    #[must_use]
    pub fn contains(&self, p: Point<N, T>) -> bool {
        iter::zip(p, iter::zip(self.lower, self.upper)).all(|(x, (lo, hi))| x >= lo && x <= hi)
    }
}

impl<const N: usize, T: Float> From<(Point<N, T>, Point<N, T>)> for BoundingBox<N, T> {
    fn from(value: (Point<N, T>, Point<N, T>)) -> Self {
        Self::new(value.0, value.1)
    }
}
