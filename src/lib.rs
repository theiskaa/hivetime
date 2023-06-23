use lexer::Lexer;
use parser::Parser;

pub mod branch;
pub mod calculator;
pub mod diagnostic;
pub mod lexer;
pub mod parser;
pub mod position;
pub mod token;

pub fn calculate(input: &str) -> f32 {
    let tokens = Lexer::lex(input);
    let (_diagnostics, _branches) = Parser::parse(tokens);

    // TODO: implement calculator to get final result

    0.0
}
