use std::sync::Arc;

use iced::{
    Element,
    widget::{Container, Column, Text, button, text},
    Length, Command, alignment::Horizontal
};
use crate::{AppMessage};

#[derive(Debug, Clone)]
pub enum SuccessPageMessage {
    //will use the success page to test functions of the Orion API after authed.
    //This command will be a catch all to reuse upon testing different things
    ExecCommand
}
pub struct SuccessPage;


impl SuccessPage {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: SuccessPageMessage) -> Command<AppMessage> {
        match message {
            SuccessPageMessage::ExecCommand => {
                Command::perform(async move {},
                |_|{
                    AppMessage::NoneMsg
                })
            }
        }
    }
    pub fn view(&self) -> Element<SuccessPageMessage> {
        let label = Text::new("Wow you successfully logged in thats very cool");

        let button = button(text("Test").horizontal_alignment(Horizontal::Center))
            .on_press(SuccessPageMessage::ExecCommand)
            .width(Length::Fixed(300.0));

        let col = Column::new()
            .push(label)
            .push(button);
        
        Container::new(col)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}