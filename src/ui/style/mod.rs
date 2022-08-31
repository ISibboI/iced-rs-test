use iced::{button, container, Background, Color, Vector};

pub const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};
pub const LIGHT_GREY: Color = Color {
    r: 0.8,
    g: 0.8,
    b: 0.8,
    a: 1.0,
};
pub const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

pub struct FramedContainer;

impl container::StyleSheet for FramedContainer {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: None,
            background: None,
            border_radius: 5.0,
            border_width: 1.0,
            border_color: Color::from_rgb(0.0, 0.0, 0.0),
        }
    }
}

pub struct ButtonStyleSheet;

impl button::StyleSheet for ButtonStyleSheet {
    fn active(&self) -> button::Style {
        button::Style {
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
    pub fn style_sheet() -> Box<dyn button::StyleSheet> {
        Box::new(Self)
    }
}

pub struct SelectedButtonStyleSheet;

impl button::StyleSheet for SelectedButtonStyleSheet {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some(Background::Color(LIGHT_GREY)),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: BLACK,
            text_color: BLACK,
        }
    }
}

impl SelectedButtonStyleSheet {
    pub fn style_sheet() -> Box<dyn button::StyleSheet> {
        Box::new(Self)
    }
}
