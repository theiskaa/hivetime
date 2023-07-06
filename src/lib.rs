use calculator::Calculator;
use lexer::Lexer;
use parser::Parser;
use utils::Utils;

pub mod branch;
pub mod calculator;
pub mod diagnostic;
pub mod lexer;
pub mod parser;
pub mod position;
pub mod token;
pub mod utils;

pub fn calculate(input: &str) -> (f64, String) {
    let tokens = Lexer::lex(input);
    let (_diagnostics, branches) = Parser::parse(tokens);
    // TODO: merge parser diagnostics with calculator diagnostics.
    let (_calculator_diagnostics, result) = Calculator::calculate(branches);
    // TODO: convert calculator result to a hivetime input style text.

    (result, Utils::to_time(result))
}
