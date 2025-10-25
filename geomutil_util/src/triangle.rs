use crate::{edge::Edge2, point::Point2, scalar::Float};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Triangle<T: Float> {
    pub a: Point2<T>,
    pub b: Point2<T>,
    pub c: Point2<T>,
    circumcenter: Point2<T>,
    circumradius_squared: T,
}

impl<T: Float> Triangle<T> {
    #[must_use]
    pub fn new(a: Point2<T>, b: Point2<T>, c: Point2<T>) -> Self {
        let mut t = Self {
            a,
            b,
            c,
            ..Default::default()
        };
        t.circumcenter = t.calc_circumcenter();
        t.circumradius_squared = t.calc_circumradius_squared();
        t
    }

    #[must_use]
    fn calc_circumcenter(&self) -> Point2<T> {
        let a = self.a;
        let b = self.b;
        let c = self.c;
        let d = T::from(2.0)
            * a.x
                .mul_add(b.y - c.y, b.x.mul_add(c.y - a.y, c.x * (a.y - b.y)));
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
        Point2::from([ux, uy])
    }

    #[must_use]
    pub const fn circumcenter(&self) -> Point2<T> {
        self.circumcenter
    }

    #[must_use]
    fn calc_circumradius_squared(&self) -> T {
        self.circumcenter.distance_squared(self.a)
    }

    #[must_use]
    pub const fn circumcircle_radius_squared(&self) -> T {
        self.circumradius_squared
    }

    #[must_use]
    pub fn circumcircle_radius(&self) -> T {
        self.circumradius_squared.sqrt()
    }

    #[must_use]
    pub fn is_inside_circumcircle(&self, p: Point2<T>) -> bool {
        let d = self.circumcenter.distance_squared(p);
        d <= self.circumradius_squared
    }

    #[must_use]
    pub fn has_point(&self, p: &Point2<T>) -> bool {
        self.a.eq(p) || self.b.eq(p) || self.c.eq(p)
    }

    #[must_use]
    pub const fn edges(&self) -> [Edge2<T>; 3] {
        [
            Edge2::new(self.a, self.b),
            Edge2::new(self.b, self.c),
            Edge2::new(self.c, self.a),
        ]
    }

    #[must_use]
    pub fn perimeter(&self) -> T {
        self.edges().iter().map(Edge2::length).sum()
    }

    #[must_use]
    pub fn area(&self) -> T {
        let edges = self.edges();
        let a = edges[0].length();
        let b = edges[1].length();
        let c = edges[2].length();
        let s = T::from(0.5) * (a + b + c);
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }
}
