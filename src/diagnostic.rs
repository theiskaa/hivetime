/*
 * TODO: add documentation about file.
 */

use super::position::Position;

// [?] document me.
#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic {
    pub position: Position,
    pub message: String,
}

impl Diagnostic {
    pub fn new(position: Position, message: String) -> Self {
        Diagnostic { position, message }
    }
}
