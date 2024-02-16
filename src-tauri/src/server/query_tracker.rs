use std::{fs::File, io::{BufRead, BufReader}};
use rand::Rng;
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
        let log_str = self.logger.log_query(log_id, QueryStatus::STARTING, meta_data);
        self.query_log.push(log_str);
    }
    
    pub fn end_query(&mut self, log_id: &str, meta_data: &str) -> (){
        let log_str = self.logger.log_query(log_id, QueryStatus::SUCCESS, meta_data);
        self.query_log.push(log_str);
    }

    pub fn error_query(&mut self, log_id: &str, meta_data: &str) -> (){
        let log_str = self.logger.log_query(log_id, QueryStatus::ERROR, meta_data);
        self.query_log.push(log_str);
    }
    //returns the query log as a json array
    pub fn get_query_log(&self) -> Result<String> {
        Ok(serde_json::to_string(&self.query_log).unwrap())
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