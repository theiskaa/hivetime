/*
 * TODO: add documentation about file.
 */

use super::{position::Positioned, token::Token};

pub enum Ast {
    Single(Positioned<Token>),
    Group(Vec<Positioned<Token>>),
}

