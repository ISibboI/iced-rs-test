use crate::game_state::character::CharacterAttributes;
use crate::game_state::currency::Currency;
use crate::game_state::story::Story;
use iced::alignment::{Horizontal, Vertical};
use iced::{scrollable, Alignment, Color, Column, Element, Length, Row, Scrollable, Space, Text};
use iced_native::widget::ProgressBar;
use std::collections::VecDeque;

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
    for quest in story.active_quests.values() {
        quest_column = quest_column
            .push(Text::new(&quest.title))
            .push(Text::new(&quest.description).size(16));
    }
    quest_column = quest_column.push(Text::new("Completed quests:").size(24));
    for quest in story.completed_quests.values() {
        quest_column = quest_column
            .push(Text::new(&quest.title))
            .push(Text::new(&quest.description).size(16));
    }
    Scrollable::new(scrollable_state)
        .scrollbar_width(20)
        .push(quest_column)
}
