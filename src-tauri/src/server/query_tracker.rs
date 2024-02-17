use std::{fs::File, io::{BufRead, BufReader}};
use rand::Rng;
use serde_json::Value;
use crate::logger::{Logger, QueryStatus};
use anyhow::Result;


pub struct QueryTracker {
    //json array of all query events (as json strings)
    query_log: Vec<String>,
    logger: Logger,
}

impl QueryTracker {
    pub fn new() -> QueryTracker {
        //make sure to read from log before initializing logger, as it may lock the file
        let query_log = Self:: populate_from_disk().expect("Error populating query log from disk"); 
        QueryTracker {
            query_log: query_log,
            logger: Logger::new(),
        }
    }

    //public functions to save query into internal query_log and log it
    pub fn start_query(&mut self, log_id: &str, meta_data: &str) -> (){
        let log_str = self.logger.log_query(log_id, QueryStatus::STARTED, &meta_data);
        self.query_log.push(log_str);
    }
    
    pub fn end_query(&mut self, log_id: &str, meta_data: &str) -> (){
        let log_str = self.logger.log_query(log_id, QueryStatus::SUCCESS, format!("\"{}\"", meta_data).as_str());
        self.query_log.push(log_str);
    }

    pub fn error_query(&mut self, log_id: &str, meta_data: &str) -> (){
        let log_str = self.logger.log_query(log_id, QueryStatus::ERROR, format!("\"{}\"", meta_data).as_str());
        self.query_log.push(log_str);
    }
    //returns the query log as a json array
    pub fn get_query_log(&self) -> Result<String> {
        let mut json_vec: Vec<Value> = Vec::new();

        
        for log_entry in &self.query_log {
            let json_entry: Value = match serde_json::from_str(log_entry) {
                Ok(json_entry) => json_entry,
                Err(e) => {
                    println!("Error deserializing query log entry. Log entry: {} Error: {}", log_entry, e);
                    Value::from("")
                }
            };
            json_vec.push(json_entry);
        }

        let log = serde_json::to_string(&json_vec)?;
        Ok(log)
    }

    //populates query log 
    fn populate_from_disk() -> Result<Vec<String>>{
        let log_path = Logger::get_query_log_path();
        let file = File::open(log_path).expect("Error opening query log file");
        let reader = BufReader::new(file);

        let mut results = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            if line.contains("[QUERY]") {
                if let (Some(start), Some(end)) = (line.find('{'), line.rfind('}')) {
                    results.push(line[start..=end].to_string());
                }
            }
        }

        Ok(results)
    }
}

//helper function to generate a 10 character unique random id where necassary
pub fn random_id() -> String {
    let length = 10;
    let mut rng = rand::thread_rng();

    let mut id = String::with_capacity(length);

    for _ in 0..length {
        // Generate a random digit and append it to the string
        id.push_str(&rng.gen_range(0..10).to_string());
    }

    id
}