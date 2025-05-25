use crate::triangulate;
use geomutil_util::{Edge2D, Point2D, Shape2D, Triangle2D};
use std::collections::{HashMap, VecDeque};

struct AlphaShape2D {
    alpha: f32,
    triangles: Vec<Triangle2D>,
    connections: Vec<[Option<usize>; 3]>,
}

impl AlphaShape2D {
    fn new(triangles: Vec<Triangle2D>, alpha: f32) -> Self {
        Self {
            alpha,
            triangles,
            connections: Vec::new(),
        }
    }

    fn prune(&mut self) {
        let r_sq = (1.0 / self.alpha) * (1.0 / self.alpha);
        for i in (0..self.triangles.len()).rev() {
            if self.triangles[i].circumcircle_radius_squared() > r_sq {
                self.triangles.swap_remove(i);
            }
        }
        self.connections = vec![[None; 3]; self.triangles.len()];
    }

    fn build_connections_graph(&mut self) {
        let mut adjacent_edges: HashMap<Edge2D, [Option<usize>; 2]> =
            HashMap::with_capacity(self.triangles.len() * 2);
        for (i, t) in self.triangles.iter().enumerate() {
            for e in t.edges() {
                let e = e.canonical();
                adjacent_edges
                    .entry(e)
                    .and_modify(|adjacent: &mut _| adjacent[1] = Some(i))
                    .or_insert([Some(i), None]);
            }
        }
        for (_, neighbours) in adjacent_edges.drain() {
            for (i, j) in [
                (neighbours[0], neighbours[1]),
                (neighbours[1], neighbours[0]),
            ] {
                if let Some(i) = i {
                    let mut adjacent = self.connections[i];
                    if let Some(neigh) = adjacent.iter_mut().find(|a| a.is_none()) {
                        *neigh = j;
                    }
                }
            }
        }
    }

    fn shapes(&mut self) -> Vec<Shape2D> {
        self.prune();
        self.build_connections_graph();
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.triangles.len()];
        let mut shapes = Vec::new();
        for i in 0..self.triangles.len() {
            if visited[i] {
                continue;
            }
            queue.push_back(i);
            let mut shape = Vec::new();
            while let Some(i) = queue.pop_front() {
                visited[i] = true;
                shape.push(self.triangles[i].clone());
                for neigh_i in self.connections[i].into_iter().flatten() {
                    if !visited[neigh_i] {
                        queue.push_back(neigh_i);
                    }
                }
            }
            shapes.push(Shape2D::new(shape));
        }
        shapes
    }
}

pub fn alpha_shape_2d(points: &[Point2D], alpha: f32) -> Option<Vec<Shape2D>> {
    let triangulation = triangulate(points)?;
    let mut alpha_shape = AlphaShape2D::new(triangulation.triangles, alpha);
    let shapes = alpha_shape.shapes();
    Some(shapes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_disconnected_rectangles() {
        // Define points for two rectangles
        let points = vec![
            Point2D::new(0.0, 0.0), // p0
            Point2D::new(1.0, 0.0), // p1
            Point2D::new(1.0, 1.0), // p2
            Point2D::new(0.0, 1.0), // p3
            Point2D::new(3.0, 0.0), // p4
            Point2D::new(4.0, 0.0), // p5
            Point2D::new(4.0, 1.0), // p6
            Point2D::new(3.0, 1.0), // p7
        ];

        // Choose an alpha value.
        // - Internal triangles (e.g., (0,0)-(1,0)-(1,1)) have R^2 = 0.5.
        // - Bridging triangles (e.g., (1,0)-(3,0)-(1,1)) have R^2 = 1.25.
        //
        // We want to prune triangles where R^2 > (1/alpha)^2.
        // To prune bridging triangles (R^2 = 1.25) but keep internal ones (R^2 = 0.5),
        // we need (1/alpha)^2 to be between 0.5 and 1.25.
        // Let's pick (1/alpha)^2 = 1.0, which means 1/alpha = 1.0, so alpha = 1.0.
        //
        // With alpha = 1.0:
        // - Internal triangles (R^2 = 0.5): 0.5 <= 1.0, so KEPT.
        // - Bridging triangles (R^2 = 1.25): 1.25 > 1.0, so PRUNED.
        let alpha = 1.25;

        let result = alpha_shape_2d(&points, alpha);

        // Assert that the result is Some and contains exactly two shapes
        assert!(
            result.is_some(),
            "alpha_shape_2d should return Some for valid input."
        );
        let shapes = result.unwrap();
        assert_eq!(shapes.len(), 2, "Expected 2 disconnected shapes.");

        // Optionally, you can add more assertions to check the content of the shapes,
        // e.g., verify that each shape contains triangles forming one of the rectangles.
        // This would involve iterating through the shapes and checking their constituent triangles.
        // For this specific test, verifying the count of shapes is sufficient for the requirement.
    }
}
