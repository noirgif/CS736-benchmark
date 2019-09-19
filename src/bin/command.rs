use std::process::{Command, Stdio};

fn main() {
    println!("\nStarting Tests ...\n");

    // UDP Latency
    let client = Command::new("./target/debug/uc")
    .arg("lat")
    .arg("1000000")
    .stderr(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn();

    let server = Command::new ("./target/debug/us")
    .arg("lat")
    .arg("1000000")
    .status().expect("Unable to start server");


    // UDP Throughput
    let client = Command::new("./target/debug/uc")
    .arg("lat")
    .arg("1000000")
    .stderr(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn();

    let server = Command::new ("./target/debug/us")
    .arg("lat")
    .arg("1000000")
    .status().expect("Unable to start server");
    
    println!("\n\nAll tests have concluded!\n\n");
}
