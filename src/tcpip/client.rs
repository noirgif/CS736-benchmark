use std::io::prelude::*;
use std::net::TcpStream;


fn measure_latency (mut stream: TcpStream) {    
    let mut in_buf =    [1u8; 1 << 19];
    let mut out_buf =   [1u8; 1 << 19];
    //let mut i = 0;
    let sizes = [4usize, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 524288];
    const num_repeat : usize = 100;

    for _i in 0..(10 * num_repeat) {
        // let mut n = 0;
        let read_size = sizes[_i / num_repeat];
        stream.read_exact(&mut in_buf[0..read_size]);
        stream.write_all(&out_buf[0..read_size]);
        // println!("{ }", n);
        //println!("{}", stream.nodelay().unwrap());
    }
}

fn measure_throughput (mut stream: TcpStream){
    // let mut array: [i32; 3] = [0; 3];
    const MAX_MSG: usize = 1 << 26;
    
    let mut in_buf = vec![1u8; MAX_MSG];
    let out_buf =   [1u8; 1];

    const MAX_REPEAT: u32 = 100;
    println!("\nBefore");
    for _i in 0..MAX_REPEAT {
        println!("Try read:\n");
        stream.read_exact(&mut in_buf);
        println!("Try write:\n");
        stream.write_all(&out_buf);
    }
    println!("\nAfter");
}

fn main() -> std::io::Result<()>{
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    stream.set_nodelay(true)?;

    measure_throughput(stream);
  Ok(())
} 
