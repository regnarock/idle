use crate::components::{DevPanel, GameView};
use crate::storage::GameStorage;
use crate::game::{GameState, GameAction};
use log::error;
use yew::prelude::*;
use web_sys::{Blob, Url, FileReader, HtmlElement, HtmlInputElement, ProgressEvent, Event};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| GameStorage::load());
    let show_dev_panel = use_state(|| false);

    let on_action = {
        let state = state.clone();
        Callback::from(move |action: GameAction| {
            let mut new_state = (*state).clone();
            match action {
                GameAction::Click => {
                    new_state.counter += 1;
                }
                GameAction::Reset => {
                    new_state = GameState::new();
                }
                GameAction::BuyAutoClicker => {
                    if new_state.counter >= 200 {
                        new_state.counter -= 200;
                        new_state.upgrades.auto_clicker += 1;
                    }
                }
                GameAction::BuyClickMultiplier => {
                    if new_state.counter >= 100 {
                        new_state.counter -= 100;
                        new_state.upgrades.click_multiplier += 1;
                    }
                }
                _ => {}
            }
            state.set(new_state);
        })
    };

    let toggle_dev_panel = {
        let show_dev_panel = show_dev_panel.clone();
        Callback::from(move |_| {
            show_dev_panel.set(!*show_dev_panel);
        })
    };

    html! {
        <div>
            <button onclick={toggle_dev_panel}>{ if *show_dev_panel { "Hide Dev Panel" } else { "Show Dev Panel" } }</button>
            if *show_dev_panel {
                <DevPanel game_state={(*state).clone()} on_parameter_change={on_action.clone()} />
            } else {
                <GameView state={state.clone()} on_action={on_action.clone()} />
            }
        </div>
    }
}