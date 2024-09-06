// use std::time::Duration;
// use clokwerk::{AsyncScheduler, TimeUnits, Job};
// use chrono::NaiveTime;
// use stargazer::libpipe::reqres::{QueryRequest, QueryResponse};

// //Allows queries to be scheduled to run at specific times
// pub struct QueryScheduler {
//     Scheduler: AsyncScheduler,
// }

// impl QueryScheduler {
//     pub fn new() -> QueryScheduler {
//         QueryScheduler {
//             Scheduler: AsyncScheduler::new(),
//         }
//     }

//     pub fn add_job(&mut self, query_job: QueryJob) {
        
        
//     }

//     pub fn remove_job(&mut self, query_job: QueryJob) {
        
//     }

//     pub async fn run_pending(&mut self) {
//         self.Scheduler.run_pending();
//         tokio::time::sleep(Duration::from_millis(100)).await;
//     }

// }

