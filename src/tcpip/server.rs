use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;

#[macro_use]
mod measure;

fn main(){
    let mut buffer = [0,0,0];
    let mut listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    match listener.accept() {
        Ok((mut _socket, addr)) => {
                _socket.set_nodelay(true);
                let x = rdtscp!({
                    _socket.write(&[1,2,3]);
                    _socket.read(&mut buffer);
                }, 10000);
                println!("{:?}", buffer);
                println!("time = {}", x/2);
            },
        Err(e) => println!("couldn't get client: {:?}", e),
    }
}
