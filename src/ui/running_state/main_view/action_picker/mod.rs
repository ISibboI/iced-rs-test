use crate::ui::running_state::RunningMessage;
use crate::ui::style::{FramedContainer, RadioStyleSheet};
use crate::ui::Message;
use crate::GameState;
use iced::{Column, Container, Element, Length, Radio, Row, Space, Text};

#[derive(Debug, Clone)]
pub struct ActionPickerState {}

impl ActionPickerState {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&mut self, game_state: &GameState) -> Element<Message> {
        let mut rows = Row::new().spacing(5).padding(5);
        let mut action_picker_column = Column::new()
            .spacing(5)
            .padding(5)
            .push(Text::new("Action").size(24));

        for action in game_state.actions.list_choosable() {
            let action = game_state.actions.action(action);
            action_picker_column = action_picker_column.push(
                Radio::new(
                    action.id,
                    action.name.clone(),
                    Some(game_state.actions.selected_action),
                    |id| RunningMessage::ActionChanged(id).into(),
                )
                .style(RadioStyleSheet),
            );
        }

        rows = rows
            .push(Container::new(action_picker_column).style(FramedContainer))
            .push(Space::new(Length::Fill, Length::Shrink));

        Container::new(rows)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(FramedContainer)
            .into()
    }
}
