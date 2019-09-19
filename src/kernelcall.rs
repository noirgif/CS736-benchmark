extern crate libc;
use std::time::Duration;
extern crate core;
use core::arch::x86_64::__rdtscp;

#[macro_use]
mod measure;

fn main() {

    let cyc = rdtscp!({unsafe {libc::getuid();}}, 100000);
    println!("{}", cyc);

    let lat = gettime!({unsafe {libc::getuid();}}, 100000);
    println!("rdtscp diff = {}ns", lat);
}