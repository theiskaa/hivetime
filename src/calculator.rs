/*
 * TODO: add documentation about file.
 */

use std::{collections::HashMap, vec};

use crate::{
    branch::{Branch, BranchType, CBranch},
    diagnostic::Diagnostic,
    token::Token,
    position::Position,
};

pub struct Calculator {
    pub branches: Vec<Branch>,
}

impl Calculator {
    /// Creates a new instance of `Calculator` with the provided branches.
    pub fn new(branches: Vec<Branch>) -> Self {
        Calculator { branches }
    }

    /// Calculates the result based on the given branches and returns the diagnostics and the calculated value.
    /// Returned times value will be interpreted in minutes.
    pub fn calculate(branches: Vec<Branch>) -> (Vec<Diagnostic>, f64) {
        let calculator = Calculator::new(branches);
        calculator.calculate_buffer()
    }

    /// Calculates the result based on the stored branches and returns the diagnostics and the calculated value.
    /// Returned times value will be interpreted in minutes.
    fn calculate_buffer(&self) -> (Vec<Diagnostic>, f64) {
        let mut result: f64 = 0.0;
        let mut diagnostics: Vec<Diagnostic> = Vec::new();

        if self.branches.is_empty() {
            return (
              vec![Diagnostic::new(Position::empty(), String::from("Empty input"))],
              result,
            );
        }


        let cbranches = self.convert_to_calculatable();
        let mut i: usize = 0;
        while i <= cbranches.len() {
            if i > cbranches.len() - 1 {
                // TODO: terminated operation.
                return (vec![], 0.0);
            }

            let mut y: f64 = 0.0;
            let x: f64 = result.clone();
            let mut operation: Token = Token::Plus;
            match Calculator::take_operation(i, cbranches.clone()) {
                Ok(v) => operation = v,
                Err(d) => diagnostics.push(d),
            };

            if let Some(v) = cbranches.clone().get(i.clone()) {
                match *v {
                    CBranch::Time(time) => y = time,
                    _ => {},
                };
            }

            // Update res by current X/Y/O.
            result = Calculator::execute_operation(x, y, operation);
            i += 2;
        }


        (diagnostics, result)
    }

    pub fn take_operation(i: usize, branches: Vec<CBranch>) -> Result<Token, Diagnostic> {
        // At first loop, operation must to be PLUS.
        // Because, the [res] is zero and we have to
        // add some value before starting working on it.
        if i == 0 {
            return Ok(Token::Plus);
        }

        if branches.len() - 1 < i - 1 {
            let err = Diagnostic::new(Position::empty(), String::from("Missing some tokens"));
            return Err(err);
        }


        Ok(match branches.clone()[i - 1].clone() {
            CBranch::Operation(token) => token,
            _ => Token::Plus,
        })
    }

    /// Converts the stored branches into calculatable branches and returns the converted branches.
    fn convert_to_calculatable(&self) -> Vec<CBranch> {
        let mut collected: Vec<CBranch> = Vec::new();

        for b in self.branches.iter() {
            match b.btype {
                BranchType::Times => {
                    collected.push(CBranch::Time(self.collect_times(b.clone())));
                }
                BranchType::Operations => {
                    collected.push(CBranch::Operation(self.collect_operations(b.clone())));
                }
            };
        }

        collected
    }

    /// Collects the total duration in minutes from the tokens within the given branch.
    fn collect_times(&self, branch: Branch) -> f64 {
        let mut minuted: f64 = 0.0;
        for t in branch.tokens {
            let m = t.value.minuted();
            if let Some(v) = m {
                let v = f64::from(v);
                minuted += v;
            }
        }

        minuted
    }

    /// Collects the final operation from the tokens within the given branch based on majority count.
    fn collect_operations(&self, branch: Branch) -> Token {
        let mut counts: HashMap<String, i32> = HashMap::new();

        for t in branch.tokens {
            match t.value {
                Token::Minuse | Token::Plus => {
                    let count = counts.entry(t.value.to_string()).or_insert(0);
                    *count += 1;
                }
                _ => {}
            };
        }

        let plus_count = *counts.get(&String::from("+")).unwrap_or(&0);
        let minus_count = *counts.get(&String::from("-")).unwrap_or(&0);

        if plus_count > minus_count {
            Token::Plus
        } else {
            Token::Minuse
        }
    }


    /// Executes the given [operation] for [X] and [Y]
    ///
    ///  Example:
    ///  ╭───╮        ╭───╮        ╭───────────╮
    ///  │ X │──▶ 48  │ Y │──▶ 42  │ Operation │──▶ MINUS
    ///  ╰───╯        ╰───╯        ╰───────────╯
    ///  ────────────────────────────────────────────────
    ///                      ╭─────────╮    ╭───╮
    ///  Answer would be ──▶ │ 48 - 42 │──▶ │ 6 │
    ///                      ╰─────────╯    ╰───╯
    fn execute_operation(x: f64, y: f64, operation: Token) -> f64 {
        let operations: HashMap<String, f64> = HashMap::from([
            (Token::Plus.to_string(), x + y),
            (Token::Minuse.to_string(), x - y),
        ]);

        match operations.get(&operation.to_string()) {
            None => 0.0,
            Some(v) => v.clone(),
        }
    }
}
