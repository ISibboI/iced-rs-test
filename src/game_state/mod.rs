use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub savegame_file: String,
    name: String,
    level: usize,
}

impl GameState {
    pub fn new(savegame_file: String, name: String) -> Self {
        Self {
            savegame_file,
            name,
            level: 1,
        }
    }
}
