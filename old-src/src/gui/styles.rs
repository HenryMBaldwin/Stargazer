//(132.0, 125.0, 87.0)
//not going to use this for now
//iced
// use iced::widget::{
//         button,
//         text,
//         text_input, TextInput,
//     };

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum Theme {
//     Light,
//     Err
// }

// impl Theme {
//     pub const ALL: [Theme; 2] = [Theme::Light, Theme::Err];
// }

// //macro to implement stylesheet from them for each widget
// macro_rules! impl_from_theme_for_stylesheet {
//     ($($widget:ident => { Light: $light_style:expr, Err: $err_style:expr }),+) => {
//         $(
//             impl<'a> From<Theme> for Box<dyn $widget::StyleSheet<Style = iced::Theme> + 'a> {
//                 fn from(theme: Theme) -> Self {
//                     match theme {
//                         Theme::Light => $light_style.into(),
//                         Theme::Err => $err_style.into(),
//                     }
//                 }
//             }
//         )+
//     };
// }


// //cont
// impl_from_theme_for_stylesheet!(
//     button => {Light: light::Button, Err: Default::default()},
//     text => {Light: Default::default(), Err: err::Text},
//     text_input => {Light: Default::default(), Err: err::TextInput}
    
// );

//light theme
mod light {
    use iced::{
        widget::{
            button,
            text,
            text_input
        },
        Color
    };

    
    //Button
    pub struct GoldButton;

    impl button::StyleSheet for GoldButton {
        type Style = iced::Theme;
        fn active(&self, style: &Self::Style) -> button::Appearance {
            button::Appearance {
                background: Some(iced::Background::Color(Color::from_rgb(175.0, 167.0, 120.0))),
                text_color: Color::WHITE,
                ..button::Appearance::default()
            }
        }

        fn hovered(&self, style: &Self::Style) -> button::Appearance {
            button::Appearance { 
                background: Some(iced::Background::Color(Color::from_rgb(132.0, 125.0, 87.0))),
                text_color: Color::WHITE,
            ..button::Appearance::default()
            }
        }

    
    }
}

//error theme
mod err {
    use iced::{
        widget::{
            button,
            text,
            text_input
        },
        Color
    };

    const ERROR_RED : Color = Color::from_rgb(204.0, 2.0, 2.0);
    //Text
    pub struct Text;

    impl text::StyleSheet for Text {
        type Style = iced::Theme;
        fn appearance(&self, style: Self::Style) -> text::Appearance {
            
            text::Appearance{
                color: Some(ERROR_RED)
            }
        }
    }

    //Text_Input

    pub struct TextInput;

    impl text_input::StyleSheet for TextInput {
        type Style = iced::Theme;
        fn active(&self, style: &Self::Style) -> text_input::Appearance {
            text_input::Appearance { 
                border_color: ERROR_RED,
                ..text_input::Appearance::default()
            }
        }
    }


}
