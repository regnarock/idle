use crate::game::{GameAction, GameState};
use yew::prelude::*;

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

    let on_buy_upgrade = {
        let on_action = props.on_action.clone();
        Callback::from(move |upgrade_name: String| {
            on_action.emit(GameAction::BuyUpgrade(upgrade_name));
        })
    };

    let x2_upgrade_cost = state.get_upgrade_cost("click_multiplier");
    let auto_click_cost = state.get_upgrade_cost("auto_clicker");

    html! {
        <div>
            <div class="upgrades">
                <h2>{ "Upgrades" }</h2>
                <div class="upgrade-list">
                    <div
                        class={classes!("upgrade-square", (state.upgrades.click_multiplier > 0).then_some("active"))}
                        onclick={Callback::from({
                            let on_buy_upgrade = on_buy_upgrade.clone();
                            move |_| on_buy_upgrade.emit("click_multiplier".to_string())
                        })}
                        title={format!(
                            "Available in: {:.1}s",
                            state.time_to_reach_resources(state.get_upgrade_cost("click_multiplier") as f64)
                        )}
                    >
                        <div class="upgrade-icon">{ "⚡" }</div>
                        <span class="upgrade-text">{ format!("Upgrade x2 (Cost: {})", x2_upgrade_cost) }</span>
                    </div>
                    <div
                        class={classes!("upgrade-square", (state.upgrades.auto_clicker > 0).then_some("active"))}
                        onclick={Callback::from({
                            let on_buy_upgrade = on_buy_upgrade.clone();
                            move |_| on_buy_upgrade.emit("auto_clicker".to_string())
                        })}
                        title={format!(
                            "Available in: {:.1}s",
                            state.time_to_reach_resources(state.get_upgrade_cost("auto_clicker") as f64)
                        )}
                    >
                        <div class="upgrade-icon">{ "⏳" }</div>
                        <span class="upgrade-text">{ format!("Auto-Click (Cost: {})", auto_click_cost) }</span>
                    </div>
                </div>
            </div>
            <div class="game-panel">
                <h1>{ "Idle Clicker Game" }</h1>
                <p>{ "Current count: " }{ state.counter }</p>
                <p>{ "Clicks per second: " }{ state.calculate_clicks_per_second() }</p>
                <p>{ "Click value: " }{ state.calculate_click_value() }</p>
                <p>{ "Multiplier: " }{ (1 + state.upgrades.click_multiplier) }</p>
                <button onclick={on_click}>{ "Click me!" }</button>
                <button onclick={on_reset}>{ "Reset" }</button>
            </div>
        </div>
    }
}
