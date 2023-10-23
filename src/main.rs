//iced
use iced::{
    Settings,
    Sandbox,
    Element,
    window::{
        Icon,
        Level, 
        Position, 
        Settings as WindowSettings
    }
};

//modules
//login page
mod login_page;
use login_page::{LoginPage, LoginPageMessage};

//styles
mod styles;

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

    //run the app
    MainApp::run(Settings {
        window: default_window_settings,
        ..Settings::default()
    })
}

//main controller and page manager for our application
pub struct MainApp{
    //currently displayed page
    current_view: Views,
    //pages
    login_page: LoginPage
    //..
}

//Message will use sub messages for each of the page
#[derive(Debug, Clone)]
pub enum AppMessage {
    //Message to change between main views
    ChangeView(Views),
    //page messages
    LoginPageMessage(LoginPageMessage),
    //..
}

#[derive(Debug,Clone,Copy)]
pub enum Views {
    //Pages
    LoginPage
    //..
}

impl Sandbox for MainApp {
    type Message = AppMessage;

    fn new() -> Self {
        MainApp { 
            //init with login page visible
            current_view: Views::LoginPage,
            login_page: LoginPage::new() }
    }

    fn title(&self) -> String{
        String::from("Stargazer")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            AppMessage::ChangeView(view) => self.current_view = view,
            AppMessage::LoginPageMessage(msg) => self.login_page.update(msg)
        }
    }

    //Displays current view
    fn view(&self) -> Element<'_, Self::Message> {
        match self.current_view {
            //Views
            Views::LoginPage => self.login_page.view().map(move |message| AppMessage::LoginPageMessage(message))
            //..
        }
    }
}

