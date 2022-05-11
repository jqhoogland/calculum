use std::io::{self, Write};

use crate::ucum::annotation;
use crate::ucum::unit;

mod ucum;


fn main() {
    let mut input = String::new();

    println!("\n-- Calculum ---------------------------------------------------------------\n");

    loop {
        print!(">>> ");
        io::stdout().flush();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let tokenizer = annotation::tokenizer::Tokenizer::new(input.trim());
        for token in tokenizer {
            println!("{:?}", token);
        }

        input.clear()
    }
}