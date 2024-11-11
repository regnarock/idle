use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LogPanelProps {
    pub logs: UseStateHandle<Vec<String>>,
}

#[function_component(LogPanel)]
pub fn log_panel(props: &LogPanelProps) -> Html {
    html! {
        <div class="log-panel">
            <h2>{ "Game Logs" }</h2>
            <ul>
                { for props.logs.iter().map(|log| html! { <li>{ log }</li> }) }
            </ul>
        </div>
    }
}