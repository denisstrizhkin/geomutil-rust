mod edge2;
mod point;
mod shape2;
mod triangle2;

pub use edge2::Edge2;
pub use point::{Point2, Point3};
pub use shape2::Shape2D;
pub use triangle2::Triangle2;

pub const EPS: f32 = 1e-10;
