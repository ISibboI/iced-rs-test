use crate::ui::create_new_game_state::{CreateNewGameMessage, CreateNewGameState};
use crate::ui::load_game_state::{LoadGameMessage, LoadGameState};
use crate::ui::main_menu_state::{MainMenuMessage, MainMenuState};
use crate::ui::running_state::{RunningMessage, RunningState};
use crate::Configuration;
use iced::{Application, Command, Element, Subscription};
use log::{debug, info};

mod create_new_game_state;
mod elements;
mod load_game_state;
mod main_menu_state;
mod running_state;

#[derive(Debug)]
pub struct ApplicationState {
    configuration: Configuration,
    ui_state: ApplicationUiState,
    should_exit: bool,
}

#[derive(Debug, Clone)]
pub enum ApplicationUiState {
    MainMenu(MainMenuState),
    Loading(LoadGameState),
    CreateNewGame(CreateNewGameState),
    Running(RunningState),
}

#[derive(Debug, Clone)]
pub enum Message {
    NativeEvent(iced_native::Event),
    ChangeState(Box<ApplicationUiState>),
    MainMenu(MainMenuMessage),
    LoadGame(LoadGameMessage),
    CreateNewGame(CreateNewGameMessage),
    Running(RunningMessage),
    Quit,
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
                should_exit: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Progress Quest".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match (message, &mut self.ui_state) {
            (Message::NativeEvent(event), ui_state) => match (event, ui_state) {
                (
                    iced_native::Event::Window(iced_native::window::Event::CloseRequested),
                    ApplicationUiState::Running(_),
                ) => {
                    info!("Saving and exiting...");
                    Command::perform(do_nothing(()), |()| RunningMessage::SaveAndQuit.into())
                }
                (iced_native::Event::Window(iced_native::window::Event::CloseRequested), _) => {
                    info!("Exiting...");
                    self.should_exit = true;
                    Command::none()
                }
                _ => Command::none(),
            },
            (Message::ChangeState(new_ui_state), ui_state) => {
                *ui_state = *new_ui_state;
                debug!("Updated ui state to {ui_state:?}");
                Command::perform(do_nothing(ui_state.init_message()), |init_message| {
                    init_message
                })
            }
            (Message::Quit, _) => {
                info!("Exiting...");
                self.should_exit = true;
                Command::none()
            }
            (
                Message::MainMenu(main_menu_message),
                ApplicationUiState::MainMenu(main_menu_state),
            ) => main_menu_state.update(&self.configuration, main_menu_message),
            (
                Message::LoadGame(load_game_message),
                ApplicationUiState::Loading(load_game_state),
            ) => load_game_state.update(&self.configuration, load_game_message),
            (
                Message::CreateNewGame(create_new_game_message),
                ApplicationUiState::CreateNewGame(create_new_game_state),
            ) => create_new_game_state.update(&self.configuration, create_new_game_message),
            (Message::Running(running_message), ApplicationUiState::Running(running_state)) => {
                running_state.update(&self.configuration, running_message)
            }
            (message, ui_state) => {
                panic!("Illegal combination of message and ui state: {message:?}; {ui_state:?}");
            }
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced_native::subscription::events().map(Message::NativeEvent)
    }

    fn view(&mut self) -> Element<Self::Message> {
        match &mut self.ui_state {
            ApplicationUiState::MainMenu(main_menu_state) => main_menu_state.view(),
            ApplicationUiState::Loading(load_game_state) => load_game_state.view(),
            ApplicationUiState::CreateNewGame(create_new_game_state) => {
                create_new_game_state.view()
            }
            ApplicationUiState::Running(running_state) => running_state.view(),
        }
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }
}

impl From<MainMenuMessage> for Message {
    fn from(main_menu_message: MainMenuMessage) -> Self {
        Self::MainMenu(main_menu_message)
    }
}

impl From<LoadGameMessage> for Message {
    fn from(load_game_message: LoadGameMessage) -> Self {
        Self::LoadGame(load_game_message)
    }
}

impl From<CreateNewGameMessage> for Message {
    fn from(create_new_game_message: CreateNewGameMessage) -> Self {
        Self::CreateNewGame(create_new_game_message)
    }
}

impl From<RunningMessage> for Message {
    fn from(running_message: RunningMessage) -> Self {
        Self::Running(running_message)
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
            ApplicationUiState::CreateNewGame(_) => CreateNewGameMessage::Init.into(),
            ApplicationUiState::Running(_) => RunningMessage::Init.into(),
        }
    }
}
