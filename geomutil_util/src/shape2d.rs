use crate::Triangle2;

#[derive(Clone)]
pub struct Shape2D {
    pub triangles: Vec<Triangle2>,
}

impl Shape2D {
    pub fn new(triangles: Vec<Triangle2>) -> Self {
        Self { triangles }
    }

    pub fn area(&self) -> f32 {
        self.triangles.iter().map(|t| t.area()).sum()
    }
}
