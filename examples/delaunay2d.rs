use anyhow::Result;
use geomutil_triangulation::triangulate;
use geomutil_util::{Point2D, points_bounding_box};
use plotters::prelude::*;

fn main() -> Result<()> {
    let points = [
        Point2D::new(0.0, 0.0),
        Point2D::new(1.0, 0.0),
        Point2D::new(1.0, 1.0),
        Point2D::new(0.0, 1.0),
        Point2D::new(5.0, 0.0),
        Point2D::new(6.0, 0.0),
        Point2D::new(6.0, 1.0),
        Point2D::new(5.0, 1.0),
    ];
    let triangulation = triangulate(&points).unwrap();
    println!("triangles: {}", triangulation.triangles.len());

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
    let mut chart = ChartBuilder::on(&root).margin(10).build_cartesian_2d(
        low_boundary.x - 0.5..up_boundary.x + 0.5,
        low_boundary.y - 0.5..up_boundary.y + 0.5,
    )?; // Use f32 for the coordinate range
    chart.configure_mesh().draw()?;

    for triangle in triangulation
        .triangles
        .iter()
        .chain(&[triangulation.bounding_triangle])
    {
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
