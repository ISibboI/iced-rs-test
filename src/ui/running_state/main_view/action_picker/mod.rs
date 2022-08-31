use crate::game_state::combat::CombatStyle;
use crate::ui::elements::{
    active_action_description, event_log, labelled_element, labelled_label, scrollable_quest_column,
};
use crate::ui::running_state::RunningMessage;
use crate::ui::style::FramedContainer;
use crate::ui::Message;
use crate::{Configuration, GameState};
use enum_iterator::all;
use iced::{
    radio, scrollable, Column, Command, Container, Element, Length, PickList, ProgressBar, Radio,
    Row,
};

#[derive(Debug, Clone)]
pub struct ActionPickerState {}

#[derive(Clone, Debug)]
pub enum ActionPickerMessage {}

impl ActionPickerState {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(
        &mut self,
        _configuration: &Configuration,
        _message: ActionPickerMessage,
    ) -> Command<Message> {
        Command::none()
    }

    pub fn view(&mut self, game_state: &GameState) -> Element<Message> {
        let mut action_picker_column = Column::new().spacing(5).padding(5);
        /*for action in game_state.list_feasible_actions() {
            action_picker_column = action_picker_column.push(Radio::new(action.name.clone(), action.name.clone(), game_state.selected_action == action.name));
        }*/

        Container::new(action_picker_column)
            .style(FramedContainer)
            .into()
    }
}
