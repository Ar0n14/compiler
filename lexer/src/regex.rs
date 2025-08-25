pub mod regex_lexer;
pub mod regex_parser;

#[derive(Debug, Clone, PartialEq)]
pub enum Regex {
    Literal(Option<char>),
    Concat(Box<Regex>, Box<Regex>),
    Union(Box<Regex>, Box<Regex>),
    Star(Box<Regex>),
    Empty,
}

impl Regex {
    pub fn empty() -> Regex {
        Regex::Empty
    }
    pub fn char(c: char) -> Regex {
        Regex::Literal(Some(c))
    }
    pub fn epsilon() -> Regex {
        Regex::Literal(None)
    }
    pub fn concatenate(self, regex: Regex) -> Regex {
        Regex::Concat(Box::new(self), Box::new(regex))
    }
    pub fn union(self, regex: Regex) -> Regex {
        Regex::Union(Box::new(self), Box::new(regex))
    }
    pub fn star(self) -> Regex {
        Regex::Star(Box::new(self))
    }
}

impl From<&str> for Regex {
    fn from(value: &str) -> Self {
        let mut lexer = regex_lexer::Lexer::new(value);
        let tokens = lexer.lex_tokens().unwrap_or_default();
        let mut parser = regex_parser::Parser::new(tokens);
        parser.parse().unwrap_or(Regex::empty())
    }
}
