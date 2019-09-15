
#![allow(unused_imports)]
use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;


#[macro_use]
mod measure;



fn measure_latency(mut _socket: TcpStream)  {
    let mut in_buf =    [1u8; 1 << 19];
    let mut out_buf =   [1u8; 1 << 19];

 for &msg_size in [4usize, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 524288].iter() {
    let lat : u64;
  
    lat = rdtscp!({
        _socket.write_all(&out_buf[0..msg_size]);        
        _socket.read_exact(&mut in_buf[0..msg_size]);
    }, 100);

    //println!("{:?}", buffer);
    println!("{} = {}",msg_size, lat as f32/2.0);
    //println!("{}", _socket.nodelay().unwrap());
 }

}

fn measure_throughput(mut _socket: TcpStream) -> std::io::Result<()> {
    const MAX_MSG: usize = 1 << 20;

    let mut in_buf =    [1u8; 1];
    let out_buf : [u8; MAX_MSG] =    [1u8; MAX_MSG];

    // send some MiB data
    // read back 1 byte
     let lat: u64 = rdtscp!({
        _socket.write(&out_buf);        
        _socket.read(&mut in_buf);
    }, 100);

    println!("throughput = {} bytes per {} cyles", MAX_MSG, lat as f32/MAX_MSG as f32);
    Ok(())
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    match listener.accept() {
        Ok((mut _socket, addr)) => {
               _socket.set_nodelay(true);
               measure_throughput(_socket);
            },
        Err(e) => println!("couldn't get client: {:?}", e),
    }

}


