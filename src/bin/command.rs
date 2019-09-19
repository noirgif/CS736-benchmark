use std::process::Command;

fn main() {
    let mut list_dir = Command::new("./target/debug/uc lat");

    // Execute `ls` in the current directory of the program.
    list_dir.status().expect("process failed to execute");

    println!("...starting...");

    // Change `ls` to execute in the root directory.
    //list_dir.current_dir("/");

    // And then execute `ls` again but in the root directory.
   // list_dir.status().expect("process failed to execute");
}
