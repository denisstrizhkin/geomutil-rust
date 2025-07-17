use anyhow::Result;
use geomutil_triangulation::triangulate;
use geomutil_util::Point2;
use plotters::prelude::*;

fn main() -> Result<()> {
    let points = [
        Point2::from([0.0, 0.0]),
        Point2::from([1.0, 0.0]),
        Point2::from([1.0, 1.0]),
        Point2::from([0.0, 1.0]),
        Point2::from([5.0, 0.0]),
        Point2::from([6.0, 0.0]),
        Point2::from([6.0, 1.0]),
        Point2::from([5.0, 1.0]),
    ];
    let triangulation = triangulate(points).unwrap();
    println!("triangles: {}", triangulation.triangles.len());

    let root = BitMapBackend::new("triangles.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let bbox = Point2::bounding_box([
        triangulation.bounding_triangle.a,
        triangulation.bounding_triangle.b,
        triangulation.bounding_triangle.c,
    ])
    .unwrap();
    println!("bbox: {bbox:?}");

    // 2. Define the chart context
    // We'll set up a coordinate system from (0,0) to (100, 100)
    let mut chart = ChartBuilder::on(&root).margin(10).build_cartesian_2d(
        bbox.lower().x..bbox.upper().x,
        bbox.lower().y..bbox.upper().y,
    )?; // Use f32 for the coordinate range
    chart.configure_mesh().draw()?;

    for triangle in triangulation
        .triangles
        .iter()
        .chain(&[triangulation.bounding_triangle])
    {
        let line_series = LineSeries::new(
            [triangle.a, triangle.b, triangle.c, triangle.a]
                .into_iter()
                .map(|p| (p.x, p.y)),
            BLACK.stroke_width(2), // Black outline, 2 pixels thick
        );

        chart.draw_series(line_series)?;
    }

    // Annotate the drawing area to show the output file path
    root.present()?;
    Ok(())
}
