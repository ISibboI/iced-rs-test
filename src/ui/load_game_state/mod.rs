use crate::ui::main_menu_state::MainMenuState;
use crate::ui::{do_nothing, ApplicationUiState, Message};
use crate::{Configuration, GameState};
use async_std::fs::File;
use async_std::io::{BufReader, ReadExt};
use iced::alignment::{Horizontal, Vertical};
use iced::{Command, Element, Length, Text};
use log::{info, warn};
use std::sync::Arc;

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
                    LoadGameMessage::Loaded(loaded).into()
                })
            }
            LoadGameMessage::Loaded(loaded) => match loaded {
                Ok(game_state) => {
                    info!("Loaded game");
                    Command::perform(do_nothing(game_state), |game_state| {
                        Message::ChangeState(ApplicationUiState::Running(game_state))
                    })
                }
                Err(error) => {
                    warn!("Error loading game: {error:?}");
                    Command::perform(
                        do_nothing((configuration.savegame_file.clone(), error)),
                        |(default_savegame_file, error)| {
                            Message::ChangeState(ApplicationUiState::MainMenu(MainMenuState::new(
                                default_savegame_file,
                                Some(match error {
                                    LoadError::IoError(error) => {
                                        format!("IO error: {}", error.to_string())
                                    }
                                    LoadError::JsonError(error) => {
                                        format!("Parsing error: {}", error.to_string())
                                    }
                                }),
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
    Loaded(Result<GameState, LoadError>),
}

async fn load_game(path: impl AsRef<str>) -> Result<GameState, LoadError> {
    let path = path.as_ref();
    let savegame_file = File::open(path).await?;
    let mut savegame = String::new();
    BufReader::new(savegame_file)
        .read_to_string(&mut savegame)
        .await?;
    Ok(serde_json::from_str(&savegame)?)
}

#[derive(Debug, Clone)]
pub enum LoadError {
    IoError(Arc<std::io::Error>),
    JsonError(Arc<serde_json::Error>),
}

impl From<std::io::Error> for LoadError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(Arc::new(error))
    }
}
impl From<serde_json::Error> for LoadError {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonError(Arc::new(error))
    }
}
