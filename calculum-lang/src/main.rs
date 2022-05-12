use std::io::{self, Write};

use crate::ucum::term;
use crate::ucum::unit;

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

        let unit_term = term::UnitTerm::new(input.trim()).unwrap();
        for unit in unit_term.units.iter() {
            println!("{:?}", unit);
        }

        input.clear()
    }
}