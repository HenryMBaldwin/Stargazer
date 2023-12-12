use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use sha2::{Sha256, Digest};
use anyhow::Result;
use serde_json::{Value, Map};
use stargazer::liberror::cache_controller_err::*;

pub struct CacheController {
    pool: Pool<SqliteConnectionManager>,
}

impl CacheController{
    pub fn new() -> Result<Self> {
        let manager = SqliteConnectionManager::file("db/queries.sqlite");
        let pool = Pool::new(manager)?;
        let cache_controller = Self { pool };
        cache_controller.init()?;
        Ok(cache_controller)
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
        //TODO, check if response is empty
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

    fn create_table(&self, table_name: String, response: &str) -> Result<()> {
        let conn = self.pool.get()?;
        let parsed: Value = serde_json::from_str(response)?;

        if let Some(arr) = parsed.as_array(){
        //get the first object in the array
            if let Some(val) = arr.get(0) {
                if let Some(obj) = val.as_object() {
                    let types_vec = Self::extract_field_types(obj)?;
                    let mut sql = format!("CREATE TABLE {} (RecordId INTEGER PRIMARY KEY NOT NULL", table_name);

                    for (key, val_type) in types_vec {
                        if val_type == "BOOLEAN"{
                            val_type = "TEXT";
                        }
                        sql.push_str(format!(", {} {}", key, val_type).as_str());
                    }
                    sql.push_str(")");
                    //drop table if it exists
                    self.drop_table_if_exists(table_name);
                    //create table
                    conn.execute(&sql, [])?;
                }
                else {
                    //TODO: Handle error
                    Err()
                }
            }
            else {
                //TODO: Handle error
                Err()
            }
            //insert all arr entries into newly created table
            for entry in arr {
                let mut sql = format!("INSERT INTO {} (", table_name);
                for (key, val_type) in types_vec{
                    if let Some(val) = entry.get(key) {
                        match val_type {
                            "TEXT" => {
                                if let Some(val) = val.as_str() {
                                    sql.push_str(format!("{},", val).as_str());
                                }
                                else {
                                    //TODO: Handle error
                                    Err()
                                }
                            },
                            "INTEGER" => {
                                if let Some(val) = val.as_i64() {
                                    sql.push_str(format!("{},", val).as_str());
                                }
                                else {
                                    //TODO: Handle error
                                    Err()
                                }
                            },
                            "BOOLEAN" => {
                                if let Some(val) = val.as_bool() {
                                    val = if val {"true"} else {"false"};
                                    sql.push_str(format!("{},", val).as_str());
                                }
                                else {
                                    //TODO: Handle error
                                    Err()
                                }
                            }
                        }
                    }
                    else{
                        //TODO: Handle error
                        Err()
                    }
                    

                }
            
            Ok(())
        }
        else{
            //TODO: Handle error
            Err()
        }
    }

    //helper functions

    //drops a table if it exists
    fn drop_table_if_exists(&self, table_name: String) ->Result<()>{
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?1")?;
        let mut rows = stmt.query([table_name])?;
        Ok(())
    }
    //gets json field types as sqlite types
    //note, boolean is not a valid sqlite type and must be handled separately from the caller function. Likely by converting to TEXT
    fn extract_field_types(obj: &Map<String, Value>) -> Result<Vec<(String, String)>> {
        let mut types_vec = Vec::new();
    
        for (key, value) in obj {
            let type_str = match value {
                Value::String(_) => "TEXT",
                Value::Number(_) => "INTEGER",
                Value::Bool(_) => "BOOLEAN",
                _ => {return Err(CacheControllerError::UnsupportedJsonType(key).into())},
            };
            types_vec.push((key.clone(), type_str.to_string()));
        }
        Ok(types_vec)
    }
}