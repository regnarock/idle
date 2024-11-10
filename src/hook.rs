use crate::game::{GameAction, GameState};
use crate::storage::GameStorage;
use gloo_timers::callback::Interval;
use yew::prelude::*;

#[derive(Clone)]
pub struct GameStateHandle {
    pub state: UseStateHandle<GameState>,
    pub on_action: Callback<GameAction>,
}

#[hook]
pub fn use_game_state() -> GameStateHandle {
    use crate::game::GameParameter;

    let state = use_state(|| GameStorage::load());

    let on_action = {
        let state = state.clone();
        Callback::from(move |action: GameAction| {
            let mut new_state = (*state).clone();

            match action {
                GameAction::Click => {
                    new_state.increment_counter();
                }
                GameAction::Reset => {
                    new_state = GameState::new();
                    GameStorage::clear();
                }
                GameAction::BuyAutoClicker => {
                    if new_state.counter >= 10 {
                        new_state.counter -= 10;
                        new_state.upgrades.auto_clicker += 1;
                        new_state.clicks_per_second = new_state.upgrades.auto_clicker;
                    }
                }
                GameAction::BuyClickMultiplier => {
                    let (cost, _) = new_state.get_upgrade_costs();
                    if new_state.counter >= cost {
                        new_state.counter -= cost;
                        new_state.upgrades.click_multiplier +=
                            if new_state.easy_mode { 10 } else { 1 };
                    }
                }
                GameAction::ToggleEasyMode => {
                    new_state.easy_mode = !new_state.easy_mode;
                }
                GameAction::UpdateGameParameter(param) => match param {
                    GameParameter::BaseMultiplier(value) => {
                        new_state.base_multiplier = value;
                    }
                    GameParameter::CostScaling(value) => {
                        new_state.cost_scaling = value;
                    }
                    GameParameter::AutoClickerEfficiency(value) => {
                        new_state.auto_clicker_efficiency = value;
                    }
                },
                _ => return,
            }
            state.set(new_state);
        })
    };

    GameStateHandle { state, on_action }
}

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
