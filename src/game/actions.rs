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
