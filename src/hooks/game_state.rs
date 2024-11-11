use crate::game::{GameAction, GameParameter, GameState};
use crate::storage::GameStorage;
use yew::prelude::*;

pub struct GameStateHandle {
    pub state: UseStateHandle<GameState>,
    pub on_action: Callback<GameAction>,
}

#[hook]
pub fn use_game_state() -> GameStateHandle {
    let state = use_state(|| GameStorage::load());
    let on_action = {
        let state = state.clone();
        Callback::from(move |action: GameAction| {
            let state = state.clone();
            match action {
                GameAction::Reset => {
                    GameStorage::clear(); // Clear storage first
                    state.set(GameState::new()); // Set completely new state
                }
                _ => {
                    // Handle all other actions by modifying existing state
                    state.set({
                        let mut new_state = (*state).clone();
                        match action {
                            GameAction::Click => {
                                new_state.increment_counter();
                            }
                            GameAction::BuyUpgrade(upgrade) => {
                                let cost = new_state.get_upgrade_cost(&upgrade);
                                if new_state.counter >= cost {
                                    new_state.counter -= cost;
                                    new_state.apply_upgrade(upgrade.as_str());
                                }
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
                            _ => {}
                        }
                        new_state
                    });
                }
            }
        })
    };

    GameStateHandle { state, on_action }
}
