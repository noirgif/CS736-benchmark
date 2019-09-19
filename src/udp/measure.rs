//pub mod measure {
/* Return the cycles the operation takes */
#![allow(unused_imports, unused_macros)]

macro_rules! rdtscp {
    ($op:block, $rep:expr) => ({
        extern crate core;
        use core::arch::x86_64::__rdtscp;

        let mut aux : u32 = 0;
        let mut t1 : u64;
        let mut t2 : u64;
        let mut min = -1i32 as u64;
        for _i in 0..($rep) {
            t1 =  unsafe {__rdtscp(&mut aux)};
            $op
            t2 =  unsafe {__rdtscp(&mut aux)};
            if t2 - t1 < min {
                min = t2 - t1;
            }
        }
        min
    })
}


/* Return the time the operation takes */
macro_rules! gettime {
    ($op:stmt, $rep:expr) => ({
        extern crate libc;
        use libc::{clock_gettime, CLOCK_REALTIME};

        let mut start = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let mut end = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };

        let mut min = -1i32 as u64;

        for i in 1..($rep)
        {
            unsafe {
                libc::clock_gettime(CLOCK_REALTIME, &mut start);
            }
            $op
            unsafe {
                libc::clock_gettime(CLOCK_REALTIME, &mut end);
            }

            let time = ((end.tv_sec - start.tv_sec) * 1000000000 + (end.tv_nsec - start.tv_nsec)) as u64;
            if time < min
            {    min = time;}
        }
        min
    })
}

//}