use lexer::regex::Regex;
use lexer::regex::regex_lexer;
use lexer::regex::regex_parser;

use lexer::nfa::*;

fn main() {
    let mut lexer = regex_lexer::Lexer::new("c*");
    let mut parser = regex_parser::Parser::new(lexer.lex_tokens().unwrap_or_default());
    print!(
        "{}",
        NFA::build_nfa(parser.parse().unwrap_or(Regex::Empty), Some("ILY Ing"))
    );
}
