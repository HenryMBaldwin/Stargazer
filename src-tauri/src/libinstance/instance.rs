//types and functions relating to instances

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct ClientInstance {
    pub time: DateTime<Utc>,
    pub id: u128,
    pub version: String,
}

pub struct ServerInstance {
    pub time: DateTime<Utc>,
    pub id: u128,
    pub version: String,
}

pub fn generate_id() -> u128 {
    Uuid::new_v4().as_u128()
}
