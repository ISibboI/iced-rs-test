use crate::game_state::character::CombatStyle;
use crate::game_state::player_actions::ACTION_EXPLORE;
use crate::ui::running_state::RunningMessage;
use crate::ui::style::{FramedContainer, RadioStyleSheet};
use crate::ui::Message;
use crate::GameState;
use enum_iterator::all;
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
        let selected_action = if game_state.actions.selected_action == ACTION_EXPLORE {
            None
        } else {
            Some(game_state.actions.selected_action)
        };

        for action in game_state
            .actions
            .list_choosable()
            .filter(|action| action != &ACTION_EXPLORE)
        {
            let action = game_state.actions.action(action);
            action_picker_column = action_picker_column.push(
                Radio::new(action.id, action.name.clone(), selected_action, |id| {
                    RunningMessage::ActionChanged(id).into()
                })
                .style(RadioStyleSheet),
            );
        }

        let mut location_picker_column = Column::new()
            .spacing(5)
            .padding(5)
            .push(Text::new("Explore").size(24));
        let selected_location = if game_state.actions.selected_action == ACTION_EXPLORE {
            Some(game_state.world.selected_location)
        } else {
            None
        };

        for location in game_state.world.active_locations() {
            location_picker_column = location_picker_column.push(
                Radio::new(
                    location.id,
                    location.name.clone(),
                    selected_location,
                    |id| RunningMessage::ActionChangedExplore(id).into(),
                )
                .style(RadioStyleSheet),
            );
        }

        let mut combat_style_picker_column = Column::new()
            .spacing(5)
            .padding(5)
            .push(Text::new("Combat style").size(24));
        let selected_combat_style = Some(game_state.character.selected_combat_style);

        for combat_style in all::<CombatStyle>() {
            combat_style_picker_column = combat_style_picker_column.push(
                Radio::new(
                    combat_style,
                    combat_style.to_string(),
                    selected_combat_style,
                    |combat_style| RunningMessage::CombatStyleChanged(combat_style).into(),
                )
                .style(RadioStyleSheet),
            );
        }

        rows = rows
            .push(Container::new(action_picker_column).style(FramedContainer))
            .push(Container::new(location_picker_column).style(FramedContainer))
            .push(Container::new(combat_style_picker_column).style(FramedContainer))
            .push(Space::new(Length::Fill, Length::Shrink));

        Container::new(rows)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(FramedContainer)
            .into()
    }
}
