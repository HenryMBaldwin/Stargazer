use std::{fmt, fs::{File, OpenOptions}};
use actix_web::web::Query;
use log::{info, error, warn, debug, trace};
use simplelog::*;
use tauri::api::path::cache_dir;
use chrono::{Local};
use rand::Rng;
pub struct Logger;

pub enum QueryStatus {
    STARTING,
    SUCCESS,
    ERROR
}

impl fmt::Display for QueryStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
              QueryStatus::STARTING => write!(f, "STARTING"),
              QueryStatus::SUCCESS => write!(f, "SUCCESS"),
              QueryStatus::ERROR => write!(f, "ERROR")
       }
    }
}

impl Logger {
    pub fn new() -> Logger {
        let _ = WriteLogger::init(
            LevelFilter::Info,
            Config::default(),
            OpenOptions::new()
                    .write(true)   // Enable write mode
                    .append(true)  // Set the file to append mode, which also prevents truncation
                    .open(Self::get_query_log_path()).expect("Error opening query log file"),
                    );
        Self
    }

    //ensures that the /log/query/ directory exists and returns the path to the query log
    fn get_query_log_path() -> String {
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
        let log_str = format!("[QUERY] {{id: {}, status: {}, meta-data: {}}}", log_id, status, meta_data);
        info!("{}", log_str);
        log_str
    }

}