//iced
use iced::Settings;
use iced::Sandbox;
use iced::Element;
//modules
//login page
use login_page::{LoginPage, LoginPageMessage};
mod login_page;

fn main() -> Result<(), iced::Error> {
    MainApp::run(Settings::default())
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
#[derive(Debug, Clone, Copy)]
pub enum AppMessage {
    //Message to change between main views
    ChangeView(Views),
    //page messages
    LoginPageMessage(LoginPageMessage)
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
            login_page: LoginPage }
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
            Views::LoginPage => self.login_page.view()
            //..
        }
    }
}

