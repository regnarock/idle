use yew::prelude::*;
use crate::game::GameState;
use crate::storage::GameStorage;
use crate::predefined_states::load_predefined_states;

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
            GameStorage::save_to_file(&*state);
        })
    };

    let on_import_state = {
        let state = props.state.clone();
        Callback::from(move |_| {
            GameStorage::load_from_file(state.clone());
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