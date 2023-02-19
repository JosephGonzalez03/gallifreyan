use language::{letters::*, math_util::*};
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("0.png", (640, 640)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-10f32..10f32, -10f32..10f32)?;

    chart.configure_mesh().draw()?;
    let b = GallifreyanLetter::C(Polar::new(6.0, Degree(0.0)));
    chart.draw_series(LineSeries::new(b.letter().base().to_points(), GREEN))?;
    b.letter().to_points().iter().for_each(|part| {
        //chart.draw_series(LineSeries::new(part.to_vec(), GREEN));
        chart
            .draw_series(LineSeries::new(part.to_vec(), RED.filled()).point_size(2))
            .unwrap();
    });
    Ok(())
}