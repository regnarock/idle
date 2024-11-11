use yew::prelude::*;
use crate::components::{DevPanel, GameView, State, UpgradeConfig, LogPanel};
use crate::game::GameAction;
use crate::hooks::{use_game_state, use_auto_save, use_auto_increment, GameStateHandle};
use crate::predefined_states::load_predefined_states;
use crate::upgrades::{load_upgrades_config, UpgradesConfig};

#[function_component(App)]
pub fn app() -> Html {
    let GameStateHandle { state, on_action } = use_game_state();
    use_auto_save(state.clone());
    use_auto_increment(state.clone());
    let predefined_states = use_state(|| load_predefined_states());
    let logs = use_state(|| vec![]);

    let upgrades_config = use_state(|| load_upgrades_config());

    let on_select_predefined_state = {
        let state = state.clone();
        let predefined_states = predefined_states.clone();
        Callback::from(move |index: usize| {
            if let Some(predefined_state) = predefined_states.get(index) {
                state.set(predefined_state.clone());
            }
        })
    };

    let on_update_upgrades_config = {
        let upgrades_config = upgrades_config.clone();
        Callback::from(move |new_config: UpgradesConfig| {
            upgrades_config.set(new_config);
        })
    };

    let on_action_with_log = {
        let on_action = on_action.clone();
        let logs = logs.clone();
        Callback::from(move |action: GameAction| {
            let log_message = match &action {
                GameAction::Click => "Clicked".to_string(),
                GameAction::Reset => "Game reset".to_string(),
                GameAction::UpdateGameParameter(param) => format!("Updated parameter: {:?}", param),
                GameAction::BuyUpgrade(upgrade) => format!("Bought upgrade: {}", upgrade),
            };
            logs.set({
                let mut new_logs = (*logs).clone();
                new_logs.push(log_message);
                new_logs
            });
            on_action.emit(action);
        })
    };

    html! {
        <div class="app-container">
            <div class="state-management-bar">
                <State state={state.clone()} on_select_predefined_state={on_select_predefined_state.clone()} />
            </div>
            <div class="main-content">
                <div class="game-view">
                    <GameView state={state.clone()} on_action={on_action_with_log.clone()} />
                    <LogPanel logs={logs.clone()} />
                </div>
                <div class="dev-panel">
                    <DevPanel game_state={state.clone()} on_parameter_change={on_action_with_log.clone()} upgrades_config={upgrades_config.clone()} on_update_upgrades_config={on_update_upgrades_config.clone()} />
                    <div class="sub-tab">
                        <UpgradeConfig game_state={state.clone()} upgrades_config={upgrades_config.clone()} />
                    </div>
                </div>
            </div>
        </div>
    }
}