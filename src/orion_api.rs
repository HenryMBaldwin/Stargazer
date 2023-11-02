use reqwest::{Client, Error, header};
use serde::Deserialize;

pub struct OrionAPI{
    //base API URL
    base_url: String
}

impl OrionAPI{

    pub fn new() -> Self {
        Self{
            //base API URL
            base_url: "https://api.orionadvisor.com/api/v1/"
        }
    }

    pub async fn Authenticate(username: String, password: String) {
        
        //combine the 
        //url 
        let auth_url = base_url + "security/token";
        //auth client
        let auth_client = Client.new();
        let request = auth_client.get(auth_url).basic_auth(&username, &password);
        println!("{}", request);
    }
}
