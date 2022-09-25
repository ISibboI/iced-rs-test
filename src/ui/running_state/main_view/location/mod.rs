use crate::io::{load_bytes, LoadError};
use crate::ui::running_state::main_view::MainViewMessage;
use crate::ui::running_state::{GameStateMessage, RunningMessage};
use crate::ui::style::FramedContainer;
use crate::ui::Message;
use crate::{GameState, RunConfiguration};
use async_std::sync::Arc;
use iced::{Command, Container, Element, Image, Length, Space};
use iced_native::image::Handle;
use log::{debug, error, info, warn};

#[derive(Debug, Clone)]
pub struct LocationState {
    url: Option<String>,
    handle: Option<Handle>,
}

#[derive(Debug, Clone)]
pub enum LocationMessage {
    Init,
    LoadedImage {
        url: String,
        bytes: Result<Vec<u8>, LoadError>,
    },
}

impl LocationState {
    pub fn new(game_state: &GameState) -> Self {
        Self {
            url: game_state.world.selected_location().url.clone(),
            handle: None,
        }
    }

    pub fn update(
        &mut self,
        configuration: Arc<RunConfiguration>,
        message: LocationMessage,
    ) -> Command<Message> {
        match message {
            LocationMessage::Init => {
                if let Some(url) = self.url.clone() {
                    load_image_command(configuration, url)
                } else {
                    Command::none()
                }
            }
            LocationMessage::LoadedImage { url, bytes } => {
                match bytes {
                    Ok(bytes) => {
                        info!("Loaded image {url:?}");
                        if Some(url) == self.url {
                            self.handle = Some(Handle::from_memory(bytes));
                        } else {
                            debug!("Image was loaded too late, url is now {:?}", self.url);
                        }
                    }
                    Err(error) => {
                        error!("Error loading image {error:?}");
                    }
                }
                Command::none()
            }
        }
    }

    pub fn update_game_state(
        &mut self,
        configuration: Arc<RunConfiguration>,
        game_state: &GameState,
        message: &GameStateMessage,
    ) -> Command<Message> {
        match message {
            GameStateMessage::ExplorationLocationChanged(location) => {
                if game_state.world.selected_location == *location {
                    if let Some(url) = game_state.world.selected_location().url.clone() {
                        return load_image_command(configuration, url);
                    } else {
                        warn!(
                            "Location {:?} is missing image url",
                            game_state.world.selected_location().name
                        );
                    }
                } else {
                    warn!(
                        "Selected location {:?} does not match location in message {location:?}",
                        game_state.world.selected_location
                    );
                }

                Command::none()
            }
            _ => Command::none(),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        if let Some(handle) = self.handle.clone() {
            let image = Image::new(handle);
            Container::new(image)
                .padding(5)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(FramedContainer)
                .into()
        } else {
            Container::new(Space::new(Length::Fill, Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(FramedContainer)
                .into()
        }
    }
}

fn load_image_command(configuration: Arc<RunConfiguration>, url: String) -> Command<Message> {
    debug!("Creating load_bytes command for url {url:?}");
    Command::perform(load_bytes(configuration, url.clone()), move |bytes| {
        LocationMessage::LoadedImage {
            url: url.clone(),
            bytes,
        }
        .into()
    })
}

impl From<LocationMessage> for Message {
    fn from(message: LocationMessage) -> Self {
        Message::Running(RunningMessage::MainView(MainViewMessage::Location(message)))
    }
}
