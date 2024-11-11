use yew::prelude::*;
use crate::game::GameState;
use crate::storage::GameStorage;
use crate::predefined_states::load_predefined_states;
use log::error;
use web_sys::{Blob, Url, FileReader, HtmlElement, HtmlInputElement, ProgressEvent, Event};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct StateProps {
    pub state: UseStateHandle<GameState>,
    pub on_select_predefined_state: Callback<usize>,
}

#[function_component(State)]
pub fn state(props: &StateProps) -> Html {
    let predefined_states = use_state(|| load_predefined_states());

    let on_export_state = {
        let state = props.state.clone();
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
                error!("Failed to export game state");
            }
        })
    };

    let on_import_state = {
        let state = props.state.clone();
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
            closure.forget();
            document.body().unwrap().remove_child(&input).unwrap();
        })
    };

    html! {
        <div>
            <h3>{"State Management"}</h3>
            <button onclick={on_export_state}>{ "Export State" }</button>
            <button onclick={on_import_state}>{ "Import State" }</button>
            <div>
                <h3>{"Select Predefined State"}</h3>
                <ul>
                    { for predefined_states.iter().enumerate().map(|(index, _)| {
                        let on_select = props.on_select_predefined_state.clone();
                        html! {
                            <li>
                                <button onclick={Callback::from(move |_| on_select.emit(index))}>
                                    { format!("State {}", index + 1) }
                                </button>
                            </li>
                        }
                    }) }
                </ul>
            </div>
        </div>
    }
}