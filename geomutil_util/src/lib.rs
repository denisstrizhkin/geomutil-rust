mod bounding_box;
mod edge2;
mod point;
mod shape2;
mod triangle;

pub use bounding_box::{BoundingBox2, BoundingBox3};
pub use edge2::Edge2;
pub use point::{Point2, Point3};
pub use shape2::Shape2D;
pub use triangle::Triangle;

pub const EPS: f32 = 1e-10;
