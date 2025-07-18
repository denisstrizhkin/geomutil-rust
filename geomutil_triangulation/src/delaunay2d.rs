use geomutil_util::{Point2, Triangle2};
use std::collections::HashMap;

pub struct Triangulation2 {
    pub bounding_triangle: Triangle2,
    pub triangles: Vec<Triangle2>,
}

impl Triangulation2 {
    fn new(bounding_triangle: Triangle2) -> Self {
        Self {
            bounding_triangle: bounding_triangle.clone(),
            triangles: vec![bounding_triangle],
        }
    }

    fn add(&mut self, point: Point2) {
        let mut triangles_to_keep = Vec::with_capacity(self.triangles.len());
        let mut triangles_to_add_edges = HashMap::with_capacity(self.triangles.len() * 2);
        for t in self.triangles.drain(..) {
            if t.is_inside_circumcircle(point) {
                for e in t.edges() {
                    let canonical_edge = e.canonical();
                    *triangles_to_add_edges.entry(canonical_edge).or_insert(0) += 1;
                }
            } else {
                triangles_to_keep.push(t);
            }
        }
        self.triangles = triangles_to_keep;
        let new_triangles = triangles_to_add_edges
            .into_iter()
            .filter(|(_, c)| c.eq(&1))
            .filter_map(|(e, _)| Triangle2::new(e.a, e.b, point));
        self.triangles.extend(new_triangles);
    }

    fn finalize(&mut self) {
        self.triangles.retain(|t| {
            !(t.has_point(&self.bounding_triangle.a)
                || t.has_point(&self.bounding_triangle.b)
                || t.has_point(&self.bounding_triangle.c))
        });
    }
}

fn get_bounding_triangle(points: impl IntoIterator<Item = Point2>) -> Option<Triangle2> {
    let bbox = Point2::bounding_box(points)?;
    let d = bbox.dimensions();
    let d = 3.0 * d.x.max(d.y);
    let center = bbox.center();
    Triangle2::new(
        Point2::from([center.x - 0.866 * d, center.y - 0.5 * d]),
        Point2::from([center.x + 0.866 * d, center.y - 0.5 * d]),
        Point2::from([center.x, center.y + d]),
    )
}

pub fn triangulate(points: impl IntoIterator<Item = Point2>) -> Option<Triangulation2> {
    let points = Point2::unique(points);
    if points.len() < 3 {
        return None;
    }
    let bounding_triangle = get_bounding_triangle(points.clone())?;
    let mut triangulation = Triangulation2::new(bounding_triangle);
    for point in points {
        triangulation.add(point);
    }
    triangulation.finalize();
    Some(triangulation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangulate_rectangle() {
        // Define the four corners of a rectangle
        let p1 = Point2::from([0.0, 0.0]);
        let p2 = Point2::from([10.0, 0.0]);
        let p3 = Point2::from([10.0, 10.0]);
        let p4 = Point2::from([0.0, 10.0]);

        let points = vec![p1, p2, p3, p4];

        // Perform the triangulation
        let result = triangulate(points);

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

        let t1 = &triangulation.triangles[0];
        let t2 = &triangulation.triangles[1];

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
