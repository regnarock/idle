use yew::prelude::*;
use crate::upgrades::UpgradesConfig;
use crate::game::GameState;

#[derive(Properties, PartialEq)]
pub struct UpgradeConfigProps {
    pub game_state: UseStateHandle<GameState>,
    pub upgrades_config: UseStateHandle<UpgradesConfig>,
}

#[function_component(UpgradeConfig)]
pub fn upgrade_config(props: &UpgradeConfigProps) -> Html {
    let upgrades_config = props.upgrades_config.clone();
    let game_state = props.game_state.clone();

    let auto_clicker_count = game_state.upgrades.auto_clicker;
    let click_multiplier_count = game_state.upgrades.click_multiplier;

    let auto_clicker_next_cost = game_state.get_upgrade_cost("auto_clicker");
    let click_multiplier_next_cost = game_state.get_upgrade_cost("click_multiplier");

    let auto_clicker_current_effect = auto_clicker_count;
    let auto_clicker_next_effect = auto_clicker_count + 1;

    let click_multiplier_current_effect = 1.0 + click_multiplier_count as f64 * upgrades_config.click_multiplier.multiplier.unwrap_or(1.0);
    let click_multiplier_next_effect = 1.0 + (click_multiplier_count + 1) as f64 * upgrades_config.click_multiplier.multiplier.unwrap_or(1.0);

    html! {
        <div class="upgrade-config">
            <h2>{ "Upgrade Configuration" }</h2>
            <div class="upgrade">
                <h3>{ "Auto Clicker" }</h3>
                <p>{ format!("Base Cost: {}", upgrades_config.auto_clicker.base_cost) }</p>
                <p>{ format!("Cost Scaling: {}", upgrades_config.auto_clicker.cost_scaling) }</p>
                <p>{ format!("Efficiency: {}", upgrades_config.auto_clicker.efficiency.unwrap_or(0.0)) }</p>
                <p>{ format!("Bought: {}", auto_clicker_count) }</p>
                <p>{ format!("Next Cost: {}", auto_clicker_next_cost) }</p>
                <p>{ format!("Current Effect: +{} clicks per second", auto_clicker_current_effect) }</p>
                <p>{ format!("Next Effect: +{} clicks per second", auto_clicker_next_effect) }</p>
            </div>
            <div class="upgrade">
                <h3>{ "Click Multiplier" }</h3>
                <p>{ format!("Base Cost: {}", upgrades_config.click_multiplier.base_cost) }</p>
                <p>{ format!("Cost Scaling: {}", upgrades_config.click_multiplier.cost_scaling) }</p>
                <p>{ format!("Multiplier: {}", upgrades_config.click_multiplier.multiplier.unwrap_or(0.0)) }</p>
                <p>{ format!("Bought: {}", click_multiplier_count) }</p>
                <p>{ format!("Next Cost: {}", click_multiplier_next_cost) }</p>
                <p>{ format!("Current Effect: x{:.2}", click_multiplier_current_effect) }</p>
                <p>{ format!("Next Effect: x{:.2}", click_multiplier_next_effect) }</p>
            </div>
        </div>
    }
}