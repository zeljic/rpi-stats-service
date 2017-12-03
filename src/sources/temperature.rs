extern crate rusqlite;

use std;
use sources::source::Source;
use std::sync::{Arc, Mutex};

use db::wrapper::Wrapper;

#[derive(Debug)]
pub struct Temperature<T> where T: Wrapper   {
    name: String,
    conn: Arc<Mutex<T>>
}

impl<T> Source<T> for Temperature<T> where T: Wrapper
{
    fn new(name: &str, conn: Arc<Mutex<T>>) -> Self {
        Temperature {
            name: name.to_string(),
            conn
        }
    }

    fn tick(&self) {
        let conn = self.conn.lock().unwrap();

        conn.insert(self.name.as_str(), std::f64::consts::PI);
    }
}