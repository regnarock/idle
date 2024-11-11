use crate::game::GameState;
use serde_json::from_str;
use std::fs;

pub fn load_predefined_states() -> Vec<GameState> {
    let mut states = Vec::new();
    let paths = vec!["predefined_states/state1.json", "predefined_states/state2.json"];

    for path in paths {
        if let Ok(contents) = fs::read_to_string(path) {
            if let Ok(state) = from_str::<GameState>(&contents) {
                states.push(state);
            }
        }
    }

    states
}