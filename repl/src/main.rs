use std::io::{stdin, stdout, Write};
use std::{env, fs};

use hivetime::lexer::Lexer;
use hivetime::parser::Parser;
use hivetime::position::Positioned;
use hivetime::token::Token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: hivetime [script]");
        return;
    }

    if args.len() == 2 {
        run_file(args[0].clone().to_string())
    } else {
        run_prompt()
    }
}

fn run_file(path: String) {
    let input = match fs::read_to_string(path) {
        Ok(v) => v,
        Err(_) => String::new(),
    };

    _execute_parser(input.as_str());
}

fn run_prompt() {
    loop {
        let mut input: String = String::new();

        print!("{} ", ">"); // The classical repl input hint.
        let _ = stdout().flush();

        match stdin().read_line(&mut input) {
            Err(e) => println!("{} {} \n", format!("[!]"), e),
            Ok(_) => {
                _execute_parser(input.as_str());
            }
        };
    }
}

fn _execute_lexer(input: &str) {
    let tokens = Lexer::lex(input);
    _print_tokens(tokens);
}

fn _execute_parser(input: &str) {
    let tokens = Lexer::lex(input);
    _print_tokens(tokens.clone());
    println!("---------------------\n");
    let (_, branches) = Parser::parse(tokens);
    for b in branches.iter() {
        println!("{}", b.to_string());
    }
}

fn _print_tokens(tokens: Vec<Positioned<Token>>) {
    for t in tokens.iter() {
        println!("[");
        println!("  {token}", token = t.value.to_string());
        println!("]");
    }
}

/*
fn execute_parser(input: &str) {
    println!("--------- TOKENS");
    let tokens = Lexer::lex(input);
    for t in tokens.iter() {
        println!("{token}", token = t.value.to_string());
    }
     let asts: Vec<Ast> = Parser::parse(tokens);
     println!("--------- ASTS");
     for a in asts.iter() {
         println!("{}", a.to_string());
     }
}
*/
