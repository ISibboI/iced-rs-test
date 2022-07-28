use crate::ui::{do_nothing, Message};
use crate::{Configuration, GameState};
use async_std::fs::File;
use async_std::io::{BufWriter, WriteExt};
use iced::alignment::Horizontal;
use iced::{Column, Command, Element, Length, Space, Text};
use log::{info, warn};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RunningState {
    game_state: GameState,
}

impl RunningState {
    pub fn new(game_state: GameState) -> Self {
        Self { game_state }
    }

    pub fn update(
        &mut self,
        configuration: &Configuration,
        message: RunningMessage,
    ) -> Command<Message> {
        match message {
            RunningMessage::Init => Command::none(),
            RunningMessage::SaveAndQuit => {
                Command::perform(save_game(self.game_state.clone()), |result| match result {
                    Ok(()) => {
                        info!("Game saved successfully!");
                        Message::Quit
                    }
                    Err(error) => {
                        warn!("Game could not be saved: {}", error.to_string());
                        Message::Quit
                    }
                })
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(Space::new(Length::Shrink, Length::Units(20)))
            .push(
                Text::new("Progress Quest")
                    .size(100)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center),
            )
            .push(Space::new(Length::Shrink, Length::Units(20)))
            .into()
    }
}

#[derive(Clone, Debug)]
pub enum RunningMessage {
    Init,
    SaveAndQuit,
}

async fn save_game(game_state: GameState) -> Result<(), SaveError> {
    let path = &game_state.savegame_file;
    let savegame_file = File::create(path).await?;
    let savegame = serde_json::to_vec(&game_state)?;
    println!("{savegame:?}");
    let mut writer = BufWriter::new(savegame_file);
    writer.write_all(&savegame).await?;
    writer.flush().await?;
    Ok(())
}

#[derive(Debug, Clone)]
pub enum SaveError {
    IoError(Arc<std::io::Error>),
    JsonError(Arc<serde_json::Error>),
}

impl From<std::io::Error> for SaveError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(Arc::new(error))
    }
}
impl From<serde_json::Error> for SaveError {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonError(Arc::new(error))
    }
}

impl ToString for SaveError {
    fn to_string(&self) -> String {
        match self {
            SaveError::IoError(error) => format!("IO error: {}", error.to_string()),
            SaveError::JsonError(error) => format!("Serialization error: {}", error.to_string()),
        }
    }
}
