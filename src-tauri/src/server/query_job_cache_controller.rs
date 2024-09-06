// use stargazer::libschedule::jobs::{Days, QueryJob};
// pub struct QueryJobCacheController {
//     pool: Pool<SqliteConnectionManager>,
// }

// impl QueryJobCacheController {
//     pub fn new() -> Result<Self> {
//         let local_app_data = cache_dir().expect("Could not find local app data directory.");
//         let db_path = local_app_data.join("stargazer/db/query_schedule_cache.db");
//         let db_dir = db_path.parent().unwrap();

//         if !db_dir.exists() {
//             create_dir_all(db_dir).expect("Error creating db directory.");
//         }

//         if !db_path.exists() {
//             let mut file = File::create(&db_path).expect("Error creating db file.");
//         }

//         let manager = SqliteConnectionManager::file(db_path);
//         let pool = Pool::new(manager).expect("Error creating db pool.");
//         // I hate this abbreviation because ideally stupid names that are not descriptive should be avoided
//         // but QueryJobCacheController is already a stupid name because it's not succinct
//         let qjcc = QueryJobCacheController {
//             pool: pool,
//         };
//         qjcc.init()?;
//         Ok(qjcc)
//     }

//     fn init(&self) -> Result<()> {
//         let conn = self.pool.get()?;
//         conn.execute("CREATE TABLE IF NOT EXISTS query_schedule_cache (
//             hash TEXT PRIMARY KEY NOT NULL,
//             query_job TEXT NOT NULL,
//         )", [])?;
//         Ok(())
//     }

//     fn generate_hash(&self, query_job: QueryJob) -> String {
//         let mut hasher = Sha256::new();
//         hasher.update(query_job);
//         let result = hasher.finalize();
//         format!("{:x}", result)
//     }

//     pub fn insert_job(&self, job: QueryJob) -> Result<()> {
//         let conn = self.pool.get()?;
//         let job_json = serde_json::to_string(&job).expect("Error serializing query job.");
//         conn.execute("INSERT INTO query_schedule_cache (hash, query_job) VALUES (?, ?)", params![hash, job_json])?;
//         Ok(())
//     }

//     pub fn remove_job(&self, job: QueryJob) -> Result<()> {
//         let conn = self.pool.get()?;
//         let hash = self.generate_hash(job);
//         conn.execute("DELETE FROM query_schedule_cache WHERE hash = ?", params![hash])?;
//         Ok(())
//     }
// }