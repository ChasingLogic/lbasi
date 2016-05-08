extern crate lbasi;

use lbasi::*;

fn main() {
    println!("Starting");
    let answer = interpreter::run("33 + 25 + 10".to_string())
        .expect("Got an error running interpreter");

    println!("Answer for addition: {}", answer);

    let sub_answer = interpreter::run("10 - 2 - 4".to_string())
        .expect("Got an error running interpreter");

    println!("Answer for subtraction: {}", sub_answer);

    println!("Stopping");
}
