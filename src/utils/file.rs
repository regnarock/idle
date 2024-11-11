use web_sys::{Blob, Url, HtmlElement, HtmlInputElement, Event, FileReader};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use log::error;
use serde_json;

use crate::game::GameState;
use crate::upgrades::{UpgradeParameters, UpgradesConfig};

pub fn save_to_file<T: ?Sized + serde::Serialize>(data: &T, filename: &str) {
    if let Ok(json) = serde_json::to_string(data) {
        let blob = Blob::new_with_str_sequence(&js_sys::Array::of1(&json.into())).unwrap();
        let url = Url::create_object_url_with_blob(&blob).unwrap();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let a = document.create_element("a").unwrap();
        a.set_attribute("href", &url).unwrap();
        a.set_attribute("download", filename).unwrap();
        a.set_attribute("style", "display: none;").unwrap();
        document.body().unwrap().append_child(&a).unwrap();
        let a: HtmlElement = a.dyn_into().unwrap();
        a.click();
        document.body().unwrap().remove_child(&a).unwrap();
        Url::revoke_object_url(&url).unwrap();
    } else {
        error!("Failed to save data to file");
    }
}

pub fn load_from_file(state: UseStateHandle<GameState>) {
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
                if let Ok(result) = reader.result() {
                    if let Some(json) = result.as_string().as_deref() {
                        match serde_json::from_str::<GameState>(&json) {
                            Ok(loaded_state) => state_clone.set(loaded_state),
                            Err(e) => error!("Failed to parse game state from file: {}", e),
                        }
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
    closure.forget();
}

pub fn save_upgrades_config(game_state: UseStateHandle<GameState>) {
    let upgrades_config = UpgradesConfig {
        auto_clicker: UpgradeParameters {
            base_cost: game_state.upgrades.auto_clicker,
            cost_scaling: game_state.cost_scaling,
            efficiency: Some(game_state.auto_clicker_efficiency),
            multiplier: None,
        },
        click_multiplier: UpgradeParameters {
            base_cost: game_state.upgrades.click_multiplier,
            cost_scaling: game_state.cost_scaling,
            efficiency: None,
            multiplier: Some(game_state.base_multiplier),
        },
    };

    save_to_file(&upgrades_config, "upgrades.json");
}