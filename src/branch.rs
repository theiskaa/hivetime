/*
 * TODO: add documentation about file.
 */

use super::{position::Positioned, token::Token};

#[derive(Clone)]
pub enum BranchType {
    Times,
    Operations,
}

#[derive(Clone)]
pub struct Branch {
    btype: BranchType,
    tokens: Vec<Positioned<Token>>,
}

impl BranchType {
    pub fn to_string(&self) -> String {
        match self {
            BranchType::Times => String::from("Times"),
            BranchType::Operations => String::from("Operations"),
        }
    }
}

impl Branch {
    pub fn new(btype: BranchType, tokens: Vec<Positioned<Token>>) -> Self {
        Self { btype, tokens }
    }

    // [?] Debug mode only
    pub fn to_string(&self) -> String {
        let mut collected = String::new();
        collected.push_str(format!("Type: {} [\n", self.btype.clone().to_string()).as_str());
        for item in self.tokens.clone() {
            collected.push_str(format!(" {}\n", item.value.to_string()).as_str())
        }

        collected.push_str("]\n");

        collected
    }
}
