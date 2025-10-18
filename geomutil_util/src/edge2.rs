use crate::Point2;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Edge2 {
    pub a: Point2,
    pub b: Point2,
}

impl Edge2 {
    #[must_use]
    pub const fn new(a: Point2, b: Point2) -> Self {
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
