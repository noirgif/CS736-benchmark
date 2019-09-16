
#![allow(unused_imports)]
use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::UdpSocket;


#[macro_use]
mod measure;

fn measure_latency(mut _socket: UdpSocket)  {
    let mut in_buf =    [1u8; 1 << 19];
    let mut out_buf = [1u8; 1 << 19];

    for &msg_size in [4usize, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 524288].iter() {
        let lat : u64;
    
        lat = rdtscp!({
            _socket.send(&out_buf[0..msg_size]); 
            match _socket.recv(&mut in_buf[0..msg_size]){
                Ok(received) => {                
                    println!("received {} bytes", received);
                },
               Err(e) => println!("recv function failed: {:?}", e),
            }
            }, 100);

        //println!("{:?}", buffer);
        println!("{} = {}", msg_size, lat as f32/2.0);
        //println!("{}", _socket.nodelay().unwrap());
    }
}

fn measure_throughput(mut _socket: UdpSocket) -> std::io::Result<()> {
    const MAX_MSG: usize = 1 << 26;

    let mut in_buf = [1u8; 1];
    let out_buf = vec![1u8; MAX_MSG];

    // send some MiB data
    // read back 1 byte
     let lat: u64 = rdtscp!({

         // ...... todo

    }, 100);

    println!("throughput = {} M bytes per {} cycles", MAX_MSG/(1024*1024), lat);
    Ok(())
}

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:3333").expect("couldn't bind to address");
    socket.connect("127.0.0.1:8080").expect("connect function failed");
   
    println!("\nMeasuring latency...\n");
    measure_latency(socket);
    println!("\nDone!\n");

    println!();
}


