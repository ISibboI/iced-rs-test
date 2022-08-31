use crate::game_state::actions::{DerefActionInProgress, ACTION_FIGHT_MONSTERS};
use crate::game_state::character::CharacterAttributes;
use crate::game_state::currency::Currency;
use crate::game_state::event_log::{GameEvent, GameEventKind};
use crate::game_state::story::Story;
use crate::game_state::time::GameTime;
use crate::utils::text::{a_or_an, ordinal_suffix};
use crate::{GameState, TITLE};
use iced::alignment::{Horizontal, Vertical};
use iced::{
    scrollable, Alignment, Color, Column, Container, Element, Length, Row, Scrollable, Space, Text,
};
use iced_native::widget::ProgressBar;
use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::VecDeque;

lazy_static! {
    pub static ref ERROR_COLOR: Color = Color::from_rgb8(220, 10, 10);
}

pub fn title<'a, T: 'a>() -> Container<'a, T> {
    Container::new(
        Column::new()
            .push(Space::new(Length::Shrink, Length::Units(20)))
            .push(
                Text::new(TITLE)
                    .size(100)
                    .horizontal_alignment(Horizontal::Center)
                    .width(Length::Fill),
            )
            .push(Space::new(Length::Shrink, Length::Units(20))),
    )
    // TODO .style()
}

pub fn labelled_element<'a, T: 'a, E: Into<Element<'a, T>>>(
    label: impl ToString,
    label_column_width: u16,
    element: E,
) -> Row<'a, T> {
    Row::new()
        .spacing(5)
        .height(Length::Units(20 + 2 * 5))
        .push(
            Text::new(label.to_string())
                .vertical_alignment(Vertical::Center)
                .width(Length::Units(label_column_width))
                .height(Length::Fill),
        )
        .push(element)
}

pub fn labelled_label<'a, T: 'a>(
    label: impl ToString,
    label_column_width: u16,
    element: impl ToString,
) -> Row<'a, T> {
    Row::new()
        .spacing(5)
        .height(Length::Units(20 + 2 * 5))
        .push(
            Text::new(label.to_string())
                .vertical_alignment(Vertical::Center)
                .width(Length::Units(label_column_width))
                .height(Length::Fill),
        )
        .push(
            Text::new(element.to_string())
                .vertical_alignment(Vertical::Center)
                .height(Length::Fill),
        )
}

pub fn attribute<'a, T: 'a>(
    name: impl AsRef<str>,
    attribute: u64,
    attribute_progress: u64,
) -> Row<'a, T> {
    let attribute_progress_bar_width = 50;
    let attribute_progress_bar_height = 10;

    Row::new()
        .spacing(5)
        .push(
            Text::new(&format!("{} {}", name.as_ref(), attribute))
                .horizontal_alignment(Horizontal::Left)
                .width(Length::Fill),
        )
        .push(
            Column::new()
                .align_items(Alignment::Start)
                .push(Space::new(Length::Shrink, Length::Units(5)))
                .push(
                    ProgressBar::new(
                        0.0..=CharacterAttributes::required_attribute_progress(attribute) as f32,
                        attribute_progress as f32,
                    )
                    .width(Length::Units(attribute_progress_bar_width))
                    .height(Length::Units(attribute_progress_bar_height)),
                ),
        )
}

pub fn currency<'a, T: 'a>(currency: Currency, align_center: bool) -> Row<'a, T> {
    let copper_color = Color::from_rgb8(184, 115, 51);
    let silver_color = Color::from_rgb8(171, 175, 183);
    let gold_color = Color::from_rgb8(212, 175, 55);

    let gold = Text::new(format!("{}g", currency.gold())).color(gold_color);
    let silver = Text::new(format!("{}s", currency.silver_of_gold())).color(silver_color);
    let copper = Text::new(format!("{}c", currency.copper_of_silver())).color(copper_color);

    let elements = if currency.gold() > 0 {
        VecDeque::from([gold, silver, copper])
    } else if currency.silver() > 0 {
        VecDeque::from([silver, copper])
    } else {
        VecDeque::from([copper])
    };

    let mut result = Row::new()
        .spacing(5)
        .align_items(Alignment::Center)
        .width(if align_center {
            Length::Fill
        } else {
            Length::Shrink
        });
    if align_center {
        result = result.push(Space::new(Length::Fill, Length::Shrink));
    }
    for element in elements {
        result = result.push(element);
    }
    if align_center {
        result = result.push(Space::new(Length::Fill, Length::Shrink));
    }
    result
}

pub fn scrollable_quest_column<'a, T: 'a>(
    story: &Story,
    scrollable_state: &'a mut scrollable::State,
) -> Scrollable<'a, T> {
    let mut quest_column = Column::new()
        .width(Length::Shrink)
        .height(Length::Shrink)
        .spacing(5)
        .padding(5)
        .push(Text::new("Active quests:").size(24));
    for quest in story
        .active_quests_by_activation_time
        .iter()
        .rev()
        .filter_map(|(_, id)| {
            let quest = story.active_quests.get(id).unwrap();
            if quest.hidden {
                None
            } else {
                Some(quest)
            }
        })
    {
        quest_column = quest_column
            .push(Text::new(&quest.title))
            .push(Text::new(&quest.description).size(16));
    }

    quest_column = quest_column.push(Text::new("Completed quests:").size(24));
    for quest in story
        .completed_quests_by_completion_time
        .iter()
        .rev()
        .filter_map(|(_, id)| {
            let quest = story.completed_quests.get(id).unwrap();
            if quest.hidden {
                None
            } else {
                Some(quest)
            }
        })
    {
        quest_column = quest_column
            .push(Text::new(&quest.title))
            .push(Text::new(&quest.description).size(16));
    }

    Scrollable::new(scrollable_state)
        .scrollbar_width(20)
        .push(quest_column)
}

pub fn event_log<'a, T: 'a>(
    game_state: &GameState,
    scrollable_state: &'a mut scrollable::State,
) -> Scrollable<'a, T> {
    let mut event_column = Column::new()
        .width(Length::Shrink)
        .height(Length::Shrink)
        .spacing(5)
        .padding(5);

    if let Some(event) = game_state.log.iter_rev().next() {
        let mut last_date = event.time.floor_day();
        for event in game_state.log.iter_rev() {
            if last_date.days() != event.time.days() {
                event_column = event_column.push(date_without_era(last_date));
                last_date = event.time.floor_day();
            }
            event_column = event_column.push(event_string(event, game_state));
        }
        event_column = event_column.push(date_without_era(last_date));
    }

    Scrollable::new(scrollable_state)
        .scrollbar_width(20)
        .push(event_column)
}

pub fn event_string<'a, T: 'a>(event: &GameEvent, game_state: &GameState) -> Row<'a, T> {
    match &event.kind {
        GameEventKind::Action(action) => {
            completed_action_description(&action.resolve(&game_state.actions), game_state)
        }
    }
}

pub fn active_action_description<'a, T: 'a>(game_state: &GameState) -> Row<'a, T> {
    let current_action = game_state.actions.in_progress();
    let current_action_currency_reward = current_action.currency_reward;
    let action_descriptor_row = Row::new().push(Text::new(&format!(
        "{} is currently {}{}",
        game_state.character.name,
        if current_action.action.id == ACTION_FIGHT_MONSTERS {
            let monster_name = current_action
                .monster
                .as_ref()
                .unwrap()
                .to_lowercase_string();
            let a = a_or_an(&monster_name);
            format!("fighting {a} {monster_name}")
        } else {
            current_action.action.verb_progressive.to_string()
        },
        match current_action_currency_reward.cmp(&Currency::zero()) {
            Ordering::Less => " costing him ",
            Ordering::Equal => "",
            Ordering::Greater => " earning ",
        },
    )));
    let action_descriptor_row = if current_action_currency_reward != Currency::zero() {
        action_descriptor_row.push(currency(current_action_currency_reward.abs(), false))
    } else {
        action_descriptor_row
    };
    if !current_action.success {
        action_descriptor_row.push(Text::new(" (failure)").color(*ERROR_COLOR))
    } else {
        action_descriptor_row
    }
}

pub fn completed_action_description<'result, 'action, T: 'result>(
    action: &DerefActionInProgress<'action>,
    game_state: &GameState,
) -> Row<'result, T> {
    let current_action_currency_reward = action.currency_reward;
    let action_descriptor_row = Row::new().push(Text::new(&format!(
        "{} {}{}",
        game_state.character.name,
        if action.action.id == ACTION_FIGHT_MONSTERS {
            let monster_name = action.monster.as_ref().unwrap().to_lowercase_string();
            let a = a_or_an(&monster_name);
            format!("fought {a} {monster_name}")
        } else {
            action.action.verb_simple_past.to_string()
        },
        match current_action_currency_reward.cmp(&Currency::zero()) {
            Ordering::Less => " costing him ",
            Ordering::Equal => "",
            Ordering::Greater => " earning ",
        },
    )));
    let action_descriptor_row = if current_action_currency_reward != Currency::zero() {
        action_descriptor_row.push(currency(current_action_currency_reward.abs(), false))
    } else {
        action_descriptor_row
    };
    if !action.success {
        action_descriptor_row.push(Text::new(" (failure)").color(*ERROR_COLOR))
    } else {
        action_descriptor_row
    }
}

pub fn clock_time(time: GameTime) -> Text {
    Text::new(&format!(
        "{:02}:{:02}",
        time.hour_of_day(),
        time.minute_of_hour(),
    ))
}

pub fn date(time: GameTime) -> Text {
    Text::new(&format!(
        "{}, {} of {}, {}{} year of the {} era",
        time.day_of_week_str(),
        time.day_of_month_str_ord(),
        time.month_of_year_str_common(),
        time.years(),
        ordinal_suffix(time.years()),
        time.era_str(),
    ))
}

pub fn date_without_era(time: GameTime) -> Text {
    Text::new(&format!(
        "{}, {} of {}, {}",
        time.day_of_week_str(),
        time.day_of_month_str_ord(),
        time.month_of_year_str_common(),
        time.years(),
    ))
}
