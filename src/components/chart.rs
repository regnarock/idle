use crate::game::GameState;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

pub fn draw_chart(canvas_ref: NodeRef, state: GameState, x_range: f32, y_range: f32) {
    if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
        let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
        let root = backend.into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption("Resource Projection", ("sans-serif", 20))
            .margin(5)
            .x_label_area_size(50)
            .y_label_area_size(60)
            .build_cartesian_2d(0f32..x_range, (0f32..y_range).log_scale())
            .unwrap();

        chart
            .configure_mesh()
            .x_desc("Time (seconds)")
            .y_desc("Resources")
            .axis_desc_style(("sans-serif", 15))
            .label_style(("sans-serif", 12))
            .draw()
            .unwrap();

        // Draw resource progression
        chart
            .draw_series(LineSeries::new(
                (0..(x_range as i32)).map(|x| {
                    let time = x as f64;
                    let resources = state.resources_at_time(time);
                    (x as f32, resources as f32)
                }),
                &BLUE,
            ))
            .unwrap()
            .label("Projected Resources")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        // Draw upgrade thresholds and time markers
        let upgrades = vec![
            (
                "Click Multiplier",
                state.get_upgrade_cost("click_multiplier"),
            ),
            ("Auto Clicker", state.get_upgrade_cost("auto_clicker")),
        ];

        for (i, (name, cost)) in upgrades.iter().enumerate() {
            let time_to_upgrade = state.time_to_reach_resources(*cost as f64);

            if time_to_upgrade <= x_range as f64 {
                // Draw horizontal cost line
                chart
                    .draw_series(LineSeries::new(
                        vec![(0f32, *cost as f32), (x_range, *cost as f32)],
                        &RED.mix(0.5),
                    ))
                    .unwrap()
                    .label(format!("{} ({})", name, cost))
                    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED.mix(0.5)));

                // Add a vertical line or marker showing current resources
                chart
                    .draw_series(LineSeries::new(
                        vec![
                            (0f32, state.counter as f32),
                            (x_range, state.counter as f32),
                        ],
                        &GREEN.mix(0.3),
                    ))
                    .unwrap()
                    .label("Current Resources")
                    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN.mix(0.3)));

                // Draw time marker
                chart
                    .draw_series(PointSeries::of_element(
                        vec![(time_to_upgrade as f32, *cost as f32)],
                        4,
                        &GREEN,
                        &|coord, size, style| {
                            EmptyElement::at(coord)
                                + Circle::new((0, 0), size, style.filled())
                                + Text::new(
                                    format!("{}: {:.1}s", name, time_to_upgrade),
                                    (15, -10),
                                    ("sans-serif", 16),
                                )
                        },
                    ))
                    .unwrap();
            }
        }

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()
            .unwrap();
    }
}
