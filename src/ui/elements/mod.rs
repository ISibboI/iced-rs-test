use crate::game_state::character::Character;
use iced::alignment::{Horizontal, Vertical};
use iced::{Alignment, Column, Element, Length, Row, Space, Text};
use iced_native::widget::ProgressBar;

pub fn title<'a, T: 'a>(title: impl ToString) -> Column<'a, T> {
    Column::new()
        .push(Space::new(Length::Shrink, Length::Units(20)))
        .push(Text::new(title.to_string()).size(100))
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
