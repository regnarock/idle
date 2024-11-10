use crate::game::{GameAction, GameState};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GameViewProps {
    pub state: GameState,
    pub on_action: Callback<GameAction>,
}

#[function_component(GameView)]
pub fn game_view(props: &GameViewProps) -> Html {
    let GameViewProps { state, on_action } = props;

    let on_click = {
        let on_action = on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::Click))
    };

    let on_reset = {
        let on_action = on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::Reset))
    };

    let on_buy_auto_clicker = {
        let on_action = on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::BuyAutoClicker))
    };

    let on_buy_multiplier = {
        let on_action = on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::BuyClickMultiplier))
    };
    let (x2_upgrade_cost, auto_click_cost) = state.get_upgrade_costs();

    let toggle_easy_mode = {
        let on_action = on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::ToggleEasyMode))
    };

    html! {
        <div class="container">
            <div class="left-panel">
                <button onclick={toggle_easy_mode} class="easy-mode-button">
                    { if state.easy_mode { "Disable Easy Mode" } else { "Enable Easy Mode" } }
                </button>
                <div
                    class={classes!("upgrade-square", (state.upgrades.click_multiplier > 0).then_some("active"))}
                    onclick={on_buy_multiplier}
                >
                    <div class="upgrade-icon">{ "💪" }</div>
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
