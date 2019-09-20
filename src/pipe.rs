// measure pipe latency
#![allow(unused_imports)]

use libc::{c_int, c_void, fork, kill, pipe, read, write, SIGKILL};
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
mod measure;

// times to loop
const LOOP_NUM: usize = 100000;
const LOOP_NUM_TPUT: usize = 5;

fn measure_latency() -> std::io::Result<()> {
    let pipe1 = &mut [0 as c_int, 0 as c_int];
    let pipe2 = &mut [0 as c_int, 0 as c_int];

    if unsafe { pipe(pipe1.as_mut_ptr()) } != 0 || unsafe { pipe(pipe2.as_mut_ptr()) } != 0 {
        panic!("Error creating pipe");
    }


    let mut results = vec![];
    let sizes = [
        4usize, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 524288,
    ];

    for &msg_size in sizes.iter() {
        let lat: u64;

        let pid = unsafe { fork() };

        let mut buf = vec![0u8; msg_size];

        if pid != 0 {
            // parent process
            lat = gettime!(
                (unsafe {
                    let mut n = 0usize;
                    while n < msg_size {
                        let result = write(
                            pipe1[1],
                            (buf.as_ptr() as *const c_void).offset(n as isize),
                            msg_size - n,
                        );
                        if result > 0 {
                            n += result as usize;
                        } else {
                            panic!("Error when writing to pipe!")
                        }
                    }
                    n = 0;
                    while n < msg_size {
                        let result = read(
                            pipe2[0],
                            (buf.as_mut_ptr() as *mut c_void).offset(n as isize),
                            msg_size - n,
                        );
                        if result > 0 {
                            n += result as usize;
                        } else {
                            panic!("Error when reading from pipe!");
                        }
                    }
                }),
                LOOP_NUM
            );
            // because the child process is in an infinite loop, simply kill it.
            // edit no need to kill
            // unsafe {
                // kill(pid, SIGKILL);
            // }
            results.push((msg_size, lat));
        } else {
            // Child process
            for _i in 0..LOOP_NUM {
                let mut n = 0usize;
                // receiving the message
                while n < msg_size {
                    let result = unsafe {
                        read(
                            pipe1[0],
                            (buf.as_mut_ptr() as *mut c_void).offset(n as isize),
                            msg_size - n,
                        )
                    };
                    if result > 0 {
                        n += result as usize;
                    } else {
                        panic!("Error when reading from pipe!")
                    }
                }
                n = 0;
                // return the same message
                while n < msg_size {
                    let result = unsafe {
                        write(
                            pipe2[1],
                            (buf.as_ptr() as *const c_void).offset(n as isize),
                            msg_size - n,
                        )
                    };
                    if result > 0 {
                        n += result as usize;
                    } else {
                        panic!("Error when writing to pipe!");
                    }
                }
            }
            std::process::exit(0);
        }
    }

    let mut f = File::create("pipe_latency")?;
    for (msg_size, lat) in results {
        f.write_fmt(format_args!("{} {}\n", msg_size, lat as f64 / 2.))?;
    }
    Ok(())
}

fn measure_throughput() -> std::io::Result<()> {
    let pipe1 = &mut [0 as c_int, 0 as c_int];
    let pipe2 = &mut [0 as c_int, 0 as c_int];

    if unsafe { pipe(pipe1.as_mut_ptr()) } != 0 || unsafe { pipe(pipe2.as_mut_ptr()) } != 0 {
        panic!("Error creating pipe");
    }

    // the time cost, in sec
    let mut results = vec![];
    // the buffer is 1GiB
    const TOTAL_SENT: usize = 1 << 30;
    // packet sizes
    let sizes = [4, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 524288];
    let mut buf = vec![1u8; 524288];
    let pid = unsafe { fork() };

    if pid != 0 {
        // parent process
        for &msg_size in sizes.iter() {
            results.push(
                gettime!(
                    (unsafe {
                        let mut n = 0usize;
                        while n < TOTAL_SENT {
                            let result = write(
                                pipe1[1],
                                buf.as_ptr() as *const c_void,
                                if n + msg_size <= TOTAL_SENT {
                                    msg_size
                                } else {
                                    TOTAL_SENT - n
                                },
                            );
                            if result > 0 {
                                n += result as usize;
                                // println!("{}", n);
                            } else {
                                panic!("Error when writing to pipe!")
                            }
                        }
                        let result: isize;
                        result = read(pipe2[0], buf.as_mut_ptr() as *mut c_void, 1);
                        if result <= 0 {
                            panic!("Error when reading from pipe!");
                        }
                    }),
                   LOOP_NUM_TPUT 
                ) as f64
                    / 1e9,
            );
        }
        // because the child process is in an infinite loop, simply kill it.
        // print!("\rresults {:?}", results);
        // std::io::stdout().flush()?;
        unsafe {
            kill(pid, SIGKILL);
        }
    } else {
        // child process
        for i in 0..(LOOP_NUM_TPUT * sizes.len()) {
            let msg_size = sizes[i / LOOP_NUM_TPUT];
            unsafe {
                // read TOTAL_SENT bytes
                let mut n = 0usize;
                // receiving the message
                while n < TOTAL_SENT {
                    let result = read(
                        pipe1[0],
                        buf.as_mut_ptr() as *mut c_void,
                        msg_size,
                    );
                    if result > 0 {
                        n += result as usize;
                    } else {
                        panic!("Error when reading from pipe!")
                    }
                }
                let result = write(pipe2[1], " ".as_ptr() as *const c_void, 1);
                if result <= 0 {
                    panic!("Error writing to pipe!");
                }
            }
        }
        std::process::exit(0);
    }

    let mut f = File::create("pipe_throughput")?;
    for (&msg_size, &time) in sizes.iter().zip(results.iter()) {
        write!(f, "{} {:.3}\n", msg_size, TOTAL_SENT as f64 / (1024 * 1024) as f64 / time)?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args : std::vec::Vec<String> = std::env::args().collect();
    if &args[1] == "lat" {
        measure_latency()?;
    }
    else if &args[1] == "tput" {
        measure_throughput()?;
    }
    else {
        eprintln!("{} [lat|tput]", &args[0]);
    }
    Ok(())
}
