use std::io::prelude::*;
use std::net::TcpStream;
use std::net::UdpSocket;
use std::time::Duration;
use std::env;

#[macro_use]
mod measure;

const MTU: usize = 65000;

pub fn measure_latency(mut _socket: UdpSocket, num_repeat:u64) -> std::io::Result<()> {
    let mut in_buf = [1u8; 1 << 19];
    let out_buf = [1u8; 1 << 19];
    //let mut i = 0;
    let sizes = [
        4usize, 16, 64, 256, 1024, 4096, 16384, 64000, //65536, 262144, 524288,
    ];
   // const num_repeat: usize = 100;

    for _i in 0..sizes.len() {
        let msg_size = sizes[_i];
        for _j in 0..num_repeat {
            // client first receives then sends
            match _socket.recv(&mut in_buf[0..msg_size]) {
                Ok(_n) => {}
                Err(e) => {
                    println!("recv err on msg_size ({}): {:?}", msg_size, e);
                    break;
                }
            }

            match _socket.send(&out_buf[0..msg_size]) {
                Ok(_n) => {
                    //println!("Sent {} bytes", n);
                }
                Err(e) => {
                    println!("send error: {:?}", e);
                }
            }
        }
    }
    Ok(())
}

pub fn measure_throughput(mut _socket: UdpSocket) {
    // let mut array: [i32; 3] = [0; 3];
    //const MAX_MSG: usize = 1 << 26;
    let mut in_buf = vec![1u8; MTU];
    let mut out_buf;

    let mut total_received: u64 = 0;

    let sizes = [
        4usize, 16, 64, 256, 1024, 4096, 16384, 64000, // 65536, 262144, 524288,
    ];

    for _x in sizes.iter() {
        // read data first until end signal then send received bytes count
        //println!("waiting to recv");
        while in_buf[1] != 0u8 {
            match _socket.recv(&mut in_buf) {
                Ok(received) => {
                    total_received += received as u64;
                }
                Err(e) => {
                    println!("recv function failed: {:?}", e);
                }
            }
        }

        out_buf = u64::to_le_bytes(total_received);
        match _socket.send(&out_buf) {
            Ok(_sent) => {}
            Err(err) => {
                println!("{:?}", err);
            }
        };

        //println!("top bit = {}, net = {}", in_buf[1], total_received);
        // reset
        total_received = 0;
        in_buf[1] = 1u8;
    }
}

fn main() -> std::io::Result<()> {
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
    match socket.recv(&mut [1]) {
        Ok(_n) => {
            // println!("HS:: recv :: {} bytes", _n);
        }
        Err(e) => {
            println!("HS recv err: {:?}", e);
        }
    }
    match socket.send(&mut [1]) {
        Ok(_n) => {
            // println!("HS:: send:: {} bytes", _n);
        }
        Err(e) => {
            println!("HS send err: {:?}", e);
        }
    }

    // println!("\nEnd Shake...\n");

    const TIMEOUT: Duration = Duration::from_millis(10000);
    socket.set_read_timeout(Some(TIMEOUT))?;
    socket.set_write_timeout(Some(TIMEOUT))?;

    if test_type == "lat" {
        measure_latency(socket, num_repeat)?;
    } else {
        measure_throughput(socket);
    }
    // println!("\nDone!\n");

    Ok(())
}