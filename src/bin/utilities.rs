

/**
 * These two utilities will convert back and forth and u32 integer!
 */
fn int_to_bytes(x: u32) -> [u8; 4] {
    [
        ((x >> 24) & 0xff) as u8,
        ((x >> 16) & 0xff) as u8,
        ((x >> 8) & 0xff) as u8,
        (x & 0xff) as u8,
    ]
}

fn bytes_to_int(bytes: &[u8; 4]) -> u32 {
    ((bytes[0] as u32) << 24)
        | ((bytes[1] as u32) << 16)
        | ((bytes[2] as u32) << 8)
        | (bytes[3] as u32)
}

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
