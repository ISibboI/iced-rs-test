use crate::ui::elements::title;
use crate::ui::load_game_state::LoadGameState;
use crate::ui::load_game_template_state::LoadGameTemplateState;
use crate::ui::{do_nothing, ApplicationUiState, Message};
use crate::RunConfiguration;
use async_std::path::PathBuf;
use iced::alignment::Horizontal;
use iced::{
    Alignment, Command, Element, Length,
};
use iced::widget::{Button,Column,Space, Text,
                   TextInput,};
use std::borrow::Borrow;
use crate::ui::style::RedText;

#[derive(Debug, Clone)]
pub struct MainMenuState {
    savegame_file: PathBuf,
    message: Option<String>,
}

impl MainMenuState {
    pub fn new(default_savegame_file: PathBuf, message: Option<String>) -> Self {
        Self {
            savegame_file: default_savegame_file,
            message,
        }
    }

    pub fn update(
        &mut self,
        _configuration: &RunConfiguration,
        message: MainMenuMessage,
    ) -> Command<Message> {
        match message {
            MainMenuMessage::LoadGame => {
                return Command::perform(do_nothing(self.savegame_file.clone()), |savegame_file| {
                    Message::ChangeState(Box::new(ApplicationUiState::Loading(Box::new(
                        LoadGameState::new(savegame_file),
                    ))))
                });
            }
            MainMenuMessage::NewGame => {
                return Command::perform(do_nothing(()), |_| {
                    Message::ChangeState(Box::new(ApplicationUiState::LoadingTemplate(Box::new(
                        LoadGameTemplateState::new(),
                    ))))
                })
            }
            MainMenuMessage::SavegameFileInputChanged(input) => self.savegame_file = input,
            MainMenuMessage::Init => {}
        }

        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        let savegame_file_input = TextInput::new(
            "",
            self.savegame_file.to_string_lossy().borrow(),
            |input| MainMenuMessage::SavegameFileInputChanged(PathBuf::from(input)).into(),
        )
        .padding(5)
        .width(Length::Units(200));
        let load_game_button = Button::new(
            Text::new("Load Game").horizontal_alignment(Horizontal::Center),
        )
        .on_press(MainMenuMessage::LoadGame.into())
        .padding(5)
        .width(Length::Units(100));
        let new_game_button = Button::new(
            Text::new("New Game").horizontal_alignment(Horizontal::Center),
        )
        .on_press(MainMenuMessage::NewGame.into())
        .padding(5)
        .width(Length::Units(100));

        let column = Column::new()
            .padding(15)
            .spacing(5)
            .align_items(Alignment::Center)
            .width(Length::Fill)
            .push(title())
            .push(savegame_file_input)
            .push(load_game_button)
            .push(new_game_button);

        let column = if let Some(message) = &self.message {
            column
                .push(Space::new(Length::Shrink, Length::Units(100)))
                .push(Text::new(message).style(RedText))
        } else {
            column
        };

        column.into()
    }
}

#[derive(Clone, Debug)]
pub enum MainMenuMessage {
    Init,
    LoadGame,
    NewGame,
    SavegameFileInputChanged(PathBuf),
}
