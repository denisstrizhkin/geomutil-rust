use serde::{Deserialize, Serialize};

use crate::point::Point;

#[derive(
    Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Edge<const N: usize> {
    pub a: Point<N>,
    pub b: Point<N>,
}

pub type Edge2 = Edge<2>;
pub type Edge3 = Edge<3>;

impl<const N: usize> Edge<N> {
    #[must_use]
    pub const fn new(a: Point<N>, b: Point<N>) -> Self {
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
    pub fn length(&self) -> f32 {
        self.a.distance(self.b)
    }

    #[must_use]
    pub fn length_squared(&self) -> f32 {
        self.a.distance_squared(self.b)
    }
}
