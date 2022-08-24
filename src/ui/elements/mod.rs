use crate::game_state::actions::{ActionInProgress, ACTION_FIGHT_MONSTERS};
use crate::game_state::character::CharacterAttributes;
use crate::game_state::currency::Currency;
use crate::game_state::event_log::GameEvent;
use crate::game_state::story::Story;
use crate::text_utils::a_or_an;
use crate::GameState;
use iced::alignment::{Horizontal, Vertical};
use iced::{scrollable, Alignment, Color, Column, Element, Length, Row, Scrollable, Space, Text};
use iced_native::widget::ProgressBar;
use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::VecDeque;

lazy_static! {
    pub static ref ERROR_COLOR: Color = Color::from_rgb8(220, 10, 10);
}

pub fn title<'a, T: 'a>(title: impl ToString) -> Column<'a, T> {
    Column::new()
        .push(Space::new(Length::Shrink, Length::Units(20)))
        .push(
            Text::new(title.to_string())
                .size(100)
                .horizontal_alignment(Horizontal::Center)
                .width(Length::Fill),
        )
        .push(Space::new(Length::Shrink, Length::Units(20)))
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
    for quest in story.active_quests.values().filter(|quest| !quest.hidden) {
        quest_column = quest_column
            .push(Text::new(&quest.title))
            .push(Text::new(&quest.description).size(16));
    }
    quest_column = quest_column.push(Text::new("Completed quests:").size(24));
    for quest in story
        .completed_quests
        .values()
        .filter(|quest| !quest.hidden)
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
    for event in game_state.log.iter_rev() {
        event_column = event_column.push(event_string(event, game_state));
    }
    Scrollable::new(scrollable_state)
        .scrollbar_width(20)
        .push(event_column)
}

pub fn event_string<'a, T: 'a>(event: &GameEvent, game_state: &GameState) -> Row<'a, T> {
    match event {
        GameEvent::Action(action) => completed_action_description(action, game_state),
    }
}

pub fn active_action_description<'a, T: 'a>(game_state: &GameState) -> Row<'a, T> {
    let current_action_currency_reward = game_state.current_action.currency_reward;
    let action_descriptor_row = Row::new().push(Text::new(&format!(
        "{} is currently {}{}",
        game_state.character.name,
        if game_state.current_action.action.name == ACTION_FIGHT_MONSTERS {
            let monster_name = game_state
                .current_action
                .monster
                .as_ref()
                .unwrap()
                .to_lowercase_string();
            let a = a_or_an(&monster_name);
            format!("fighting {a} {monster_name}")
        } else {
            game_state
                .current_action
                .action
                .verb_progressive
                .to_string()
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
    if !game_state.current_action.success {
        action_descriptor_row.push(Text::new(" (failure)").color(*ERROR_COLOR))
    } else {
        action_descriptor_row
    }
}

pub fn completed_action_description<'a, T: 'a>(
    action: &ActionInProgress,
    game_state: &GameState,
) -> Row<'a, T> {
    let current_action_currency_reward = action.currency_reward;
    let action_descriptor_row = Row::new().push(Text::new(&format!(
        "{} {}{}",
        game_state.character.name,
        if action.action.name == ACTION_FIGHT_MONSTERS {
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
