/// # Unified Code for Units and Measures
///
/// UCUM was designed by the folks at [Regenstrief](https://www.regenstrief.org/),
/// (who also happen to be the wizards behind LOINC) out of frustration with
/// standards like ISO 2955 and ANSI X3.50. In particular, they weren't fans of:
///
/// - Naming conflicts (e.g., "a" for both "year" and "are"),
/// - Missing units (e.g., ISO neglects the degree Fahrenheit), &
/// - Ambiguity surrounding customary units (did you know there are
///   distinctions between an international foot, a US foot, a survey foot,
///   and a British foot?).
///
/// UCUM provides an unambiguous and exhaustive coding system for units.
/// (No surprises -- it's in the name!).
///
/// # Examples
/// It ends up looking something like this:
/// - `N = kg.m/s2` (Newton)
/// - `G = m3.kg-1.s-2` (Gravitation constant)
/// - `mu_0 = 4.[pi].10*-7.N/A2` (Magnetic permeability)
/// - `[in_i'Hg] = m[hg].[in_i]/m`  (One inch of mercury column)
/// - `deg = 2 [pi].rad/360` (Degree)
///
/// That is: `.` for multiplication, `/` for division, and nothing for
/// exponentiation (there's no ambiguity when a number follows a unit symbol).
///
/// Customary (non-metric) units, as well as potentially conflicting units, are
/// wrapped in square brackets (intended to shame their users), and you can add
/// annotations with curly brace blocks. `{...}`.
///
/// This crate provides the means to parse UCUM strings into UCUM structs, and
/// to do unit-checked arithmetic with these structs.
///
/// **See Also**:
/// - [dimensioned::unit_systems::ucum](https://docs.rs/dimensioned/0.6.0/i686-pc-windows-gnu/dimensioned/unit_systems/ucum/index.html),
///   which provides type definitions for commonly used units (though not the means to parse strings).
///
/// # Note
/// - This departs from UCUM in one significant way, and that is that `*` is
///   parsed as an alternative to `.`, (multiplication) rather than `^`
///   (exponentiation). (In what universe does `*` make sense as exponentiation?)
/// - Also `.12` won't work but `0.12` will.

use std::collections::VecDeque;
use crate::ucum::unit::UnitToken;

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Annotation(String),
    Mul,
    Div,
    Exp,
    Unit(UnitToken),
    Int(i32),
    Float(f64),
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    // Required in case we need to backtrack `10.1` -> `10.1` but `10.a` -> `10`, `.`, `a`
    visited: VecDeque<char>,
    chars: std::str::Chars<'a>,
}

trait Tokenizes {
    type Item;
    fn read_annotation(&mut self) -> Option<Self::Item>;
    fn read_number(&mut self, _: char) -> Option<Self::Item>;
    fn read_unit(&mut self, _: char) -> Option<Self::Item>;
}

impl<'a> Tokenizer<'a> {
    pub fn new(s: &'a str) -> Self {
        Tokenizer {
            visited: VecDeque::new(),
            chars: s.chars(),
        }
    }
}

impl<'a> Tokenizes for Tokenizer<'a> {
    type Item = Token;
    /// After encountering an opening curly brace, read until we encounter a
    /// closing brace. Return None if we reach the end of the string without
    /// encountering a closing brace.
    fn read_annotation(&mut self) -> Option<Self::Item> {
        let mut subs = String::from("");
        let mut maybe_c = self.chars.next();

        loop {
            match maybe_c {
                None | Some('}') => break,
                Some(c) => subs.push(c),
            };

            maybe_c = self.chars.next();
        }

        maybe_c.map(|_| Token::Annotation(subs))
    }

    fn read_number(&mut self, first_digit: char) -> Option<Self::Item> {
        let mut subs = String::from("");
        subs.push(first_digit);

        let mut maybe_c = self.chars.next();

        loop {
            match maybe_c {
                Some(c) => match c {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => subs.push(c),
                    '.' => {
                        self.visited.push_back('.');

                        // Already encountered a '.'
                        if self.visited.len() > 1 {
                            break
                        }
                        subs.push(c)
                    }
                    _ => {
                        self.visited.push_back(c);
                        break
                    }
                },
                None => break,
            };

            maybe_c = self.chars.next();
        }

        if self.visited.len() > 0 && self.visited[0] == '.' {
            self.visited.pop_front();
            subs.parse::<f64>().ok().map(|float| Token::Float(float))
        } else {
            subs.parse::<i32>().ok().map(|int| Token::Int(int))
        }
    }

    fn read_unit(&mut self, first_char: char) -> Option<Self::Item> {
        let mut subs = String::from("");
        subs.push(first_char);

        let mut maybe_c = self.chars.next();

        loop {
            match maybe_c {
                Some(c) => match c {
                    '.' | '*' | '/' | '^' | '(' | ')' | '{' | '}' | '+' | '-' |
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        self.visited.push_back(c);
                        break
                    }
                    _ => {
                    subs.push(c);
                    }
                },
                None => break
            };
            maybe_c = self.chars.next();
        }
        Some(Token::Unit(UnitToken::new(subs)))
    }
}


impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {

        println!("v:{:?}", &self.visited);

        let mut maybe_c: Option<char> = self.visited.pop_front().or_else(|| self.chars.next());

        match maybe_c {
            Some(c) => match c {
                '.' | '*' => Some(Token::Mul),
                '/' => Some(Token::Div),
                '^' => Some(Token::Exp),
                '(' => Some(Token::OpenParen),
                ')' => Some(Token::CloseParen),
                '{' => self.read_annotation(),
                '+' | '-' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.read_number(c),
                '}' => panic!("[Syntax Error]: '}}' encountered without matching opening brace."),
                _ => self.read_unit(c),
            },
            None => None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{Token, Tokenizer};
    use crate::ucum::unit::UnitToken;

    #[test]
    fn it_splits_on_mul_and_div() {
        let tokens: Vec<Token> = Tokenizer::new("m.kg/s2").collect();
        let unit = |u: &str| Token::Unit(UnitToken::new(u.to_string()));
        
        assert_eq!(tokens, vec![unit("m"),
                                Token::Mul,
                                unit("kg"),
                                Token::Div,
                                unit("s"),
                                Token::Int(2)])
    }

     #[test]
     fn it_splits_on_pos_and_neg() {
        let tokens: Vec<Token> = Tokenizer::new("m.(kg+1).s-2").collect();
        let unit = |u: &str| Token::Unit(UnitToken::new(u.to_string()));
         
        assert_eq!(tokens, vec![unit("m"),
                                Token::Mul,
                                Token::OpenParen,
                                unit("kg"),
                                Token::Int(1),
                                Token::CloseParen,
                                Token::Mul,
                                unit("s"),
                                Token::Int(-2)])
    }

    #[test]
    fn it_splits_annotations() {
        let tokens: Vec<Token> = Tokenizer::new("m{meters}").collect();
        let unit = |u: &str| Token::Unit(UnitToken::new(u.to_string()));

        assert_eq!(tokens, vec![unit("m"),
                                Token::Annotation("meters".to_string())])
    }

    #[test]
    fn it_reads_exponentiation() {
        let tokens: Vec<Token> = Tokenizer::new("10.1^5m").collect();
        let unit = |u: &str| Token::Unit(UnitToken::new(u.to_string()));

        assert_eq!(tokens, vec![
            Token::Float(10.1),
            Token::Exp,
            Token::Int(5),
            unit("m"),
        ])
    }
      #[test]
    fn it_distinguishes_ints_and_floats() {
        let tokens: Vec<Token> = Tokenizer::new("10.m3.02kg/5s").collect();
        let unit = |u: &str| Token::Unit(UnitToken::new(u.to_string()));

        assert_eq!(tokens, vec![
            Token::Float(10.),
            unit("m"),
            Token::Float(3.02),
            unit("kg"),
            Token::Div,
            Token::Int(5),
            unit("s")
        ])
    }

}