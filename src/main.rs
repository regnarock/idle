use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);

    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(*counter + 1);
        })
    };

    html! {
        <div>
            <h1>{ "Idle Clicker Game" }</h1>
            <div>
                <p>{ "Current count: " }{ *counter }</p>
                <button {onclick}>{ "Click me!" }</button>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
