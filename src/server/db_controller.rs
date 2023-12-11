use rusqlite::{Connection, Result};
use sha2::{Sha256, Digest};

pub struct DBController {
    conn: Connection,
}

impl DBController{
    pub fn new() -> Result<Self> {
        let conn = Connection::open("db/queries.db")?;
        self.init()?;
        Ok(Self{conn})
    }

    //creates table if it doesn't exist
    fn init(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS queries (
                hash TEXT PRIMARY KEY NOT NULL,
                response TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    //generates a hash as the primary key for a query
    fn generate_hash(id: i32, args: &[String]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(id.to_string());
        for arg in args {
            hasher.update(arg);
        }
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    //checks if a query exists in the database
    pub fn query_exists(&self, id: i32, args: &[String]) -> Result<bool> {
        let hash = Self::generate_hash(id, args);
        let mut stmt = self.conn.prepare("SELECT hash FROM queries WHERE hash = ?1")?;
        let mut rows = stmt.query([hash])?;
        Ok(rows.next()?.is_some())
    }

    pub fn get_query(&self, id: i32, args: &[String]) -> Result<String> {
        let hash = Self::generate_hash(id, args);
        let mut stmt = self.conn.prepare("SELECT response FROM queries WHERE hash = ?1")?;
        let mut rows = stmt.query([hash])?;
        let row = rows.next()?.unwrap();
        Ok(row.get(0)?)
    }

    //inserts a query with result
    pub fn insert_query(&self, id: i32, args: &[String], response: &str) -> Result<()> {
        let hash = Self::generate_hash(id, args);
        self.conn.execute(
            "INSERT OR REPLACE INTO queries (hash, response) VALUES (?1, ?2)",
            [hash, response],
        )?;
        Ok(())
    }
}