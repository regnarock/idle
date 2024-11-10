use crate::components::{DevPanel, GameView};
use crate::game::GameAction;
use crate::hook::{use_auto_increment, use_auto_save, use_game_state};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let game = use_game_state();
    let show_dev_panel = use_state(|| false);

    // Set up auto-increment and auto-save
    use_auto_increment(game.state.clone());
    use_auto_save(game.state.clone());

    let toggle_dev_panel = {
        let show_dev_panel = show_dev_panel.clone();
        Callback::from(move |_| {
            show_dev_panel.set(!*show_dev_panel);
        })
    };

    html! {
        <>
            <GameView
                state={(*game.state).clone()}
                on_action={game.on_action.clone()}
            />
            <button
                class="dev-panel-toggle"
                onclick={toggle_dev_panel}
            >
                {"🛠️"}
            </button>
            if *show_dev_panel {
                <DevPanel
                    game_state={(*game.state).clone()}
                    on_parameter_change={game.on_action.clone()}
                />
            }
        </>
    }
}
