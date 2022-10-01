pub mod web;

pub trait Fuzzer {
    fn new(source: &str, target: &str, verbose: bool) -> Self;
    fn run(self);
}