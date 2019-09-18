use std::io::prelude::*;
use std::net::TcpStream;
use std::net::UdpSocket;

#[macro_use]
mod measure;

const MTU: usize = 16384;

fn measure_latency(mut _socket: UdpSocket) -> std::io::Result<()> {
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
            println!("...Trying to send: {} bytes", msg_size);

            if msg_size > MTU {
                let mut remaining = msg_size;
                let mut size = MTU;

                while size > 0 {
                    match _socket.send(&out_buf[0..size]) {
                        Ok(n) => println!("Multi::Sent {} bytes", n),
                        Err(e) => println!("error: {:?}", e),
                    }
                    remaining = remaining - size;
                    size = if remaining > MTU { MTU } else { remaining };
                }
            } else {
                match _socket.send(&out_buf[0..msg_size]) {
                    Ok(n) => println!("Sent {} bytes", n),
                    Err(e) => {
                        println!("error: {:?}", e);
                    }
                }
            };

            let mut net_received = 0;
            while net_received < msg_size {
                println!("Waiting to receive: {}", msg_size - net_received);
                match _socket.recv(&mut in_buf[0..msg_size]) {
                    Ok(received) => {
                        println!("received {} bytes", received);
                        net_received += received;
                    }
                    Err(e) => println!("recv function failed: {:?}", e),
                }
            }
        }
    }
    Ok(())
}

fn measure_throughput(mut _socket: UdpSocket) {
    // let mut array: [i32; 3] = [0; 3];
    const MAX_MSG: usize = 1 << 26;
    let mut in_buf = vec![1u8; MAX_MSG];
    let out_buf = [1u8; 1];

    const MAX_REPEAT: u32 = 100;
    println!("\nBefore");
    for _i in 0..MAX_REPEAT {
        // .... todo
    }
    println!("\nAfter");
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080").expect("couldn't bind to address");
    socket
        .connect("127.0.0.1:3333")
        .expect("connect function failed");

    println!("\nMeasuring latency...\n");
    measure_latency(socket);
    println!("\nDone!\n");

    Ok(())
}
