use reqwest::{Client, Error, header, StatusCode};
use serde::Deserialize;
use secstr::*;
use futures::lock::Mutex;
pub struct OrionAPI{
    //base API URL
    base_url: String,
    //because this will be running in the background
    auth_token: Mutex<String>,
    //variable to hold whether the instance currently has a valid auth token
    auth_valid: bool,
    //username
    username: Mutex<String>,
    //store password in memory
    password: Mutex<SecVec<u8>>
}

impl OrionAPI{

    pub fn new() -> Self {
        Self{
            //base API URL
            base_url: String::from("https://api.orionadvisor.com/api/v1/"),
            auth_token: Mutex::new(String::new()),
            auth_valid: false,
            username: Mutex::new(String::new()),
            password: Mutex::new(SecStr::new("".into()))  
        }
    }

    //sets username and password then attempts to authenticate
    pub async fn login(&self, username: &str, password: &str) -> Result<StatusCode, Error>{

        
        {
            let mut username_lock = self.username.lock().await;
            let mut password_lock = self.password.lock().await;

            *username_lock = username.to_string();
            *password_lock = SecStr::from(password);
        } 

        self.authenticate().await
    }

    //attempts to authenticate with Orion using saved username and password
    async fn authenticate(&self) -> Result<StatusCode, Error> {
        
        //combine the 
        //url 
        let auth_url =format!("{}security/token", self.base_url);
        //auth client
        let auth_client = Client::new();
        let response = auth_client
            .get(auth_url)
            .basic_auth(&self.username.lock().await.clone(), Some(String::from_utf8(self.password.lock().await.unsecure().to_vec()).unwrap())) 
            .send()
            .await?;
        Ok(response.status())
    }

    //gets auth token, attempts reauthentication if auth token is empty
    pub async fn update_auth(&self)-> String{
        if self.auth_valid {
            self.auth_token.lock().await.clone()
        }
        else {
            
            String::new()
        }


    }
}
