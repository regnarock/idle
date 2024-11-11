use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpgradeParameters {
    pub base_cost: i32,
    pub cost_scaling: f64,
    pub efficiency: Option<f64>,
    pub multiplier: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpgradesConfig {
    pub auto_clicker: UpgradeParameters,
    pub click_multiplier: UpgradeParameters,
}

pub fn load_upgrades_config() -> UpgradesConfig {
    let upgrades_json = include_str!("../upgrades/upgrades.json");
    from_str::<UpgradesConfig>(upgrades_json).expect("Failed to parse upgrades.json")
}