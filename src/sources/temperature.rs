use std;
use sources::source::Source;
use std::sync::{Arc, Mutex};

use db::wrapper::Wrapper;

#[derive(Debug)]
pub struct Temperature<T> where T: Wrapper {
    name: String,
    conn: Arc<Mutex<T>>
}

impl<T> Source<T> for Temperature<T> where T: Wrapper
{
    fn new(name: &str, connection: Arc<Mutex<T>>) -> Self {
        Temperature {
            name: name.to_string(),
            conn: connection
        }
    }

    fn tick(&self) {
        let conn = self.conn.lock().unwrap();

        use std::error::Error;

        println!("{}", self.name);

        let result = std::process::Command::new("/opt/vc/bin/vcgencmd")
            .arg("measure_temp")
            .output();

        match result {
            Ok(v) => {
                println!("Read: {:?}", v);
            }
            Err(e) => {
                println!("Error: {}", e.description());
            }
        }

        conn.insert(self.name.as_str(), std::f64::consts::PI);
    }
}