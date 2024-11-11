use crate::storage::GameStorage;
use crate::game::{GameState, GameAction};
use log::error;
use yew::prelude::*;
use web_sys::{Blob, Url, FileReader, HtmlElement, HtmlInputElement, ProgressEvent, Event};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct GameViewProps {
    pub state: UseStateHandle<GameState>,
    pub on_action: Callback<GameAction>,
}

#[function_component(GameView)]
pub fn game_view(props: &GameViewProps) -> Html {
    let state = props.state.clone();

    let on_click = {
        let on_action = props.on_action.clone();
        Callback::from(move |_: MouseEvent| {
            on_action.emit(GameAction::Click);
        })
    };

    let on_reset = {
        let on_action = props.on_action.clone();
        Callback::from(move |_: MouseEvent| {
            on_action.emit(GameAction::Reset);
        })
    };

    let on_buy_x2_upgrade = {
        let on_action = props.on_action.clone();
        Callback::from(move |_| {
            on_action.emit(GameAction::BuyClickMultiplier);
        })
    };

    let on_buy_auto_clicker = {
        let on_action = props.on_action.clone();
        Callback::from(move |_| {
            on_action.emit(GameAction::BuyAutoClicker);
        })
    };

    let x2_upgrade_cost = 100;
    let auto_click_cost = 200;

    html! {
        <div>
            <div class="upgrades">
                <div
                    class={classes!("upgrade-square", (state.upgrades.click_multiplier > 0).then_some("active"))}
                    onclick={on_buy_x2_upgrade}
                >
                    <div class="upgrade-icon">{ "⚡" }</div>
                    <span class="upgrade-text">{ format!("Upgrade x2 (Cost: {})", x2_upgrade_cost) }</span>
                </div>
                <div
                    class={classes!("upgrade-square", (state.upgrades.auto_clicker > 0).then_some("active"))}
                    onclick={on_buy_auto_clicker}
                >
                    <div class="upgrade-icon">{ "⏳" }</div>
                    <span class="upgrade-text">{ format!("Auto-Click (Cost: {})", auto_click_cost) }</span>
                </div>
            </div>
            <div class="game-panel">
                <h1>{ "Idle Clicker Game" }</h1>
                <p>{ "Current count: " }{ state.counter }</p>
                <p>{ "Clicks per second: " }{ state.clicks_per_second }</p>
                <p>{ "Multiplier: " }{ (1 + state.upgrades.click_multiplier) }</p>
                <button onclick={on_click}>{ "Click me!" }</button>
                <button onclick={on_reset}>{ "Reset" }</button>
            </div>
        </div>
    }
}