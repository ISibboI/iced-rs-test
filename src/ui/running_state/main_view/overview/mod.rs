use crate::game_state::character::CombatStyle;
use crate::game_state::player_actions::PlayerActionId;
use crate::game_state::world::locations::LocationId;
use crate::ui::elements::{event_log, labelled_element, labelled_label, scrollable_quest_column};
use crate::ui::running_state::RunningMessage;
use crate::ui::Message;
use crate::utils::ui::PickListContainer;
use crate::GameState;
use enum_iterator::all;
use iced::{pick_list, scrollable, Column, Element, Length, PickList, Row};

#[derive(Debug, Clone)]
pub struct OverviewState {
    action_picker_state: pick_list::State<PickListContainer<PlayerActionId>>,
    exploration_location_picker_state: pick_list::State<PickListContainer<LocationId>>,
    combat_style_picker_state: pick_list::State<CombatStyle>,
    quest_column_scrollable_state: scrollable::State,
    event_log_scrollable_state: scrollable::State,
}

impl OverviewState {
    pub fn new() -> Self {
        Self {
            action_picker_state: Default::default(),
            exploration_location_picker_state: Default::default(),
            combat_style_picker_state: Default::default(),
            quest_column_scrollable_state: Default::default(),
            event_log_scrollable_state: Default::default(),
        }
    }

    pub fn view(&mut self, game_state: &GameState) -> Element<Message> {
        let label_column_width = 160;

        let mut active_locations: Vec<_> = game_state.world.active_locations().collect();
        active_locations.sort_by_key(|location| location.state.activation_time().unwrap());

        let action_column = Column::new()
            .width(Length::Shrink)
            .height(Length::Fill)
            .spacing(5)
            .padding(5)
            .push(labelled_element(
                "Selected action:",
                label_column_width,
                PickList::new(
                    &mut self.action_picker_state,
                    game_state
                        .actions
                        .list_choosable()
                        .map(|action| {
                            PickListContainer::new(
                                game_state.actions.action(action).name.clone(),
                                action,
                            )
                        })
                        .collect::<Vec<_>>(),
                    Some(PickListContainer::new(
                        game_state
                            .actions
                            .action(game_state.actions.selected_action)
                            .name
                            .clone(),
                        game_state.actions.selected_action,
                    )),
                    |action| RunningMessage::ActionChanged(action.data).into(),
                ),
            ))
            .push(labelled_element(
                "Exploration location:",
                label_column_width,
                PickList::new(
                    &mut self.exploration_location_picker_state,
                    active_locations
                        .iter()
                        .map(|location| PickListContainer::new(location.name.clone(), location.id))
                        .collect::<Vec<_>>(),
                    Some(PickListContainer::new(
                        game_state.world.selected_location().name.clone(),
                        game_state.world.selected_location().id,
                    )),
                    |location| RunningMessage::ExplorationLocationChanged(location.data).into(),
                ),
            ))
            .push(labelled_element(
                "Combat style:",
                label_column_width,
                PickList::new(
                    &mut self.combat_style_picker_state,
                    all::<CombatStyle>().collect::<Vec<_>>(),
                    Some(game_state.character.selected_combat_style),
                    |combat_style| RunningMessage::CombatStyleChanged(combat_style).into(),
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
                            &mut self.quest_column_scrollable_state,
                        )
                        .width(Length::Units(300))
                        .height(Length::Fill),
                    )
                    .push(
                        event_log(game_state, &mut self.event_log_scrollable_state)
                            .width(Length::Units(300))
                            .height(Length::Fill),
                    ),
            )
            .into()
    }
}
