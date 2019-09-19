use std::io::prelude::*;
use std::net::TcpStream;
use std::net::UdpSocket;
use std::time::Duration;

#[macro_use]
mod measure;

const MTU: usize = 16384;

pub fn measure_latency(mut _socket: UdpSocket) -> std::io::Result<()> {
    let mut in_buf = [1u8; 1 << 19];
    let out_buf = [1u8; 1 << 19];
    //let mut i = 0;
    let sizes = [
        4usize, 16, 64, 256, 1024, 4096, 16384, 65536, //262144, 524288,
    ];
    const NUM_REPEAT: usize = 1000000;

    for _i in 0..8 {
        let msg_size = sizes[_i];
        for _j in 0..NUM_REPEAT {
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
    let mut out_buf = [1u8; 8];

    let mut total_received: u64 = 0;

    let sizes = [
        4usize, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 524288,
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
    let socket = UdpSocket::bind("127.0.0.1:8080").expect("couldn't bind to address");
    socket
        .connect("127.0.0.1:3333")
        .expect("connect function failed");

    // initial handshake
    socket.set_read_timeout(Some(Duration::new(10, 0)))?;
    socket.set_write_timeout(Some(Duration::new(10, 0)))?;
    println!("init handshake...");
    match socket.recv(&mut [1]) {
        Ok(n) => {
            println!("HS:: recv :: {} bytes", n);
        }
        Err(e) => {
            println!("HS recv err: {:?}", e);
        }
    }
    match socket.send(&mut [1]) {
        Ok(n) => {
            println!("HS:: send:: {} bytes", n);
        }
        Err(e) => {
            println!("HS send err: {:?}", e);
        }
    }

    println!("\nEnd Shake...\n");

    const TIMEOUT: Duration = Duration::from_millis(10000);
    socket.set_read_timeout(Some(TIMEOUT))?;
    socket.set_write_timeout(Some(TIMEOUT))?;

    println!("\nMeasuring latency...\n");
    measure_latency(socket)?;
    println!("\nMeasuring throughput...\n");
    //measure_throughput(socket);
    println!("\nDone!\n");

    Ok(())
}

pub fn measure_latency_mtu(mut _socket: UdpSocket) -> std::io::Result<()> {
    let mut in_buf = [1u8; 1 << 19];
    let out_buf = [1u8; 1 << 19];
    //let mut i = 0;
    let sizes = [
        4usize, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 524288,
    ];
    const NUM_REPEAT: usize = 2;

    for _i in 0..10 {
        let msg_size = sizes[_i];
        for _j in 0..NUM_REPEAT {
            // client first receives then sends

            let mut net_received = 0;
            while net_received < msg_size {
                //println!("Waiting to receive: {}", msg_size - net_received);
                match _socket.recv(&mut in_buf[0..msg_size]) {
                    Ok(received) => {
                        //println!("received {} bytes of {}", received, msg_size);
                        net_received += received;
                        // println!(
                        //     "net_received = {}, cond = {}",
                        //     net_received,
                        //     net_received < msg_size
                        // );
                    }
                    Err(e) => {
                        println!("recv function failed: {:?}", e);
                        break;
                    }
                }
            }

            //println!("...Trying to send: {} bytes", msg_size);
            if msg_size > MTU {
                let mut remaining = msg_size;
                let mut size = MTU;

                while size > 0 {
                    match _socket.send(&out_buf[0..size]) {
                        Ok(n) => {
                            //println!("Multi::Sent {} bytes", n);
                        }
                        Err(e) => {
                            println!("send error: {:?}", e);
                            break;
                        }
                    }
                    remaining = remaining - size;
                    size = if remaining > MTU { MTU } else { remaining };
                    //println!("remaining = {}", remaining);
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
    }
    Ok(())
}
