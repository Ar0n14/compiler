use std::collections::HashSet;

// based on Rob Nystroms crafting interpreters.

#[derive(Debug)]
pub struct Lexer {
    pub source: String,
    start: usize,
    current: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            source: source.to_string(),
            start: 0,
            current: 0,
        }
    }
    pub fn lex_tokens(&mut self) -> Option<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();
        while !self.is_at_end() {
            self.start = self.current;
            match self.lex_token() {
                Some(t) => {
                    tokens.push(t);
                }
                None => {
                    continue;
                }
            }
        }
        tokens.push(Token::Eof);
        Some(tokens)
    }
    pub fn lex_token(&mut self) -> Option<Token> {
        let c = self.peek()?;
        self.current += 1;
        match c {
            '"' => {
                let mut chars: HashSet<char> = HashSet::new();
                while !self.is_at_end() && self.peek()? != '"' {
                    chars.insert(self.peek()?);
                    self.current += 1;
                }
                self.current += 1;
                if self.is_at_end() || chars.is_empty() {
                    return None;
                }
                Some(Token::LiteralGroup(chars))
            }
            '|' => Some(Token::Union),
            '.' => Some(Token::Concatenate),
            '*' => Some(Token::Star),
            '+' => Some(Token::Plus),
            '?' => Some(Token::Question),
            '[' => {
                let mut chars: HashSet<char> = HashSet::new();

                //only case where we don't need it. (What if we match ']' without anything?)
                self.current -= 1;
                while !self.is_at_end() && self.peek_next()? != ']' {
                    self.current += 1;
                    if self.peek_next()? == '-' {
                        for c in self.peek()?..=self.source.chars().nth(self.current + 2)? {
                            chars.insert(c);
                        }
                        self.current += 2;
                    } else {
                        chars.insert(self.peek()?);
                    }
                }
                self.current += 2;
                Some(Token::LiteralGroup(chars))
            }
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            ' ' | '\t' | '\r' | '\n' => {
                if !self.is_at_end() {
                    self.start = self.current;
                    self.lex_token()
                } else {
                    None
                }
            }
            _ => Some(Token::Literal(c)),
        }
    }
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            Some('\0')
        } else {
            self.source.chars().nth(self.current)
        }
    }
    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            Some('\0')
        } else {
            Some(self.source.chars().nth(self.current + 1)?)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Literal(char),
    LeftParen,
    RightParen,
    Concatenate,
    Union,
    Star,
    Plus,
    Question,
    LiteralGroup(HashSet<char>), // e.g., [a-zA-Z]
    Eof,
}
