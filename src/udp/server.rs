#![allow(unused_imports)]
use core::arch::x86_64::__rdtscp;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::UdpSocket;
use std::time::Duration;

#[macro_use]
mod measure;

const MTU: usize = 16384;
const CPU_GHZ: f64 = 2.0e9;

/**
 * These two utilities will convert back and forth and u32 integer!
 */
fn int_to_bytes(x: u32) -> [u8; 4] {
    [
        ((x >> 24) & 0xff) as u8,
        ((x >> 16) & 0xff) as u8,
        ((x >> 8) & 0xff) as u8,
        (x & 0xff) as u8,
    ]
}

fn bytes_to_int(bytes: &[u8; 4]) -> u32 {
    ((bytes[0] as u32) << 24)
        | ((bytes[1] as u32) << 16)
        | ((bytes[2] as u32) << 8)
        | (bytes[3] as u32)
}

fn send_long_msg(mut _socket: UdpSocket, out_buf: &[u8], msg_size: usize) {
    let mut size = MTU;

    while (size > 0) {
        match _socket.send(&out_buf[0..size]) {
            Ok(n) => println!("Sent {} bytes", n),
            Err(e) => println!("error: {:?}", e),
        }
        size -= MTU;
    }
}

fn measure_latency(mut _socket: UdpSocket) -> std::io::Result<()> {
    let mut in_buf = [1u8; 1 << 19];
    let out_buf = [1u8; 1 << 19];
    const num_repeat: usize = 2;

    for &msg_size in [
        4usize, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 524288,
    ]
    .iter()
    {
        let lat: u64;

        // server is first sending  then reading  the test data
        lat = rdtscp!(
            {
                //println!("...Trying to send: {} bytes", msg_size);
                if msg_size > MTU {
                    let mut remaining = msg_size;
                    let mut size = MTU;

                    while size > 0 {
                        //println!("While top");
                        match _socket.send(&out_buf[0..size]) {
                            Ok(_n) => {
                                //println!("Multi::Sent {} bytes", _n);
                            }
                            Err(e) => {
                                println!("send error: {:?}", e);
                                break;
                            }
                        }
                        remaining = remaining - size;
                        size = if remaining > MTU { MTU } else { remaining };
                        //println!("while bottom");
                    }
                } else {
                    match _socket.send(&out_buf[0..msg_size]) {
                        Ok(n) => {
                            //println!("Sent {} bytes", n);
                        }
                        Err(e) => {
                            println!("send error: {:?}", e);
                        }
                    }
                };

                let mut net_received = 0;
                while net_received < msg_size {
                    //println!("waiting to receive");
                    match _socket.recv(&mut in_buf[0..msg_size]) {
                        Ok(received) => {
                            //println!("received {} bytes", received);
                            net_received += received;
                        }
                        Err(e) => {
                            println!("recv err on msg_size ({}): {:?}", msg_size, e);
                            break;
                        }
                    }
                }
                // skip if not correct message received
                if net_received != msg_size {
                    continue;
                }
            },
            num_repeat
        );

        //println!("{:?}", buffer);
        println!(
            "<size, cpb, c, ns> = <{}, {}, {}, {}>",
            msg_size,
            lat as f32 / (2.0 * msg_size as f32),
            lat,
            lat as f64 / CPU_GHZ
        );
        //println!("{}", _socket.nodelay().unwrap());
    }
    Ok(())
}

fn measure_throughput(mut _socket: UdpSocket) -> std::io::Result<()> {
    const MAX_MSG: usize = 1 << 26;

    let mut in_buf = [1u8; 4];
    let out_buf = vec![1u8; MAX_MSG];

    const PACKET_MULTIPLIER: usize = 1000;

        let mut aux: u32 = 0;
        let mut t1: u64;
        let mut t2: u64;

    for &msg_size in [
        4usize, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 524288,
    ]
    .iter()
    {
        let tput;
        t1 = unsafe { __rdtscp(&mut aux) };
        // server is first sending  then reading  the test data
        for _x in 0..PACKET_MULTIPLIER {
            //println!("...Trying to send: {} bytes", msg_size);
            if msg_size > MTU {
                let mut remaining = msg_size;
                let mut size = MTU;

                while size > 0 {
                    //println!("While top");
                    match _socket.send(&out_buf[0..size]) {
                        Ok(_n) => {
                            //println!("Multi::Sent {} bytes", _n);
                        }
                        Err(e) => {
                            println!("send error: {:?}", e);
                            break;
                        }
                    }
                    remaining = remaining - size;
                    size = if remaining > MTU { MTU } else { remaining };
                    //println!("while bottom");
                }
            } else {
                match _socket.send(&out_buf[0..msg_size]) {
                    Ok(n) => {
                        //println!("Sent {} bytes", n);
                    }
                    Err(e) => {
                        println!("send error: {:?}", e);
                    }
                }
            };
        }

        // at last indicate the end by sending zeroes
        for _x in 0..1 {
            match _socket.send(&[0u8; 4]) {
                Ok(_n) => {
                    //println!("Multi::Sent {} bytes", _n);
                }
                Err(e) => {
                    println!("send error: {:?}", e);
                    break;
                }
            }
        }

        // read back ACK
        //println!("waiting to receive");
        match _socket.recv(&mut in_buf) {
            Ok(_received) => {
                t2 = unsafe { __rdtscp(&mut aux) };
                let total_data = bytes_to_int(&in_buf);
                println!("total data transfer = {}, dt = {}", total_data, t2-t1);
                tput = total_data as f64 / (t2 - t1) as f64; // bytes per cycle
            }
            Err(e) => {
                println!("recv err on msg_size ({}): {:?}", msg_size, e);
                break;
            }
        }

        //println!("{:?}", buffer);
        println!(
            "<{}, {} MiBps>",
            msg_size,
            tput/(1024*1024) as f64 * CPU_GHZ
        );
        //println!("{}", _socket.nodelay().unwrap());
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("\nPlease make sure client is started first!");

    let socket = UdpSocket::bind("127.0.0.1:3333").expect("couldn't bind to address");
    socket
        .connect("127.0.0.1:8080")
        .expect("connect function failed");
    // initial handshake
    socket.set_read_timeout(Some(Duration::new(10, 0)))?;
    socket.set_write_timeout(Some(Duration::new(10, 0)))?;

    println!("init handshake...");
    match socket.send(&mut [1]) {
        Ok(n) => {
            println!("HS:: send:: {} bytes", n);
        }
        Err(e) => {
            println!("HS send err: {:?}", e);
        }
    }
    match socket.recv(&mut [1]) {
        Ok(n) => {
            println!("HS:: recv :: {} bytes", n);
        }
        Err(e) => {
            println!("HS recv err: {:?}", e);
        }
    }

    println!("\nEnd Shake...\n");

    const TIMEOUT: Duration = Duration::from_millis(10000);
    socket.set_read_timeout(Some(TIMEOUT))?;
    socket.set_write_timeout(Some(TIMEOUT))?;

    println!("\nMeasuring latency...\n");
    //measure_latency(socket)?;
    measure_throughput(socket)?;
    println!("\nDone!\n");

    println!();

    Ok(())
}
