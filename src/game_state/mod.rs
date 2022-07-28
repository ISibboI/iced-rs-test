use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    name: String,
    level: usize,
}

impl GameState {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            level: 1,
        }
    }
}
