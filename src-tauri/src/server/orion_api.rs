use std::sync::Arc;

use actix_web::http::header::q;
use reqwest::{Client, header, StatusCode, Response};
//use anyhow::{Result, Error, anyhow};
use serde_json::Value;
use secstr::*;
use futures::lock::Mutex;
use stargazer::liberror::orion_api_err::*;
use anyhow::{Result};
//use credential manager
use crate::{credential_manager, query_tracker};
use credential_manager::CredentialManager;
use query_tracker::{QueryTracker, random_id};
pub struct OrionAPI{
    //base API URL
    base_url: String,
    //because this will be running in the background
    auth_token: Mutex<String>,
    //variable to hold whether the instance currently has a valid auth token
    auth_valid: Mutex<bool>,
    //username
    username: Mutex<String>,
    //store password in memory
    password: Mutex<SecVec<u8>>,
    //credential manager instance to work with credentials on disk
    credential_manager: CredentialManager,
    //for tracking query events
    query_tracker: Arc<Mutex<QueryTracker>>,
}

impl OrionAPI{

    pub fn new(query_tracker: Arc<Mutex<QueryTracker>>) -> Self {
        Self{
            //base API URL
            base_url: String::from("https://api.orionadvisor.com/api/v1/"),
            auth_token: Mutex::new(String::new()),
            auth_valid: Mutex::new(false),
            username: Mutex::new(String::new()),
            password: Mutex::new(SecStr::new("".into())),
            credential_manager: CredentialManager::new().expect("Error creating CredentialManager"),
            query_tracker,
        }
    }

    //attempts to log in using saved credentials and returns self
    pub async fn init(self) -> Result<OrionAPI> {
        //check if there are saved credentials
        if self.credential_manager.has_credentials().await {
            //if there are saved credentials then attempt to authenticate
            //has_credentials() should only return true if both username and password are Some()
            let username = self.credential_manager.get_username().await.unwrap();
            let password = self.credential_manager.get_password().await.unwrap();

            self.login(&username, &String::from_utf8(password.unsecure().to_vec()).expect("Error unsecuring string")).await?;
        }
        Ok(self)
    }
    //sets username and password then attempts to authenticate
    pub async fn login(&self, username: &str, password: &str) -> Result<StatusCode>{
        {
            let mut username_lock = self.username.lock().await;
            let mut password_lock = self.password.lock().await;

            *username_lock = username.to_string();
            *password_lock = SecStr::from(password);
        } 

        self.authenticate().await
    }

    //attempts to authenticate with Orion using saved username and password
    async fn authenticate(&self) -> Result<StatusCode> {
        //url 
        let auth_url =format!("{}security/token", self.base_url);
        //auth client
        let auth_client = Client::new();
        let response = auth_client
            .get(auth_url)
            .basic_auth(&self.username.lock().await.clone(), Some(String::from_utf8(self.password.lock().await.unsecure().to_vec())?)) 
            .send()
            .await?;

        let status = response.status();
        if response.status().is_success() {

            //parse response body as JSON
            let json: Value = response.
                json::<serde_json::Value>()
                .await?;
            if let Some(token) = json["access_token"].as_str() {
                let mut auth_token = self.auth_token.lock().await;
                *auth_token = token.to_string();
                //set auth token to valid
                let mut valid = self.auth_valid.lock().await;
                *valid = true;
                self.credential_manager.set_username_and_password(self.username.lock().await.clone(), String::from_utf8(self.password.lock().await.unsecure().to_vec())?).await?;
            }
            else {
                *self.auth_valid.lock().await = false;
                return Err(AuthError::Unknown(format!("Incorrect json response from Orion: {}", json.to_string())).into());
            }
            Ok(status)
        }
        else {
            *self.auth_valid.lock().await = false;
            //if authentication was not successful return autherror with status code
            //TODO: implement more granular errors for different statuses
            return Err(AuthError::InavalidLogin(status).into())
        }
        
    }

    //returns whether the instance currently has a valid auth token
    pub async fn check_auth(&self) -> bool {
        self.auth_valid.lock().await.clone()
    }

    //gets auth token, attempts reauthentication if auth token is empty or invalid
    async fn get_auth(&self)-> Result<String>{
        //if auth token is valid then return auth token
        if self.auth_valid.lock().await.clone() {
            Ok(self.auth_token.lock().await.clone())
        }
        else {
            //attempt to auth using saved credentials
            //check if credentials aren't empty 
            if !(self.username.lock().await.clone().is_empty()) {
                if !(self.password.lock().await.unsecure().is_empty()){
                    //attempt to reauth with saved user and pass
                    match self.authenticate().await {
                        Ok(_) => Ok(self.auth_token.lock().await.clone()),
                        Err(e) => Err(e)
                    }
                }
                else {
                    Err(AuthError::EmptyPassword.into())
                }
            }
            else {
                Err(AuthError::EmptyUsername.into())
            }
        }
    }

    //returns the prompt field of a query as a json string
    pub async fn get_query_prompts(&self, id: String) -> Result<String> {
        let client = Client::new();
        let query_url = format!("{}Reporting/Custom/{}",self.base_url, id);
        let auth_header = {
            let token = self.get_auth().await?;

            let header = format!("Session {}", token);
            header
        };

        let response = client
            .get(&query_url)
            .header("Authorization", auth_header)
            .send()
            .await?;
        let json_string = response.text().await?;
        let body: Value = serde_json::from_str(&json_string)?;
        let prompts = body.get("prompts");
        if let Some(prompts) = prompts {
            let prompt_string = serde_json::to_string(&prompts)?;
            Ok(prompt_string)
        }
        else {
            Err(QueryError::NoPromptField(json_string).into())
        }
        
    }

    
    //generic query that returns JSON string of result or error
    pub async fn query(&self, id: String, args: Vec<String>) -> Result<String>{

        println!("Starting Query {}", id);
        
        //random id for logging purposes
        let log_id = random_id();
        //logging
        let mut query_string = format!("{{id:{} args: [", id);
        for item in &args {
            query_string.push_str(&format!("{},", item));
        }
        //remove trailing comma
        query_string.pop();
        query_string.push_str("]}");
        self.query_tracker.lock().await.start_query(log_id.as_str(), &query_string);
        
        
        
        let client = Client::new();
        let query_url = format!("{}Reporting/Custom/{}",self.base_url, id);
        let auth_header = {
            let token = self.get_auth().await?;

            let header = format!("Session {}", token);
            header
        };

        let response = client
            .get(&query_url)
            .header("Authorization", auth_header.clone())
            .send()
            .await?;
        let json_string = response.text().await?;
        let mut body: Value = serde_json::from_str(&json_string)?;

        if let Some(prompts) = body.get_mut("prompts").and_then(Value::as_array_mut) {
            let mut args_iter = args.iter();
            for prompt in prompts.iter_mut() {
                if let Some(default_value) = prompt.as_object_mut().and_then(|obj| obj.get_mut("defaultValue")) {
                    if let Some(arg) = args_iter.next() {
                        *default_value = serde_json::Value::String((*arg).to_string());
                        println!("Updated defaultValue for {}: {}", prompt["prompt"], arg);
                    } else {
                        break; // Stop updating prompts if there are no more args
                    }
                }
            }
            
            // Handle the case where there are more args than prompts
            if args_iter.next().is_some() {
                let error = QueryError::TooManyArgs;
                self.query_tracker.lock().await.error_query(log_id.as_str(), &error.to_string());
                return Err(error.into());
            }
        }
         else {
            let error =QueryError::NoPromptField(json_string);
            self.query_tracker.lock().await.error_query(log_id.as_str(), &error.to_string());
            return Err(error.into());
        }
        let query_url = format!("{}/Generate/Table", query_url);

        let post_response = client
            .post(&query_url)
            .header("Authorization", auth_header)
            .json(&body)
            .send()
            .await?;

        if post_response.status().is_success() {
            // The POST request was successful
            self.query_tracker.lock().await.end_query(log_id.as_str(), "Query successful");
            let response_body = post_response.text().await?;
            Ok(response_body)
        } else {
            // Handle the case where the POST request was not successful
            let error = QueryError::PostRequestFailed(post_response.status());
            self.query_tracker.lock().await.error_query(log_id.as_str(), &error.to_string());
            Err(error.into())
        }

    }
}
