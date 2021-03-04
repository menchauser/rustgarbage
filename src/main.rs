extern crate json;

use std::env;
use std::fs;
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = fs::read_to_string(&args[1]).unwrap();
    let start = Instant::now();
    let parsed = json::parse(&text).unwrap();
    let elapsed = start.elapsed();
    println!("Parsed {} messages during {} us", parsed["messages"].len(), elapsed.as_micros());
}
