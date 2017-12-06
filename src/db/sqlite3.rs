extern crate rusqlite;

use std::sync::{Mutex, Arc};
use std::path::Path;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use db::sqlite3::rusqlite::Connection;
use db::wrapper::Wrapper;

#[derive(Debug)]
pub struct SQLite3 {
    conn: Arc<Mutex<Connection>>
}

impl Wrapper for SQLite3 {
    fn new() -> Self {
        let conn: Connection = Connection::open(Path::new("db.sqlite")).unwrap();

        SQLite3 {
            conn: Arc::new(Mutex::new(conn))
        }
    }

    fn init(&self)
    {
        match self.conn.lock() {
            Ok(conn) => {
                conn.execute(r"CREATE TABLE IF NOT EXISTS stats (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT,
                `source` VARCHAR(16),
                `time` BIG INTEGER,
                `value` REAL
                );", &[]).unwrap();
            }
            Err(e) => {
                println!("{:?}", e.description())
            }
        }
    }

    fn insert(&self, source: &str, value: f64)
    {
        let current_time: u32 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs() as u32;

        match self.conn.lock() {
            Ok(conn) => {
                conn.execute("INSERT INTO stats (`source`, `value`, `time`) VALUES (?1, ?2, ?3);", &[&source, &value, &current_time]).unwrap();
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}

impl Drop for SQLite3 {
    fn drop(&mut self) {
        self.conn.lock().unwrap().execute("VACUUM;", &[]).unwrap();
        println!("DROP: {:?}", self);
    }
}