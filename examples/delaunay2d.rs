use anyhow::Result;
use plotters::prelude::*;
use triangulation::triangulate;
use util::{Point2D, points_bounding_box};

fn main() -> Result<()> {
    let points = [
        Point2D::new(0.0, 0.0),
        Point2D::new(1.0, 0.0),
        Point2D::new(1.0, 1.0),
        Point2D::new(0.0, 1.0),
    ];
    let triangulation = triangulate(&points).unwrap();

    let root = BitMapBackend::new("triangles.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let (low_boundary, up_boundary) = points_bounding_box(&[
        triangulation.bounding_triangle.a,
        triangulation.bounding_triangle.b,
        triangulation.bounding_triangle.c,
    ])
    .unwrap();

    // 2. Define the chart context
    // We'll set up a coordinate system from (0,0) to (100, 100)
    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .build_cartesian_2d(low_boundary.x..up_boundary.y, low_boundary.x..up_boundary.y)?; // Use f32 for the coordinate range
    chart.configure_mesh().draw()?;

    for (i, triangle) in triangulation
        .triangles
        .iter()
        .chain(&[triangulation.bounding_triangle])
        .enumerate()
    {
        // You can set different colors for each triangle if needed
        let color = if i % 2 == 0 { &BLUE } else { &RED }.mix(0.6); // Make it slightly transparent

        // Create a Polygon element from the triangle's points
        let polygon = Polygon::new(
            vec![triangle.a.xy(), triangle.b.xy(), triangle.c.xy()],
            color.filled(), // Fill the polygon
        );

        // Draw the polygon to the chart
        chart.draw_series(std::iter::once(polygon))?;

        // Optionally, draw the outline of the triangle
        let line_series = LineSeries::new(
            vec![
                triangle.a.xy(),
                triangle.b.xy(),
                triangle.c.xy(),
                triangle.a.xy(),
            ], // Close the loop
            BLACK.stroke_width(2), // Black outline, 2 pixels thick
        );

        chart.draw_series(line_series)?;
    }

    // Annotate the drawing area to show the output file path
    root.present()?;
    Ok(())
}
