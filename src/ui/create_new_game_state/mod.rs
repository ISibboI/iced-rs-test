use crate::game_state::character::CharacterRace;
use crate::game_state::GameStateInitialisation;
use crate::game_template::CompiledGameTemplate;
use crate::ui::elements::{labelled_element, title};
use crate::ui::running_state::RunningState;
use crate::ui::{do_nothing, ApplicationUiState, Message};
use crate::{GameState, RunConfiguration};
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
    savegame_file_field: text_input::State,
    name_field: text_input::State,
    pronoun_field: text_input::State,
    race_field: pick_list::State<CharacterRace>,
    create_game_button: button::State,
    message: Option<String>,
    game_initialisation: GameStateInitialisation,
    game_template: Option<CompiledGameTemplate>,
}

#[derive(Debug, Clone)]
pub enum CreateNewGameMessage {
    Init,
    NameChanged(String),
    PronounChanged(String),
    SavegameFileChanged(PathBuf),
    RaceChanged(CharacterRace),
    CreateGame,
}

impl CreateNewGameState {
    pub fn new(game_template: CompiledGameTemplate, savegame_file: PathBuf) -> Self {
        let game_initialisation = GameStateInitialisation {
            savegame_file,
            name: "Hugo".to_string(),
            pronoun: "he".to_string(),
            race: Default::default(),
        };
        Self {
            savegame_file_field: Default::default(),
            name_field: Default::default(),
            pronoun_field: Default::default(),
            race_field: Default::default(),
            create_game_button: Default::default(),
            message: Default::default(),
            game_initialisation,
            game_template: Some(game_template),
        }
    }

    pub fn update(
        &mut self,
        _configuration: &RunConfiguration,
        message: CreateNewGameMessage,
    ) -> Command<Message> {
        match message {
            CreateNewGameMessage::Init => {}
            CreateNewGameMessage::NameChanged(name) => {
                self.game_initialisation.name = name;
            }
            CreateNewGameMessage::PronounChanged(pronoun) => {
                self.game_initialisation.pronoun = pronoun;
            }
            CreateNewGameMessage::SavegameFileChanged(savegame_file) => {
                self.game_initialisation.savegame_file = savegame_file;
            }
            CreateNewGameMessage::CreateGame => {
                if self.game_initialisation.name.is_empty() {
                    self.message = Some("Error: name is empty".to_string());
                } else {
                    return Command::perform(
                        do_nothing(Box::new(RunningState::new(GameState::new(
                            self.game_template.take().unwrap(),
                            self.game_initialisation.clone(),
                        )))),
                        |running_state| {
                            Message::ChangeState(Box::new(ApplicationUiState::Running(
                                running_state,
                            )))
                        },
                    );
                }
            }
            CreateNewGameMessage::RaceChanged(race) => {
                self.game_initialisation.race = race;
            }
        }

        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        let label_column_width = 130;

        let savegame_file_field_input = TextInput::new(
            &mut self.savegame_file_field,
            "",
            self.game_initialisation
                .savegame_file
                .to_string_lossy()
                .borrow(),
            |input| CreateNewGameMessage::SavegameFileChanged(PathBuf::from(input)).into(),
        )
        .padding(5)
        .width(Length::Fill);

        let name_field_input = TextInput::new(
            &mut self.name_field,
            "",
            &self.game_initialisation.name,
            |input| CreateNewGameMessage::NameChanged(input).into(),
        )
        .padding(5)
        .width(Length::Fill);

        let pronoun_field_input = TextInput::new(
            &mut self.pronoun_field,
            "",
            &self.game_initialisation.pronoun,
            |input| CreateNewGameMessage::PronounChanged(input).into(),
        )
        .padding(5)
        .width(Length::Fill);

        let race_field_input = PickList::new(
            &mut self.race_field,
            all::<CharacterRace>().collect::<Vec<_>>(),
            Some(self.game_initialisation.race),
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
                            "Pronoun:",
                            label_column_width,
                            pronoun_field_input,
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
                .on_press(CreateNewGameMessage::CreateGame.into()),
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
