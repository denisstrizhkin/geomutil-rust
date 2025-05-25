mod edge2d;
mod point2d;
mod triangle2d;

pub use edge2d::Edge2D;
pub use point2d::{
    Point2D, points_average, points_bounding_box, points_from_file, points_max, points_min,
    points_unique,
};
pub use triangle2d::Triangle2D;

pub const EPS: f32 = 1e-10;
