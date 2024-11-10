use crate::game::{GameAction, GameState};
use crate::storage::GameStorage;
use log::error;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GameViewProps {
    pub state: GameState,
    pub on_action: Callback<GameAction>,
}

#[function_component(GameView)]
pub fn game_view(props: &GameViewProps) -> Html {
    let (x2_upgrade_cost, auto_click_cost) = props.state.get_upgrade_costs();

    let on_click = {
        let on_action = props.on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::Click))
    };

    let on_reset = {
        let on_action = props.on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::Reset))
    };

    let on_save_to_file = {
        let state = props.state.clone();
        Callback::from(move |_| {
            if let Err(e) = GameStorage::save_to_file(&state) {
                error!("Failed to save game state to file: {}", e);
            }
        })
    };

    let on_load_from_file = {
        let on_action = props.on_action.clone();
        Callback::from(move |_| {
            match GameStorage::load_from_file() {
                Ok(loaded_state) => {
                    on_action.emit(GameAction::Reset);
                    // Add logic to apply loaded state
                }
                Err(e) => error!("Failed to load game state from file: {}", e),
            }
        })
    };

    let on_buy_auto_clicker = {
        let on_action = props.on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::BuyAutoClicker))
    };

    let on_buy_multiplier = {
        let on_action = props.on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::BuyClickMultiplier))
    };

    let toggle_easy_mode = {
        let on_action = props.on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::ToggleEasyMode))
    };

    html! {
        <div>
            <div class="upgrades">
                <div
                    class={classes!("upgrade-square", (props.state.upgrades.click_multiplier > 0).then_some("active"))}
                    onclick={on_buy_multiplier}
                >
                    <div class="upgrade-icon">{ "⚡" }</div>
                    <span class="upgrade-text">{ format!("Upgrade x2 (Cost: {})", x2_upgrade_cost) }</span>
                </div>
                <div
                    class={classes!("upgrade-square", (props.state.upgrades.auto_clicker > 0).then_some("active"))}
                    onclick={on_buy_auto_clicker}
                >
                    <div class="upgrade-icon">{ "⏳" }</div>
                    <span class="upgrade-text">{ format!("Auto-Click (Cost: {})", auto_click_cost) }</span>
                </div>
            </div>
            <div class="game-panel">
                <h1>{ "Idle Clicker Game" }</h1>
                <p>{ "Current count: " }{ props.state.counter }</p>
                <p>{ "Clicks per second: " }{ props.state.clicks_per_second }</p>
                <p>{ "Multiplier: " }{ (1 + props.state.upgrades.click_multiplier) }</p>
                <button onclick={on_click}>{ "Click me!" }</button>
                <button onclick={on_reset}>{ "Reset" }</button>
                <button onclick={on_save_to_file}>{ "Save to File" }</button>
                <button onclick={on_load_from_file}>{ "Load from File" }</button>
                <button onclick={toggle_easy_mode}>
                    { if props.state.easy_mode { "Disable Easy Mode" } else { "Enable Easy Mode" } }
                </button>
            </div>
        </div>
    }
}
