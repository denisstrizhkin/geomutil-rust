use crate::{EPS, Edge2, Point2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Triangle {
    pub a: Point2,
    pub b: Point2,
    pub c: Point2,
    circumcenter: Point2,
    circumradius_squared: f32,
}

impl Eq for Triangle {}

impl Triangle {
    #[must_use]
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

    #[must_use]
    fn calc_circumcenter(&self) -> Option<Point2> {
        let a = self.a;
        let b = self.b;
        let c = self.c;
        let d = 2.0
            * a.x
                .mul_add(b.y - c.y, b.x.mul_add(c.y - a.y, c.x * (a.y - b.y)));

        if d.abs() == 0.0 {
            return None;
        }

        let a_len_sq = a.length_squared();
        let b_len_sq = b.length_squared();
        let c_len_sq = c.length_squared();

        let ux = (a_len_sq.mul_add(
            b.y - c.y,
            b_len_sq.mul_add(c.y - a.y, c_len_sq * (a.y - b.y)),
        )) / d;
        let uy = (a_len_sq.mul_add(
            c.x - b.x,
            b_len_sq.mul_add(a.x - c.x, c_len_sq * (b.x - a.x)),
        )) / d;

        Some(Point2::from([ux, uy]))
    }

    #[must_use]
    pub const fn circumcenter(&self) -> Point2 {
        self.circumcenter
    }

    #[must_use]
    fn calc_circumradius_squared(&self) -> f32 {
        self.circumcenter.distance_squared(self.a)
    }

    #[must_use]
    pub const fn circumcircle_radius_squared(&self) -> f32 {
        self.circumradius_squared
    }

    #[must_use]
    pub fn circumcircle_radius(&self) -> f32 {
        self.circumradius_squared.sqrt()
    }

    #[must_use]
    pub fn is_inside_circumcircle(&self, p: Point2) -> bool {
        let d = self.circumcenter.distance_squared(p);
        d <= self.circumradius_squared + EPS
    }

    #[must_use]
    pub fn has_point(&self, p: &Point2) -> bool {
        self.a.eq(p) || self.b.eq(p) || self.c.eq(p)
    }

    #[must_use]
    pub const fn edges(&self) -> [Edge2; 3] {
        [
            Edge2::new(self.a, self.b),
            Edge2::new(self.b, self.c),
            Edge2::new(self.c, self.a),
        ]
    }

    #[must_use]
    pub fn perimeter(&self) -> f32 {
        self.edges().iter().map(Edge2::length).sum()
    }

    #[must_use]
    pub fn area(&self) -> f32 {
        let edges = self.edges();
        let a = edges[0].length();
        let b = edges[1].length();
        let c = edges[2].length();
        let s = 0.5f32 * (a + b + c);
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }
}
