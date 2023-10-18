//iced
use iced::widget::{Button, Text, Column, Container};
use iced::{Element, Length};
use crate::{AppMessage, Views};

pub struct LoginPage;

#[derive(Debug, Clone, Copy)]
pub enum LoginPageMessage{
    //Messages for Login page
    AttemptLogin,
    Error
}
impl LoginPage {
    
    pub fn new() -> Self {
        LoginPage
    }

    pub fn update(&mut self, message: LoginPageMessage) {
        match message {
            //TODO
            _ => ()
        }
    }
    pub fn view(&self) -> Element<AppMessage> {
        //placeholder for login screen
        let placeholder = Text::new("This is a placeholder for a login screen.");

        Container::new(placeholder)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

