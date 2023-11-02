use iced::{
    Element,
    Widget::{Container, Column, Text},
    Length
};

use crate::AppMessage;

pub struct SuccessPage;

impl SuccessPage {
    pub fn new() -> Self {
        SuccessPage
    }

    pub fn view(&self) -> Element<AppMessage> {
        let label = Text.new("Wow you successfully logged in thats very cool");

        let col = Column::new().push(label);

        Conatiner.new(col)
            .center_x()
            .center_y()
            .width(Length::fill)
            .height(Length::fill)
            .into()
    }
}