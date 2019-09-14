extern crate libc;
use std::time::Duration;
extern crate core;
use core::arch::x86_64::__rdtscp;

fn main() {
    //pub unsafe extern fn getuid() -> pid_t

   let real_clock = libc::CLOCK_REALTIME;
    let mut start = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut end = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe {
        libc::clock_gettime(real_clock, &mut start);
        for i in 0..1000 {
            libc::getuid();
        }
        libc::clock_gettime(real_clock, &mut end);
    }  
    
    let start_dur = Duration::new(start.tv_sec as u64, start.tv_nsec as u32);
    let end_dur = Duration::new(end.tv_sec as u64, end.tv_nsec as u32);

    println!("Duration = {}ns", (end_dur - start_dur).as_nanos());

    unsafe {
        println!("{}", libc::getuid());
        //println!("{}", libc::gettimeofday()");
    }

      let mut aux : u32 = 0;
        let t1 : u64;
        let t2 : u64;

     unsafe { 
        t1 =  __rdtscp(&mut aux);
        //for i in 0..1000 {
        libc::getuid();
        //} 
        t2 =  __rdtscp(&mut aux);
    }
    println!("rdtscp diff = {}ns", (t2-t1)/2);
}