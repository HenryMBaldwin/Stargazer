use iced::{Command, Element, widget::{Text, button, text, Column, Container, Row}, alignment::Horizontal, Length};
use crate::{views::components::error_components::ErrorText, AppMessage, Views};


pub struct DashBoard {
}

#[derive(Debug, Clone)]
pub enum DashBoardMessage {
    //this is a temporary catch all message
    ExecCommand
}

impl DashBoard{
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: DashBoardMessage) -> Command<AppMessage> {
        match message {
            DashBoardMessage::ExecCommand => {
                Command::perform(async move {},
                |_|{
                    AppMessage::NoneMsg
                })
            }
        }
    }
    pub fn view(&self) -> Element<DashBoardMessage> {
        let label = Text::new("This is the dashboard");

        let _button = button(text("Test").horizontal_alignment(Horizontal::Center))
            .on_press(DashBoardMessage::ExecCommand)
            .width(Length::Fixed(300.0));

        let main_content = Column::new()
            .push(label)
            .push(_button);

        // Define sidebar buttons
        let sidebar_button_1 = button(text("Sidebar Item 1")).on_press(DashBoardMessage::ExecCommand);
        let sidebar_button_2 = button(text("Sidebar Item 2")).on_press(DashBoardMessage::ExecCommand);
        // Add more buttons as needed

        // Construct the sidebar
        let sidebar = Column::new()
            .push(sidebar_button_1)
            .push(sidebar_button_2);
            // Add more buttons to the column as needed

        // Combine sidebar and main content in a row
        let layout = Row::new()
            .push(sidebar)
            .push(main_content);

        Container::new(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}






