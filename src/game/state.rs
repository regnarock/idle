use crate::upgrades::load_upgrades_config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GameState {
    pub counter: i32,
    pub clicks_per_second: i32,
    pub last_saved: f64,
    pub upgrades: Upgrades,
    pub x2_upgrade_cost: i32,

    // Developer panel parameters
    pub base_multiplier: f64,
    pub cost_scaling: f64,
    pub auto_clicker_efficiency: f64,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Upgrades {
    pub auto_clicker: i32,
    pub click_multiplier: i32,
}

impl GameState {
    pub fn new() -> Self {
        let upgrades_config = load_upgrades_config();
        Self {
            counter: 0,
            clicks_per_second: 0,
            last_saved: js_sys::Date::now(),
            upgrades: Upgrades::default(),
            base_multiplier: upgrades_config.click_multiplier.multiplier.unwrap_or(1.0),
            cost_scaling: upgrades_config.click_multiplier.cost_scaling,
            auto_clicker_efficiency: upgrades_config.auto_clicker.efficiency.unwrap_or(1.0),
            x2_upgrade_cost: upgrades_config.click_multiplier.base_cost,
        }
    }

    pub fn increment_counter(&mut self) {
        self.counter += self.calculate_click_value();
    }

    pub fn reset(&mut self) {
        *self = Self::new();
        self.last_saved = js_sys::Date::now();
        self.counter = 0;
        self.clicks_per_second = 0;
        self.upgrades = Upgrades::default();
    }

    pub fn calculate_clicks_per_second(&self) -> f64 {
        if self.upgrades.auto_clicker > 0 {
            let base_cps = self.upgrades.auto_clicker as f64;
            let multiplier = (1.0 + self.upgrades.click_multiplier as f64) * self.base_multiplier;
            base_cps * multiplier * self.auto_clicker_efficiency
        } else {
            0.0
        }
    }

    pub fn calculate_click_value(&self) -> i32 {
        let multiplier = (1.0 + self.upgrades.click_multiplier as f64) * self.base_multiplier;
        multiplier.round() as i32
    }

    pub fn calculate_upgrade_cost(&self, current_level: i32) -> i32 {
        let upgrades_config = load_upgrades_config();
        let base_cost = upgrades_config.click_multiplier.base_cost;
        (base_cost as f64 * self.cost_scaling.powi(current_level)).round() as i32
    }

    pub fn time_to_next_upgrade(&self) -> f64 {
        let next_upgrade_cost = self.calculate_upgrade_cost(self.upgrades.click_multiplier);
        let current_production = self.calculate_clicks_per_second();
        if current_production <= 0.0 {
            f64::INFINITY
        } else {
            (next_upgrade_cost as f64 - self.counter as f64) / current_production
        }
    }

    pub fn get_upgrade_cost(&self, upgrade_name: &str) -> i32 {
        let upgrades_config = load_upgrades_config();
        match upgrade_name {
            "auto_clicker" => (upgrades_config.auto_clicker.base_cost as f64
                * upgrades_config
                    .auto_clicker
                    .cost_scaling
                    .powi(self.upgrades.auto_clicker))
            .round() as i32,
            "click_multiplier" => (upgrades_config.click_multiplier.base_cost as f64
                * upgrades_config
                    .click_multiplier
                    .cost_scaling
                    .powi(self.upgrades.click_multiplier))
            .round() as i32,
            _ => 0,
        }
    }

    pub fn apply_upgrade(&mut self, upgrade_name: &str) {
        match upgrade_name {
            "auto_clicker" => {
                self.upgrades.auto_clicker += 1;
                // Don't need to set clicks_per_second directly anymore
            }
            "click_multiplier" => {
                self.upgrades.click_multiplier += 1;
            }
            _ => {}
        }
    }

    pub fn time_to_reach_resources(&self, target: f64) -> f64 {
        let cps = self.calculate_clicks_per_second();
        if cps <= 0.0 {
            f64::INFINITY
        } else {
            (target - self.counter as f64) / cps
        }
    }

    pub fn resources_at_time(&self, seconds: f64) -> f64 {
        let cps = self.calculate_clicks_per_second();
        self.counter as f64 + (cps * seconds)
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum GameParameter {
    BaseMultiplier(f64),
    CostScaling(f64),
    AutoClickerEfficiency(f64),
}
