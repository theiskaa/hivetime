use lexer::Lexer;
use parser::Parser;

pub mod branch;
pub mod diagnostic;
pub mod lexer;
pub mod parser;
pub mod position;
pub mod token;

pub fn calculate(input: &str) -> i32 {
    let tokens = Lexer::lex(input);
    let (_diagnostics, _branches) = Parser::parse(tokens);

    0
}
