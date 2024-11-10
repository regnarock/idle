use gloo_utils::document;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

use crate::game::GameState;

#[derive(Properties, PartialEq)]
pub struct DevPanelProps {
    pub game_state: GameState,
    pub on_parameter_change: Callback<DevPanelAction>,
}

#[derive(Clone)]
pub enum DevPanelAction {
    UpdateBaseMultiplier(f64),
    UpdateCostScaling(f64),
    UpdateAutoClickerEfficiency(f64),
}

#[function_component(DevPanel)]
pub fn dev_panel(props: &DevPanelProps) -> Html {
    let canvas_ref = use_node_ref();

    // Draw progression chart
    use_effect_with(
        (canvas_ref.clone(), props.game_state.clone()),
        move |(canvas_ref, state)| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
                let root = backend.into_drawing_area();
                root.fill(&WHITE).unwrap();

                let mut chart = ChartBuilder::on(&root)
                    .caption("Resource Progression", ("sans-serif", 20))
                    .margin(5)
                    .x_label_area_size(30)
                    .y_label_area_size(30)
                    .build_cartesian_2d(0f32..100f32, 0f32..1000f32)
                    .unwrap();

                chart.configure_mesh().draw().unwrap();

                // Plot resource accumulation curve
                chart
                    .draw_series(LineSeries::new(
                        (0..100).map(|x| {
                            let x = x as f32;
                            (x, state.calculate_progress_at_time(x))
                        }),
                        &RED,
                    ))
                    .unwrap();
            }
            || ()
        },
    );

    html! {
        <div class="dev-panel">
            <h2>{"Developer Panel"}</h2>

            // Resource Progression Chart
            <div class="chart-container">
                <canvas ref={canvas_ref} width="600" height="400"/>
            </div>

            // Game Parameters
            <div class="parameters">
            <h3>{"Game Parameters"}</h3>
            <div class="parameter-group">
                <label>{"Base Multiplier"}</label>
                <input
                    type="range"
                    min="1"
                    max="10"
                    step="0.1"
                    value={props.game_state.base_multiplier.to_string()}
                    onchange={props.on_parameter_change.reform(|e: Event| {
                        if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                            if let Ok(value) = input.value().parse::<f64>() {
                                DevPanelAction::UpdateBaseMultiplier(value)
                            } else {
                                DevPanelAction::UpdateBaseMultiplier(1.0)
                            }
                        } else {
                            DevPanelAction::UpdateBaseMultiplier(1.0)
                        }
                    })}
                />
                <span>{props.game_state.base_multiplier}</span>
            </div>

            // Add more parameter controls here
            </div>

            // Formula Display
            <div class="formulas">
                <h3>{"Current Formulas"}</h3>
                <pre>
                    {"Cost = base_cost * (1.15 ^ level)\n"}
                    {"Production = base_prod * (multiplier ^ level)"}
                </pre>
            </div>

            // Statistics
            <div class="statistics">
                <h3>{"Real-time Statistics"}</h3>
                <p>{format!("Current CPS: {:.2}", props.game_state.calculate_clicks_per_second())}</p>
                <p>{format!("Time to next upgrade: {:.2}s", props.game_state.time_to_next_upgrade())}</p>
            </div>
        </div>
    }
}
