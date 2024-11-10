use crate::components::{DevPanel, GameView};
use crate::game::{GameAction, GameParameter, GameState};
use crate::storage::GameStorage;
use gloo_timers::callback::Interval;
use yew::prelude::*;

pub struct App {
    state: UseStateHandle<GameState>,
    show_dev_panel: bool,
}

impl Component for App {
    type Message = GameAction;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let state = use_state(|| GameStorage::load());
        let show_dev_panel = false;

        // Example interval to increment counter automatically
        {
            let state = state.clone();
            Interval::new(1000, move || {
                let mut new_state = (*state).clone();
                new_state.counter += new_state.upgrades.auto_clicker;
                state.set(new_state);
            })
            .forget();
        }

        Self { state, show_dev_panel }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GameAction::Click => {
                let mut new_state = (*self.state).clone();
                new_state.counter += 1;
                self.state.set(new_state);
                true
            }
            GameAction::Reset => {
                self.state.set(GameState::new());
                true
            }
            GameAction::ToggleEasyMode => {
                let mut new_state = (*self.state).clone();
                new_state.easy_mode = !new_state.easy_mode;
                self.state.set(new_state);
                true
            }
            GameAction::ToggleDevPanel => {
                self.show_dev_panel = !self.show_dev_panel;
                true
            }
            GameAction::UpdateGameParameter(param) => {
                let mut new_state = (*self.state).clone();
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
                self.state.set(new_state.clone());
                let _ = GameStorage::save(&new_state);
                true
            }
            GameAction::AutoIncrement => {
                let mut new_state = (*self.state).clone();
                new_state.counter += new_state.upgrades.auto_clicker;
                self.state.set(new_state);
                true
            }
            GameAction::Save => {
                let _ = GameStorage::save(&self.state);
                true
            }
            GameAction::BuyAutoClicker => {
                let mut new_state = (*self.state).clone();
                if new_state.counter >= 200 {
                    new_state.counter -= 200;
                    new_state.upgrades.auto_clicker += 1;
                    self.state.set(new_state);
                }
                true
            }
            GameAction::BuyClickMultiplier => {
                let mut new_state = (*self.state).clone();
                if new_state.counter >= 100 {
                    new_state.counter -= 100;
                    new_state.upgrades.click_multiplier += 1;
                    self.state.set(new_state);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_action = ctx.link().callback(|action| action);
        html! {
            <>
                <GameView state={self.state.clone()} on_action={ctx.link().callback(|action| action)} />
                <button
                    class="dev-panel-toggle"
                    onclick={ctx.link().callback(|_| GameAction::ToggleDevPanel)}
                >
                    {"🛠️"}
                </button>
                if self.show_dev_panel {
                    <DevPanel
                        game_state={(*self.state).clone()}
                        on_parameter_change={on_action.clone()}
                    />
                }
            </>
        }
    }
}
