#[macro_use]
mod measure;

fn main() {
    let a = gettime!({}, 10000);
    println!("{}", a);
}