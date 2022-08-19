use crate::game_state::character::Character;
use crate::game_state::currency::Currency;
use iced::alignment::{Horizontal, Vertical};
use iced::{Alignment, Color, Column, Element, Length, Row, Space, Text};
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
    attribute: usize,
    attribute_progress: f64,
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
                        0.0..=Character::required_attribute_progress(attribute) as f32,
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

    let mut elements = if currency.gold() > 0 {
        VecDeque::from([gold, silver, copper])
    } else if currency.silver() > 0 {
        VecDeque::from([silver, copper])
    } else {
        VecDeque::from([copper])
    };

    if align_center {
        if elements.len() >= 2 {
            let first = elements.pop_front().unwrap();
            elements.push_front(
                first
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Right),
            );

            let last = elements.pop_back().unwrap();
            elements.push_back(
                last.width(Length::Fill)
                    .horizontal_alignment(Horizontal::Left),
            );
        } else {
            let element = elements.pop_front().unwrap();
            elements.push_front(
                element
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center),
            );
        }
    }

    let mut result = Row::new()
        .spacing(5)
        .align_items(Alignment::Center)
        .width(if align_center {
            Length::Fill
        } else {
            Length::Shrink
        });
    for element in elements {
        result = result.push(element);
    }
    result
}
