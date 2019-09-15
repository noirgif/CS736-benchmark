
#![allow(unused_imports)]
use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;


#[macro_use]
mod measure;



fn measure_latency(mut _socket: TcpStream) {
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
    println!("{}", _socket.nodelay().unwrap());
 }
 
}

fn measure_throughput() {

}

fn main(){
    let mut listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    match listener.accept() {
        Ok((mut _socket, addr)) => {
               _socket.set_nodelay(true);
               measure_latency(_socket);
            },
        Err(e) => println!("couldn't get client: {:?}", e),
    }
}


