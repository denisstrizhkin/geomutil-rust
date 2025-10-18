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
        let min = {
            let mut min = Point::<N>::default();
            iter::zip(&mut min, iter::zip(a, b)).for_each(|(val, (a, b))| *val = a.min(b));
            min
        };
        let max = {
            let mut max = Point::<N>::default();
            iter::zip(&mut max, iter::zip(a, b)).for_each(|(val, (a, b))| *val = a.max(b));
            max
        };
        Self { min, max }
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
        // can unwrap because we always have 2 points
        Point::avg([self.min, self.max]).unwrap()
    }

    #[must_use]
    pub fn volume(&self) -> f32 {
        self.dimensions()
            .iter()
            .copied()
            .reduce(|acc, x| (acc * x))
            .unwrap()
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
