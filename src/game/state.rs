use serde::{Deserialize, Serialize};
use crate::upgrades::load_upgrades_config;

#[derive(Copy, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GameState {
    pub counter: i32,
    pub clicks_per_second: i32,
    pub last_saved: f64,
    pub upgrades: Upgrades,
    pub easy_mode: bool,
    pub x2_upgrade_cost: i32,

    // Developer panel parameters
    pub base_multiplier: f64,
    pub cost_scaling: f64,
    pub auto_clicker_efficiency: f64,
}

#[derive(Copy, Clone, Serialize, Deserialize, Default, PartialEq)]
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
            easy_mode: false,
            base_multiplier: upgrades_config.click_multiplier.multiplier.unwrap_or(1.0),
            cost_scaling: upgrades_config.click_multiplier.cost_scaling,
            auto_clicker_efficiency: upgrades_config.auto_clicker.efficiency.unwrap_or(1.0),
            x2_upgrade_cost: upgrades_config.click_multiplier.base_cost,
        }
    }

    pub fn increment_counter(&mut self) {
        self.counter += 1;
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn get_upgrade_costs(&self) -> (i32, i32) {
        let upgrades_config = load_upgrades_config();
        let auto_clicker_cost = (upgrades_config.auto_clicker.base_cost as f64 * upgrades_config.auto_clicker.cost_scaling.powi(self.upgrades.auto_clicker)).round() as i32;
        let click_multiplier_cost = (upgrades_config.click_multiplier.base_cost as f64 * upgrades_config.click_multiplier.cost_scaling.powi(self.upgrades.click_multiplier)).round() as i32;
        (auto_clicker_cost, click_multiplier_cost)
    }

    pub fn calculate_progress_at_time(&self, time: f32) -> f32 {
        let base_production = self.clicks_per_second as f32;
        let multiplier =
            (self.base_multiplier * (1.0 + self.upgrades.click_multiplier as f64)) as f32;
        base_production * multiplier * time
    }

    pub fn calculate_clicks_per_second(&self) -> f64 {
        let base = self.clicks_per_second as f64;
        let multiplier = self.base_multiplier * (1.0 + self.upgrades.click_multiplier as f64);
        base * multiplier * self.auto_clicker_efficiency
    }

    pub fn calculate_upgrade_cost(&self, current_level: i32) -> i32 {
        let upgrades_config = load_upgrades_config();
        let base_cost = if self.easy_mode { 10 } else { upgrades_config.click_multiplier.base_cost };
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
            "auto_clicker" => {
                (upgrades_config.auto_clicker.base_cost as f64 * upgrades_config.auto_clicker.cost_scaling.powi(self.upgrades.auto_clicker)).round() as i32
            }
            "click_multiplier" => {
                (upgrades_config.click_multiplier.base_cost as f64 * upgrades_config.click_multiplier.cost_scaling.powi(self.upgrades.click_multiplier)).round() as i32
            }
            _ => 0,
        }
    }

    pub fn buy_upgrade(&mut self, upgrade_name: &str) {
        let upgrades_config = load_upgrades_config();
        match upgrade_name {
            "auto_clicker" => {
                let cost = self.get_upgrade_cost("auto_clicker");
                if self.counter >= cost {
                    self.counter -= cost;
                    self.upgrades.auto_clicker += 1;
                    self.clicks_per_second = self.upgrades.auto_clicker;
                }
            }
            "click_multiplier" => {
                let cost = self.get_upgrade_cost("click_multiplier");
                if self.counter >= cost {
                    self.counter -= cost;
                    self.upgrades.click_multiplier += if self.easy_mode { 10 } else { 1 };
                }
            }
            _ => {}
        }
    }

    pub fn apply_upgrade(&mut self, upgrade_name: &str) {
        match upgrade_name {
            "auto_clicker" => {
                self.upgrades.auto_clicker += 1;
                self.clicks_per_second = self.upgrades.auto_clicker;
            }
            "click_multiplier" => {
                self.upgrades.click_multiplier += if self.easy_mode { 10 } else { 1 };
            }
            _ => {}
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum GameParameter {
    BaseMultiplier(f64),
    CostScaling(f64),
    AutoClickerEfficiency(f64),
}
