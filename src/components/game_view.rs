use crate::game::{GameAction, GameState};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GameViewProps {
    pub state: GameState,
    pub on_action: Callback<GameAction>,
}

#[function_component(GameView)]
pub fn game_view(props: &GameViewProps) -> Html {
    let GameViewProps { state, on_action } = props;

    let on_click = {
        let on_action = on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::Click))
    };

    let on_reset = {
        let on_action = on_action.clone();
        Callback::from(move |_| on_action.emit(GameAction::Reset))
    };

    html! {
        <div>
            <h1>{ "Idle Clicker Game" }</h1>
            <div>
                <p>{ "Current count: " }{ state.counter }</p>
                <p>{ "Clicks per second: " }{ state.clicks_per_second }</p>
                <button onclick={on_click}>{ "Click me!" }</button>
                <button onclick={on_reset}>{ "Reset" }</button>
            </div>
        </div>
    }
}
