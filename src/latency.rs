#[macro_use]
mod measure;

fn main() {
    let a = rdtscp!({}, 10000);
    println!("{}", a);
}