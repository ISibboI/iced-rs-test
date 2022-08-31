use crate::ui::running_state::main_view::action_picker::{ActionPickerMessage, ActionPickerState};
use crate::ui::running_state::main_view::overview::{OverviewMessage, OverviewState};
use crate::ui::running_state::RunningMessage;
use crate::ui::style::{ButtonStyleSheet, FramedContainer, SelectedButtonStyleSheet};
use crate::ui::Message;
use crate::{Configuration, GameState};
use iced::{button, Button, Column, Command, Container, Element, Row, Text};

mod action_picker;
mod overview;

#[derive(Debug, Clone)]
pub struct MainViewState {
    selected_view: SelectedView,
    overview_state: OverviewState,
    action_picker_state: ActionPickerState,

    overview_button: button::State,
    action_picker_button: button::State,
    calendar_button: button::State,
}

#[derive(Clone, Debug)]
pub enum MainViewMessage {
    SelectView(SelectedView),
    Overview(OverviewMessage),
    ActionPicker(ActionPickerMessage),
}

impl MainViewState {
    pub fn new() -> Self {
        Self {
            selected_view: SelectedView::Overview,
            overview_state: OverviewState::new(),
            action_picker_state: ActionPickerState::new(),

            overview_button: Default::default(),
            action_picker_button: Default::default(),
            calendar_button: Default::default(),
        }
    }

    pub fn update(
        &mut self,
        configuration: &Configuration,
        message: MainViewMessage,
    ) -> Command<Message> {
        match message {
            MainViewMessage::SelectView(selected_view) => self.selected_view = selected_view,
            MainViewMessage::Overview(overview_message) => {
                return self.overview_state.update(configuration, overview_message);
            }
            MainViewMessage::ActionPicker(action_picker_message) => {
                return self
                    .action_picker_state
                    .update(configuration, action_picker_message);
            }
        }

        Command::none()
    }

    pub fn view(&mut self, game_state: &GameState) -> Element<Message> {
        Container::new(
            Column::new()
                .push(
                    Container::new(
                        Row::new()
                            .padding(5)
                            .spacing(5)
                            .push(
                                Button::new(&mut self.overview_button, Text::new("Overview"))
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
                                Button::new(&mut self.action_picker_button, Text::new("Actions"))
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
                                Button::new(&mut self.calendar_button, Text::new("Calendar"))
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
                    SelectedView::ActionPicker => self.action_picker_state.view(game_state),
                    SelectedView::Calendar => todo!(),
                }),
        )
        .padding(5)
        .style(FramedContainer)
        .into()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SelectedView {
    Overview,
    ActionPicker,
    Calendar,
}

impl From<MainViewMessage> for Message {
    fn from(message: MainViewMessage) -> Self {
        Message::Running(RunningMessage::MainView(message))
    }
}
