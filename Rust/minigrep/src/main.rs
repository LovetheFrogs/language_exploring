// Bring into scope std::env to make use of the std::env::args function.
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}