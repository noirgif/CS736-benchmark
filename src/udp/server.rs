#![allow(unused_imports)]
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::UdpSocket;
use std::time::Duration;

#[macro_use]
mod measure;

const MTU: usize = 16384;

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

        // server is first reading then writing the test data
        lat = rdtscp!(
            {
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

                //println!("...Trying to send: {} bytes", msg_size);
                if msg_size > MTU {
                    let mut remaining = msg_size;
                    let mut size = MTU;

                    while size > 0 {
                        //println!("While top");
                        match _socket.send(&out_buf[0..size]) {
                            Ok(_n) => {
                                //{println!("Multi::Sent {} bytes", n); },
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
            },
            num_repeat
        );

        //println!("{:?}", buffer);
        println!(
            "<size, cycles/byte> = <{}, {}>>",
            msg_size,
            lat as f32 / (2.0 * msg_size as f32)
        );
        //println!("{}", _socket.nodelay().unwrap());
    }
    Ok(())
}

fn measure_throughput(mut _socket: UdpSocket) -> std::io::Result<()> {
    const MAX_MSG: usize = 1 << 26;

    let mut in_buf = [1u8; 1];
    let out_buf = vec![1u8; MAX_MSG];

    // send some MiB data
    // read back 1 byte
    let lat: u64 = rdtscp!(
        {

            // ...... todo

        },
        100
    );

    println!(
        "throughput = {} M bytes per {} cycles",
        MAX_MSG / (1024 * 1024),
        lat
    );
    Ok(())
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:3333").expect("couldn't bind to address");
    socket
        .connect("127.0.0.1:8080")
        .expect("connect function failed");
    // initial handshake
    socket.set_read_timeout(Some(Duration::new(5, 0)))?;
    socket.set_write_timeout(Some(Duration::new(5, 0)))?;
    socket.send(&mut [1])?;

    const TIMEOUT: Duration = Duration::from_millis(100);
    socket.set_read_timeout(Some(TIMEOUT))?;
    socket.set_write_timeout(Some(TIMEOUT))?;

    println!("\nMeasuring latency...\n");
    measure_latency(socket)?;
    println!("\nDone!\n");

    println!();

    Ok(())
}
