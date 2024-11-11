pub mod auto_increment;
pub mod auto_save;
pub mod game_state;

pub use game_state::use_game_state;
pub use auto_save::use_auto_save;

pub use game_state::GameStateHandle;