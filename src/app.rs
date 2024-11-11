use yew::prelude::*;
use crate::components::{DevPanel, GameView, State};
use crate::hooks::{{use_game_state, GameStateHandle}};
use crate::predefined_states::load_predefined_states;


#[function_component(App)]
pub fn app() -> Html {
    let GameStateHandle { state, on_action } = use_game_state();
    let show_dev_panel = use_state(|| false);
    let predefined_states = use_state(|| load_predefined_states());
    let active_tab = use_state(|| "GameView".to_string());

    let toggle_view = {
        let show_dev_panel = show_dev_panel.clone();
        Callback::from(move |_: MouseEvent| {
            show_dev_panel.set(!*show_dev_panel);
        })
    };

    let on_select_predefined_state = {
        let state = state.clone();
        let predefined_states = predefined_states.clone();
        Callback::from(move |index: usize| {
            if let Some(predefined_state) = predefined_states.get(index) {
                state.set(predefined_state.clone());
            }
        })
    };

    let switch_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: String| {
            active_tab.set(tab);
        })
    };

    html! {
        <div>
            <div class="tabs">
                <button onclick={switch_tab.reform(|_| "GameView".to_string())}>{ "Game View" }</button>
                <button onclick={switch_tab.reform(|_| "DevPanel".to_string())}>{ "Dev Panel" }</button>
                <button onclick={switch_tab.reform(|_| "State".to_string())}>{ "State" }</button>
            </div>
            {
                if *active_tab == "GameView" {
                    html! { <GameView state={state.clone()} on_action={on_action.clone()} /> }
                } else if *active_tab == "DevPanel" {
                    html! { <DevPanel game_state={state.clone()} on_parameter_change={on_action.clone()} /> }
                } else {
                    html! { <State state={state.clone()} on_select_predefined_state={on_select_predefined_state.clone()} /> }
                }
            }
        </div>
    }
}