use crate::Triangle2D;

pub struct Shape2D {
    pub triangles: Vec<Triangle2D>,
}

impl Shape2D {
    pub fn new(triangles: Vec<Triangle2D>) -> Self {
        Self { triangles }
    }

    pub fn area(&self) -> f32 {
        self.triangles.iter().map(|t| t.area()).sum()
    }
}
