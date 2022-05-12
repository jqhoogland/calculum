//! # Unit Terms
//! - [ ] Support for parentheticals `(...)`. (Currently interpreted
//!   as part of a symbol)
//! - [ ] Support for exponents `10^`
//!
use std::ops;
use std::slice::Iter;
use std::fmt::{self, Display};

use crate::ucum::unit;


pub mod tokenizer {
    use std::iter::Iterator;
    use std::collections::VecDeque;

    use crate::ucum::unit;

    #[derive(Debug, PartialEq)]
    pub enum Token {
        Mul,
        Div,
        Unit(unit::Unit),
        Int(i8),
    }

    #[derive(Debug, )]
    pub struct Tokenizer<'a> {
        // Required in case we need to backtrack `10.1` -> `10.1` but `10.a` -> `10`, `.`, `a`
        visited: VecDeque<char>,
        chars: std::str::Chars<'a>,
    }

    impl<'a> Tokenizer<'a> {
        pub fn new(s: &'a str) -> Self {
            Tokenizer {
                visited: VecDeque::new(),
                chars: s.chars(),
            }
        }
    }

    trait Tokenizes {
        type Item;
        fn read_annotation(&mut self) -> Option<String>;
        fn read_int(&mut self, _: char) -> Option<Self::Item>;
        fn read_unit(&mut self, _: char) -> Option<Self::Item>;
    }

    impl<'a> Tokenizes for Tokenizer<'a> {
        type Item = Token;

        fn read_annotation(&mut self) -> Option<String> {
            let mut subs = String::from("");
            let mut maybe_c = self.chars.next();

            loop {
                match maybe_c {
                    None | Some('}') => break,
                    Some(c) => subs.push(c),
                };

                maybe_c = self.chars.next();
            }

            maybe_c.map(|_| subs)
        }
        
        fn read_int(&mut self, first_digit: char) -> Option<Self::Item> {
            let mut subs = String::from("");
            subs.push(first_digit);

            let mut maybe_c = self.chars.next();

            loop {
                match maybe_c {
                    Some(c) => match c {
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => subs.push(c),
                        _ => {
                            self.visited.push_back(c);
                            break
                        }
                    },
                    None => break,
                };

                maybe_c = self.chars.next();
            }

            subs.parse::<i8>().ok().map(|int| Token::Int(int))
        }

        fn read_unit(&mut self, first_char: char) -> Option<Self::Item> {
            let mut subs = String::from("");
            let mut exp: i8 = 1;
            let mut annotation: Option<String> = None;
            let mut maybe_c = Some(first_char);

            loop {
                match maybe_c {
                    Some(c) => match c {
                        '.' | '*' | '/' | '^'   => {
                            self.visited.push_back(c);
                            break
                        },
                        '{' => {
                            annotation = self.read_annotation();
                            break
                        },
                        '+' | '-' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                            // Exponentiation doesn't accept terms
                            exp = match self.read_int(c).unwrap() {
                                Token::Int(i) => i.try_into().unwrap(),
                                _ => {panic!("read_int should have returned an int here!")}
                            };
                            break;
                        },
                        _ => {
                            subs.push(c);
                        }
                    },
                    None => break
                };
                maybe_c = self.chars.next();
            }
            Some(Token::Unit(unit::Unit::new(subs, exp, annotation)))
        }
    }

    impl<'a> Iterator for Tokenizer<'a> {
        type Item = Token;

        fn next(&mut self) -> Option<Self::Item> {
            let mut maybe_c: Option<char> = self.visited.pop_front().or_else(|| self.chars.next());

            match maybe_c {
                Some(c) => match c {
                    '.' | '*' => Some(Token::Mul),
                    '/' => Some(Token::Div),
                    '+' | '-' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.read_int(c),
                    '}' | ')' | ']' => panic!("[Syntax Error]: '{:?}' encountered without matching opening brace.", c),
                    ' ' => None,
                    _ => self.read_unit(c),
                },
                None => None
            }
        }
    }
}

mod parser {
    use super::tokenizer::*;
    use crate::ucum::unit;
    use std::ops::MulAssign;

    #[derive(Debug)]
    pub struct Parser<'a> {
        pub tokenizer: Tokenizer<'a>,
        pub mag: f64,
    }

    impl<'a> Parser<'a> {
        pub fn new(tokenizer: Tokenizer<'a>) -> Self {
            Parser {
                tokenizer,
                mag: 1.0
            }
        }

        pub fn mul_i(&mut self, rhs: i8) -> Option<unit::Unit> {
            self.mag *= rhs as f64;
            self.next()
        }

        pub fn div_i(&mut self, rhs: i8) -> Option<unit::Unit> {
            self.mag /= rhs as f64;
            self.next()
        }

        pub fn div_next(&mut self) -> Option<unit::Unit> {
            let maybe_t = self.tokenizer.next();
            match maybe_t {
                Some(t) => match  t {
                    Token::Unit(u) => Some(u.invert()),
                    Token::Int(i) => self.div_i(i),
                    _ => panic!("[Syntax Error] Invalid token {:?} encountered after division '/'", t)
                },
                _ => panic!("[Syntax Error] Invalid token {:?} encountered after division '/'", maybe_t)
            }
        }
    }

    impl<'a> Iterator for Parser<'a> {
        type Item = unit::Unit;

        fn next(&mut self) -> Option<Self::Item> {
            let maybe_t = self.tokenizer.next();

            match maybe_t {
                Some(t) => match t {
                    Token::Mul => self.next(),
                    Token::Div => self.div_next(),
                    Token::Unit(u) => Some(u),
                    Token::Int(i) => self.mul_i(i)
                },
                None => None
            }
        }
    }
}


#[derive(Clone)]
pub struct UnitTerm {
    mag: f64,
    pub units: Vec<unit::Unit>,
}

/// TODO: Use iterators (rather than creating a vector with each recursive call)
trait ReducibleUnit {
    fn normalize_units(units: Vec<unit::Unit>) -> Vec<unit::Unit> {
        units
    }
    fn as_base_units(&self) -> Vec<unit::Unit>;
}

impl<'a, 'b> ReducibleUnit for unit::Unit {
    /// Normalize units collapses duplicate instances of the same unit
    /// into one, and normalizes the exponents.
    /// It assumes your list of units is already normalized.
    fn as_base_units(&self) -> Vec<unit::Unit> {
        let res = unit::constants::CONVERSIONS.iter()
            .find(|&&entry| entry.0 == self.unit());

        match res {
            Some((unit, _, reduced)) => {
                if unit == reduced {
                  return vec![self.clone()];
                }
                match &reduced[..1] {
                    "@" | "=" | "1" => vec![self.clone()],
                    _ => UnitTerm::new(reduced).unwrap().as_base_units(),
                }
            },
            None => vec![self.clone()]
        }
    }
}

impl UnitTerm {
    pub fn new(s: &str) -> Option<Self> {
        let mut p = parser::Parser::new(
            tokenizer::Tokenizer::new(s)
        );

        let mut units: Vec<unit::Unit> = vec![];

        loop {
            let maybe_u = p.next();
            match maybe_u {
                Some(u) => units.push(u.clone()),
                None => break
            }
        }

        Some(UnitTerm {
            mag: p.mag,
            units: Self::normalize_units(units),
        })
    }

    pub fn invert(&mut self) {
        self.units = self.units.iter().map(|u| u.invert()).collect();
    }
}

impl ReducibleUnit for UnitTerm {
    fn normalize_units(mut units: Vec<unit::Unit>) -> Vec<unit::Unit> {
        units.sort_by(|a, b| a.atom.cmp(&b.atom));
        let mut result: Vec<unit::Unit> = vec![];

        for unit in units.into_iter() {
            match result.last_mut() {
                None => result.push(unit),
                Some(last) => {

                    if last.exp == 0 {
                        *last = unit
                    } else if last.has_same_unit(&unit) {
                        last.exp += unit.exp;
                    } else {
                        result.push(unit)
                    }
                }
            }
        }
        result
    }

    fn as_base_units(&self) -> Vec<unit::Unit> {
        let mut units: Vec<unit::Unit> = self.units.iter()
            .flat_map(|u| u.as_base_units())
            .collect();

        Self::normalize_units(units)
    }
}

impl PartialEq for UnitTerm {
    fn eq(&self, other: &UnitTerm) -> bool {
        self.as_base_units() == other.as_base_units()
    }
}

impl fmt::Debug for UnitTerm {
    fn fmt(&self, f: &mut fmt::
    Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({})",
            self, unit::Units(&self.as_base_units()))
    }
}

impl fmt::Display for UnitTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", unit::Units(&self.units))
    }
}

impl ops::AddAssign for UnitTerm {
    fn add_assign(&mut self, other: Self) {
        if self != &other {
            panic!("Units {:?} and {:?} are not compatible", self, other)
        }
    }
}

impl ops::SubAssign for UnitTerm {
    fn sub_assign(&mut self, other: Self) {
        if self != &other {
            panic!("Units {:?} and {:?} are not compatible", self, other)
        }
    }
}

impl ops::MulAssign for UnitTerm {
    fn mul_assign(&mut self, other: Self) {
        self.units.extend(other.units);
        self.units = Self::normalize_units((*self.units).to_vec());
    }
}

impl ops::DivAssign for UnitTerm {
    fn div_assign(&mut self, other: Self) {
        self.invert();
        self.units.extend(other.units);
        self.invert();

        self.units = Self::normalize_units((*self.units).to_vec());
    }
}


#[cfg(test)]
mod tests {
    use super::tokenizer::{Token, Tokenizer};
    use super::*;

    // -- Tokenizer
    fn unit_token(u: &str, p: i8) -> Token {
        Token::Unit(unit::Unit::new(u.to_string(), p, None))
    }
    
    #[test]
    fn it_splits_on_mul_and_div() {
        let tokens: Vec<Token> = Tokenizer::new("m.kg/s2").collect();

        assert_eq!(tokens, vec![
            unit_token("m", 1),
            Token::Mul,
            unit_token("kg", 1),
            Token::Div,
            unit_token("s", 2),])
    }

     #[test]
     fn it_splits_on_pos_and_neg() {
        let tokens: Vec<Token> = Tokenizer::new("m.kg+5.s-2").collect();
        assert_eq!(tokens, vec![
            unit_token("m", 1),
            Token::Mul,
            unit_token("kg", 5),
            Token::Mul,
            unit_token("s", -2),
        ])
    }

    #[test]
    fn it_reads_annotations() {
        let tokens: Vec<Token> = Tokenizer::new("m{meters}").collect();

        assert_eq!(tokens, vec![
            Token::Unit(unit::Unit::new("m".to_string(), 1, Some("meters".to_string())))
        ]);

        let tokens: Vec<Token> = Tokenizer::new("m.{meters}").collect();

        assert_eq!(tokens, vec![
            Token::Unit(unit::Unit::new("m".to_string(), 1, None)),
            Token::Mul,
            Token::Unit(unit::Unit::new("".to_string(), 1, Some("meters".to_string())))
        ])
    }

    #[test]
    fn it_reads_ints() {
        let tokens: Vec<Token> = Tokenizer::new("10.m3.02kg/5s").collect();

        assert_eq!(tokens, vec![
            Token::Int(10),
            Token::Mul,
            unit_token("m", 3),
            Token::Mul,
            Token::Int(2),
            unit_token("kg", 1),
            Token::Div,
            Token::Int(5),
            unit_token("s", 1)
        ])
    }

    // -- UnitTerm

    fn unit(u: &str, p: i8) -> unit::Unit {
        unit::Unit::new(u.to_string(), p, None)
    }

    #[test]
    fn it_interprets_division_of_units() {
        let unit_term = UnitTerm::new("m.kg/s2").unwrap();

        assert_eq!(unit_term.mag, 1.);
        assert_eq!(unit_term.units, vec![
            unit("kg", 1),
            unit("m", 1),
            unit("s", -2)
        ])
    }

    #[test]
    fn it_interprets_division_of_scalars() {
        let unit_term = UnitTerm::new("m.kg/5s2").unwrap();

        assert_eq!(unit_term.mag, 0.2);
    }

    // -- Conversions
    #[test]
    fn it_combines_redundant_units() {
        let base_units = |s| UnitTerm::new(s).unwrap();

        assert_eq!(base_units("m.m.m"), base_units("m3"));
        assert_eq!(base_units("m.m.m/m.m/m"), base_units("m2"));
        assert_eq!(base_units("m.m.s2m/m3.m/s.m.s1"), base_units("m2.s2"));
    }

    #[test]
    fn it_converts_composite_si_units() {
        let base_units = |s| UnitTerm::new(s).unwrap();

        println!("{}", base_units("m.kg/s2"));

        assert_eq!(base_units("m.kg/s2"), base_units("N"));
        assert_eq!(base_units("m/s2.kg"), base_units("N"));
        assert_eq!(base_units("N/m2"), base_units("Pa"));

        assert_eq!(base_units("g.cm/s2"), base_units("dyn"));
    }

}