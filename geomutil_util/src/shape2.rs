use crate::{scalar::Float, triangle::Triangle};

#[derive(Clone)]
pub struct Shape2D<T: Float> {
    pub triangles: Vec<Triangle<T>>,
}

impl<T: Float> Shape2D<T> {
    #[must_use]
    pub const fn new(triangles: Vec<Triangle<T>>) -> Self {
        Self { triangles }
    }

    #[must_use]
    pub fn area(&self) -> T {
        self.triangles.iter().map(Triangle::area).sum()
    }
}
