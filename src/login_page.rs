use std::sync::Arc;
use reqwest::StatusCode;
//iced
use iced::widget::{button, image, text_input, Button, Text, Column, Container, Image, Row, TextInput};
use iced::alignment::{Alignment, Horizontal};
use iced::{Color, Element, Length, theme, Command};

//Styles
use crate::{styles, AppMessage, orion_api, Views};

use styles::LoginButtonStyle;

//Orion API
use orion_api::OrionAPI;

pub struct LoginPage {
    oapi: Arc<OrionAPI>,
    username: String,
    password: String,
    password_id: text_input::Id,
    stargazer_image: image::Handle,
} 

#[derive(Debug, Clone)]
pub enum LoginPageMessage{
    //Messages for Login page
    UsernameChanged(String),
    UsernameSubmitted,
    PasswordChanged(String),
    //Password submitted is the same as LoginPressed
    LoginPressed,
    
}


//Impl
impl LoginPage {
    
    pub fn new(oapi: Arc<OrionAPI>) -> Self {
        Self { 
            oapi,
            username: String::new(),
            password: String::new(), 
            password_id: text_input::Id::unique(),
            stargazer_image: image::Handle::from_path("assets/stargazer_black_vert_transparent.png")
         }
    }

    pub fn update(&mut self, message: LoginPageMessage) -> Command<AppMessage>{
        match message {
            LoginPageMessage::UsernameChanged(user) => {
                self.username = user;
                Command::none()
            }
            LoginPageMessage::PasswordChanged(pass) => {
                self.password = pass;
                Command::none()
            }
            LoginPageMessage::UsernameSubmitted => text_input::focus(self.password_id.clone()),
            LoginPageMessage::LoginPressed => {
                let username = self.username.clone();
                let password = self.password.clone();
                let oapi = self.oapi.clone();
                Command::perform(async move {oapi.login(&username, &password).await},
             |status| {
                if status.unwrap().is_success() { 
                    AppMessage::ChangeView(Views::SuccessPage)
                }
                else {
                    AppMessage::LoginFailed
                }
            })}
                
        }
    }

    
    pub fn view(&self) -> Element<LoginPageMessage> {
        
        let img = Image::new(self.stargazer_image.clone())
            .width(Length::Fixed(300.0))
            .height(Length::Fixed(200.0));

        // Input fields
        let username_input = text_input(
            "Username...",
            &self.username
        )
        .on_input(LoginPageMessage::UsernameChanged)
        .on_submit(LoginPageMessage::UsernameSubmitted)
        .padding(10)
        .width(Length::Fixed(300.0));

        let password_input = text_input(
            "Password...",
            &self.password,
        )
        .on_input(LoginPageMessage::PasswordChanged)
        .on_submit(LoginPageMessage::LoginPressed)
        .id(self.password_id.clone())
        .padding(10)
        .width(Length::Fixed(300.0))
        .password();

        // Login button
        let login_button = Button::new( Text::new("Login").horizontal_alignment(Horizontal::Center))
            .on_press(LoginPageMessage::LoginPressed)
            .width(Length::Fixed(300.0));

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

