use crate::game::{GameState, GameAction, GameParameter};
use crate::components::chart::draw_chart;
use yew::prelude::*;

pub enum DevPanelAction {
    UpdateBaseMultiplier(f64),
    UpdateCostScaling(f64),
    UpdateAutoClickerEfficiency(f64),
}

#[derive(Properties, PartialEq)]
pub struct DevPanelProps {
    pub game_state: UseStateHandle<GameState>,
    pub on_parameter_change: Callback<GameAction>,
}

#[derive(PartialEq, Clone)]
pub enum ScaleType {
    Linear,
    Logarithmic,
}

#[function_component(DevPanel)]
pub fn dev_panel(props: &DevPanelProps) -> Html {
    let canvas_ref = use_node_ref();
    let x_range = use_state(|| 200f32);
    let y_range = use_state(|| 10000f32);
    let scale_type = use_state(|| ScaleType::Logarithmic);

    // Draw progression chart effect
    {
        let canvas_ref = canvas_ref.clone();
        let state = *props.game_state.clone();
        let x_range = *x_range;
        let y_range = *y_range;
        use_effect(move || {
            draw_chart(canvas_ref, state, x_range, y_range);
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
            <button onclick={
                let scale_type = scale_type.clone();
                Callback::from(move |_| {
                    scale_type.set(match *scale_type {
                        ScaleType::Linear => ScaleType::Logarithmic,
                        ScaleType::Logarithmic => ScaleType::Linear,
                    });
                })
            }>
                {format!("Toggle {} Scale", if *scale_type == ScaleType::Linear { "Logarithmic" } else { "Linear" })}
            </button>
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
