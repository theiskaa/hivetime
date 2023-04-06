use std::io::{stdin, stdout, Write};
use std::{env, fs};

use hivetime::interpreter::lexer::Lexer;

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

    execute(input.as_str());
}

fn run_prompt() {
    loop {
        let mut input: String = String::new();

        print!("{} ", ">"); // The classical repl input hint.
        let _ = stdout().flush();

        match stdin().read_line(&mut input) {
            Err(e) => println!("{} {} \n", format!("[!]"), e),
            Ok(_) => {
                execute(input.as_str());
            }
        };
    }
}

fn execute(input: &str) {
    let tokens = Lexer::lex(input);
    for t in tokens.iter() {
        println!("{token}", token = t.value.to_string());
    }
}
