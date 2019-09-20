use std::io::prelude::*;
use std::net::TcpStream;

const NUM_LOOP : usize = 100000;

// Measures the latency, client receives the message and pass it back
fn measure_latency(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut buf = [1u8; 1 << 19];
    //let mut i = 0;
    let sizes = [
        4usize, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 524288,
    ];

    for _i in 0..(10 * NUM_LOOP) {
        // let mut n = 0;
        let read_size = sizes[_i / NUM_LOOP];
        stream.read_exact(&mut buf[0..read_size])?;
        stream.write_all(&buf[0..read_size])?;
    }
    Ok(())
}

// Measure the throughput
fn measure_throughput(stream: &mut TcpStream) -> std::io::Result<()> {
    // let mut array: [i32; 3] = [0; 3];
    const TOTAL_RECV: usize = 1 << 30;
    // The receiving buffer
    let mut in_buf = vec![1u8; TOTAL_RECV];
    // The replying message(ACK)
    let out_buf = [1u8; 1];

    let sizes = [
        4usize, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 524288,
    ];

    for _i in 0..sizes.len() {
        // let mut n = 0;
        // let read_size = sizes[_i / NUM_LOOP];
        stream.read_exact(&mut in_buf)?;
        stream.write_all(&out_buf)?;
    }
    Ok(())
}

fn print_err_msg(args0 : &str) -> !
{
    println!("{} (latency|throughput) SERVER_NAME:PORT",  args0);
    std::process::exit(1);
}

fn main() -> std::io::Result<()> {
    let args : std::vec::Vec<String> = std::env::args().collect();
    // Whether latency is measured, if false, the throughput is measured
    let is_latency : bool;

    if args.len() < 3 {
        print_err_msg(&args[0]);
    }

    // determine to measure latency, or throughput, or neither
    match &args[1].chars().nth(0).unwrap() {
        'l' => is_latency = true,
        't' => is_latency = false,
        _ => print_err_msg(&args[0]),
    }

    let mut stream = TcpStream::connect(&args[2]).unwrap();
    stream.set_nodelay(true)?;
    if is_latency {
        measure_latency(&mut stream)?;
    }
    else {
        measure_throughput(&mut stream)?;
    }  
    Ok(())
}
