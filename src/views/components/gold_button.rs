use iced::{
        widget::{
            button,
            text,
            text_input
        },
        Color
    };

    

pub struct GoldButton;

impl button::StyleSheet for GoldButton {
    type Style = iced::Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb(175.0, 167.0, 120.0))),
            text_color: Color::WHITE,
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance { 
            background: Some(iced::Background::Color(Color::from_rgb(132.0, 125.0, 87.0))),
            text_color: Color::WHITE,
        ..button::Appearance::default()
        }
    }
}