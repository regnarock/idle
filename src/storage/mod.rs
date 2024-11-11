use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use yew::UseStateHandle;
use crate::game::GameState;
use crate::predefined_states::load_predefined_states;
use crate::utils::file::{save_to_file, load_from_file};

const SAVE_KEY: &str = "idle_game_save";

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

    pub fn save_to_file(state: &GameState) {
        save_to_file(state);
    }

    pub fn load_from_file(state: UseStateHandle<GameState>) {
        load_from_file(state);
    }
}