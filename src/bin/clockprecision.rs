#![allow(unused)]
use std::time::{Duration, Instant, SystemTime};

fn main() {
    const LOOP_TIMES : u64 = 10000;
    let mut minimum : u64 = 0;
    let now = SystemTime::now();
    // for j in 1..1 {}
    let elapsed = now.elapsed().unwrap();
    print!("{}", elapsed.as_nanos());
}
