use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

const COUNTER_KEY: &str = "idle_counter";

#[function_component(App)]
fn app() -> Html {
    // Initialize counter from local storage or default to 0
    let counter = use_state(|| LocalStorage::get(COUNTER_KEY).unwrap_or(0));

    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| {
            let new_value = *counter + 1;
            // Save to local storage
            LocalStorage::set(COUNTER_KEY, new_value).unwrap();
            counter.set(new_value);
        })
    };

    // Optional: Add a reset button
    let on_reset = {
        let counter = counter.clone();
        Callback::from(move |_| {
            LocalStorage::delete(COUNTER_KEY);
            counter.set(0);
        })
    };

    html! {
        <div>
            <h1>{ "Idle Clicker Game" }</h1>
            <div>
                <p>{ "Current count: " }{ *counter }</p>
                <button {onclick}>{ "Click me!" }</button>
                <button onclick={on_reset}>{ "Reset" }</button>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
