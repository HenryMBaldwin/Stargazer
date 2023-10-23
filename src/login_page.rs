//iced
use iced::widget::{button, image, text_input, Button, Text, Column, Container, Image, Row, TextInput};
use iced::alignment::{Alignment, Horizontal};
use iced::{Color, Element, Length, theme};

//Styles
use crate::styles;
use styles::LoginButtonStyle;
pub struct LoginPage {
    username: String,
    password: String,
    stargazer_image: image::Handle,
} 

#[derive(Debug, Clone)]
pub enum LoginPageMessage{
    //Messages for Login page
    UsernameChanged(String),
    PasswordChanged(String),
    LoginPressed,
    
}


//Impl
impl LoginPage {
    
    pub fn new() -> Self {
        Self { 
            
            username: String::new(),
            password: String::new(), 
            stargazer_image: image::Handle::from_path("assets/stargazer_black_vert_transparent.png")
         }
    }

    pub fn update(&mut self, message: LoginPageMessage) {
        match message {
            //TODO
            _ => ()
        }
    }

    
    pub fn view(&self) -> Element<LoginPageMessage> {
        
        let img = Image::new(self.stargazer_image.clone())
            .width(Length::Fixed(300.0))
            .height(Length::Fixed(200.0));

        // Input fields
        let username_input = TextInput::new(
            "Username...",
            &self.username
        )
        .on_input(LoginPageMessage::UsernameChanged)
        .padding(10)
        .width(Length::Fixed(300.0));

        let password_input = TextInput::new(
            "Password...",
            &self.password,
        )
        .on_input(LoginPageMessage::PasswordChanged)
        .padding(10)
        .width(Length::Fixed(300.0))
        .password();

        // Login button
        let login_button = Button::new( Text::new("Login").horizontal_alignment(Horizontal::Center))
            .on_press(LoginPageMessage::LoginPressed)
            .width(Length::Fixed(300.0))
            .style(iced::theme::Button::Custom(Box::new(LoginButtonStyle)));

        // Layout
        let col = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(img)
            .push(username_input)
            .push(password_input)
            .push(login_button);
        


        //placeholder for login screen
        // let placeholder = Text::new("This is a placeholder for a login screen.");
        Container::new(col)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

}

