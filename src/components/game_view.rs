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

    html! {
        <div class="container">
                    <div class="left-panel">
                        <button
                            class={classes!("upgrade-button", (state.upgrades.auto_clicker > 0).then_some("active"))}
                            onclick={on_buy_auto_clicker}
                        >
                            <span class="upgrade-icon">{"⚡"}</span>
                            {"Auto Clicker"}
                        </button>
                        <button
                            class={classes!("upgrade-button", (state.upgrades.click_multiplier > 0).then_some("active"))}
                            onclick={on_buy_multiplier}
                        >
                            <span class="upgrade-icon">{"×"}</span>
                            {"Click Multiplier"}
                        </button>
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
