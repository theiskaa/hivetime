/*
 * TODO: add documentation about file.
 */

use std::collections::HashMap;

use crate::{
    branch::{Branch, BranchType, CBranch},
    diagnostic::Diagnostic,
    token::Token,
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
    pub fn calculate(tokens: Vec<Branch>) -> (Vec<Diagnostic>, f32) {
        let calculator = Calculator::new(tokens);
        calculator.calculate_buffer()
    }

    /// Calculates the result based on the stored branches and returns the diagnostics and the calculated value.
    fn calculate_buffer(&self) -> (Vec<Diagnostic>, f32) {
        let diagnostics: Vec<Diagnostic> = Vec::new();

        let cbranches = self.convert_to_calculatable();
        for b in cbranches.iter() {
            // TODO: calculate directly
        }

        (diagnostics, 0.0)
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
    fn collect_times(&self, branch: Branch) -> f32 {
        let mut minuted: f32 = 0.0;
        for t in branch.tokens {
            let m = t.value.minuted();
            if let Some(v) = m {
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
            Token::Minuse
        } else {
            Token::Plus
        }
    }
}
