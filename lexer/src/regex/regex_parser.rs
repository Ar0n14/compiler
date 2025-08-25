/*
* Literal(char),
* LeftParen,
* RightParen,
* Concatenate,
* Union,
* Star,
* Plus,
* Question,
* LiteralGroup(HashSet<char>),
* Eof,

* Regex: Concatenate
* Concatenate: Union, Union
* Union: Star, Star
* Star: Primitive
* Primitive: char, epsilon, Grouping, +, ?
*/

use crate::regex::Regex;
use crate::regex::regex_lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Option<Regex> {
        let mut regex = Regex::empty();
        while !self.is_at_end() {
            regex = self.union()?;
        }
        Some(regex)
    }
    pub fn union(&mut self) -> Option<Regex> {
        let mut regex = self.concatenate()?;
        while self._match(&Token::Union) {
            self.current += 1;
            regex = Regex::Union(Box::new(regex), Box::new(self.concatenate()?));
        }

        Some(regex)
    }
    pub fn concatenate(&mut self) -> Option<Regex> {
        let mut regex = self.question()?;

        loop {
            if self._match(&Token::Union) || self.is_at_end() {
                break;
            }
            if self._match(&Token::Concatenate) {
                self.current += 1;
            }
            regex = Regex::Concat(Box::new(regex), Box::new(self.question()?));
        }
        Some(regex)
    }
    pub fn question(&mut self) -> Option<Regex> {
        let mut regex = self.plus()?;

        if self._match(&Token::Question) {
            self.current += 1;
            regex = Regex::Union(Box::new(regex), Box::new(Regex::Literal(None)));
        }
        Some(regex)
    }
    pub fn plus(&mut self) -> Option<Regex> {
        let mut regex = self.star()?;

        if self._match(&Token::Plus) {
            self.current += 1;
            regex = Regex::Concat(
                Box::new(regex.clone()),
                Box::new(Regex::Star(Box::new(regex))),
            );
        }
        Some(regex)
    }
    pub fn star(&mut self) -> Option<Regex> {
        let mut regex = self.primary()?;
        if self._match(&Token::Star) {
            self.current += 1;
            regex = Regex::Star(Box::new(regex));
        }
        Some(regex)
    }
    pub fn primary(&mut self) -> Option<Regex> {
        match self.peek() {
            Some(&Token::Literal(c)) => {
                self.current += 1;
                Some(Regex::Literal(Some(c)))
            }
            Some(Token::LeftParen) => {
                let regex = self.union()?;

                self.current += 1;
                if self.peek()? == &Token::RightParen {
                    Some(regex)
                } else {
                    println!("Error: mismatched ')'.");
                    None
                }
            }
            Some(Token::LiteralGroup(h)) => {
                let mut regex = Regex::empty();
                for c in h.iter() {
                    if regex == Regex::Empty {
                        regex = Regex::Literal(Some(*c));
                    } else {
                        regex = Regex::Union(Box::new(regex), Box::new(Regex::Literal(Some(*c))));
                    }
                }
                self.current += 1;
                Some(regex)
            }
            Some(t) => {
                println!("Error: unexpected token: {:#?}", t);
                None
            }
            None => None,
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.peek() == Some(&Token::Eof) || self.current >= self.tokens.len()
    }
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }
    pub fn _match(&self, token: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            match self.peek() {
                Some(t) => t == token,
                None => {
                    println!("Unwrap on None in Parser::_match (&self, token: &Token)!");
                    false
                }
            }
        }
    }
}
