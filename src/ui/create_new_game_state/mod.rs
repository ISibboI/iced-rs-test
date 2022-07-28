use crate::ui::running_state::RunningState;
use crate::ui::{do_nothing, ApplicationUiState, Message};
use crate::{Configuration, GameState};
use iced::alignment::{Horizontal, Vertical};
use iced::{
    button, text_input, Alignment, Button, Column, Command, Container, Element, Length, Row, Space,
    Text, TextInput,
};

#[derive(Debug, Clone)]
pub struct CreateNewGameState {
    savegame_file: String,
    savegame_file_field: text_input::State,
    name: String,
    name_field: text_input::State,
    create_game_button: button::State,
}

impl CreateNewGameState {
    pub fn new(savegame_file: String) -> Self {
        Self {
            savegame_file,
            savegame_file_field: Default::default(),
            name: Default::default(),
            name_field: Default::default(),
            create_game_button: Default::default(),
        }
    }

    pub fn update(
        &mut self,
        _configuration: &Configuration,
        message: CreateNewGameMessage,
    ) -> Command<Message> {
        match message {
            CreateNewGameMessage::Init => {}
            CreateNewGameMessage::NameChanged(name) => {
                self.name = name;
            }
            CreateNewGameMessage::SavegameFileChanged(savegame_file) => {
                self.savegame_file = savegame_file;
            }
            CreateNewGameMessage::CreateGame => {
                return Command::perform(
                    do_nothing(GameState::new(
                        self.savegame_file.clone(),
                        self.name.clone(),
                    )),
                    |game_state| {
                        Message::ChangeState(ApplicationUiState::Running(RunningState::new(
                            game_state,
                        )))
                    },
                );
            }
        }

        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        let label_column_width = 130;

        let savegame_file_field_input = TextInput::new(
            &mut self.savegame_file_field,
            "",
            &self.savegame_file,
            |input| CreateNewGameMessage::SavegameFileChanged(input).into(),
        )
        .padding(5)
        .size(20)
        .width(Length::Fill);
        let name_field_input = TextInput::new(&mut self.name_field, "", &self.name, |input| {
            CreateNewGameMessage::NameChanged(input).into()
        })
        .padding(5)
        .size(20)
        .width(Length::Fill);

        Column::new()
            .padding(15)
            .spacing(5)
            .align_items(Alignment::Center)
            .width(Length::Fill)
            .push(Space::new(Length::Shrink, Length::Units(20)))
            .push(Text::new("Create New Game").size(100))
            .push(Space::new(Length::Shrink, Length::Units(20)))
            .push(
                Container::new(
                    Column::new()
                        .spacing(5)
                        .height(Length::Shrink)
                        .push(
                            Row::new()
                                .spacing(5)
                                .height(Length::Units(20 + 2 * 5))
                                .push(
                                    Text::new("Savegame file:")
                                        .vertical_alignment(Vertical::Center)
                                        .width(Length::Units(label_column_width))
                                        .height(Length::Fill),
                                )
                                .push(savegame_file_field_input),
                        )
                        .push(
                            Row::new()
                                .spacing(5)
                                .height(Length::Units(20 + 2 * 5))
                                .push(
                                    Text::new("Name:")
                                        .vertical_alignment(Vertical::Center)
                                        .width(Length::Units(label_column_width))
                                        .height(Length::Fill),
                                )
                                .push(name_field_input),
                        ),
                )
                .width(Length::Units(500))
                .height(Length::Shrink),
            )
            .push(
                Button::new(
                    &mut self.create_game_button,
                    Text::new("Create Game")
                        .horizontal_alignment(Horizontal::Center)
                        .vertical_alignment(Vertical::Center),
                )
                .on_press(CreateNewGameMessage::CreateGame.into()),
            )
            .into()
    }
}

#[derive(Clone, Debug)]
pub enum CreateNewGameMessage {
    Init,
    NameChanged(String),
    SavegameFileChanged(String),
    CreateGame,
}
