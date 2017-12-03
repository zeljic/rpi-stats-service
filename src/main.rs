extern crate rusqlite;

mod web;
mod db;
mod sources;

use std::sync::{Mutex, Arc};

use db::wrapper::Wrapper;
use db::sqlite3::SQLite3;

fn get_connection<T>() -> Arc<Mutex<T>> where T: Wrapper
{
    Arc::new(Mutex::new(T::new()))
}

fn main()
{
    let conn = get_connection::<SQLite3>();

    {
        conn.lock().unwrap().init();
    }
}