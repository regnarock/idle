use crate::game::GameState;
use gloo_timers::callback::Interval;
use yew::prelude::*;

const UPDATE_INTERVAL_MS: u32 = 50; // Update every 50ms for smoother increments

#[hook]
pub fn use_auto_increment(state: UseStateHandle<GameState>) {
    let fractional_clicks = use_state(|| 0.0);

    use_effect(move || {
        let interval = Interval::new(UPDATE_INTERVAL_MS, move || {
            let mut current_state = (*state).clone();
            if current_state.upgrades.auto_clicker > 0 {
                // Calculate fractional increment per tick
                let cps = current_state.calculate_clicks_per_second();
                let increment_per_tick = cps * UPDATE_INTERVAL_MS as f64 / 1000.0;

                // Add to accumulated fractional clicks
                let new_fraction = *fractional_clicks + increment_per_tick;
                let whole_clicks = new_fraction.floor();

                if whole_clicks >= 1.0 {
                    current_state.counter += whole_clicks as i32;
                    fractional_clicks.set(new_fraction - whole_clicks);
                    state.set(current_state);
                } else {
                    fractional_clicks.set(new_fraction);
                }
            }
        });
        || drop(interval)
    });
}
