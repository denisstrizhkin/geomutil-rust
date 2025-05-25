use std::collections::HashMap;
use util::{Point2D, Triangle2D, points_average, points_bounding_box, points_unique};

pub struct Triangulation2D {
    pub bounding_triangle: Triangle2D,
    pub triangles: Vec<Triangle2D>,
}

impl Triangulation2D {
    fn new(bounding_triangle: Triangle2D) -> Self {
        Self {
            bounding_triangle,
            triangles: vec![bounding_triangle],
        }
    }

    fn add(mut self, point: Point2D) -> Self {
        let bad_triangles = self
            .triangles
            .iter()
            .filter(|t| t.is_inside_circumcircle(point).unwrap())
            .copied()
            .collect::<Vec<_>>();
        let mut edges = HashMap::new();
        for t in bad_triangles.iter() {
            for e in t.edges() {
                let e = e.canonical();
                edges.entry(e).and_modify(|c| *c += 1).or_insert(0);
            }
        }
        let polygon = edges
            .into_iter()
            .filter(|(_, c)| c.eq(&1))
            .map(|(e, _)| e)
            .collect::<Vec<_>>();
        self.triangles = self
            .triangles
            .into_iter()
            .filter(|t| !bad_triangles.contains(t))
            .collect::<Vec<_>>();
        let new_triangles = polygon
            .into_iter()
            .map(|e| Triangle2D::new(e.a, e.b, point));
        self.triangles.extend(new_triangles);
        self
    }

    fn finalize(mut self) -> Self {
        self.triangles = self
            .triangles
            .into_iter()
            .filter(|t| {
                !(t.has_point(&self.bounding_triangle.a)
                    || t.has_point(&self.bounding_triangle.b)
                    || t.has_point(&self.bounding_triangle.c))
            })
            .collect::<Vec<_>>();
        self
    }
}

fn get_bounding_triangle(points: &[Point2D]) -> Triangle2D {
    let (lower_bound, upper_bound) = points_bounding_box(points).unwrap();
    let d = upper_bound - lower_bound;
    let d = 3.0 * d.x.max(d.y);
    let center = points_average(&[lower_bound, upper_bound]).unwrap();
    Triangle2D::new(
        Point2D::new(center.x - 0.866 * d, center.y - 0.5 * d),
        Point2D::new(center.x + 0.866 * d, center.y - 0.5 * d),
        Point2D::new(center.x, center.y + d),
    )
}

pub fn triangulate(points: &[Point2D]) -> Option<Triangulation2D> {
    let points = points_unique(points);
    if points.len() < 3 {
        return None;
    }
    let bounding_triangle = get_bounding_triangle(&points);
    let triangulation = points
        .iter()
        .copied()
        .fold(Triangulation2D::new(bounding_triangle), |t, p| t.add(p))
        .finalize();
    Some(triangulation)
}
