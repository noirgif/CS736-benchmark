#![feature(asm)]
#![allow(unused_unsafe)]

#[macro_use]
mod measure;

fn main() {

    let mut res : i64 = 0;

    let nop_time = rdtscp!({ unsafe {
        asm!("inc r12" : "={r12}"(res) : "{r12}"(res) : : "intel");
        asm!("inc r12" : "={r12}"(res) : "{r12}"(res) : : "intel");
        // asm!("inc r12" : "={r12}"(res) : "{r12}"(res) : : "intel");
        // asm!("inc r12" : "={r12}"(res) : "{r12}"(res) : : "intel");
        asm!("nop");
        asm!("nop");
        asm!("nop");
    } }, 100000);

    let empty_time = rdtscp!({ unsafe {} }, 100000);

    println!("{} {} {} {}", empty_time, nop_time, nop_time - empty_time, res);
}
