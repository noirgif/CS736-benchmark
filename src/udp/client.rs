#![allow(unused_imports)]
use core::arch::x86_64::__rdtscp;
use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::UdpSocket;
use std::time::Duration;

#[macro_use]
mod measure;

const MTU: usize = 65000;
const TSC_FREQ: f64 = 3.2e9;

pub fn measure_latency(mut _socket: UdpSocket, num_repeat: u64) -> std::io::Result<()> {
    let mut in_buf = [1u8; 1 << 19];
    let out_buf = [1u8; 1 << 19];
    //const num_repeat: usize = 100;

    println!("\n\n UDP Latency Test:");
    println!("{:10}\t\t{}", "MSG Size", "ns");
    for &msg_size in [
        4usize, 16, 64, 256, 1024, 4096, 16384, 64000//65536, 262144, 524288,
    ]
    .iter()
    {
        let lat: u64;

        // server is first sending  then reading  the test data
        lat = rdtscp!(
            {
                //println!("...Trying to send: {} bytes", msg_size);
                match _socket.send(&out_buf[0..msg_size]) {
                    Ok(_n) => {
                        //println!("Sent {} bytes", n);
                    }
                    Err(e) => {
                        println!("send error: {:?}", e);
                    }
                }
                match _socket.recv(&mut in_buf[0..msg_size]) {
                    Ok(_n) => {}
                    Err(e) => {
                        println!("recv err on msg_size ({}): {:?}", msg_size, e);
                        break;
                    }
                }
            },
            num_repeat
        );

        //println!("{:?}", buffer);
        println!(
            "{:10}\t\t{}",
            msg_size,
            lat as f64 / (2.0 * TSC_FREQ) * 1.0e+9
        );
        //println!("{}", _socket.nodelay().unwrap());
    }
    Ok(())
}

pub fn measure_throughput(mut _socket: UdpSocket, num_repeat: u64) -> std::io::Result<()> {
    println!("\n\nUDP Throughput Test:");
    println!("{:10}\t\t{}", "MSG Size", "MiBps");
    const MAX_MSG: usize = 1 << 26;

    let mut in_buf = [1u8; 8];
    let out_buf = vec![1u8; MAX_MSG];


    let mut aux: u32 = 0;
    let mut t1: u64;
    let mut t2: u64;

    for &msg_size in [
        4usize, 16, 64, 256, 1024, 4096, 16384, 64000, // 65536, 262144, 524288,
    ]
    .iter()
    {
        let tput;
        t1 = unsafe { __rdtscp(&mut aux) };
        // server is first sending  then reading  the test data
        for _x in 0..num_repeat {
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
                    Ok(_n) => {
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
                let total_data: u64 = u64::from_le_bytes(in_buf);
                //println!("total data transfer = {}, dt = {}", total_data, t2-t1);
                tput = total_data as f64 / (t2 - t1) as f64; // bytes per cycle
            }
            Err(e) => {
                println!("recv err on msg_size ({}): {:?}", msg_size, e);
                break;
            }
        }

        //println!("{:?}", buffer);
        println!(
            "{:7}\t\t{}",
            msg_size,
            tput / (1024 * 1024) as f64 * TSC_FREQ
        );
        //println!("{}", _socket.nodelay().unwrap());
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    // println!("\nPlease make sure server is started first!");

    // command line args
    let args: Vec<String> = env::args().collect();
    let test_type = &args[1];
    let num_repeat = match (args[2]).parse::<u64>() { Ok(n) => n, Err(_e) => 100};
    let self_addr = &args[3];
    let other_addr = &args[4];

    let socket = UdpSocket::bind(self_addr).expect("couldn't bind to address");
    socket
        .connect(other_addr)
        .expect("connect function failed");
    // initial handshake
    socket.set_read_timeout(Some(Duration::new(10, 0)))?;
    socket.set_write_timeout(Some(Duration::new(10, 0)))?;

    // println!("init handshake...");
    match socket.send(&mut [1]) {
        Ok(n) => {
            // println!("HS:: send:: {} bytes", n);
        }
        Err(e) => {
            println!("Handshake send err: {:?}", e);
        }
    }
    match socket.recv(&mut [1]) {
        Ok(n) => {
            // println!("HS:: recv :: {} bytes", n);
        }
        Err(e) => {
            println!("Handshake recv err: {:?}", e);
        }
    }

    // println!("\nEnd Shake...\n");

    const TIMEOUT: Duration = Duration::from_millis(10000);
    socket.set_read_timeout(Some(TIMEOUT))?;
    socket.set_write_timeout(Some(TIMEOUT))?;

    if test_type == "lat" {
        measure_latency(socket, num_repeat)?;
    } else {
        measure_throughput(socket, num_repeat)?;
    }
    // println!("\nDone!\n");
    // println!();

    Ok(())
}