#![allow(dead_code)]

pub enum Token {
    // Time types.
    Minute(f32),
    Hour(f32),
    // TODO: add Second(f32)
    // TODO: add Week(f32)
    // TODO: add Month(f32)
    // TODO: add Year(f32)

    // Operations.
    Plus,
    Minuse,
    // TODO: add Prod,
    // TODO: add Divide,
}

/// A type(identification) implementation for [Token].
pub enum TokenType {
    // TODO: add Second(f32)
    Minute,
    Hour,
    // TODO: add Week(f32)
    // TODO: add Month(f32)
    // TODO: add Year(f32)

    // Operations.
    Plus,
    Minuse,
    // TODO: add Prod,
    // TODO: add Divide,
}
