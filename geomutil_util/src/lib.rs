mod edge2d;
mod point;
mod shape2d;
mod triangle2d;

pub use edge2d::Edge2;
pub use point::{Point2, Point3};
pub use shape2d::Shape2D;
pub use triangle2d::Triangle2;

pub const EPS: f32 = 1e-10;
