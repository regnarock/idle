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

        // Pre-calculate some values
        let cps = state.calculate_clicks_per_second();
        let current_resources = state.counter as f64;

        // Reduce number of points (only plot 100 points)
        let step_size = (x_range as i32 / 100).max(1);

        let mut chart = ChartBuilder::on(&root)
            .caption("Resource Projection", ("sans-serif", 20))
            .margin(5)
            .x_label_area_size(50)
            .y_label_area_size(60)
            .build_cartesian_2d((1f32..x_range).log_scale(), (0f32..y_range).log_scale())
            .unwrap();

        chart
            .configure_mesh()
            .x_desc("Time (seconds)")
            .y_desc("Resources")
            .axis_desc_style(("sans-serif", 15))
            .label_style(("sans-serif", 12))
            .draw()
            .unwrap();

        // Draw resource progression with fewer points
        chart
            .draw_series(LineSeries::new(
                (0..(x_range as i32)).step_by(step_size as usize).map(|x| {
                    let time = x as f64;
                    let resources = current_resources + (cps * time);
                    (x as f32, resources as f32)
                }),
                &BLUE,
            ))
            .unwrap();
        // Only draw upgrade lines if they're within the visible range
        let upgrades = vec![
            (
                "Click Multiplier",
                state.get_upgrade_cost("click_multiplier"),
            ),
            ("Auto Clicker", state.get_upgrade_cost("auto_clicker")),
        ];

        for (name, cost) in upgrades.iter() {
            let time_to_upgrade = state.time_to_reach_resources(*cost as f64);

            // Draw horizontal cost line
            chart
                .draw_series(LineSeries::new(
                    vec![(0f32, *cost as f32), (x_range, *cost as f32)],
                    &RED.mix(0.5),
                ))
                .unwrap()
                .label(format!("{} ({})", name, cost))
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED.mix(0.5)));

            // Add current resources line
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

            // Only draw time marker if we need time to reach the upgrade
            if time_to_upgrade > 0.0 && time_to_upgrade <= x_range as f64 {
                // Draw time marker for future upgrades
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
            } else if time_to_upgrade <= 0.0 {
                // Draw "Available Now" marker
                chart
                    .draw_series(PointSeries::of_element(
                        vec![(0f32, *cost as f32)],
                        4,
                        &GREEN,
                        &|coord, size, style| {
                            EmptyElement::at(coord)
                                + Circle::new((0, 0), size, style.filled())
                                + Text::new(
                                    format!("{}: Available Now!", name),
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
