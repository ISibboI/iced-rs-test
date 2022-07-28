use crate::GameState;
use iced::{button, text_input};

#[derive(Debug, Clone)]
pub struct CreateNewGameState {
    name: text_input::State,
    start_game_button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    StartGame,
}

pub fn create_new_game() -> GameState {
    todo!()
}
