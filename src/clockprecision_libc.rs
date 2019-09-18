#![feature(asm)]

#[macro_use]
mod measure;

use libc::{clock_getres, CLOCK_REALTIME, timespec};

fn main() {
    
    let mut res : i64 = 0;

    let nop_time = gettime!({ unsafe {
        // asm!("nop");
        // asm!("nop");
        // asm!("nop");
        // asm!("nop");
        // asm!("nop");
        asm!("inc r12" : "={r12}"(res) : "{r12}"(res) : : "intel");
    } }, 100000);

    let empty_time = gettime!({ unsafe {} }, 100000);

    println!("{} {} {} {}", empty_time, nop_time, nop_time - empty_time, res);
    
    let mut res = timespec {tv_sec: 0, tv_nsec: 0};
    unsafe {clock_getres(CLOCK_REALTIME, &mut res); }
    println!("{}", res.tv_nsec);
}
