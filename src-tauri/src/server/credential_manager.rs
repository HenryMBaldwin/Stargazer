use std::{fs::{self, File}, io::{self, BufRead, Write}, path::{Path, PathBuf}};

//manages saving and retrieving credentials from the system
use secstr::*;
use keyring::Entry;
use futures::lock::Mutex;
use tauri::api::path::data_dir;
use anyhow::{Result};
pub struct CredentialManager {
    username: Mutex<String>,
    service: String,
    has_credentials: bool,
}

impl CredentialManager {
    pub fn new() -> Result<Self> {
        let username_cache_path = Self::get_username_cache_path()?;
        let username = Self::get_username_from_cache(&username_cache_path)?;
        let service = "stargazer".to_string();
        let password = Self::get_password_from_keyring(&service, &username)?;
        let has_credentials = !password.unsecure().is_empty(); 

        Ok(Self {
            username: Mutex::new(username),
            service: service,
            has_credentials: has_credentials,
        })
    }
    
    //public functions to get username and password and check for credentials
    pub async fn get_username(&self) -> Option<String> {
        let username=  self.username.lock().await.clone();
        match username.is_empty() {
            true => Some(username),
            false => None
        }
    }

    pub async fn get_password(&self) -> Option<SecStr> {
        let password = Self::get_password_from_keyring(&self.service, &self.username.lock().await.clone()).expect("Error getting password from keyring");
        match password.unsecure().is_empty() {
            true => Some(password),
            false => None
        }
    }

    pub fn has_credentials(&self) -> bool {
        self.has_credentials
    }

    //public function to update cached credentials
    //only call this function after the user has been authenticated
    pub async fn set_username_and_password(&mut self, username: String, password: String) -> Result<()> {
        if username.is_empty() || password.is_empty() {
            return Err(anyhow::anyhow!("Error: username or password is empty"));
        }
        let username_cache_path = Self::get_username_cache_path()?;
        let mut username_lock = self.username.lock().await;
        *username_lock = username.clone();
        let service = self.service.clone();
        let entry = Entry::new(&service, &username).expect("Error creating keyring entry");
        entry.set_password(&password).expect("Error setting password");
        let mut file = File::create(username_cache_path)?;
        file.write_all(username.as_bytes())?;
        self.has_credentials = true;
        Ok(())
    }

    fn get_password_from_keyring(service: &String, username: &String) -> Result<SecVec<u8>> {
        let username = username.clone();
        if username.is_empty() {
            //return empty secvec
            return Ok(SecStr::new("".into()));
        }
        let entry = Entry::new(service, &username).expect("Error creating keyring entry");
        let password = match entry.get_password() {
            Ok(password) => password,
            Err(_) => {
                return Ok(SecStr::new("".into()));
            }
        };
        Ok(SecStr::from(password))
    }
    //ensures that the username cache path exists and return it
    fn get_username_cache_path() -> Result<PathBuf> {
        let roaming_app_data = data_dir().expect("Could not find roaming app data directory");
        //concatenate local_app_data with the db path
        let username_path = roaming_app_data.join("stargazer/cred/cred.txt");
        let username_dir = username_path.parent().expect("Error getting parent directory");

        if !username_dir.exists() {
            fs::create_dir_all(username_dir).expect("Failed to create directory");
        }
        // Check if the file exists, create it if it doesn't
        if !username_path.exists() {
            File::create(&username_path).expect("Failed to create file");
        }
        Ok(username_path)
    }

    //attempts to get the username from the roaming app data
    pub fn get_username_from_cache(username_cache_path: &PathBuf) -> Result<String> {
        let path = username_cache_path.clone();
        let file = File::open(path)?;
        let mut lines = io::BufReader::new(file).lines();
        let username = match lines.next() {
            Some(line) => line?,
            None => String::new()
        };
        Ok(username)
    }  
}

