use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {

    let mut in_buf =    [1u8; 1 << 19];
    let mut out_buf =   [1u8; 1 << 19];

    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    stream.set_nodelay(true)?;
    let mut i = 0;
    let sizes = [4usize, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 524288];
    const num_repeat : usize = 100;

    for _i in 0..(10 * num_repeat) {
        // let mut n = 0;
        let read_size = sizes[_i / num_repeat];
        stream.read_exact(&mut in_buf[0..read_size])?;
        stream.write_all(&out_buf[0..read_size]);
        // println!("{ }", n);
        println!("{}", stream.nodelay().unwrap());
    }
    Ok(())
} 

fn testConnect() {
    if let Ok(stream) = TcpStream::connect("127.0.0.1:8080") {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }
}
