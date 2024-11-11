use crate::components::DevPanelAction;
use crate::game::{GameParameter, GameState};
use crate::storage::GameStorage;
use yew::prelude::*;

#[derive(Clone)]
pub enum GameAction {
    Click,
    Reset,
    UpdateGameParameter(GameParameter),
    BuyUpgrade(String), // New action for buying upgrades
}

impl From<DevPanelAction> for GameAction {
    fn from(action: DevPanelAction) -> Self {
        match action {
            DevPanelAction::UpdateBaseMultiplier(value) => {
                GameAction::UpdateGameParameter(GameParameter::BaseMultiplier(value))
            }
            DevPanelAction::UpdateCostScaling(value) => {
                GameAction::UpdateGameParameter(GameParameter::CostScaling(value))
            }
            DevPanelAction::UpdateAutoClickerEfficiency(value) => {
                GameAction::UpdateGameParameter(GameParameter::AutoClickerEfficiency(value))
            }
        }
    }
}

pub fn use_game_action(state: UseStateHandle<GameState>) -> Callback<GameAction> {
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
            GameAction::BuyUpgrade(upgrade_name) => {
                new_state.apply_upgrade(&upgrade_name);
            }
        }
        state.set(new_state);
    })
}
