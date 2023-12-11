use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use sha2::{Sha256, Digest};
use anyhow::{Result,Error};

pub struct DBController {
    pool: Pool<SqliteConnectionManager>,
}

impl DBController{
    pub fn new() -> Result<Self> {
        let manager = SqliteConnectionManager::file("db/queries.sqlite");
        let pool = Pool::new(manager)?;
        let db_controller = Self { pool };
        db_controller.init()?;
        Ok(db_controller)
    }
    //creates table if it doesn't exist
    fn init(&self) -> Result<()> {
        let conn = self.pool.get()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS queries (
                hash TEXT PRIMARY KEY NOT NULL,
                response TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    //generates a hash as the primary key for a query
    fn generate_hash(id: String, args: &Vec<String>) -> String {
        let mut hasher = Sha256::new();
        hasher.update(id);
        for arg in args {
            hasher.update(arg);
        }
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    //checks if a query exists in the database
    pub fn query_exists(&self, id: String, args: &Vec<String>) -> Result<bool> {
        let conn = self.pool.get()?;
        let hash = Self::generate_hash(id, args);
        let mut stmt = conn.prepare("SELECT hash FROM queries WHERE hash = ?1")?;
        let mut rows = stmt.query([hash])?;
        Ok(rows.next()?.is_some())
    }

    pub fn get_query(&self, id: String, args: &Vec<String>) -> Result<String> {
        let conn = self.pool.get()?;
        let hash = Self::generate_hash(id, args);
        let mut stmt = conn.prepare("SELECT response FROM queries WHERE hash = ?1")?;
        let mut rows = stmt.query([hash])?;
        let row = rows.next()?.unwrap();
        Ok(row.get(0)?)
    }

    //inserts a query with result
    pub fn insert_query(&self, id: String, args: &Vec<String>, response: &str) -> Result<()> {
        let conn = self.pool.get()?;
        let hash = Self::generate_hash(id, args);
        conn.execute(
            "INSERT OR REPLACE INTO queries (hash, response) VALUES (?1, ?2)",
            [hash, String::from(response)],
        )?;
        Ok(())
    }
}