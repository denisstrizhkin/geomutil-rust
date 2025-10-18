use std::iter;

use crate::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundingBox<const N: usize> {
    min: Point<N>,
    max: Point<N>,
}

pub type BoundingBox2 = BoundingBox<2>;
pub type BoundingBox3 = BoundingBox<3>;

impl<const N: usize> BoundingBox<N> {
    #[must_use]
    pub fn new(a: Point<N>, b: Point<N>) -> Self {
        Point::bounding_box([a, b]).unwrap()
    }

    #[must_use]
    pub const fn lower(&self) -> Point<N> {
        self.min
    }

    #[must_use]
    pub const fn upper(&self) -> Point<N> {
        self.max
    }

    #[must_use]
    pub fn dimensions(&self) -> Point<N> {
        self.max - self.min
    }

    #[must_use]
    pub fn center(&self) -> Point<N> {
        Point::avg([self.min, self.max]).unwrap()
    }

    #[must_use]
    pub fn volume(&self) -> f32 {
        self.dimensions().into_iter().fold(1.0, |vol, x| vol * x)
    }

    #[must_use]
    pub fn contains(&self, p: Point<N>) -> bool {
        iter::zip(p, iter::zip(self.min, self.max)).all(|(x, (lo, hi))| x >= lo && x <= hi)
    }
}

impl<const N: usize> From<(Point<N>, Point<N>)> for BoundingBox<N> {
    fn from(value: (Point<N>, Point<N>)) -> Self {
        Self::new(value.0, value.1)
    }
}
