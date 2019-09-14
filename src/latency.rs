#[macro_use]
mod measure;

fn main() {
    let a = rdtscp!({}, 100000);
    println!("{}", a);
}