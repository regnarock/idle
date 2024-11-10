use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GameState {
    pub counter: i32,
    pub clicks_per_second: i32,
    pub last_saved: f64,
    pub upgrades: Upgrades,
}

#[derive(Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Upgrades {
    pub auto_clicker: i32,
    pub click_multiplier: i32,
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn increment_counter(&mut self) {
        self.counter += 1 * (1 + self.upgrades.click_multiplier);
        self.last_saved = js_sys::Date::now();
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
