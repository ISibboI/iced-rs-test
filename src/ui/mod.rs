use crate::ui::create_new_game_state::CreateNewGameState;
use crate::ui::load_game_state::{LoadGameMessage, LoadGameState};
use crate::ui::main_menu_state::{MainMenuMessage, MainMenuState};
use crate::{Configuration, GameState};
use iced::{Application, Command, Element};
use log::{debug, error, info};

mod create_new_game_state;
mod load_game_state;
mod main_menu_state;
mod running_state;

#[derive(Debug)]
pub struct ApplicationState {
    configuration: Configuration,
    ui_state: ApplicationUiState,
}

#[derive(Debug, Clone)]
pub enum ApplicationUiState {
    MainMenu(MainMenuState),
    Loading(LoadGameState),
    CreateNewGame(CreateNewGameState),
    Running(GameState),
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeState(ApplicationUiState),
    MainMenuMessage(MainMenuMessage),
    LoadGameMessage(LoadGameMessage),
}

impl Application for ApplicationState {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = Configuration;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                ui_state: ApplicationUiState::MainMenu(MainMenuState::new(
                    flags.savegame_file.clone(),
                    None,
                )),
                configuration: flags,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Progress Quest".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match (message, &mut self.ui_state) {
            (Message::ChangeState(new_ui_state), ui_state) => {
                *ui_state = new_ui_state;
                debug!("Updated ui state to {ui_state:?}");
                Command::perform(do_nothing(ui_state.init_message()), |init_message| {
                    init_message
                })
            }
            (
                Message::MainMenuMessage(main_menu_message),
                ApplicationUiState::MainMenu(main_menu_state),
            ) => main_menu_state.update(&self.configuration, main_menu_message),
            (
                Message::LoadGameMessage(load_game_message),
                ApplicationUiState::Loading(load_game_state),
            ) => load_game_state.update(&self.configuration, load_game_message),
            (message, ui_state) => {
                panic!("Illegal combination of message and ui state: {message:?}; {ui_state:?}");
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        match &mut self.ui_state {
            ApplicationUiState::MainMenu(main_menu_state) => main_menu_state.view(),
            ApplicationUiState::Loading(load_game_state) => load_game_state.view(),
            ApplicationUiState::CreateNewGame(_) => todo!(),
            ApplicationUiState::Running(_) => todo!(),
        }
    }
}

impl From<MainMenuMessage> for Message {
    fn from(main_menu_message: MainMenuMessage) -> Self {
        Self::MainMenuMessage(main_menu_message)
    }
}

impl From<LoadGameMessage> for Message {
    fn from(load_game_message: LoadGameMessage) -> Self {
        Self::LoadGameMessage(load_game_message)
    }
}

async fn do_nothing<T>(t: T) -> T {
    t
}

impl ApplicationUiState {
    pub fn init_message(&self) -> Message {
        match self {
            ApplicationUiState::MainMenu(_) => MainMenuMessage::Init.into(),
            ApplicationUiState::Loading(_) => LoadGameMessage::Init.into(),
            ApplicationUiState::CreateNewGame(_) => todo!(),
            ApplicationUiState::Running(_) => todo!(),
        }
    }
}
