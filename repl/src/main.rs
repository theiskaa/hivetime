use std::io::{stdin, stdout, Write};
use std::env;

use hivetime::calculate;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let input  = args.iter().skip(1).map(|s| s.to_string()).collect::<Vec<_>>().join(" ");
        return execute_calculator(input.as_str());
    }

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
