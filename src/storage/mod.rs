use gloo_storage::{LocalStorage, Storage};
use crate::game::GameState;

const SAVE_KEY: &str = "idle_game_save";

pub struct GameStorage;

impl GameStorage {
    pub fn load() -> GameState {
        LocalStorage::get(SAVE_KEY).unwrap_or_else(|_| GameState::new())
    }

    pub fn save(state: &GameState) -> Result<(), String> {
        LocalStorage::set(SAVE_KEY, state)
            .map_err(|e| format!("Failed to save game: {}", e))
    }

    pub fn clear() {
        LocalStorage::delete(SAVE_KEY);
    }
}