use std::{fmt, fs::{File, OpenOptions}};
use actix_web::web::Query;
use log::{info, error, warn, debug, trace};
use simplelog::*;
use dirs_next::cache_dir;
use chrono::{Local};
use rand::Rng;
pub struct Logger;

pub enum QueryStatus {
    STARTED,
    SUCCESS,
    ERROR
}

impl fmt::Display for QueryStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
              QueryStatus::STARTED => write!(f, "STARTED"),
              QueryStatus::SUCCESS => write!(f, "SUCCESS"),
              QueryStatus::ERROR => write!(f, "ERROR")
       }
    }
}

impl Logger {
    pub fn new() -> Logger {
        let _ = WriteLogger::init(
            LevelFilter::Info,
            ConfigBuilder::new()
                    .set_time_offset_to_local().expect("Error setting time offset on logger.")
                    .build(),
            OpenOptions::new()
                    .write(true)   // Enable write mode
                    .append(true)  // Set the file to append mode, which also prevents truncation
                    .open(Self::get_query_log_path()).expect("Error opening query log file"),
                    );
        Self
    }

    //ensures that the /log/query/ directory exists and returns the path to the query log
    pub fn get_query_log_path() -> String {
        //get todays date
        let date = Local::now();
        let cache_dir = cache_dir().expect("Error getting cache dir");
        let query_log_path = cache_dir.join(format!("stargazer/logs/query/{}.log", date.format("%m-%d-%y")));
        
        //make sure logs/ and logs/query/ exist if they don't
        if !query_log_path.exists() {
            std::fs::create_dir_all(query_log_path.parent().unwrap()).expect("Error creating query log directory");
        }
        //create the file if it doesn't exist
        if !query_log_path.exists() {
            File::create(&query_log_path).expect("Error creating query log file");
        }

        query_log_path.to_str().unwrap().to_string()
    }

    //logs query and returns log string
    pub fn log_query(&self, log_id: &str, status: QueryStatus, meta_data: &str) -> String {
        //ensure log exists
        Self::get_query_log_path();
        let log_str = format!("{{\"id\": \"{}\", \"timestamp\": \"{}\",  \"status\": \"{}\", \"metadata\": {}}}", log_id, Self::generate_timestamp(), status, meta_data);
        info!("[QUERY]{}", log_str);
        log_str
    }

    fn generate_timestamp() -> String {
        let date = Local::now();
        date.format("%H:%M:%S").to_string()
    }

    //ensures that the /log/general/ directory exists and returns the path to the general log
    pub fn get_general_log_path() -> String {
        //get todays date
        let date = Local::now();
        let cache_dir = cache_dir().expect("Error getting cache dir");
        let general_log_path = cache_dir.join(format!("stargazer/logs/general/{}.log", date.format("%m-%d-%y")));
        
        //make sure logs/ and logs/general/ exist if they don't
        if !general_log_path.exists() {
            std::fs::create_dir_all(general_log_path.parent().unwrap()).expect("Error creating general log directory");
        }
        //create the file if it doesn't exist
        if !general_log_path.exists() {
            File::create(&general_log_path).expect("Error creating general log file");
        }

        general_log_path.to_str().unwrap().to_string()
    }

    //logs general message
    pub fn log_general(&self, message: &str) -> () {
        //ensure log exists
        Self::get_general_log_path();
        info!("[GENERAL]{}", message);
    }

}