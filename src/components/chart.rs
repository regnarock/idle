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

        // Draw resource progression
        chart
            .draw_series(LineSeries::new(
                (0..(x_range as i32)).map(|x| {
                    let time = x as f64;
                    let mut projected_state = state.clone();
                    let mut current_resources = state.counter as f64;
                    let mut last_check_time = 0.0;

                    // Check for upgrades that would have happened before this time
                    while last_check_time < time {
                        let next_upgrade_cost =
                            projected_state.get_upgrade_cost("click_multiplier");
                        let time_to_upgrade = (next_upgrade_cost as f64 - current_resources)
                            / projected_state.calculate_clicks_per_second();

                        if time_to_upgrade > 0.0 && (last_check_time + time_to_upgrade) <= time {
                            // Update resources up to upgrade point
                            current_resources +=
                                projected_state.calculate_clicks_per_second() * time_to_upgrade;
                            // Apply upgrade
                            current_resources -= next_upgrade_cost as f64;
                            projected_state.upgrades.click_multiplier += 1;
                            last_check_time += time_to_upgrade;
                        } else {
                            // Add remaining time's resources
                            current_resources += projected_state.calculate_clicks_per_second()
                                * (time - last_check_time);
                            break;
                        }
                    }

                    (x as f32, current_resources as f32)
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
