use crate::game::{GameAction, GameParameter, GameState};
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

#[derive(Clone)]
pub enum DevPanelAction {
    UpdateBaseMultiplier(f64),
    UpdateCostScaling(f64),
    UpdateAutoClickerEfficiency(f64),
}

#[derive(Properties, PartialEq)]
pub struct DevPanelProps {
    pub game_state: GameState,
    pub on_parameter_change: Callback<GameAction>,
}

#[function_component(DevPanel)]
pub fn dev_panel(props: &DevPanelProps) -> Html {
    let canvas_ref = use_node_ref();

    let x_range = use_state(|| 200f32);
    let y_range = use_state(|| 10000f32);

    // Draw progression chart effect
    {
        let canvas_ref = canvas_ref.clone();
        let state = props.game_state.clone();
        let x_range = *x_range;
        let y_range = *y_range;

        use_effect(move || {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
                let root = backend.into_drawing_area();
                root.fill(&WHITE).unwrap();

                let mut chart = ChartBuilder::on(&root)
                    .caption("Income vs. Cost", ("sans-serif", 20))
                    .margin(5)
                    .x_label_area_size(50)
                    .y_label_area_size(60)
                    .build_cartesian_2d(0f32..x_range, 0f32..y_range)
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
                            (x, (10.0 * (1.15f32.powf(x))) as f32)
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
                            (x, base_income * x)
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
            || ()
        });
    }

    let on_x_range_change = {
        let x_range = x_range.clone();
        Callback::from(move |e: Event| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f32>() {
                    x_range.set(value);
                }
            }
        })
    };

    let on_y_range_change = {
        let y_range = y_range.clone();
        Callback::from(move |e: Event| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f32>() {
                    y_range.set(value);
                }
            }
        })
    };
    let on_base_multiplier_change = {
        let on_parameter_change = props.on_parameter_change.clone();
        Callback::from(move |e: Event| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    on_parameter_change.emit(GameAction::UpdateGameParameter(
                        GameParameter::BaseMultiplier(value),
                    ));
                }
            }
        })
    };

    let on_cost_scaling_change = {
        let on_parameter_change = props.on_parameter_change.clone();
        Callback::from(move |e: Event| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    on_parameter_change.emit(GameAction::UpdateGameParameter(
                        GameParameter::CostScaling(value),
                    ));
                }
            }
        })
    };

    let on_auto_clicker_efficiency_change = {
        let on_parameter_change = props.on_parameter_change.clone();
        Callback::from(move |e: Event| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<f64>() {
                    on_parameter_change.emit(GameAction::UpdateGameParameter(
                        GameParameter::AutoClickerEfficiency(value),
                    ));
                }
            }
        })
    };

    html! {
        <div class="dev-panel">
            <h2>{"Developer Panel"}</h2>

            // Add chart range controls
            <div class="chart-controls">
                <h3>{"Chart Controls"}</h3>
                <div class="parameter-group">
                    <label>{"X-Axis Range"}</label>
                    <input
                        type="range"
                        min="50"
                        max="500"
                        step="50"
                        value={x_range.to_string()}
                        onchange={on_x_range_change}
                    />
                    <span>{*x_range}</span>
                </div>
                <div class="parameter-group">
                    <label>{"Y-Axis Range"}</label>
                    <input
                        type="range"
                        min="1000"
                        max="100000"
                        step="1000"
                        value={y_range.to_string()}
                        onchange={on_y_range_change}
                    />
                    <span>{*y_range}</span>
                </div>
            </div>

            <div class="chart-container">
                <canvas ref={canvas_ref} width="800" height="500"/>
            </div>

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
                        onchange={on_base_multiplier_change}
                    />
                    <span>{props.game_state.base_multiplier}</span>
                </div>

                <div class="parameter-group">
                    <label>{"Cost Scaling"}</label>
                    <input
                        type="range"
                        min="1.0"
                        max="2.0"
                        step="0.05"
                        value={props.game_state.cost_scaling.to_string()}
                        onchange={on_cost_scaling_change}
                    />
                    <span>{props.game_state.cost_scaling}</span>
                </div>

                <div class="parameter-group">
                    <label>{"Auto Clicker Efficiency"}</label>
                    <input
                        type="range"
                        min="0.1"
                        max="2.0"
                        step="0.1"
                        value={props.game_state.auto_clicker_efficiency.to_string()}
                        onchange={on_auto_clicker_efficiency_change}
                    />
                    <span>{props.game_state.auto_clicker_efficiency}</span>
                </div>
            </div>

            <div class="formulas">
                <h3>{"Current Formulas"}</h3>
                <pre>
                    {"Cost = base_cost * (1.15 ^ level)\n"}
                    {"Production = base_prod * (multiplier ^ level)"}
                </pre>
            </div>

            <div class="statistics">
                <h3>{"Real-time Statistics"}</h3>
                <p>{format!("Current CPS: {:.2}", props.game_state.calculate_clicks_per_second())}</p>
                <p>{format!("Time to next upgrade: {:.2}s", props.game_state.time_to_next_upgrade())}</p>
            </div>
        </div>
    }
}
