

//iced
use iced::{
    widget::{
        button,
        button::Appearance,
    },
    Color,
    Background,
};

//Styles
#[derive(Debug, Clone, Copy, Default)]
pub struct LoginButtonStyle;

impl button::StyleSheet for LoginButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(132.0, 125.0, 87.0))),
            ..Default::default()
        }
    }
    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(132.0, 125.0, 87.0))),
            ..Appearance::default()
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(132.0, 125.0, 87.0))),
            ..Appearance::default()
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(132.0, 125.0, 87.0))),
            ..Appearance::default()
        }
    }

    
}