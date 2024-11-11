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
            .caption("Income vs. Cost", ("sans-serif", 20))
            .margin(5)
            .x_label_area_size(50)
            .y_label_area_size(60)
            .build_cartesian_2d(0f32..x_range, (0f32..y_range).log_scale())
            .unwrap();
        chart
            .configure_mesh()
            .x_desc("Count")
            .y_desc("Resources")
            .axis_desc_style(("sans-serif", 15))
            .label_style(("sans-serif", 12))
            .draw()
            .unwrap();
        // Draw cost curve
        chart
            .draw_series(LineSeries::new(
                (0..(x_range as i32)).map(|x| {
                    let x = x as f32;
                    let cost = (state.x2_upgrade_cost as f32 * state.cost_scaling as f32).powf(x);
                    (x, cost.max(1.0))
                }),
                &BLUE,
            ))
            .unwrap()
            .label("Cost")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
        // Draw income curve
        chart
            .draw_series(LineSeries::new(
                (0..(x_range as i32)).map(|x| {
                    let x = x as f32;
                    let base_income = state.calculate_clicks_per_second() as f32;
                    (x, (base_income * x).max(1.0)) // Ensure we don't go below 1 for log scale
                }),
                &RED,
            ))
            .unwrap()
            .label("Income")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()
            .unwrap();
    }
}
