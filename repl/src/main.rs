use std::io::{stdin, stdout, Write};
use std::{env, fs};

use hivetime::calculate;

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

    execute_calculator(input.as_str());
}

fn run_prompt() {
    loop {
        let mut input: String = String::new();

        print!("{} ", ">"); // The classical repl input hint.
        let _ = stdout().flush();

        match stdin().read_line(&mut input) {
            Err(e) => println!("{} {} \n", format!("[!]"), e),
            Ok(_) => execute_calculator(input.as_str()),
        };
    }
}

fn execute_calculator(input: &str) {
   let (r, t) = calculate(input); // TODO: handle diagnostics
   println!("{} or ({}m)\n", t, r);
}
