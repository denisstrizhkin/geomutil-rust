use crate::Triangle;

#[derive(Clone)]
pub struct Shape2D {
    pub triangles: Vec<Triangle>,
}

impl Shape2D {
    #[must_use]
    pub const fn new(triangles: Vec<Triangle>) -> Self {
        Self { triangles }
    }

    #[must_use]
    pub fn area(&self) -> f32 {
        self.triangles.iter().map(Triangle::area).sum()
    }
}
