use yew::prelude::*;
use crate::components::{DevPanel, GameView, State};
use crate::hooks::{{use_game_state, GameStateHandle}};
use crate::predefined_states::load_predefined_states;

#[function_component(App)]
pub fn app() -> Html {
    let GameStateHandle { state, on_action } = use_game_state();
    let predefined_states = use_state(|| load_predefined_states());
    let active_tab = use_state(|| "GameView".to_string());

    let switch_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: String| {
            active_tab.set(tab);
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

    html! {
        <div class="app-container">
            <div class="state-management-bar">
                <State state={state.clone()} on_select_predefined_state={on_select_predefined_state.clone()} />
            </div>
            <div class="main-content">
                <div class="navbar">
                    <button onclick={switch_tab.reform(|_| "GameView".to_string())}>{ "Game View" }</button>
                    <button onclick={switch_tab.reform(|_| "DevPanel".to_string())}>{ "Dev Panel" }</button>
                </div>
                <div class="content">
                    {
                        if *active_tab == "GameView" {
                            html! { <GameView state={state.clone()} on_action={on_action.clone()} /> }
                        } else {
                            html! { <DevPanel game_state={state.clone()} on_parameter_change={on_action.clone()} /> }
                        }
                    }
                </div>
            </div>
        </div>
    }
}