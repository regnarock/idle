use yew::prelude::*;
use crate::components::{DevPanel, GameView, State, UpgradeConfig};
use crate::hooks::{use_game_state, GameStateHandle};
use crate::predefined_states::load_predefined_states;

#[function_component(App)]
pub fn app() -> Html {
    let GameStateHandle { state, on_action } = use_game_state();
    let predefined_states = use_state(|| load_predefined_states());

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
            <div class="main-content">
                <div class="tabs">
                    <div class="tab">
                        <h2>{ "Game View" }</h2>
                        <GameView state={state.clone()} on_action={on_action.clone()} />
                        <div class="sub-tab">
                        <UpgradeConfig />
                    </div>
                    </div>
                    <div class="tab">
                        <h2>{ "Dev Panel" }</h2>
                        <DevPanel game_state={state.clone()} on_parameter_change={on_action.clone()} />
                    </div>
                    <div class="tab">
                        <h2>{ "State Management" }</h2>
                        <State state={state.clone()} on_select_predefined_state={on_select_predefined_state.clone()} />
                    </div>
                </div>
            </div>
        </div>
    }
}