use crate::ui::elements::active_action_description;
use crate::ui::running_state::main_view::action_picker::ActionPickerState;
use crate::ui::running_state::main_view::calendar::{CalendarMessage, CalendarState};
use crate::ui::running_state::main_view::location::{LocationMessage, LocationState};
use crate::ui::running_state::main_view::overview::OverviewState;
use crate::ui::running_state::main_view::story::{StoryMessage, StoryState};
use crate::ui::running_state::{GameStateMessage, RunningMessage};
use crate::ui::style::{ButtonStyleSheet, FramedContainer, SelectedButtonStyleSheet};
use crate::ui::Message;
use crate::{GameState, RunConfiguration};
use async_std::sync::Arc;
use iced::{Command,  Element, Length};
use iced::widget::{Button, Column, Container, ProgressBar, Row, Text};

mod action_picker;
mod calendar;
mod location;
mod overview;
mod story;

#[derive(Debug, Clone)]
pub struct MainViewState {
    selected_view: SelectedView,
    overview_state: OverviewState,
    location_state: LocationState,
    action_picker_state: ActionPickerState,
    story_state: StoryState,
    calendar_state: CalendarState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SelectedView {
    Overview,
    Location,
    ActionPicker,
    Story,
    Calendar,
}

#[derive(Clone, Debug)]
pub enum MainViewMessage {
    Init,
    SelectView(SelectedView),
    Calendar(CalendarMessage),
    Story(StoryMessage),
    Location(LocationMessage),
}

impl MainViewState {
    pub fn new(game_state: &GameState) -> Self {
        Self {
            selected_view: SelectedView::Overview,
            overview_state: OverviewState::new(),
            location_state: LocationState::new(game_state),
            action_picker_state: ActionPickerState::new(),
            story_state: StoryState::new(),
            calendar_state: CalendarState::new(game_state),
        }
    }

    pub fn update(
        &mut self,
        configuration: Arc<RunConfiguration>,
        message: MainViewMessage,
        game_state: &GameState,
    ) -> Command<Message> {
        match message {
            MainViewMessage::Init => Command::batch([
                self.location_state
                    .update(configuration, LocationMessage::Init),
                self.story_state.update(StoryMessage::Init, game_state),
            ]),
            MainViewMessage::SelectView(selected_view) => {
                self.selected_view = selected_view;
                Command::none()
            }
            MainViewMessage::Calendar(calendar_message) => {
                self.calendar_state.update(calendar_message)
            }
            MainViewMessage::Story(story_message) => {
                self.story_state.update(story_message, game_state)
            }
            MainViewMessage::Location(location_message) => {
                self.location_state.update(configuration, location_message)
            }
        }
    }

    pub fn update_game_state(
        &mut self,
        configuration: Arc<RunConfiguration>,
        game_state: &GameState,
        message: &GameStateMessage,
    ) -> Command<Message> {
        Command::batch([self
            .location_state
            .update_game_state(configuration, game_state, message)])
    }

    pub fn view(&self, game_state: &GameState) -> Element<Message> {
        Container::new(
            Column::new()
                .spacing(5)
                .padding(5)
                .push(
                    Container::new(
                        Row::new()
                            .width(Length::Fill)
                            .padding(5)
                            .spacing(5)
                            .push(
                                Button::new( Text::new("Overview"))
                                    .on_press(
                                        MainViewMessage::SelectView(SelectedView::Overview).into(),
                                    )
                                    .style(if self.selected_view == SelectedView::Overview {
                                        SelectedButtonStyleSheet::style_sheet()
                                    } else {
                                        ButtonStyleSheet::style_sheet()
                                    }),
                            )
                            .push(
                                Button::new( Text::new("Location"))
                                    .on_press(
                                        MainViewMessage::SelectView(SelectedView::Location).into(),
                                    )
                                    .style(if self.selected_view == SelectedView::Location {
                                        SelectedButtonStyleSheet::style_sheet()
                                    } else {
                                        ButtonStyleSheet::style_sheet()
                                    }),
                            )
                            .push(
                                Button::new( Text::new("Actions"))
                                    .on_press(
                                        MainViewMessage::SelectView(SelectedView::ActionPicker)
                                            .into(),
                                    )
                                    .style(if self.selected_view == SelectedView::ActionPicker {
                                        SelectedButtonStyleSheet::style_sheet()
                                    } else {
                                        ButtonStyleSheet::style_sheet()
                                    }),
                            )
                            .push(
                                Button::new( Text::new("Quests"))
                                    .on_press(
                                        MainViewMessage::SelectView(SelectedView::Story).into(),
                                    )
                                    .style(if self.selected_view == SelectedView::Story {
                                        SelectedButtonStyleSheet::style_sheet()
                                    } else {
                                        ButtonStyleSheet::style_sheet()
                                    }),
                            )
                            .push(
                                Button::new( Text::new("Calendar"))
                                    .on_press(
                                        MainViewMessage::SelectView(SelectedView::Calendar).into(),
                                    )
                                    .style(if self.selected_view == SelectedView::Calendar {
                                        SelectedButtonStyleSheet::style_sheet()
                                    } else {
                                        ButtonStyleSheet::style_sheet()
                                    }),
                            ),
                    )
                    .style(FramedContainer),
                )
                .push(match self.selected_view {
                    SelectedView::Overview => self.overview_state.view(game_state),
                    SelectedView::Location => self.location_state.view(),
                    SelectedView::ActionPicker => self.action_picker_state.view(game_state),
                    SelectedView::Story => self.story_state.view(game_state),
                    SelectedView::Calendar => self.calendar_state.view(game_state),
                })
                .push(active_action_description(game_state))
                .push(ProgressBar::new(
                    0.0..=1.0,
                    game_state.current_action_progress(),
                )),
        )
        .padding(5)
        .style(FramedContainer)
        .into()
    }
}

impl From<MainViewMessage> for Message {
    fn from(message: MainViewMessage) -> Self {
        Message::Running(RunningMessage::MainView(message))
    }
}
