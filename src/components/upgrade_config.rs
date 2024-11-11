use yew::prelude::*;
use crate::upgrades::load_upgrades_config;

#[function_component(UpgradeConfig)]
pub fn upgrade_config() -> Html {
    let upgrades_config = load_upgrades_config();

    html! {
        <div class="upgrade-config">
            <h2>{ "Upgrade Configuration" }</h2>
            <div class="upgrade">
                <h3>{ "Auto Clicker" }</h3>
                <p>{ format!("Base Cost: {}", upgrades_config.auto_clicker.base_cost) }</p>
                <p>{ format!("Cost Scaling: {}", upgrades_config.auto_clicker.cost_scaling) }</p>
                <p>{ format!("Efficiency: {}", upgrades_config.auto_clicker.efficiency.unwrap_or(0.0)) }</p>
            </div>
            <div class="upgrade">
                <h3>{ "Click Multiplier" }</h3>
                <p>{ format!("Base Cost: {}", upgrades_config.click_multiplier.base_cost) }</p>
                <p>{ format!("Cost Scaling: {}", upgrades_config.click_multiplier.cost_scaling) }</p>
                <p>{ format!("Multiplier: {}", upgrades_config.click_multiplier.multiplier.unwrap_or(0.0)) }</p>
            </div>
        </div>
    }
}