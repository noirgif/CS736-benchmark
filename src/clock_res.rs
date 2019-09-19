#![allow(unused)]
#![feature(asm)]
#![allow(unused_unsafe)]

#[macro_use]
mod measure;

fn rdtscp_resolution() {

    // TSC Freq, about the same as CPU freq
    const freq : f64 = 3.2;
    let mut res : i64 = 0;

    // dummy instructions
    let nop_time = rdtscp!({ unsafe {
        // asm!("inc r12" : "={r12}"(res) : "{r12}"(res) : : "intel", "volatile");
        asm!("nop" :::: "volatile");
        asm!("nop" :::: "volatile");
        asm!("nop" :::: "volatile");
        asm!("nop" :::: "volatile");
        asm!("nop" :::: "volatile");
        // asm!("nop");
    } }, 1000000);

    let empty_time = rdtscp!({ unsafe {} }, 1000000);

    println!("rdtscp {}", (nop_time - empty_time) as isize as f64 / freq);
}


use libc::{clock_getres, CLOCK_REALTIME, timespec};

fn gettime_resolution() {
    
    let mut res : i64 = 0;

    let nop_time = gettime!({ unsafe {
        asm!("nop" :::: "volatile");
        asm!("nop" :::: "volatile");
        asm!("nop" :::: "volatile");
        asm!("nop" :::: "volatile");
        asm!("nop" :::: "volatile");
        asm!("nop" :::: "volatile");
        asm!("nop" :::: "volatile");
        asm!("nop" :::: "volatile");
        asm!("nop" :::: "volatile");
        // asm!("inc r12" : "={r12}"(res) : "{r12}"(res) : : "intel", "volatile");
    } }, 1000000);

    let empty_time = gettime!({ unsafe {} }, 100000);

    println!("gettime {}", (nop_time - empty_time) as isize);
    let mut res = timespec {tv_sec: 0, tv_nsec: 0};
    unsafe {clock_getres(CLOCK_REALTIME, &mut res); }
    println!("{}", res.tv_nsec);
}


fn main() {
    gettime_resolution();
    rdtscp_resolution();
}
