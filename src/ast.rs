/*
 * TODO: add documentation about file.
 */

use super::{position::Positioned, token::Token};

#[derive(Clone)]
/// A left / right branch element of [Ast].
pub enum Branch {
    Single(Positioned<Token>),
    Group(Vec<Positioned<Token>>),
}

impl Branch {
    // [?] Debug mode only
    pub fn to_string(&self) -> String {
        match self.clone() {
            Branch::Single(v) => v.value.to_string(),
            Branch::Group(v) => {
                let mut collected = String::new();
                for item in v {
                    collected.push_str(format!("{} ", item.value.to_string()).as_str())
                }

                collected
            }
        }
    }
}

/// TODO: diagram docs
///       -----------
///      | OPERATION |
///       -----------
///        /     \
///    ------    -------
///   | LEFT |  | Right |
///    ------    -------
#[derive(Clone)]
pub struct Ast {
    operation: Branch,
    left: Branch,
    right: Branch,
}

impl Ast {
    pub fn new(operation: Branch, left: Branch, right: Branch) -> Self {
        Self {
            operation,
            left,
            right,
        }
    }

    // [?] Debug mode only
    pub fn to_string(&self) -> String {
        format!(
            "operation: {}\nleft: {}\nright: {}",
            self.operation.to_string(),
            self.left.to_string(),
            self.right.to_string()
        )
    }
}
