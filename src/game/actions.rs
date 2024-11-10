use crate::components::DevPanelAction;

#[derive(Clone, PartialEq)]
pub enum GameAction {
    Click,
    Reset,
    AutoIncrement,
    Save,
    BuyAutoClicker,
    BuyClickMultiplier,
    ToggleEasyMode,
    ToggleDevPanel,
    UpdateGameParameter(GameParameter),
}

#[derive(Clone, PartialEq)]
pub enum GameParameter {
    BaseMultiplier(f64),
    CostScaling(f64),
    AutoClickerEfficiency(f64),
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
