//! # Expressions
//!
//! ## Todos
//! - [ ] Support for parentheses (`(...)`).
//! - [ ] Support for exponentiation (`a^b`).
//! - [ ] Support for comparisons (binary & unary). Departure from excel -> `==` & `!=`
//!   - [ ] Support for assessing commensurability (`~~`).
//! - [ ] Support for strings (`'...'`, `"..."`).
//! - [ ] Support for string concatenation (`a & b`).
//! - [ ] Support for variable assignmet.
//! - [ ] Support for unit terms with unary division (`/s`)
//! - [ ] Support for automatic conversions.
//!
use std::{fmt, ops};
use crate::ucum::term;


#[derive(PartialEq, Clone)]
pub struct Quantity {
    mag: f64,
    term: Option<term::UnitTerm>
}

impl Quantity {
    pub fn new(mag: f64, t: &str) -> Self {
        Quantity {
            mag,
            term: term::UnitTerm::new(t),
        }
    }
}

impl ops::AddAssign for Quantity {
    fn add_assign(&mut self, other: Self) {
        self.mag += other.mag;
        match &mut self.term {
            Some(t) => *t += other.term.unwrap(),
            None => assert!(other.term.is_none())
        }
    }
}

impl ops::SubAssign for Quantity {
    fn sub_assign(&mut self, other: Self) {
        self.mag -= other.mag;
        match &mut self.term {
            Some(t) => *t -= other.term.unwrap(),
            None => assert!(other.term.is_none())
        }
    }
}

impl ops::MulAssign for Quantity {
    fn mul_assign(&mut self, other: Self) {
        self.mag *= other.mag;
        match &mut self.term {
            Some(t) => *t *= other.term.unwrap(),
            None => assert!(other.term.is_none())
        }
    }
}

impl ops::DivAssign for Quantity {
    fn div_assign(&mut self, other: Self) {
        self.mag /= other.mag;
        match &mut self.term {
            Some(t) => *t /= other.term.unwrap(),
            None => assert!(other.term.is_none())
        }
    }
}


impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.term {
            Some(t) => write!(f, "{} '{}'", self.mag, t),
            None => write!(f, "{}", self.mag),
        }
    }
}

impl fmt::Debug for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.term {
            Some(t) => write!(f, "{} '{:?}'", self.mag, t),
            None => write!(f, "{}", self.mag),
        }
    }
}

pub mod tokenizer {
    use std::fmt;
    use std::collections::VecDeque;

    use crate::ucum::{unit, term};
    use super::Quantity;

    #[derive(PartialEq)]
    pub enum Token {
        Add,
        Sub,
        Mul,
        Div,
        Quantity(super::Quantity)
    }

    impl fmt::Debug for Token {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Add => write!(f, "+"),
                Self::Sub => write!(f, "-"),
                Self::Mul => write!(f, "*"),
                Self::Div => write!(f, "/"),
                Self::Quantity(q) => write!(f, "({:?})", q),
            }
        }
    }

    #[derive(Debug, )]
    pub struct Tokenizer<'a> {
        chars: std::str::Chars<'a>,
        visited: VecDeque<char>,
    }

    impl<'a> Tokenizer<'a> {
        pub fn new(s: &'a str) -> Self {
            Tokenizer {
                chars: s.chars(),
                visited: VecDeque::new(),
            }
        }

        fn visited_to_mag(&mut self) -> f64 {
            let mag: f64 = self.visited.iter().collect::<String>().parse().unwrap();
            self.visited.clear();
            mag
        }

        fn next_mag(&mut self) -> f64 {
            loop {
                match self.chars.next() {
                    Some(c) => match c {
                        '.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                            self.visited.push_back(c)
                        },
                        _ => {
                            let mag = self.visited_to_mag();
                            self.visited.push_back(c);
                            return mag
                        }
                    },
                    None => {
                        return self.visited_to_mag()
                    }
                }
            }
        }

        /// This is redundant with `term::UnitTerm.next_annotation()`.
        /// Except for within an annotation, we're not allowed to use
        /// whitespace in unit terms.
        fn next_annotation(&mut self) {
            loop {
                match self.chars.next() {
                    Some(c) => match c {
                        '}' => {
                            self.visited.push_back('}');
                            break
                        },
                        _ => self.visited.push_back(c)
                    },
                    None => break
                }
            }
        }

        fn next_term(&mut self) -> String {
            if self.visited[0] == ' ' {
                self.visited.pop_front();
            }
            loop {
                match self.chars.next() {
                    Some(c) => match c {
                        ' ' => break,
                        '{' => self.next_annotation(),
                        _ => self.visited.push_back(c),
                    },
                    None => break
                }
            }
            let term = self.visited.iter().collect::<String>();
            self.visited.clear();
            term
        }

        pub fn next_quantity(&mut self, c_0: char) -> Option<Token> {
            self.visited.push_back(c_0);
            let mag: f64 = self.next_mag();
            let maybe_term: String = self.next_term();
            Some(Token::Quantity(Quantity::new(mag, &maybe_term)))
        }
    }

    impl<'a> Iterator for Tokenizer<'a> {
        type Item = Token;

        fn next(&mut self) -> Option<Self::Item> {
            let maybe_c = self.visited
                .pop_front()
                .or_else(|| self.chars.next());

            match maybe_c {
                Some(c) => match c {
                    '+' => Some(Token::Add),
                    '-' => Some(Token::Sub),
                    '*' => Some(Token::Mul),
                    '/' => Some(Token::Div),
                    '.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
                        => self.next_quantity(c),
                    ' ' => self.next(),
                    _ => panic!("[Syntax Error] Encountered an invalid token {:?}.", c)
                },
                None => None
            }
        }
    }
}

mod interpreter {
    use super::{tokenizer, Quantity};
    use super::tokenizer::Token;

    pub fn interpret(s: &str) -> Quantity {
        let mut ts: tokenizer::Tokenizer = tokenizer::Tokenizer::new(s);

        let next_qn = |ts: &mut tokenizer::Tokenizer, err: &str| {
            match ts.next().unwrap() {
                Token::Quantity(qn) => qn,
                _ => panic!("{}", err)
            }
        };

        let mut qn = next_qn(
            &mut ts, "[Syntax Error] Your statement must begin with a quantity."
        );

        let err = "[Syntax Error] Two quantities must be separated by one of the following tokens: '+', '-', '*', '/'";

        loop {
            match ts.next() {
                Some(t) => match t {
                    Token::Add => { qn += next_qn(&mut ts, err) },
                    Token::Sub => { qn -= next_qn(&mut ts, err) },
                    Token::Mul => { qn *= next_qn(&mut ts, err) },
                    Token::Div => { qn /= next_qn(&mut ts, err) },
                    Token::Quantity(qn) => panic!("{}", err)
                },
                None => break
            }
        }
        qn
    }
}

pub use interpreter::interpret;


#[cfg(test)]
mod tests {
    use crate::ucum::{unit, term};
    use super::*;

    fn tokens(s: &str) -> Vec<tokenizer::Token> {
        tokenizer::Tokenizer::new(s).collect()
    }

    fn quantity(mag: f64, term: &str) -> tokenizer::Token {
        tokenizer::Token::Quantity(Quantity::new(mag, term))
    }

    #[test]
    fn it_reads_the_next_quantity() {
        assert_eq!(tokens("1 kg.m/s2")[0], quantity(1., "kg.m/s2"));
    }

    #[test]
    fn it_ignores_spaces_bw_mag_and_term() {
        assert_eq!(tokens("1 kg.m/s2"), tokens("1kg.m/s2"));
    }

    #[test]
    fn it_reads_add() {
        let ts = tokens("1 kg.m/s2 + 5 kg.m/s2");

        assert_eq!(ts, vec![
            quantity(1., "kg.m/s2"),
            tokenizer::Token::Add,
            quantity(5., "kg.m/s2")
        ])
    }

    #[test]
    fn it_reads_sub() {
        let ts = tokens("1 kg.m/s2 - 5 kg.m/s2");

        assert_eq!(ts, vec![
            quantity(1., "kg.m/s2"),
            tokenizer::Token::Sub,
            quantity(5., "kg.m/s2")
        ])
    }

    #[test]
    fn it_reads_mul() {
        let ts = tokens("1 kg.m/s2 * 5 s2/kg/m");

        assert_eq!(ts, vec![
            quantity(1., "kg.m/s2"),
            tokenizer::Token::Mul,
            quantity(5., "s2/kg/m")
        ])
    }

    #[test]
    fn it_reads_div() {
        let ts = tokens("1 kg.m/s2 / 5 kg.m/s2");

        assert_eq!(ts, vec![
            quantity(1., "kg.m/s2"),
            tokenizer::Token::Div,
            quantity(5., "kg.m/s2")
        ])
    }

    // -- Interpreter

    #[test]
    #[should_panic]
    fn it_requires_separator_between_quantities() {
        interpret("1 kg.m/s 2 s/m");
    }

    #[test]
    fn it_multiplies_quantities() {
        assert_eq!(interpret("1 kg.m/s2 * 2 s/m"), Quantity::new(2., "kg/s"));
    }

    #[test]
    fn it_divides_quantities() {
        assert_eq!(interpret("1 kg.m/s2 / 2 m/s"), Quantity::new(0.5, "kg/s"));
    }

    #[test]
    fn it_adds_quantities_with_the_same_units() {
        assert_eq!(interpret("1 kg.m/s2 + 5 N"), Quantity::new(6., "N"))
    }

    #[test]
    fn it_subs_quantities_with_the_same_units() {
        assert_eq!(interpret("10 kg.m/s2 - 5 N"), Quantity::new(5., "N"))
    }

    #[test]
    #[should_panic]
    fn it_doesnt_add_quantities_with_different_units() {
        println!("{}", interpret("10 kg.m/s + 5 N"));
    }

    #[test]
    #[should_panic]
    fn it_doesnt_sub_quantities_with_different_units() {
        println!("{}", interpret("10 kg.m/s2 - 5 kg/m.s2"));
    }
}