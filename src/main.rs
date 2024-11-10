use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);
    let multiplier = use_state(|| 1);
    let upgrade_purchased = use_state(|| false);

    let onclick = {
        let counter = counter.clone();
        let multiplier = multiplier.clone();
        Callback::from(move |_| {
            counter.set(*counter + *multiplier);
        })
    };

    let on_upgrade_click = {
        let counter = counter.clone();
        let multiplier = multiplier.clone();
        let upgrade_purchased = upgrade_purchased.clone();
        Callback::from(move |_| {
            if *counter >= 100 && !*upgrade_purchased {
                counter.set(*counter - 100);
                multiplier.set(2);
                upgrade_purchased.set(true);
            }
        })
    };

    html! {
        <div class="container">
            <div class="left-panel">
                <div class={if *upgrade_purchased { "upgrade-button active" } else { "upgrade-button" }}
                    onclick={on_upgrade_click}>
                    <div class="upgrade-icon">{"💪"}</div>
                    <span>{ "Upgrade x2 (Cost: 100 points)" }</span>
                </div>
            </div>
            <div class="game-panel">
                <h1>{ "Idle Clicker Game" }</h1>
                <div>
                    <p>{ "Current count: " }{ *counter }</p>
                    <button class="click-button" {onclick}>{ "Click me!" }</button>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}