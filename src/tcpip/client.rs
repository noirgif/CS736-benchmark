use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {

    let mut buffer = [0,0,0,0];

    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    stream.set_nodelay(true)?;
    stream.read(&mut buffer)?;
    stream.write(&[1,2,3]);
    Ok(())
} 

fn testConnect() {
    if let Ok(stream) = TcpStream::connect("127.0.0.1:8080") {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }
}
