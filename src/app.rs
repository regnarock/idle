use crate::components::GameView;
use crate::game::{GameAction, GameState};
use crate::storage::GameStorage;
use gloo_timers::callback::Interval;
use yew::prelude::*;

pub struct App {
    state: GameState,
    _interval: Option<Interval>,
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

        Self {
            state,
            _interval: Some(interval),
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
                    let _ = GameStorage::save(&self.state);
                    true
                } else {
                    false
                }
            }
            GameAction::BuyClickMultiplier => {
                if self.state.counter >= 50 {
                    self.state.counter -= 50;
                    self.state.upgrades.click_multiplier += 1;
                    let _ = GameStorage::save(&self.state);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_action = ctx.link().callback(|action| action);
        html! {
            <GameView state={self.state.clone()} {on_action} />
        }
    }
}
