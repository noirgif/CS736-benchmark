use std::process::{Command, Stdio};

fn main() {
    println!("\nStarting Tests ...\n");
    // let mut client = Command::new("./target/debug/uc lat")
    // .stderr(Stdio::null())
    // .stdout(Stdio::null())
    // .stderr(Stdio::null())
    // .spawn();

    let mut server = Command::new ("./target/debug/us thp").expect("Unable to start server");

    // Execute `ls` in the current directory of the program.
    //list_dir.status().expect("process failed to execute");

    println!("...starting...");

    // Change `ls` to execute in the root directory.
    //list_dir.current_dir("/");

    // And then execute `ls` again but in the root directory.
   // list_dir.status().expect("process failed to execute");
}
