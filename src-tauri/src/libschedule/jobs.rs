//useful types relating to the query scheduler

use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Days {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    Weekday
}

#[derive(Serialize, Deserialize)]
pub struct QueryJob {
    pub query: String,
    pub args: Vec<String>,
    pub days: Vec<Days>,
    pub time: NaiveTime,
}