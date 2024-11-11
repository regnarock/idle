use crate::game::GameState;
use crate::storage::GameStorage;
use gloo_timers::callback::Interval;
use yew::prelude::*;

#[hook]
pub fn use_auto_save(state: UseStateHandle<GameState>) {
    use_effect(move || {
        let state = state.clone();
        let interval = Interval::new(5000, move || {
            let _ = GameStorage::save(&state);
            log::debug!("Auto-saving game state");
        });
        || drop(interval)
    });
}