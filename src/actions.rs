use yew::prelude::*;
use crate::game::{GameState, GameAction, GameParameter};
use crate::storage::GameStorage;

pub fn use_game_action(state: UseStateHandle<GameState>) -> Callback<GameAction> {
    Callback::from(move |action: GameAction| {
        state.set({
            let mut new_state = (*state).clone();
            match action {
                GameAction::Click => {
                    new_state.increment_counter();
                }
                GameAction::ToggleEasyMode => {
                    new_state.easy_mode = !new_state.easy_mode;
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
                GameAction::UpdateGameParameter(param) => {
                    match param {
                        GameParameter::BaseMultiplier(value) => {
                            new_state.base_multiplier = value;
                        }
                        GameParameter::CostScaling(value) => {
                            new_state.cost_scaling = value;
                        }
                        GameParameter::AutoClickerEfficiency(value) => {
                            new_state.auto_clicker_efficiency = value;
                        }
                    }
                }
            }
            new_state
        });
    })
}