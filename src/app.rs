use yew::prelude::*;
use crate::components::{DevPanel, GameView};
use crate::hooks::{use_game_state, GameStateHandle};

#[function_component(App)]
pub fn app() -> Html {
    let GameStateHandle { state, on_action } = use_game_state();
    let show_dev_panel = use_state(|| false);

    let toggle_view = {
        let show_dev_panel = show_dev_panel.clone();
        Callback::from(move |_| {
            show_dev_panel.set(!*show_dev_panel);
        })
    };

    html! {
        <div>
            <button onclick={toggle_view}>
                { if *show_dev_panel { "Switch to Game View" } else { "Switch to Dev Panel" } }
            </button>
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