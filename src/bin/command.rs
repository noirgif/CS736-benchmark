use std::process::{Command, Stdio};

pub fn run_udp_lat() {
    // UDP Latency
    let _client = Command::new("./target/debug/uc")
        .arg("lat")
        .arg("1000")
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    let _server = Command::new("./target/debug/us")
        .arg("lat")
        .arg("1000")
        .status()
        .expect("Unable to start server");
}

pub fn run_udp_tp() {
    // UDP Throughput
    let _client = Command::new("./target/debug/uc")
        .arg("tp")
        .arg("1000")
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    let _server = Command::new("./target/debug/us")
        .arg("tp")
        .arg("1000")
        .status()
        .expect("Unable to start server");
}


fn main() {
    println!("\nStarting Tests ...\n");

    run_udp_lat();
    run_udp_tp();

    println!("\n\nAll tests have concluded!\n\n");
}
