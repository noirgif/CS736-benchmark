#![feature(asm)]
#![allow(unused)]

extern crate libc;
use std::time::Duration;
extern crate core;
use core::arch::x86_64::__rdtscp;

#[macro_use]
mod measure;

fn main() {
    // TSC Frequency in GHz
    const TSC_FREQ: f64 = 3.2;
    let mut pid: libc::pid_t = -1;
    let cyc_getpid = rdtscp!(
        { unsafe { asm!("syscall" : "={rax}"(pid) : "{rax}"(39) :: "intel", "volatile") } },
        100000
    );
    println!("getpid {}", cyc_getpid as f64 / TSC_FREQ);

    // println!("{}", pid);

    let mut uid: libc::uid_t = 0xFFFF;
    let cyc_getuid = rdtscp!(
        { unsafe { asm!("syscall" : "={rax}"(uid) : "{rax}"(102) :: "intel", "volatile") } },
        100000
    );
    println!("getuid {}", cyc_getuid as f64 / TSC_FREQ);

    //println!("{}", uid);
}
