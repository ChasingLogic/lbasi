extern crate lbasi;

use lbasi::*;

fn main() {
    println!("Starting");
    let answer = interpreter::run("33 + 25 + 10".to_string())
        .expect("Got an error running interpreter");
    println!("Stopping");
}
