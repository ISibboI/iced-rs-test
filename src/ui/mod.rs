use crate::ui::bulk_update_state::{BulkUpdateMessage, BulkUpdateState};
use crate::ui::create_new_game_state::{CreateNewGameMessage, CreateNewGameState};
use crate::ui::load_game_state::{LoadGameMessage, LoadGameState};
use crate::ui::load_game_template_state::{LoadGameTemplateMessage, LoadGameTemplateState};
use crate::ui::main_menu_state::{MainMenuMessage, MainMenuState};
use crate::ui::running_state::{RunningMessage, RunningState};
use crate::{RunConfiguration, TITLE};
use iced::{Application, Command, Element, Subscription};
use log::{debug, info};

mod bulk_update_state;
mod create_new_game_state;
mod elements;
mod load_game_state;
mod load_game_template_state;
mod main_menu_state;
mod running_state;
mod style;

#[derive(Debug)]
pub struct ApplicationState {
    configuration: RunConfiguration,
    ui_state: ApplicationUiState,
    should_exit: bool,
}

#[derive(Debug, Clone)]
pub enum ApplicationUiState {
    MainMenu(Box<MainMenuState>),
    Loading(Box<LoadGameState>),
    LoadingTemplate(Box<LoadGameTemplateState>),
    BulkUpdate(Box<BulkUpdateState>),
    CreateNewGame(Box<CreateNewGameState>),
    Running(Box<RunningState>),
}

#[derive(Debug, Clone)]
pub enum Message {
    NativeEvent(iced_native::Event),
    ChangeState(Box<ApplicationUiState>),
    MainMenu(MainMenuMessage),
    LoadGame(LoadGameMessage),
    LoadGameTemplate(LoadGameTemplateMessage),
    BulkUpdate(BulkUpdateMessage),
    CreateNewGame(CreateNewGameMessage),
    Running(RunningMessage),
    Quit,
}

impl Application for ApplicationState {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = RunConfiguration;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                ui_state: ApplicationUiState::MainMenu(Box::new(MainMenuState::new(
                    flags.savegame_file.clone(),
                    None,
                ))),
                configuration: flags,
                should_exit: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        TITLE.into()
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
                Message::LoadGameTemplate(load_game_template_message),
                ApplicationUiState::LoadingTemplate(load_game_template_state),
            ) => load_game_template_state.update(&self.configuration, load_game_template_message),
            (
                Message::BulkUpdate(bulk_update_message),
                ApplicationUiState::BulkUpdate(bulk_update_state),
            ) => bulk_update_state.update(&self.configuration, bulk_update_message),
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
        let mut subscriptions = vec![iced_native::subscription::events().map(Message::NativeEvent)];
        if let ApplicationUiState::Running(_) = &self.ui_state {
            subscriptions.push(
                iced::time::every(std::time::Duration::from_nanos(
                    (1e9 / self.configuration.target_fps) as u64,
                ))
                .map(|_| Message::Running(RunningMessage::Update)),
            );
        }
        Subscription::batch(subscriptions)
    }

    fn view(&mut self) -> Element<Self::Message> {
        match &mut self.ui_state {
            ApplicationUiState::MainMenu(main_menu_state) => main_menu_state.view(),
            ApplicationUiState::Loading(load_game_state) => load_game_state.view(),
            ApplicationUiState::LoadingTemplate(load_game_template_state) => {
                load_game_template_state.view()
            }
            ApplicationUiState::BulkUpdate(bulk_update_state) => bulk_update_state.view(),
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

impl From<LoadGameTemplateMessage> for Message {
    fn from(load_game_message: LoadGameTemplateMessage) -> Self {
        Self::LoadGameTemplate(load_game_message)
    }
}

impl From<BulkUpdateMessage> for Message {
    fn from(bulk_update_message: BulkUpdateMessage) -> Self {
        Self::BulkUpdate(bulk_update_message)
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
            ApplicationUiState::LoadingTemplate(_) => LoadGameTemplateMessage::Init.into(),
            ApplicationUiState::BulkUpdate(_) => BulkUpdateMessage::Init.into(),
            ApplicationUiState::CreateNewGame(_) => CreateNewGameMessage::Init.into(),
            ApplicationUiState::Running(_) => RunningMessage::Init.into(),
        }
    }
}
