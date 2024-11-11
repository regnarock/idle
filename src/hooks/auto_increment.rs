use crate::game::GameState;
use gloo_timers::callback::Interval;
use yew::prelude::*;

const UPDATE_INTERVAL_MS: u32 = 50; // Update every 50ms for smoother increments

#[hook]
pub fn use_auto_increment(state: UseStateHandle<GameState>) {
    use_effect(move || {
        let state = state.clone();
        let interval = Interval::new(UPDATE_INTERVAL_MS, move || {
            let mut current_state = (*state).clone();
            if current_state.upgrades.auto_clicker > 0 {
                // Calculate fractional increment per tick
                let cps = current_state.calculate_clicks_per_second();
                let increment_per_tick = (cps * UPDATE_INTERVAL_MS as f64 / 1000.0) as i32;

                if increment_per_tick > 0 {
                    current_state.counter += increment_per_tick;
                    state.set(current_state);
                }
            }
        });
        || drop(interval)
    });
}
