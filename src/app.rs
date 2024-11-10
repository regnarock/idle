use crate::components::{DevPanel, GameView};
use crate::game::{GameAction, GameParameter, GameState};
use crate::storage::GameStorage;
use gloo_timers::callback::Interval;
use yew::prelude::*;
use crate::components::GameView;

pub struct App {
    state: GameState,
<<<<<<< HEAD
    show_dev_panel: bool,

=======
>>>>>>> 264cb8f (feat:add autosave)
    _interval: Option<Interval>,
    _save_interval: Option<Interval>,
}

impl Component for App {
    type Message = GameAction;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let state = GameStorage::load();

        let link = ctx.link().clone();
        let interval = Interval::new(1000, move || {
            link.send_message(GameAction::AutoIncrement);
        });

        // Auto-save interval (every 5 seconds)
        let link = ctx.link().clone();
        let save_interval = Interval::new(5000, move || {
            link.send_message(GameAction::Save);
            log::debug!("Auto-saving game state");
        });

        Self {
            state,
            _interval: Some(interval),
            _save_interval: Some(save_interval),
<<<<<<< HEAD
            show_dev_panel: false,
=======
>>>>>>> 264cb8f (feat:add autosave)
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GameAction::Click => {
                self.state.increment_counter();
                true
            }
            GameAction::Reset => {
                self.state.reset();
                GameStorage::clear();
                true
            }
            GameAction::AutoIncrement => {
                if self.state.upgrades.auto_clicker > 0 {
                    self.state.counter += self.state.upgrades.auto_clicker;
                    true
                } else {
                    false
                }
            }
            GameAction::Save => {
                let _ = GameStorage::save(&self.state);
                false
            }
            GameAction::BuyAutoClicker => {
                if self.state.counter >= 10 {
                    self.state.counter -= 10;
                    self.state.upgrades.auto_clicker += 1;
                    self.state.clicks_per_second = self.state.upgrades.auto_clicker;
                    true
                } else {
                    false
                }
            }
            GameAction::BuyClickMultiplier => {
                let (cost, _) = self.state.get_upgrade_costs();
                if self.state.counter >= cost {
                    self.state.counter -= cost;
                    self.state.upgrades.click_multiplier +=
                        if self.state.easy_mode { 10 } else { 1 };
                    true
                } else {
                    false
                }
            }
            GameAction::ToggleEasyMode => {
                self.state.easy_mode = !self.state.easy_mode;
                true
            }
            GameAction::ToggleDevPanel => {
                self.show_dev_panel = !self.show_dev_panel;
                true
            }
            GameAction::UpdateGameParameter(param) => {
                match param {
                    GameParameter::BaseMultiplier(value) => {
                        self.state.base_multiplier = value;
                    }
                    GameParameter::CostScaling(value) => {
                        self.state.cost_scaling = value;
                    }
                    GameParameter::AutoClickerEfficiency(value) => {
                        self.state.auto_clicker_efficiency = value;
                    }
                }
                let _ = GameStorage::save(&self.state);
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
                        game_state={self.state.clone()}
                        on_parameter_change={on_action.clone()}
                    />
                }
            </>
        }
    }
}
