use crate::game_state::character::CharacterRace;
use crate::game_template::parser::error::ParserError;
use crate::game_template::parser::parse_game_template_file;
use crate::game_template::CompiledGameTemplate;
use crate::ui::elements::{labelled_element, title};
use crate::ui::running_state::RunningState;
use crate::ui::{do_nothing, ApplicationUiState, Message};
use crate::{Configuration, GameState};
use async_std::fs::File;
use async_std::path::PathBuf;
use enum_iterator::all;
use iced::alignment::{Horizontal, Vertical};
use iced::{
    button, pick_list, text_input, Alignment, Button, Color, Column, Command, Container, Element,
    Length, PickList, Space, Text, TextInput,
};
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct CreateNewGameState {
    savegame_file: PathBuf,
    savegame_file_field: text_input::State,
    name: String,
    name_field: text_input::State,
    race: CharacterRace,
    race_field: pick_list::State<CharacterRace>,
    create_game_button: button::State,
    message: Option<String>,
}

impl CreateNewGameState {
    pub fn new(savegame_file: PathBuf) -> Self {
        Self {
            savegame_file,
            savegame_file_field: Default::default(),
            name: Default::default(),
            name_field: Default::default(),
            race: Default::default(),
            race_field: Default::default(),
            create_game_button: Default::default(),
            message: Default::default(),
        }
    }

    pub fn update(
        &mut self,
        configuration: &Configuration,
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
            CreateNewGameMessage::CompileGame => {
                return Command::perform(
                    create_game_boxed(configuration.game_template_file.clone()),
                    |game_template| CreateNewGameMessage::CreateGame(game_template).into(),
                );
            }
            CreateNewGameMessage::CreateGame(game_template) => {
                if self.name.is_empty() {
                    self.message = Some("Error: name is empty".to_string());
                } else {
                    match *game_template {
                        Ok(game_template) => {
                            return Command::perform(
                                do_nothing(GameState::new(
                                    game_template,
                                    self.savegame_file.clone(),
                                    self.name.clone(),
                                    self.race,
                                )),
                                |game_state| {
                                    Message::ChangeState(Box::new(ApplicationUiState::Running(
                                        Box::new(RunningState::new(game_state)),
                                    )))
                                },
                            );
                        }
                        Err(error) => {
                            self.message =
                                Some(format!("Error compiling game template: {:?}", error));
                        }
                    }
                }
            }
            CreateNewGameMessage::RaceChanged(race) => {
                self.race = race;
            }
        }

        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        let label_column_width = 130;

        let savegame_file_field_input = TextInput::new(
            &mut self.savegame_file_field,
            "",
            self.savegame_file.to_string_lossy().borrow(),
            |input| CreateNewGameMessage::SavegameFileChanged(PathBuf::from(input)).into(),
        )
        .padding(5)
        .width(Length::Fill);

        let name_field_input = TextInput::new(&mut self.name_field, "", &self.name, |input| {
            CreateNewGameMessage::NameChanged(input).into()
        })
        .padding(5)
        .width(Length::Fill);

        let race_field_input = PickList::new(
            &mut self.race_field,
            all::<CharacterRace>().collect::<Vec<_>>(),
            Some(self.race),
            |race| CreateNewGameMessage::RaceChanged(race).into(),
        )
        .padding(5);

        let column = Column::new()
            .padding(15)
            .spacing(5)
            .align_items(Alignment::Center)
            .width(Length::Fill)
            .push(title())
            .push(Text::new("Create New Game").size(40))
            .push(Space::new(Length::Shrink, Length::Units(10)))
            .push(
                Container::new(
                    Column::new()
                        .spacing(5)
                        .height(Length::Shrink)
                        .push(labelled_element(
                            "Savegame file:",
                            label_column_width,
                            savegame_file_field_input,
                        ))
                        .push(labelled_element(
                            "Name:",
                            label_column_width,
                            name_field_input,
                        ))
                        .push(labelled_element(
                            "Race:",
                            label_column_width,
                            race_field_input,
                        )),
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
                .on_press(CreateNewGameMessage::CompileGame.into()),
            );

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

pub async fn create_game_boxed(
    game_template_file: PathBuf,
) -> Box<Result<CompiledGameTemplate, ParserError>> {
    Box::new(create_game(game_template_file).await)
}

pub async fn create_game(game_template_file: PathBuf) -> Result<CompiledGameTemplate, ParserError> {
    let game_template_file = File::open(game_template_file)
        .await
        .map_err(|error| ParserError::without_coordinates(error.into()))?;
    let mut game_template = Default::default();
    parse_game_template_file(&mut game_template, game_template_file).await?;
    Ok(game_template.compile())
}

#[derive(Debug, Clone)]
pub enum CreateNewGameMessage {
    Init,
    NameChanged(String),
    SavegameFileChanged(PathBuf),
    RaceChanged(CharacterRace),
    CompileGame,
    CreateGame(Box<Result<CompiledGameTemplate, ParserError>>),
}
