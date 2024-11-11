use crate::storage::GameStorage;
use crate::game::{GameState, GameAction};
use log::error;
use yew::prelude::*;
use web_sys::{Blob, Url, FileReader, HtmlElement, HtmlInputElement, ProgressEvent, Event};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct GameViewProps {
    pub state: UseStateHandle<GameState>,
    pub on_action: Callback<GameAction>,
}

#[function_component(GameView)]
pub fn game_view(props: &GameViewProps) -> Html {
    let state = props.state.clone();

    let on_click = {
        let state = state.clone();
        Callback::from(move |_| {
            let mut new_state = (*state).clone();
            new_state.counter += 1;
            state.set(new_state);
        })
    };

    let on_reset = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(GameState::new());
        })
    };

    let on_save_to_file = {
        let state = state.clone();
        Callback::from(move |_| {
            if let Ok(json) = serde_json::to_string(&*state) {
                let blob = Blob::new_with_str_sequence(&js_sys::Array::of1(&json.into())).unwrap();
                let url = Url::create_object_url_with_blob(&blob).unwrap();
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let a = document.create_element("a").unwrap();
                a.set_attribute("href", &url).unwrap();
                a.set_attribute("download", "game_state.json").unwrap();
                a.set_attribute("style", "display: none;").unwrap();
                document.body().unwrap().append_child(&a).unwrap();
                let a: HtmlElement = a.dyn_into().unwrap();
                a.click();
                document.body().unwrap().remove_child(&a).unwrap();
                Url::revoke_object_url(&url).unwrap();
            } else {
                error!("Failed to save game state to file");
            }
        })
    };

    let on_load_from_file = {
        let state = state.clone();
        Callback::from(move |_| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let input = document.create_element("input").unwrap();
            input.set_attribute("type", "file").unwrap();
            input.set_attribute("style", "display: none;").unwrap();
            document.body().unwrap().append_child(&input).unwrap();
            let input: HtmlInputElement = input.dyn_into().unwrap();
            let state_clone = state.clone();
            let closure = Closure::wrap(Box::new(move |event: Event| {
                let input: HtmlInputElement = event.target().unwrap().dyn_into().unwrap();
                let files = input.files().unwrap();
                if files.length() > 0 {
                    let file = files.get(0).unwrap();
                    let reader = FileReader::new().unwrap();
                    let state_clone = state_clone.clone();
                    let onloadend = Closure::wrap(Box::new(move |event: ProgressEvent| {
                        let reader: FileReader = event.target().unwrap().dyn_into().unwrap();
                        if let Some(result) = reader.result().ok().and_then(|v| v.as_string()) {
                            if let Ok(loaded_state) = serde_json::from_str(&result) {
                                state_clone.set(loaded_state);
                            } else {
                                error!("Failed to parse imported game state");
                            }
                        }
                    }) as Box<dyn FnMut(_)>);
                    reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
                    reader.read_as_text(&file).unwrap();
                    onloadend.forget();
                }
            }) as Box<dyn FnMut(_)>);
            input.set_onchange(Some(closure.as_ref().unchecked_ref()));
            input.click();
            closure.forget();            document.body().unwrap().remove_child(&input).unwrap();
        })
    };

    let on_buy_x2_upgrade = {
        let state = state.clone();
        Callback::from(move |_| {
            let mut new_state = (*state).clone();
            if new_state.counter >= 100 {
                new_state.counter -= 100;
                new_state.upgrades.click_multiplier += 1;
                state.set(new_state);
            }
        })
    };

    let on_buy_auto_clicker = {
        let state = state.clone();
        Callback::from(move |_| {
            let mut new_state = (*state).clone();
            if new_state.counter >= 200 {
                new_state.counter -= 200;
                new_state.upgrades.auto_clicker += 1;
                state.set(new_state);
            }
        })
    };

    let x2_upgrade_cost = 100;
    let auto_click_cost = 200;

    html! {
        <div>
            <div class="upgrades">
                <div
                    class={classes!("upgrade-square", (state.upgrades.click_multiplier > 0).then_some("active"))}
                    onclick={on_buy_x2_upgrade}
                >
                    <div class="upgrade-icon">{ "⚡" }</div>
                    <span class="upgrade-text">{ format!("Upgrade x2 (Cost: {})", x2_upgrade_cost) }</span>
                </div>
                <div
                    class={classes!("upgrade-square", (state.upgrades.auto_clicker > 0).then_some("active"))}
                    onclick={on_buy_auto_clicker}
                >
                    <div class="upgrade-icon">{ "⏳" }</div>
                    <span class="upgrade-text">{ format!("Auto-Click (Cost: {})", auto_click_cost) }</span>
                </div>
            </div>
            <div class="game-panel">
                <h1>{ "Idle Clicker Game" }</h1>
                <p>{ "Current count: " }{ state.counter }</p>
                <p>{ "Clicks per second: " }{ state.clicks_per_second }</p>
                <p>{ "Multiplier: " }{ (1 + state.upgrades.click_multiplier) }</p>
                <button onclick={on_click}>{ "Click me!" }</button>
                <button onclick={on_reset}>{ "Reset" }</button>
                <button onclick={on_save_to_file}>{ "Save to File" }</button>
                <button onclick={on_load_from_file}>{ "Load from File" }</button>
            </div>
        </div>
    }
}