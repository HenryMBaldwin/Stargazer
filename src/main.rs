#![allow(unused_imports)]
//iced
use iced::{
    Command,
    theme::Theme,
    Settings,
    Application,
    Element,
    window::{
        Icon,
        Level, 
        Position, 
        Settings as WindowSettings
    }
};
use std::sync::Arc;

//modules
mod views;
use views::{
    login_page::{LoginPage, LoginPageMessage},
    success_page::{SuccessPage, SuccessPageMessage},
    components
};

//Orion APi
mod orion_api;
use orion_api::OrionAPI;
//Consts
const WINDOW_WIDTH: u32 = 910;
const WINDOW_HEIGHT: u32 = 496;



fn main() -> Result<(), iced::Error> {

    //initial window settings
    let default_window_settings = WindowSettings{
        size: (WINDOW_WIDTH, WINDOW_HEIGHT),
        position: Position::default(),
        min_size: None,
        max_size: None,
        visible: true,
        resizable: false,
        decorations: true,
        transparent: false,
        level: Level::default(),
        icon: None,
        platform_specific: Default::default(),
    }; 
    
    //new orion_api

    //run the app
    MainApp::run(Settings {
        window: default_window_settings,
        ..Settings::default()
    })
}

//main controller and page manager for our application
pub struct MainApp{
    oapi: Arc<OrionAPI>,
    //currently displayed page
    current_view: Views,
    //pages
    login_page: LoginPage,
    success_page: SuccessPage
    //..
}

//Message will use sub messages for each of the page
#[derive(Debug, Clone)]
pub enum AppMessage {
    //Message to change between main views
    ChangeView(Views),
    //None message for calling futures with no action up returning
    NoneMsg,
    //page messages
    LoginPageMessage(LoginPageMessage),
    SuccessPageMessage(SuccessPageMessage)

    //..
}

#[derive(Debug,Clone,Copy)]
pub enum Views {
    //Pages
    LoginPage,
    SuccessPage
    //..
}

impl Application for MainApp { 
    type Message = AppMessage;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flage: ()) -> (MainApp, Command<AppMessage>) {
        let oapi = Arc::new(OrionAPI::new()).clone();
        (   
            MainApp { 
                oapi: oapi.clone(),
                //init with login page visible
                current_view: Views::LoginPage,
                login_page: LoginPage::new(oapi.clone()), 
                success_page: SuccessPage::new(oapi.clone())
            },
            Command::none()
        )
        
    }

    fn title(&self) -> String{
        String::from("Stargazer")
    }

    fn update(&mut self, message: Self::Message) -> Command<AppMessage> {
        match message {
            AppMessage::ChangeView(view) => {
                self.current_view = view;
                Command::none()
            },
            AppMessage::LoginPageMessage(msg) => {
                self.login_page.update(msg)
            },
            AppMessage::SuccessPageMessage(msg) => {
                self.success_page.update(msg)
            },
            _ => Command::none()
        }
    }

    //Displays current view
    fn view(&self) -> Element<'_, Self::Message> {
        match self.current_view {
            //Views
            Views::LoginPage => self.login_page.view().map(move |message| AppMessage::LoginPageMessage(message)),
            Views::SuccessPage => self.success_page.view().map(move |message| AppMessage::SuccessPageMessage(message))
            //..
        }
    }
}

