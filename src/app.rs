use yew::prelude::*;
use crate::game::{GameState, GameAction};
use crate::storage::GameStorage;
use crate::components::GameView;
use gloo_timers::callback::Interval;

pub enum AppMsg {
    GameAction(GameAction),
    Tick,
}

pub struct App {
    state: GameState,
    _interval: Option<Interval>,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let state = GameStorage::load();
        
        // Set up auto-save and auto-increment interval
        let link = ctx.link().clone();
        let interval = Interval::new(1000, move || {
            link.send_message(AppMsg::Tick);
        });

        Self {
            state,
            _interval: Some(interval),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::GameAction(action) => {
                match action {
                    GameAction::Click => self.state.increment_counter(),
                    GameAction::Reset => {
                        self.state.reset();
                        GameStorage::clear();
                    },
                    GameAction::Save => {
                        let _ = GameStorage::save(&self.state);
                    },
                    GameAction::AutoIncrement => {
                        self.state.counter += self.state.clicks_per_second;
                    },
                }
                let _ = GameStorage::save(&self.state);
                true
            },
            AppMsg::Tick => {
                if self.state.clicks_per_second > 0 {
                    self.state.counter += self.state.clicks_per_second;
                    let _ = GameStorage::save(&self.state);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_action = ctx.link().callback(AppMsg::GameAction);
        
        html! {
            <GameView state={self.state.clone()} {on_action} />
        }
    }
}