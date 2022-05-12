use std::io::{self, Write};

use crate::ucum::expression;

mod ucum;


fn main() {
    let mut input = String::new();

    println!("\n-- Calculum ---------------------------------------------------------------\n");

    loop {
        print!(">>> ");
        io::stdout().flush().expect("There was an error printing to stdout.");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("{:?}", expression::interpret(input.trim()));

        input.clear()
    }
}