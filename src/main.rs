use yew::prelude::*;
use gloo_timers::callback::Interval;

// Upgrade properties
#[derive(Properties, PartialEq)]
struct UpgradeProps {
    pub label: &'static str,
    pub icon: &'static str,
    pub cost: i32,
    pub on_purchase: Callback<()>,
    pub purchased: bool,
}

// Reusable component for an upgrade button
#[function_component(UpgradeButton)]
fn upgrade_button(props: &UpgradeProps) -> Html {
    let purchased = props.purchased;
    let on_purchase = props.on_purchase.clone();
    let label = props.label;
    let icon = props.icon;
    let cost = props.cost;

    let onclick = Callback::from(move |_| {
        if !purchased {
            on_purchase.emit(());
        }
    });

    html! {
        <div class={if purchased { "upgrade-square active" } else { "upgrade-square" }}
            onclick={onclick}>
            <div class="upgrade-icon">{ icon }</div>
            <span class="upgrade-text">{ format!("{} (Cost: {} points)", label, cost) }</span>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);
    let multiplier = use_state(|| 1);
    let x2_upgrade_purchased = use_state(|| false);
    let auto_click_purchased = use_state(|| false);
    let easy_mode = use_state(|| false); // Easy mode toggle state

    // Parameterized costs for each upgrade, modified based on easy mode
    let x2_upgrade_cost = if *easy_mode { 1 } else { 10 };
    let auto_click_cost = if *easy_mode { 10 } else { 200 };

    // Click button handler
    let onclick = {
        let counter = counter.clone();
        let multiplier = multiplier.clone();
        Callback::from(move |_| {
            counter.set(*counter + *multiplier);
        })
    };

    // x2 Multiplier upgrade handler
    let on_x2_upgrade_purchase = {
        let counter = counter.clone();
        let multiplier = multiplier.clone();
        let x2_upgrade_purchased = x2_upgrade_purchased.clone();
        let easy_mode = easy_mode.clone();
        Callback::from(move |_| {
            if *counter >= x2_upgrade_cost && !*x2_upgrade_purchased {
                counter.set(*counter - x2_upgrade_cost);
                multiplier.set(if *easy_mode { 10 } else { 2 }); // Higher multiplier in easy mode
                x2_upgrade_purchased.set(true);
            }
        })
    };

    // Auto-click upgrade handler
    let on_auto_click_upgrade = {
        let counter = counter.clone();
        let auto_click_purchased = auto_click_purchased.clone();
        Callback::from(move |_| {
            if *counter >= auto_click_cost && !*auto_click_purchased {
                counter.set(*counter - auto_click_cost);
                auto_click_purchased.set(true);
            }
        })
    };

    // Toggle Easy Mode
    let toggle_easy_mode = {
        let easy_mode = easy_mode.clone();
        Callback::from(move |_| {
            easy_mode.set(!*easy_mode);
        })
    };

    // Set up an interval task for the auto-click mechanism
    {
        let counter = counter.clone();
        let auto_click_purchased = auto_click_purchased.clone();
        use_effect_with(auto_click_purchased.clone(), move |auto_click_purchased| {
            if **auto_click_purchased {
                let handle = Interval::new(1000, move || {
                    counter.set(*counter + 1); // Increment counter every second
                });
                Box::new(move || drop(handle)) as Box<dyn FnOnce()>
            } else {
                Box::new(|| ()) as Box<dyn FnOnce()>
            }
        });
    }

    html! {
        <div class="container">
            <div class="left-panel">
                <button onclick={toggle_easy_mode} class="easy-mode-button">
                    { if *easy_mode { "Disable Easy Mode" } else { "Enable Easy Mode" } }
                </button>
                // x2 Multiplier Upgrade Button with parameterized cost
                <UpgradeButton
                    label="Upgrade x2"
                    icon="💪"
                    cost={x2_upgrade_cost}
                    on_purchase={on_x2_upgrade_purchase}
                    purchased={*x2_upgrade_purchased}
                />
                // Auto-Click Upgrade Button with parameterized cost
                <UpgradeButton
                    label="Auto-Click"
                    icon="⏳"
                    cost={auto_click_cost}
                    on_purchase={on_auto_click_upgrade}
                    purchased={*auto_click_purchased}
                />
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