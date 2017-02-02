extern crate lbasi;

use lbasi::*;

use std::io;
use std::io::Write;

fn main() {
    loop {

        print!("calc > ");
        io::stdout().flush().unwrap();

        // read the user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();


        if input == "exit" || input == "quit" {
            break;
        }

        let res = interpreter::run(input);
        match res {
            Ok(x) => println!("Answer: {}", x),
            Err(e) => println!("Error: {}", e),
        };
    }
}
