// measure pipe latency
#![allow(unused_imports)]

use libc::{pipe, fork, c_int, c_void, read, write, kill, SIGKILL};

#[macro_use]
mod measure;

fn measure_latency() {
    let pipe1 = &mut [0 as c_int, 0 as c_int];
    let pipe2 = &mut [0 as c_int, 0 as c_int];

    if unsafe {pipe(pipe1.as_mut_ptr())} != 0 || unsafe {pipe(pipe2.as_mut_ptr())} != 0 {
        panic!("Error creating pipe");
    }

    // maximum message size: 512MiB
    const MAX_MSG: usize = 1 << 29;

    for msg_size in [4usize, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 524288].iter() {
        let lat : u64;

        let pid = unsafe { fork() };

        let mut buf = String::with_capacity(MAX_MSG);
        if pid != 0 {
            // parent process
            lat = gettime!((
                unsafe {
                    write(pipe1[1], " ".as_ptr() as *const c_void, *msg_size);
                    read(pipe2[0], buf.as_mut_ptr() as *mut c_void, *msg_size);
                }
            ), 100000);
            
            // because the child process is in an infinite loop, simply kill it.
            unsafe {
                kill(pid, SIGKILL);
            }
        }
        else {
            loop {
                // child process
                unsafe {
                    read(pipe1[0], buf.as_mut_ptr() as *mut c_void, *msg_size);
                    write(pipe2[1], " ".as_ptr() as *const c_void, *msg_size);
                }
            }
        }

        println!("Size: {}, latency: {}ns", msg_size, lat as f64 / 2.);
    }
}


fn measure_throughput() {
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
    println!("Pipe throughput: {}B/sec", MAX_MSG as f64 / time);
}

fn main() {
    measure_latency();
    measure_throughput();
}