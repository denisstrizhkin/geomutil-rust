use serde::{Deserialize, Serialize};

use crate::{point::Point, scalar::Float};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Edge<const N: usize, T: Float> {
    pub a: Point<N, T>,
    pub b: Point<N, T>,
}

pub type Edge2<T> = Edge<2, T>;
pub type Edge3<T> = Edge<3, T>;

impl<const N: usize, T: Float> Edge<N, T> {
    #[must_use]
    pub const fn new(a: Point<N, T>, b: Point<N, T>) -> Self {
        Self { a, b }
    }

    #[must_use]
    pub fn canonical(&self) -> Self {
        Self::new(self.a.min(self.b), self.a.max(self.b))
    }

    #[must_use]
    pub const fn flip(&self) -> Self {
        Self::new(self.b, self.a)
    }

    #[must_use]
    pub fn length(&self) -> T {
        self.a.distance(self.b)
    }

    #[must_use]
    pub fn length_squared(&self) -> T {
        self.a.distance_squared(self.b)
    }
}
