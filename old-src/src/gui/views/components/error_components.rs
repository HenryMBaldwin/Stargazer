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

// impl<'a> From<Theme> for Box<dyn text::StyleSheet + 'a> {
//     fn from(theme: Theme) -> Self {
        
//     }
// }
pub struct ErrorText;

impl text::StyleSheet for ErrorText {
    type Style = iced::Theme;
    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        
        text::Appearance{
            color: Some(ERROR_RED)
        }
    }
}

//Text_Input

// pub struct ErrorTextInput;

// impl text_input::StyleSheet for ErrorTextInput {
//     type Style = iced::Theme;
//     fn active(&self, _style: &Self::Style) -> text_input::Appearance {
//         text_input::Appearance { 
//             border_color: ERROR_RED,
//             ..text_input::Appearance::default()
//         }
//     }

//     fn focused(&self, style: &Self::Style) -> text_input::Appearance {
//         todo!()
//     }

//     fn placeholder_color(&self, style: &Self::Style) -> Color {
//         todo!()
//     }

//     fn value_color(&self, style: &Self::Style) -> Color {
//         todo!()
//     }

//     fn disabled_color(&self, style: &Self::Style) -> Color {
//         todo!()
//     }

//     fn selection_color(&self, style: &Self::Style) -> Color {
//         todo!()
//     }

//     fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
//         todo!()
//     }
// }