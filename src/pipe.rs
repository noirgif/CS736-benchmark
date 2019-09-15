// measure pipe latency
#![allow(unused_imports)]

use libc::{pipe, fork, c_int, c_void, read, write, kill, SIGKILL};
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
mod measure;


fn measure_latency() -> std::io::Result<()> {
    let mut f = File::create("pipe_latency")?;

    let pipe1 = &mut [0 as c_int, 0 as c_int];
    let pipe2 = &mut [0 as c_int, 0 as c_int];

    if unsafe {pipe(pipe1.as_mut_ptr())} != 0 || unsafe {pipe(pipe2.as_mut_ptr())} != 0 {
        panic!("Error creating pipe");
    }

    // maximum message size: 1024MiB
    const MAX_MSG: usize = 1 << 30;

    // times to loop
    const LOOP_NUM: usize = 10000;

    for msg_size in [4usize, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 524288].iter() {
        let lat : u64;

        let pid = unsafe { fork() };

        let mut buf = String::with_capacity(MAX_MSG);
        if pid != 0 {
            // parent process
            lat = gettime!((
                unsafe {
                    write(pipe1[1], " ".as_ptr() as *const c_void, *msg_size);
                    pipe1[1].flush();
                    read(pipe2[0], buf.as_mut_ptr() as *mut c_void, *msg_size);
                }
            ), LOOP_NUM);
            
            // because the child process is in an infinite loop, simply kill it.
            unsafe {
                kill(pid, SIGKILL);
            }
            write!(f, "{} {}\n", msg_size, lat as f64 / 2.)?;
        }
        else {
            for _i in 0..LOOP_NUM {
                // child process
                unsafe {
                    read(pipe1[0], buf.as_mut_ptr() as *mut c_void, *msg_size);
                    write(pipe2[1], " ".as_ptr() as *const c_void, *msg_size);
                }
            }
        }
    }
    Ok(())
}


fn measure_throughput() -> std::io::Result<()> {
    let mut f = File::create("pipe_throughput")?;

    let pipe1 = &mut [0 as c_int, 0 as c_int];
    let pipe2 = &mut [0 as c_int, 0 as c_int];

    if unsafe {pipe(pipe1.as_mut_ptr())} != 0 || unsafe {pipe(pipe2.as_mut_ptr())} != 0 {
        panic!("Error creating pipe");
    }

    // the time cost, in sec
    let time : f64;
    const MAX_MSG: usize = 1 << 29;


    let pid = unsafe { fork() };

    // the buffer 512MiB
    let mut buf = String::with_capacity(MAX_MSG);
    if pid != 0 {
        // parent process
        time = gettime!((
            unsafe {
                write(pipe1[1], buf.as_ptr() as *const c_void, MAX_MSG);
                read(pipe2[0], buf.as_mut_ptr() as *mut c_void, 1);
            }
        ), 10) as f64 / 1e9;
        
        // because the child process is in an infinite loop, simply kill it.
        unsafe {
            kill(pid, SIGKILL);
        }
    }
    else {
        loop {
            // child process
            unsafe {
                // read 512MiB
                read(pipe1[0], buf.as_mut_ptr() as *mut c_void, MAX_MSG);
                write(pipe2[1], " ".as_ptr() as *const c_void, 1);
            }
        }
    }
    write!(f, "{}\n", MAX_MSG as f64 / time)?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    measure_latency()?;
    measure_throughput()?;
    Ok(())
}