use std::sync::Arc;

use iced::{
    Element,
    widget::{Container, Column, Text, button, text},
    Length, Command, alignment::Horizontal
};
use crate::{AppMessage, orion_api};

use orion_api::OrionAPI;



#[derive(Debug, Clone)]
pub enum SuccessPageMessage {
    //will use the success page to test functions of the Orion API after authed.
    //This command will be a catch all to reuse upon testing different things
    ExecCommand
}
pub struct SuccessPage {
    oapi: Arc<OrionAPI>,
}

impl SuccessPage {
    pub fn new(oapi: Arc<OrionAPI>) -> Self {
        Self {
            oapi
        }
    }

    pub fn update(&mut self, message: SuccessPageMessage) -> Command<AppMessage> {
        match message {
            SuccessPageMessage::ExecCommand => {
                let oapi = self.oapi.clone();
                Command::perform(async move {
                    oapi.print_auth().await;
                },
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