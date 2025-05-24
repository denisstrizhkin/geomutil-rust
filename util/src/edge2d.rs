use crate::Point2D;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Edge2D {
    pub a: Point2D,
    pub b: Point2D,
}

impl Edge2D {
    pub fn new(a: Point2D, b: Point2D) -> Self {
        Self { a, b }
    }

    pub fn length(&self) -> f32 {
        self.a.distance(self.b)
    }

    pub fn length_squared(&self) -> f32 {
        self.a.distance_squared(self.b)
    }
}
