pub trait Wrapper
{
    fn new() -> Self;
    fn init(&self);
    fn insert(&self, source: &str, value: f64);
}