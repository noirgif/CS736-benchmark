use std::process::{Command, Stdio};

fn main() {
    println!("\nStarting Tests ...\n");
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

    // Execute `ls` in the current directory of the program.
    //list_dir.status().expect("process failed to execute");

    println!("...starting...");

    // Change `ls` to execute in the root directory.
    //list_dir.current_dir("/");

    // And then execute `ls` again but in the root directory.
   // list_dir.status().expect("process failed to execute");
}
