extern crate rusqlite;

use std::sync::{Arc, Mutex};
use db::wrapper::Wrapper;

pub trait Source<T> where T: Wrapper {
    fn new(name: &str, connection: Arc<Mutex<T>>) -> Self;
    fn tick(&self);
}