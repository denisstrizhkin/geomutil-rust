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
    pub fn new(a: Point2, b: Point2) -> Self {
        Self { a, b }
    }

    pub fn canonical(&self) -> Edge2 {
        Edge2::new(self.a.min(self.b), self.a.max(self.b))
    }

    pub fn flip(&self) -> Edge2 {
        Edge2::new(self.b, self.a)
    }

    pub fn length(&self) -> f32 {
        self.a.distance(self.b)
    }

    pub fn length_squared(&self) -> f32 {
        self.a.distance_squared(self.b)
    }
}
