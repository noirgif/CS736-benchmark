#![allow(unused)]
#![feature(asm)]
#![allow(unused_unsafe)]

#[macro_use]
mod measure;

use std::collections::BTreeSet;

// TSC frequency, in GHz
const TSC_FREQ : f64 = 3.2;

fn rdtscp_resolution() {

    // TSC Freq, about the same as CPU freq
    const freq : f64 = 3.2;
    let mut res : i64 = 0;

    // Ordered set
    let mut set = BTreeSet::new();

    // dummy instructions
    for i in 0..100000 {
        set.insert(rdtscp!({ unsafe {
            asm!("nop" :::: "volatile");
            // asm!("nop" :::: "volatile");
        } }, 1));
    }

    for i in 0..100000 {
        set.insert(rdtscp!({ unsafe {} }, 1));
    }

    
    const MAX_DIFF : i64 = 99999;
    let mut min_diff = MAX_DIFF;

    // iterate possible time in ascending order
    // to find the smallest difference
    let mut current = 0u64;
    for &i in set.iter()
    {
        if i > current && ((i - current) as i64) < min_diff {
            min_diff = (i - current) as i64;
        }
        current = i;
    }
    

    println!("rdtscp {}", min_diff as f64 / TSC_FREQ);
}


use libc::{clock_getres, CLOCK_REALTIME, timespec};

fn gettime_resolution() {
    let mut res : i64 = 0;
    let mut set = BTreeSet::new();

    // dummy instructions
    for i in 0..5000 {
        set.insert(gettime!({ unsafe {
            // asm!("inc r12" : "={r12}"(res) : "{r12}"(res) : : "intel", "volatile");
            asm!("nop" :::: "volatile");
            asm!("nop" :::: "volatile");
            asm!("nop" :::: "volatile");
            asm!("nop" :::: "volatile");
            // asm!("nop");
        } }, 1));
    }

    for i in 0..5000 {
        set.insert(gettime!({ unsafe {} }, 1));
    }

    
    const MAX_DIFF : i64 = 99999;
    let mut min_diff = MAX_DIFF;

    // iterate possible time in ascending order
    // to find the smallest difference
    let mut current = 0u64;
    for &i in set.iter()
    {
        if i > current && ((i - current) as i64) < min_diff {
            min_diff = (i - current) as i64;
        }
        current = i;
    }
    

    println!("clock_gettime {}", min_diff);

    let mut res = timespec {tv_sec: 0, tv_nsec: 0};
    unsafe {clock_getres(CLOCK_REALTIME, &mut res); }
    println!("clock_getres {}", res.tv_nsec);
}


fn main() {
    gettime_resolution();
    rdtscp_resolution();
}
