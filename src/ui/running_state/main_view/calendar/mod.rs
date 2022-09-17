use crate::game_state::time::{GameTime, DAYS_PER_MONTH, FIRST_DAY_OF_MONTH};
use crate::ui::elements::{date, year_of_era};
use crate::ui::running_state::main_view::MainViewMessage;
use crate::ui::running_state::RunningMessage;
use crate::ui::style::FramedContainer;
use crate::ui::Message;
use crate::GameState;
use iced::alignment::{Horizontal, Vertical};
use iced::{
    button, Alignment, Button, Color, Column, Command, Container, Element, Length, Row, Space, Text,
};

#[derive(Debug, Clone)]
pub struct CalendarState {
    minus_button_state: button::State,
    plus_button_state: button::State,
    current_year: i128,
}

#[derive(Debug, Clone)]
pub enum CalendarMessage {
    PlusButtonPressed,
    MinusButtonPressed,
}

impl CalendarState {
    pub fn new(game_state: &GameState) -> Self {
        Self {
            minus_button_state: Default::default(),
            plus_button_state: Default::default(),
            current_year: game_state.current_time.years(),
        }
    }

    pub fn update(&mut self, message: CalendarMessage) -> Command<Message> {
        match message {
            CalendarMessage::PlusButtonPressed => self.current_year += 1,
            CalendarMessage::MinusButtonPressed => self.current_year = 0.max(self.current_year - 1),
        }

        Command::none()
    }

    pub fn view(&mut self, game_state: &GameState) -> Element<Message> {
        let day_width = Length::Units(27);
        let day_height = Length::Units(20);
        let months_per_row = 6;

        let months = (0..12).map(|month| {
            let first_day_of_month = GameTime::from_years(self.current_year)
                + GameTime::from_days(FIRST_DAY_OF_MONTH[month]);
            let first_day_of_week = first_day_of_month.day_of_week();
            let mut column = Column::new()
                .spacing(5)
                .padding(5)
                .align_items(Alignment::Fill)
                .push(Text::new(first_day_of_month.month_of_year_str_common()));
            let mut current_row = Row::new().align_items(Alignment::Fill);

            for _ in 0..first_day_of_week {
                current_row = current_row.push(Space::new(day_width, day_height));
            }

            let mut current_week = first_day_of_month.weeks();
            for day in 0..DAYS_PER_MONTH[month] {
                let current_day = first_day_of_month + GameTime::from_days(day);
                if current_week < current_day.weeks() {
                    current_week = current_day.weeks();
                    column = column.push(current_row);
                    current_row = Row::new().align_items(Alignment::Fill);
                }

                current_row = current_row.push(
                    Text::new(&format!("{}", day + 1))
                        .width(day_width)
                        .height(day_height)
                        .horizontal_alignment(Horizontal::Center)
                        .color(if current_day.days() == game_state.current_time.days() {
                            Color::from_rgb(0.9, 0.05, 0.1)
                        } else {
                            Color::BLACK
                        }),
                );
            }

            column = column.push(current_row);
            Container::new(column).style(FramedContainer)
        });

        let plus_minus_size = 20;
        let year_selector = Row::new()
            .spacing(5)
            .padding(5)
            .push(
                Button::new(
                    &mut self.minus_button_state,
                    Text::new("-")
                        .size(plus_minus_size)
                        .horizontal_alignment(Horizontal::Center)
                        .vertical_alignment(Vertical::Center),
                )
                .padding(0)
                .width(Length::Units(plus_minus_size))
                .on_press(CalendarMessage::MinusButtonPressed.into()),
            )
            .push(
                year_of_era(self.current_year)
                    .width(Length::Units(200))
                    .horizontal_alignment(Horizontal::Center),
            )
            .push(
                Button::new(
                    &mut self.plus_button_state,
                    Text::new("+")
                        .size(plus_minus_size)
                        .horizontal_alignment(Horizontal::Center)
                        .vertical_alignment(Vertical::Center),
                )
                .padding(0)
                .width(Length::Units(plus_minus_size))
                .on_press(CalendarMessage::PlusButtonPressed.into()),
            )
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(date(game_state.current_time));

        let mut column = Column::new().spacing(5).padding(5).push(year_selector);
        let mut current_row = Row::new().spacing(5).padding(5);

        for (month, container) in months.enumerate() {
            if month % months_per_row == 0 && month > 0 {
                column = column.push(current_row);
                current_row = Row::new().spacing(5).padding(5);
            }

            current_row = current_row.push(container)
        }
        column = column.push(current_row);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(FramedContainer)
            .into()
    }
}

impl From<CalendarMessage> for Message {
    fn from(message: CalendarMessage) -> Self {
        Message::Running(RunningMessage::MainView(MainViewMessage::Calendar(message)))
    }
}
