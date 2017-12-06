extern crate regex;

use std;
use std::sync::{Arc, Mutex};

use self::regex::Regex;

use db::wrapper::Wrapper;
use sources::source::Source;

#[derive(Debug)]
pub struct Memory<T> where T: Wrapper
{
    name: String,
    conn: Arc<Mutex<T>>
}

impl<T> Source<T> for Memory<T> where T: Wrapper {
    fn new(name: &str, connection: Arc<Mutex<T>>) -> Self {
        Memory {
            name: name.to_string(),
            conn: connection
        }
    }

    fn tick(&self) {
        let conn = self.conn.lock().unwrap();

        let what: String = String::from("");

        let re = Regex::new(r"Mem:\s+(\d+)\s+(\d+)").unwrap();

        println!("{:?}", re.captures(what.as_str()));

        for cap in re.captures_iter(what.as_str()) {
            let all = &cap[1];
            let used = &cap[2];

            conn.insert("memory_size", all.parse::<u32>().unwrap() as f64);
            conn.insert("memory_used", used.parse::<u32>().unwrap() as f64);
        }

        conn.insert(self.name.as_str(), std::f64::consts::PI);
    }
}