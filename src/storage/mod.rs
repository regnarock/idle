use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io;
use crate::game::GameState;
use crate::predefined_states::load_predefined_states;

const SAVE_KEY: &str = "idle_game_save";
const SAVE_FILE: &str = "game_state.json";

#[derive(Serialize, Deserialize)]
pub struct GameStorage;

impl GameStorage {
    pub fn load() -> GameState {
        LocalStorage::get(SAVE_KEY).unwrap_or_else(|_| {
            let predefined_states = load_predefined_states();
            predefined_states.get(0).cloned().unwrap_or_else(GameState::new)
        })
    }

    pub fn save(state: &GameState) -> Result<(), String> {
        LocalStorage::set(SAVE_KEY, state)
            .map_err(|e| format!("Failed to save game: {}", e))
    }

    pub fn clear() {
        LocalStorage::delete(SAVE_KEY);
    }

    pub fn save_to_file(state: &GameState) -> Result<(), io::Error> {
        let file = File::create(SAVE_FILE)?;
        serde_json::to_writer(file, state)?;
        Ok(())
    }

    pub fn load_from_file() -> Result<GameState, io::Error> {
        let file = File::open(SAVE_FILE)?;
        let state = serde_json::from_reader(file)?;
        Ok(state)
    }
}