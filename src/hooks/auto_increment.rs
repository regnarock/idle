use crate::game::GameState;
use gloo_timers::callback::Interval;
use yew::prelude::*;

#[hook]
pub fn use_auto_increment(state: UseStateHandle<GameState>) {
    use_effect(move || {
        let state = state.clone();
        let interval = Interval::new(1000, move || {
            let mut current_state = (*state).clone();
            if current_state.upgrades.auto_clicker > 0 {
                current_state.counter += current_state.upgrades.auto_clicker;
                state.set(current_state);
            }
        });
        || drop(interval)
    });
}