use crate::ui::create_new_game_state::CreateNewGameState;
use crate::ui::elements::title;
use crate::ui::load_game_state::LoadGameState;
use crate::ui::{do_nothing, ApplicationUiState, Message};
use crate::Configuration;
use async_std::path::PathBuf;
use iced::alignment::Horizontal;
use iced::{
    button, text_input, Alignment, Button, Color, Column, Command, Element, Length, Space, Text,
    TextInput,
};
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct MainMenuState {
    savegame_file_input: text_input::State,
    savegame_file: PathBuf,
    message: Option<String>,
    load_game_button: button::State,
    new_game_button: button::State,
}

impl MainMenuState {
    pub fn new(default_savegame_file: PathBuf, message: Option<String>) -> Self {
        Self {
            savegame_file_input: Default::default(),
            savegame_file: default_savegame_file,
            message,
            load_game_button: Default::default(),
            new_game_button: Default::default(),
        }
    }

    pub fn update(
        &mut self,
        _configuration: &Configuration,
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
                return Command::perform(do_nothing(self.savegame_file.clone()), |savegame_file| {
                    Message::ChangeState(Box::new(ApplicationUiState::CreateNewGame(Box::new(
                        CreateNewGameState::new(savegame_file),
                    ))))
                })
            }
            MainMenuMessage::SavegameFileInputChanged(input) => self.savegame_file = input,
            MainMenuMessage::Init => {}
        }

        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        let savegame_file_input = TextInput::new(
            &mut self.savegame_file_input,
            "",
            self.savegame_file.to_string_lossy().borrow(),
            |input| MainMenuMessage::SavegameFileInputChanged(PathBuf::from(input)).into(),
        )
        .padding(5)
        .width(Length::Units(200));
        let load_game_button = Button::new(
            &mut self.load_game_button,
            Text::new("Load Game").horizontal_alignment(Horizontal::Center),
        )
        .on_press(MainMenuMessage::LoadGame.into())
        .padding(5)
        .width(Length::Units(100));
        let new_game_button = Button::new(
            &mut self.new_game_button,
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
                .push(Text::new(message).color(Color::from_rgb(0.9, 0.1, 0.1)))
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
