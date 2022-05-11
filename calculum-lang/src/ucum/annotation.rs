pub mod tokenizer {
    /// Within a unit annotation, only a small set of operations are well-defined:
    /// Negation, exponentiation (of numbers), multiplication, and division.
    /// It doesn't make any sense to add noncommensurable units.

    use std::collections::VecDeque;
    use crate::ucum::unit;

    #[derive(Debug, PartialEq)]
    pub enum Token {
        OpenParen,
        CloseParen,
        Annotation(String),
        Mul,
        Div,
        Exp,
        Unit(unit::parser::Unit),
        Int(i32),
        Float(f64),
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
        fn read_annotation(&mut self) -> Option<Self::Item>;
        fn read_int(&mut self, _: char) -> Option<Self::Item>;
        fn read_number(&mut self, _: char) -> Option<Self::Item>;
        fn read_unit(&mut self, _: char) -> Option<Self::Item>;
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

            subs.parse::<i32>().ok().map(|int| Token::Int(int))
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
            let mut exp: i8 = 1;
            subs.push(first_char);

            let mut maybe_c = self.chars.next();

            loop {
                match maybe_c {
                    Some(c) => match c {
                        '.' | '*' | '/' | '^' | '(' | ')' | '{' | '}' | '+' | '-' => {
                            self.visited.push_back(c);
                            break
                        },
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                            // Exponentiation doesn't accept floats
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
            Some(Token::Unit(unit::parser::Unit::new(subs, exp)))
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
                    '^' => Some(Token::Exp),
                    '(' => Some(Token::OpenParen),
                    ')' => Some(Token::CloseParen),
                    '{' => self.read_annotation(),
                    '+' | '-' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.read_number(c),
                    '}' => panic!("[Syntax Error]: '}}' encountered without matching opening brace."),
                    ' ' => None,
                    _ => self.read_unit(c),
                },
                None => None
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::tokenizer::{Token, Tokenizer};
    use crate::ucum::unit::parser::Unit;

    #[test]
    fn it_splits_on_mul_and_div() {
        let tokens: Vec<Token> = Tokenizer::new("m.kg/s2").collect();
        let unit = |u: &str| Token::Unit(Unit::new(u.to_string(), 1));
        
        assert_eq!(tokens, vec![unit("m"),
                                Token::Mul,
                                unit("kg"),
                                Token::Div,
                                unit("s"),
                                Token::Int(2)])
    }

     #[test]
     fn it_splits_on_pos_and_neg() {
        let tokens: Vec<Token> = Tokenizer::new("m.kg+1.s-2").collect();
        let unit = |u: &str| Token::Unit(Unit::new(u.to_string(), 1));
         
        assert_eq!(tokens, vec![unit("m"),
                                Token::Mul,
                                unit("kg"),
                                Token::Int(1),
                                Token::Mul,
                                unit("s"),
                                Token::Int(-2)])
    }

    #[test]
    fn it_splits_annotations() {
        let tokens: Vec<Token> = Tokenizer::new("m{meters}").collect();
        let unit = |u: &str| Token::Unit(Unit::new(u.to_string(), 1));

        assert_eq!(tokens, vec![unit("m"),
                                Token::Annotation("meters".to_string())])
    }

    #[test]
    fn it_reads_exponentiation() {
        let tokens: Vec<Token> = Tokenizer::new("10.1^5m").collect();
        let unit = |u: &str| Token::Unit(Unit::new(u.to_string(), 1));

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
        let unit = |u: &str| Token::Unit(Unit::new(u.to_string(), 1));

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