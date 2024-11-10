use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GameState {
    pub counter: i32,
    pub clicks_per_second: i32,
    pub last_saved: f64,
    pub upgrades: Upgrades,
    pub easy_mode: bool,

    // Developer panel parameters
    pub base_multiplier: f64,
    pub cost_scaling: f64,
    pub auto_clicker_efficiency: f64,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Upgrades {
    pub auto_clicker: i32,
    pub click_multiplier: i32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            counter: 0,
            clicks_per_second: 0,
            last_saved: js_sys::Date::now(),
            upgrades: Upgrades::default(),
            easy_mode: false,
            base_multiplier: 1.0,
            cost_scaling: 1.15, // Common idle game scaling factor
            auto_clicker_efficiency: 1.0,
        }
    }

    pub fn increment_counter(&mut self) {
        self.counter +=
            (1.0 * self.base_multiplier * (1.0 + self.upgrades.click_multiplier as f64)) as i32;
        self.last_saved = js_sys::Date::now();
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn get_upgrade_costs(&self) -> (i32, i32) {
        if self.easy_mode {
            (1, 10) // Easy mode costs
        } else {
            (10, 200) // Normal mode costs
        }
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
        let base_cost = if self.easy_mode { 10 } else { 50 };
        (base_cost as f64 * self.cost_scaling.powi(current_level)) as i32
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
}
