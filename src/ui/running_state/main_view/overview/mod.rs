use crate::game_state::combat::CombatStyle;
use crate::ui::elements::{event_log, labelled_element, labelled_label, scrollable_quest_column};
use crate::ui::running_state::RunningMessage;
use crate::ui::Message;
use crate::GameState;
use enum_iterator::all;
use iced::{pick_list, scrollable, Column, Element, Length, PickList, Row};

#[derive(Debug, Clone)]
pub struct OverviewState {
    action_picker_state: pick_list::State<String>,
    combat_style_picker_state: pick_list::State<CombatStyle>,
    combat_location_picker_state: pick_list::State<String>,
    quest_column_scrollable_state: scrollable::State,
    event_log_scrollable_state: scrollable::State,
}

impl OverviewState {
    pub fn new() -> Self {
        Self {
            action_picker_state: Default::default(),
            combat_style_picker_state: Default::default(),
            combat_location_picker_state: Default::default(),
            quest_column_scrollable_state: Default::default(),
            event_log_scrollable_state: Default::default(),
        }
    }

    pub fn view(&mut self, game_state: &GameState) -> Element<Message> {
        let label_column_width = 160;

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
                        .map(|action| game_state.actions.action(action).name.clone())
                        .collect::<Vec<_>>(),
                    Some(
                        game_state
                            .actions
                            .action(game_state.actions.selected_action)
                            .name
                            .clone(),
                    ),
                    |action| RunningMessage::ActionChangedString(action).into(),
                ),
            ))
            .push(labelled_element(
                "Combat style:",
                label_column_width,
                PickList::new(
                    &mut self.combat_style_picker_state,
                    all::<CombatStyle>().collect::<Vec<_>>(),
                    Some(game_state.selected_combat_style.clone()),
                    |combat_style| RunningMessage::CombatStyleChanged(combat_style).into(),
                ),
            ))
            .push(labelled_label(
                "Damage per minute:",
                label_column_width,
                format!("{:.0}", game_state.damage_output()),
            ))
            .push(labelled_element(
                "Combat location:",
                label_column_width,
                PickList::new(
                    &mut self.combat_location_picker_state,
                    game_state
                        .list_feasible_locations()
                        .map(|location| location.name.clone())
                        .collect::<Vec<_>>(),
                    Some(game_state.selected_combat_location.clone()),
                    |combat_location| RunningMessage::CombatLocationChanged(combat_location).into(),
                ),
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
