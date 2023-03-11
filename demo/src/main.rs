use language::{glyphs::*, letters::*};
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("0.png", (640, 640)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-10f32..10f32, -10f32..10f32)?;

    chart.configure_mesh().draw()?;
    let gallifreyan_word = GallifreyanWord::from("TCHXD", 6.0);

    gallifreyan_word.to_gallifreyan_characters().iter().for_each(|gallifreyan_character| {
        chart
            .draw_series(LineSeries::new(
                gallifreyan_character.draw_base().to_vec(),
                BLUE,
            ))
            .unwrap();

        match gallifreyan_character.modifier {
            Some(Modifier::Line1 | Modifier::Line2 | Modifier::Line3) => {
                gallifreyan_character
                    .draw_modifier()
                    .expect("Already checked if modifier exists.")
                    .iter()
                    .for_each(|drawing| {
                        chart
                            .draw_series(LineSeries::new(drawing.to_vec(), BLUE))
                            .unwrap();
                    });
            }
            Some(Modifier::Dot1 | Modifier::Dot2 | Modifier::Dot3 | Modifier::Dot4) => {
                gallifreyan_character
                    .draw_modifier()
                    .expect("Already checked if modifier exists.")
                    .iter()
                    .for_each(|drawing| {
                        chart
                            .draw_series(
                                LineSeries::new(drawing.to_vec(), BLUE.filled()).point_size(2),
                            )
                            .unwrap();
                    });
            }
            None => (),
        }
    });

    gallifreyan_word
        .draw_edges()
        .into_iter()
        .for_each(|drawing| {
            chart
                .draw_series(LineSeries::new(drawing.to_vec(), BLUE))
                .unwrap();
        });
    Ok(())
}
