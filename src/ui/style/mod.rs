use iced::{application, Background, Color, Vector};
use iced::widget::{button, container, radio, text};

pub const WHITE: Color = Color::from_rgb(1.0, 1.0, 1.0);
pub const LIGHT_GREY: Color = Color::from_rgb(0.9, 0.9, 0.9);
pub const GREY: Color = Color::from_rgb(0.8, 0.8, 0.8);
pub const BLACK: Color = Color::from_rgb(0.0, 0.0, 0.0);
pub const RED: Color = Color::from_rgb(0.9, 0.1, 0.1);
pub const ERROR_COLOR: Color = Color::from_rgb8(220, 10, 10);

#[derive(Default)]
pub struct ApplicationStyleSheet;

impl application::StyleSheet for ApplicationStyleSheet {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: WHITE,
            text_color: BLACK,
        }
    }
}

pub struct RedText;

impl text::StyleSheet for RedText {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        text::Appearance {
            color: Some(RED),
        }
    }
}

pub struct FramedContainer;

impl container::StyleSheet for FramedContainer {
    type Style = ();

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {text_color: None,
            background: None,
            border_radius: 5.0,
            border_width: 1.0,
            border_color: Color::from_rgb(0.0, 0.0, 0.0),
        }
    }
}

pub struct ColoredFramedContainer {
    pub border_color: Color,
}

impl ColoredFramedContainer {
    pub fn new(border_color: Color) -> Self {
        Self { border_color }
    }
}

impl container::StyleSheet for ColoredFramedContainer {
    type Style = ();

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: None,
            background: None,
            border_radius: 5.0,
            border_width: 1.0,
            border_color: self.border_color,
        }
    }
}

pub struct ButtonStyleSheet;

impl button::StyleSheet for ButtonStyleSheet {
    type Style = ();

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some(Background::Color(WHITE)),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: BLACK,
            text_color: BLACK,
        }
    }
}

impl ButtonStyleSheet {
    pub fn style_sheet() -> Box<dyn button::StyleSheet<Style = ()>> {
        Box::new(Self)
    }
}

pub struct SelectedButtonStyleSheet;

impl button::StyleSheet for SelectedButtonStyleSheet {
    type Style = ();

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some(Background::Color(GREY)),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: BLACK,
            text_color: BLACK,
        }
    }
}

impl SelectedButtonStyleSheet {
    pub fn style_sheet() -> Box<dyn button::StyleSheet<Style = ()>> {
        Box::new(Self)
    }
}

pub struct RadioStyleSheet;

impl radio::StyleSheet for RadioStyleSheet {
    type Style = ();

    fn active(&self, _style: &Self::Style, _is_selected: bool) -> radio::Appearance {
        radio::Appearance {
            background: Background::Color(LIGHT_GREY),
            dot_color: BLACK,
            border_width: 0.5,
            border_color: BLACK,
            text_color: None,
        }
    }

    fn hovered(&self, _style: &Self::Style, _is_selected: bool) -> radio::Appearance {
        radio::Appearance {
            background: Background::Color(GREY),
            dot_color: BLACK,
            border_width: 0.5,
            border_color: BLACK,
            text_color: None,
        }
    }
}
