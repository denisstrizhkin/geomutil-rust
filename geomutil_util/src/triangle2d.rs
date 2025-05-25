use crate::{EPS, Edge2D, Point2D};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Triangle2D {
    pub a: Point2D,
    pub b: Point2D,
    pub c: Point2D,
    circumcenter: Point2D,
    circumradius_squared: f32,
}

impl Eq for Triangle2D {}

impl Triangle2D {
    pub fn new(a: Point2D, b: Point2D, c: Point2D) -> Option<Self> {
        let mut t = Self {
            a,
            b,
            c,
            ..Default::default()
        };
        t.circumcenter = t.calc_circumcenter()?;
        t.circumradius_squared = t.calc_circumradius_squared();
        Some(t)
    }

    fn calc_circumcenter(&self) -> Option<Point2D> {
        let a = self.a;
        let b = self.b;
        let c = self.c;
        let d = 2.0 * (a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y));

        if d.abs() < EPS {
            return None;
        }

        let a_len_sq = a.length_squared();
        let b_len_sq = b.length_squared();
        let c_len_sq = c.length_squared();

        let ux = (a_len_sq * (b.y - c.y) + b_len_sq * (c.y - a.y) + c_len_sq * (a.y - b.y)) / d;
        let uy = (a_len_sq * (c.x - b.x) + b_len_sq * (a.x - c.x) + c_len_sq * (b.x - a.x)) / d;

        Some(Point2D::new(ux, uy))
    }

    pub fn circumcenter(&self) -> Point2D {
        self.circumcenter
    }

    fn calc_circumradius_squared(&self) -> f32 {
        self.circumcenter.distance_squared(self.a)
    }

    pub fn circumcircle_radius_squared(&self) -> f32 {
        self.circumradius_squared
    }

    pub fn circumcircle_radius(&self) -> f32 {
        self.circumradius_squared.sqrt()
    }

    pub fn is_inside_circumcircle(&self, p: Point2D) -> bool {
        let d = self.circumcenter.distance_squared(p);
        d <= self.circumradius_squared + EPS
    }

    pub fn has_point(&self, p: &Point2D) -> bool {
        self.a.eq(p) || self.b.eq(p) || self.c.eq(p)
    }

    pub fn edges(&self) -> [Edge2D; 3] {
        [
            Edge2D::new(self.a, self.b),
            Edge2D::new(self.b, self.c),
            Edge2D::new(self.c, self.a),
        ]
    }

    pub fn perimeter(&self) -> f32 {
        self.edges().iter().map(|e| e.length()).sum()
    }

    pub fn area(&self) -> f32 {
        let edges = self.edges();
        let a = edges[0].length();
        let b = edges[1].length();
        let c = edges[2].length();
        let s = 0.5 * (a + b + c);
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }
}
