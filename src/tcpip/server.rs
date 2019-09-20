#![allow(unused_imports)]
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

#[macro_use]
mod measure;

// TSC Frequency in GHz
const TSC_FREQ : f64 = 3.2;
const NUM_LOOP : i64 = 100000;

fn measure_latency(mut _socket: TcpStream) -> std::io::Result<()> {
    let mut buf = vec![1u8; 1 << 19];
    let mut results = vec![];

    for &msg_size in [
        4usize, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 524288,
    ]
    .iter()
    {
        let lat: u64;

        lat = rdtscp!(
            {
                _socket.write_all(&buf[0..msg_size])?;
                _socket.read_exact(&mut buf[0..msg_size])?;
            },
            NUM_LOOP
        );

        results.push((msg_size, lat as f64 / TSC_FREQ / 2.));
    }

    for &(msg_size, lat) in results.iter() {
        println!("{} {}", msg_size, lat);
    }
    Ok(())
}

fn measure_throughput(mut _socket: TcpStream) -> std::io::Result<()> {
    const MAX_MSG: usize = 524288;
    const TOTAL_SENT: usize = 1 << 30;

    let mut in_buf = vec![1u8; 1];
    let out_buf = vec![1u8; MAX_MSG];
    let mut results = vec![];

    // send some MiB data
    // read back 1 byte
    for &msg_size in [
        4usize, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 524288,
    ]
    .iter()
    {
        let lat: u64;

        lat = rdtscp!(
            {  
                let mut n = 0usize;
                while n < TOTAL_SENT {
                    n += _socket.write(&out_buf[0..msg_size])?;
                }
                _socket.read_exact(&mut in_buf)?;
            },
            1
        );

        results.push((msg_size, lat as f64 / TSC_FREQ));
    }
    
    // print the speed for different packets, in MB/s
    for &(msg_size, lat) in results.iter() {
        println!("{} {}", msg_size, msg_size as f64 / (1024. * 1024.) / lat * 1e9);
    }
    Ok(())
}

fn print_err_msg(args0: &str) -> ! {
    println!("{} (latency|throughput) [SERVER_ADDRESS:PORT]", args0);
    std::process::exit(1);
}

fn main() -> std::io::Result<()> {
    let args: std::vec::Vec<String> = std::env::args().collect();
    // Whether latency is measured, if false, the throughput is measured
    let is_latency: bool;

    if args.len() < 2 {
        print_err_msg(&args[0]);
    }

    match args[1].as_ref() {
        "latency" => is_latency = true,
        "throughput" => is_latency = false,
        _ => print_err_msg(&args[0]),
    }

    // decide the binding address, default 127.0.0.1:8080
    let bind_address = if args.len() >= 3 {
        &args[2]
    } else {
        "127.0.0.1:8080"
    };

    // bind to address
    let listener = TcpListener::bind(bind_address).unwrap();
    println!("Binding to {} ...", bind_address);
    match listener.accept() {
        Ok((socket, _addr)) => {
            socket.set_nodelay(true)?;
            if is_latency{
                measure_latency(socket)?;
            }
            else {
                measure_throughput(socket)?;
            }
        }
        Err(e) => println!("couldn't get client: {:?}", e),
    }
    Ok(())
}
