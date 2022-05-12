
mod tokenizer {
    use std::collections::VecDeque;
    use crate::ucum::unit::Unit;


    #[derive(Debug, PartialEq)]
    pub enum Token {
        Int(i32),
        Float(f64),
        Add,
        Sub,
        Mul,
        Div,
        OpenParen,
        CloseParen,
        Exp,
        Unit(Unit)
    }

    #[derive(Debug, )]
    pub struct Tokenizer<'a> {
        // Required in case we need to backtrack `10.1` -> `10.1` but `10.a` -> `10`, `.`, `a`
        visited: VecDeque<char>,
        chars: std::str::Chars<'a>,
    }
}

#[cfg(test)]
mod tests {
}