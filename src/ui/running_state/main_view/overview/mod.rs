use crate::game_state::character::CombatStyle;
use crate::ui::elements::{event_log, labelled_element, labelled_label, scrollable_quest_column};
use crate::ui::running_state::GameStateMessage;
use crate::ui::Message;
use crate::utils::ui::PickListContainer;
use crate::GameState;
use enum_iterator::all;
use iced::{Element, Length};
use iced::widget::{ Column, PickList, Row};

#[derive(Debug, Clone)]
pub struct OverviewState {
}

impl OverviewState {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn view(&self, game_state: &GameState) -> Element<Message> {
        let label_column_width = 160;

        let mut active_locations: Vec<_> = game_state.world.active_locations().collect();
        active_locations.sort_by_key(|location| location.state.activation_time().unwrap());
        let mut choosable_actions: Vec<_> = game_state.actions.list_choosable().collect();
        choosable_actions.sort_by_key(|action| &action.name);

        let action_column = Column::new()
            .width(Length::Shrink)
            .height(Length::Fill)
            .spacing(5)
            .padding(5)
            .push(labelled_element(
                "Selected action:",
                label_column_width,
                PickList::new(
                    choosable_actions
                        .iter()
                        .map(|action| PickListContainer::new(action.name.clone(), action.id))
                        .collect::<Vec<_>>(),
                    Some(PickListContainer::new(
                        game_state
                            .actions
                            .action(game_state.actions.selected_action)
                            .name
                            .clone(),
                        game_state.actions.selected_action,
                    )),
                    |action| GameStateMessage::ActionChanged(action.data).into(),
                ),
            ))
            .push(labelled_element(
                "Exploration location:",
                label_column_width,
                PickList::new(
                    active_locations
                        .iter()
                        .map(|location| PickListContainer::new(location.name.clone(), location.id))
                        .collect::<Vec<_>>(),
                    Some(PickListContainer::new(
                        game_state.world.selected_location().name.clone(),
                        game_state.world.selected_location().id,
                    )),
                    |location| GameStateMessage::ExplorationLocationChanged(location.data).into(),
                ),
            ))
            .push(labelled_element(
                "Combat style:",
                label_column_width,
                PickList::new(
                    all::<CombatStyle>().collect::<Vec<_>>(),
                    Some(game_state.character.selected_combat_style),
                    |combat_style| GameStateMessage::CombatStyleChanged(combat_style).into(),
                ),
            ))
            .push(labelled_label(
                "Damage per minute:",
                label_column_width,
                format!("{:.0}", game_state.character.damage_output()),
            ));

        Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(5)
            .padding(5)
            .push(
                Row::new()
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .spacing(5)
                    .padding(5)
                    .push(action_column)
                    .push(
                        scrollable_quest_column(
                            &game_state.story,
                            &game_state.triggers,
                        )
                        .width(Length::Units(300))
                        .height(Length::Fill),
                    )
                    .push(
                        event_log(game_state)
                            .width(Length::Units(300))
                            .height(Length::Fill),
                    ),
            )
            .into()
    }
}
