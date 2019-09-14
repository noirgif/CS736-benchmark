extern crate libc;
use std::time::Duration;

fn main() {
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
    }
    
    unsafe {
        libc::clock_gettime(real_clock, &mut end);
    }

    let start_dur = Duration::new(start.tv_sec as u64, start.tv_nsec as u32);
    let end_dur = Duration::new(end.tv_sec as u64, end.tv_nsec as u32);

    print!("{}", (end_dur - start_dur).as_nanos());
}
