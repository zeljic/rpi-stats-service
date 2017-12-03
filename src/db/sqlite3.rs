extern crate rusqlite;

use std::sync::{Mutex, Arc};
use std::path::Path;
use std::error::Error;

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
                conn.execute("CREATE TABLE IF NOT EXISTS stats (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT,
                `source` VARCHAR(16),
                `time` BIG INTEGER,
                `value` REAL
                );", &[]).unwrap();

                conn.execute("VACUUM;", &[]).unwrap_or_else(|e| {
                    println!("{:?}", e);
                    0
                });
            }
            Err(e) => {
                println!("{:?}", e.description())
            }
        }
    }

    fn insert(&self, source: &str, value: f64)
    {
        let conn = self.conn.lock().unwrap();

        use std::time::{SystemTime, UNIX_EPOCH, Duration};

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs() as f64;

        conn.execute("insert into stats (`source`, `value`, `t`) values (?1, ?2, ?3);", &[&source, &value, &current_time]).unwrap();
    }
}