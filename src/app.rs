use yew::prelude::*;
use crate::components::{DevPanel, GameView, State, UpgradeConfig};
use crate::hooks::{use_game_state, use_auto_save, use_auto_increment, GameStateHandle};
use crate::predefined_states::load_predefined_states;
use crate::upgrades::{load_upgrades_config, UpgradesConfig};

#[function_component(App)]
pub fn app() -> Html {
    let GameStateHandle { state, on_action } = use_game_state();
    use_auto_save(state.clone());
    use_auto_increment(state.clone());
    let predefined_states = use_state(|| load_predefined_states());

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

    html! {
        <div class="app-container">
            <div class="state-management-bar">
                <State state={state.clone()} on_select_predefined_state={on_select_predefined_state.clone()} />
            </div>
            <div class="main-content">
                <div class="game-view">
                    <GameView state={state.clone()} on_action={on_action.clone()} />
                </div>
                <div class="dev-panel">
                    <DevPanel game_state={state.clone()} on_parameter_change={on_action.clone()} upgrades_config={upgrades_config.clone()} on_update_upgrades_config={on_update_upgrades_config.clone()} />
                    <div class="sub-tab">
                        <UpgradeConfig game_state={state.clone()} upgrades_config={upgrades_config.clone()} />
                    </div>
                </div>
            </div>
        </div>
    }
}
