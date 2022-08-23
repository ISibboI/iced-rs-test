use crate::savegames::{load_game, LoadError};
use crate::ui::bulk_update_state::BulkUpdateState;
use crate::ui::main_menu_state::MainMenuState;
use crate::ui::{do_nothing, ApplicationUiState, Message};
use crate::{Configuration, GameState};
use iced::alignment::{Horizontal, Vertical};
use iced::{Command, Element, Length, Text};
use log::{info, warn};

#[derive(Debug, Clone)]
pub struct LoadGameState {
    path: String,
}

impl LoadGameState {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn update(
        &mut self,
        configuration: &Configuration,
        message: LoadGameMessage,
    ) -> Command<Message> {
        match message {
            LoadGameMessage::Init => {
                info!("Loading '{}'", self.path);
                Command::perform(load_game(self.path.clone()), |loaded| {
                    LoadGameMessage::Loaded(Box::new(loaded)).into()
                })
            }
            LoadGameMessage::Loaded(loaded) => match *loaded {
                Ok(game_state) => {
                    info!("Loaded game");
                    Command::perform(
                        do_nothing(Box::new(BulkUpdateState::new(game_state))),
                        |bulk_update_state| {
                            Message::ChangeState(Box::new(ApplicationUiState::BulkUpdate(
                                bulk_update_state,
                            )))
                        },
                    )
                }
                Err(error) => {
                    warn!("Error loading game: {error:?}");
                    Command::perform(
                        do_nothing(Box::new(MainMenuState::new(
                            configuration.savegame_file.clone(),
                            Some(error.to_string()),
                        ))),
                        |main_menu_state| {
                            Message::ChangeState(Box::new(ApplicationUiState::MainMenu(
                                main_menu_state,
                            )))
                        },
                    )
                }
            },
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Text::new("Loading...")
            .size(100)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

#[derive(Clone, Debug)]
pub enum LoadGameMessage {
    Init,
    Loaded(Box<Result<GameState, LoadError>>),
}
