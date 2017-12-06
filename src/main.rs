#![feature(never_type)]

mod web;
mod db;
mod sources;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use db::wrapper::Wrapper;
use db::sqlite3::SQLite3;
use sources::memory::Memory;
use sources::temperature::Temperature;
use sources::source::Source;

fn get_connection<T>() -> Arc<Mutex<T>> where T: Wrapper
{
    Arc::new(Mutex::new(T::new()))
}

fn main()
{
    let conn = get_connection::<SQLite3>();
    let mut threads: Vec<thread::JoinHandle<!>> = Vec::new();

    {
        conn.lock().unwrap().init();
    }

    let memory_conn = Arc::clone(&conn);

    threads.push(thread::spawn(|| {
        let source = Memory::new("memory", memory_conn);

        loop {
            source.tick();

            thread::sleep(Duration::from_millis(1000));
        }
    }));

    let temperature_conn = Arc::clone(&conn);

    threads.push(thread::spawn(|| {
        let source = Temperature::new("temperature", temperature_conn);

        loop {
            source.tick();

            thread::sleep(Duration::from_millis(1000));
        }
    }));


    threads.into_iter().for_each(move |thread| {

        thread.join().unwrap();

    });
}