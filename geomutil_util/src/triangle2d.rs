use crate::{EPS, Edge2, Point2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Triangle2 {
    pub a: Point2,
    pub b: Point2,
    pub c: Point2,
    circumcenter: Point2,
    circumradius_squared: f32,
}

impl Eq for Triangle2 {}

impl Triangle2 {
    pub fn new(a: Point2, b: Point2, c: Point2) -> Option<Self> {
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

    fn calc_circumcenter(&self) -> Option<Point2> {
        let a = self.a;
        let b = self.b;
        let c = self.c;
        let d = 2.0 * (a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y));

        if d.abs() == 0.0 {
            return None;
        }

        let a_len_sq = a.length_squared();
        let b_len_sq = b.length_squared();
        let c_len_sq = c.length_squared();

        let ux = (a_len_sq * (b.y - c.y) + b_len_sq * (c.y - a.y) + c_len_sq * (a.y - b.y)) / d;
        let uy = (a_len_sq * (c.x - b.x) + b_len_sq * (a.x - c.x) + c_len_sq * (b.x - a.x)) / d;

        Some(Point2::from([ux, uy]))
    }

    pub fn circumcenter(&self) -> Point2 {
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

    pub fn is_inside_circumcircle(&self, p: Point2) -> bool {
        let d = self.circumcenter.distance_squared(p);
        d <= self.circumradius_squared + EPS
    }

    pub fn has_point(&self, p: &Point2) -> bool {
        self.a.eq(p) || self.b.eq(p) || self.c.eq(p)
    }

    pub fn edges(&self) -> [Edge2; 3] {
        [
            Edge2::new(self.a, self.b),
            Edge2::new(self.b, self.c),
            Edge2::new(self.c, self.a),
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
        let s = 0.5f32 * (a + b + c);
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }
}
