use std::process::{Command, Stdio};

pub fn t1() {
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

pub fn t2() {
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

    t1();
    t2();

    println!("\n\nAll tests have concluded!\n\n");
}
