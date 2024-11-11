use yew::prelude::*;
use crate::components::{DevPanel, GameView};
use crate::hooks::{use_game_state, GameStateHandle};
use crate::predifined_states::load_predefined_states;

#[function_component(App)]
pub fn app() -> Html {
    let GameStateHandle { state, on_action } = use_game_state();
    let show_dev_panel = use_state(|| false);
    let predefined_states = use_state(|| load_predefined_states());

    let toggle_view = {
        let show_dev_panel = show_dev_panel.clone();
        Callback::from(move |_| {
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

    html! {
        <div>
            <button onclick={toggle_view}>
                { if *show_dev_panel { "Switch to Game View" } else { "Switch to Dev Panel" } }
            </button>
            <div>
                <h3>{"Select Predefined State"}</h3>
                <ul>
                    { for predefined_states.iter().enumerate().map(|(index, _)| {
                        let on_select = on_select_predefined_state.clone();
                        html! {
                            <li>
                                <button onclick={Callback::from(move |_| on_select.emit(index))}>
                                    { format!("State {}", index + 1) }
                                </button>
                            </li>
                        }
                    }) }
                </ul>
            </div>
            {
                if *show_dev_panel {
                    html! { <DevPanel game_state={state.clone()} on_parameter_change={on_action.clone()} /> }
                } else {
                    html! { <GameView state={state.clone()} on_action={on_action.clone()} /> }
                }
            }
        </div>
    }
}