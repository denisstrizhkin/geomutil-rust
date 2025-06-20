use crate::triangulate;
use geomutil_util::{Edge2, Point2, Shape2D, Triangle2};
use std::collections::{HashMap, VecDeque};

struct AlphaShape2D {
    alpha: f32,
    triangles: Vec<Triangle2>,
    connections: Vec<[Option<usize>; 3]>,
}

impl AlphaShape2D {
    fn new(triangles: Vec<Triangle2>, alpha: f32) -> Self {
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
        let mut adjacent_edges: HashMap<Edge2, [Option<usize>; 2]> =
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
        // for (_, neighbours) in adjacent_edges.drain() {
        //     for (i, j) in [
        //         (neighbours[0], neighbours[1]),
        //         (neighbours[1], neighbours[0]),
        //     ] {
        //         if let Some(i) = i {
        //             if let Some(neighbour) = self.connections[i].iter_mut().find(|a| a.is_none()) {
        //                 *neighbour = j;
        //             }
        //         }
        //     }
        // }
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

pub fn alpha_shape_2d(
    points: impl IntoIterator<Item = Point2>,
    alpha: f32,
) -> Option<Vec<Shape2D>> {
    let triangulation = triangulate(points)?;
    let mut alpha_shape = AlphaShape2D::new(triangulation.triangles, alpha);
    let shapes = alpha_shape.shapes();
    Some(shapes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_square() {
        let points = vec![
            Point2::from([0.0, 0.0]),
            Point2::from([1.0, 0.0]),
            Point2::from([1.0, 1.0]),
            Point2::from([0.0, 1.0]),
        ];
        let shapes = alpha_shape_2d(points, 1.25);
        assert!(shapes.is_some());
        let shapes = shapes.unwrap();
        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].triangles.len(), 2);
    }

    #[test]
    fn test_two_disconnected_squares() {
        let points = vec![
            Point2::from([0.0, 0.0]),
            Point2::from([1.0, 0.0]),
            Point2::from([1.0, 1.0]),
            Point2::from([0.0, 1.0]),
            Point2::from([3.0, 0.0]),
            Point2::from([4.0, 0.0]),
            Point2::from([4.0, 1.0]),
            Point2::from([3.0, 1.0]),
        ];
        let shapes = alpha_shape_2d(points, 1.25);
        assert!(shapes.is_some());
        let shapes = shapes.unwrap();
        assert_eq!(shapes.len(), 2);
        assert_eq!(shapes[0].triangles.len(), 2);
        assert_eq!(shapes[1].triangles.len(), 2);
    }
}
