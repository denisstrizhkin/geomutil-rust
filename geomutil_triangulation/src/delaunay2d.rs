use geomutil_util::{Point2D, Triangle2D, points_average, points_bounding_box, points_unique};
use std::collections::HashMap;

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
                *edges.entry(e).or_insert(0) += 1;
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

#[cfg(test)]
mod tests {
    use super::*;
    use util::Point2D;

    #[test]
    fn test_triangulate_rectangle() {
        // Define the four corners of a rectangle
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(10.0, 0.0);
        let p3 = Point2D::new(10.0, 10.0);
        let p4 = Point2D::new(0.0, 10.0);

        let points = vec![p1, p2, p3, p4];

        // Perform the triangulation
        let result = triangulate(&points);

        // Assert that a triangulation was returned
        assert!(result.is_some(), "Triangulation should not be None");

        let triangulation = result.unwrap();

        // A rectangle should be triangulated into exactly two triangles
        assert_eq!(
            triangulation.triangles.len(),
            2,
            "A rectangle should be triangulated into 2 triangles"
        );

        // Further assertions to check the specific triangles
        // For a rectangle (0,0), (10,0), (10,10), (0,10),
        // it should be split into two triangles, e.g.,
        // T1: (0,0), (10,0), (10,10)
        // T2: (0,0), (10,10), (0,10)
        // Or the other diagonal:
        // T1: (0,0), (10,0), (0,10)
        // T2: (10,0), (10,10), (0,10)

        let t1 = triangulation.triangles[0];
        let t2 = triangulation.triangles[1];

        // Check if the two triangles cover all original points
        let mut all_points_in_triangulation = std::collections::HashSet::new();
        all_points_in_triangulation.insert(t1.a);
        all_points_in_triangulation.insert(t1.b);
        all_points_in_triangulation.insert(t1.c);
        all_points_in_triangulation.insert(t2.a);
        all_points_in_triangulation.insert(t2.b);
        all_points_in_triangulation.insert(t2.c);

        assert!(all_points_in_triangulation.contains(&p1));
        assert!(all_points_in_triangulation.contains(&p2));
        assert!(all_points_in_triangulation.contains(&p3));
        assert!(all_points_in_triangulation.contains(&p4));

        // Check that the two triangles share a common edge (the diagonal)
        let mut all_edges = HashMap::new();
        for t in &triangulation.triangles {
            for edge in t.edges().iter().map(|e| e.canonical()) {
                *all_edges.entry(edge).or_insert(0) += 1;
            }
        }

        // In a valid triangulation of a convex polygon, internal edges appear twice,
        // and boundary edges appear once. For a rectangle split into two triangles,
        // there should be 4 boundary edges and 1 internal (shared) edge.
        let boundary_edges_count = all_edges.values().filter(|&&c| c == 1).count();
        let internal_edges_count = all_edges.values().filter(|&&c| c == 2).count();

        assert_eq!(boundary_edges_count, 4, "Expected 4 boundary edges");
        assert_eq!(internal_edges_count, 1, "Expected 1 internal (shared) edge");

        println!(
            "Triangulation successful! Triangles: {:?}",
            triangulation.triangles
        );
    }
}
