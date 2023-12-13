use std::io::{Write, Read};
use std::sync::Arc;
use reqwest::StatusCode;
//iced
use iced::widget::{button, image, text, text_input, Button, Text, Column, Container, Image, Row, TextInput};
use iced::alignment::{Alignment, Horizontal};
use iced::{Color, Element, Length, theme, Command};

use stargazer::libpipe::reqres::{LoginRequest, RequestType};
//shared pipe lib
use stargazer::libpipe::{reqres,consts};

//named pipes
use crate::pipe_client;
//Styles
use crate::{views::components::error_components::ErrorText, AppMessage, Views};

use super::components::gold_button::GoldButton;

pub struct LoginPage {
    username: String,
    password: String,
    password_id: text_input::Id,
    stargazer_image: image::Handle,
    error_text: String,
} 

#[derive(Debug, Clone)]
pub enum LoginPageMessage{
    //Messages for Login page
    UsernameChanged(String),
    UsernameSubmitted,
    PasswordChanged(String),
    //Password submitted is the same as LoginPressed
    LoginPressed,
    LoginFailed(StatusCode)
    
}


//Impl
impl LoginPage {
    
    pub fn new() -> Self {
        Self { 
            username: String::new(),
            password: String::new(), 
            password_id: text_input::Id::unique(),
            stargazer_image: image::Handle::from_path("assets/stargazer_black_vert_transparent.png"),
            error_text: String::new()
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
                
            Command::perform(async move {pipe_client::login(&username, &password).await},
                |status| {
                    if status.is_success(){
                        AppMessage::ChangeView(Views::SuccessPage)
                    }
                    else{
                        AppMessage::LoginPageMessage(LoginPageMessage::LoginFailed(status))
                    }
                })
            }
            LoginPageMessage::LoginFailed(status) => {
                match status {
                    StatusCode::UNAUTHORIZED => self.error_text = String::from("Error: Incorrect Username or Password."),
                    StatusCode::BAD_GATEWAY => self.error_text = String::from("Error: Not connected to the Internet."),
                    _ => self.error_text = String::from(format!("Error: Status Code {}", status.as_u16()))
                }
                Command::none()
            }

        }
    }

    
    pub fn view(&self) -> Element<LoginPageMessage> {
        
        let img = Image::new(self.stargazer_image.clone())
            .width(Length::Fixed(300.0))
            .height(Length::Fixed(200.0));

        //ErrorText ***THIS SHOULD BE RED***
        let err_text: Text<'_, iced::Renderer> = text(&self.error_text);//.style(iced::theme::Text::Color(Color::from_rgb(204.0, 2.0, 2.0)));

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
            .width(Length::Fixed(300.0))
            ;//.style(iced::theme::Button::Custom(Box::new(GoldButton)));

        // Layout
        let mut col = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(img);

// Conditionally push the error text if it's not empty.
        if !self.error_text.is_empty() {
            col = col.push(err_text);
        }

        // Continue building the column.
        col = col.push(username_input)
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

