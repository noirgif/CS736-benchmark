// measure pipe latency
#![allow(unused_imports)]

use libc::{c_int, c_void, fork, kill, pipe, read, write, SIGKILL};
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
mod measure;

fn measure_latency() -> std::io::Result<()> {
    let pipe1 = &mut [0 as c_int, 0 as c_int];
    let pipe2 = &mut [0 as c_int, 0 as c_int];

    if unsafe { pipe(pipe1.as_mut_ptr()) } != 0 || unsafe { pipe(pipe2.as_mut_ptr()) } != 0 {
        panic!("Error creating pipe");
    }

    // times to loop
    const LOOP_NUM: usize = 1;

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
            unsafe {
                // kill(pid, SIGKILL);
            }
            results.push((msg_size, lat));
        } else {
            for _i in 0..LOOP_NUM {
                // child process
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
    for (i, j) in results {
        f.write_fmt(format_args!("{} {}\n", i, j))?;
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
    const TOTAL_SENT: usize = 1 << 30;

    let sizes = [4, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 524288];

    for &msg_size in sizes.iter() {
        // the buffer 1GiB
        let mut buf = String::with_capacity(TOTAL_SENT);
        let pid = unsafe { fork() };
        if pid != 0 {
            // parent process
            results.push(
                gettime!(
                    (unsafe {
                        let mut n = 0usize;
                        while n < TOTAL_SENT {
                            let result = write(
                                pipe1[1],
                                (buf.as_ptr() as *const c_void).offset(n as isize),
                                if n + msg_size > TOTAL_SENT {
                                    TOTAL_SENT - n
                                } else {
                                    msg_size
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
                    1
                ) as f64
                    / 1e9,
            );
            // because the child process is in an infinite loop, simply kill it.
            // print!("\rresults {:?}", results);
            unsafe {
                kill(pid, SIGKILL);
            }
        } else {
            // child process
            unsafe {
                // read TOTAL_SENT bytes
                let mut n = 0usize;
                // receiving the message
                while n < TOTAL_SENT {
                    let result = read(
                        pipe1[0],
                        (buf.as_mut_ptr() as *mut c_void).offset(n as isize),
                        TOTAL_SENT - n,
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
    }

    let mut f = File::create("pipe_throughput")?;
    for (&msg_size, &time) in sizes.iter().zip(results.iter()) {
        write!(f, "{} {:.3}\n", msg_size, TOTAL_SENT as f64 / time)?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    measure_latency()?;
    measure_throughput()?;
    Ok(())
}
