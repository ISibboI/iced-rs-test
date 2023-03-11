use crate::game_template::CompiledGameTemplate;
use crate::io::{load_game_template, LoadError};
use crate::ui::create_new_game_state::CreateNewGameState;
use crate::ui::main_menu_state::MainMenuState;
use crate::ui::{do_nothing, ApplicationUiState, Message};
use crate::RunConfiguration;
use async_std::sync::Arc;
use iced::alignment::{Horizontal, Vertical};
use iced::{Command, Element, Length};
use iced::widget::Text;
use log::{error, info};

#[derive(Debug, Clone)]
pub struct LoadGameTemplateState {}

#[derive(Clone, Debug)]
pub enum LoadGameTemplateMessage {
    Init,
    Loaded(Box<Result<CompiledGameTemplate, LoadError>>),
}

impl LoadGameTemplateState {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(
        &mut self,
        configuration: Arc<RunConfiguration>,
        message: LoadGameTemplateMessage,
    ) -> Command<Message> {
        match message {
            LoadGameTemplateMessage::Init => {
                Command::perform(load_game_template(configuration), |loaded| {
                    LoadGameTemplateMessage::Loaded(Box::new(loaded)).into()
                })
            }
            LoadGameTemplateMessage::Loaded(loaded) => match *loaded {
                Ok(game_template) => {
                    info!("Loaded game template");
                    Command::perform(
                        do_nothing(Box::new(CreateNewGameState::new(
                            game_template,
                            configuration.savegame_file.clone(),
                        ))),
                        |running_state| {
                            Message::ChangeState(Box::new(ApplicationUiState::CreateNewGame(
                                running_state,
                            )))
                        },
                    )
                }
                Err(error) => {
                    error!("Error loading game template: {error:?}");
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
        Text::new("Loading game template...")
            .size(100)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
