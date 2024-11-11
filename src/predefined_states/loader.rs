use crate::game::GameState;
use serde_json::from_str;

pub fn load_predefined_states() -> Vec<GameState> {
    let mut states = Vec::new();
    let state1_json = include_str!("state1.json");
    let state2_json = include_str!("state2.json");

    let paths = vec![state1_json, state2_json];

    for (index, contents) in paths.iter().enumerate() {
        let path = format!("state{}.json", index + 1);
        log::info!("Attempting to read file: {}", path);
        match from_str::<GameState>(contents) {
            Ok(state) => {
                states.push(state);
                log::info!("Loaded state from {}", path);
            }
            Err(err) => {
                log::error!("Failed to parse JSON from {}: {}", path, err);
            }
        }
    }

    log::info!("Loaded {} predefined states", states.len());
    states
}