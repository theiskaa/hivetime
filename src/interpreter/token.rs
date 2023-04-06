/*
 * TODO: add documentation about file.
 */

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

    // Others
    Unknown(String),
}

impl Token {
    pub fn time(t: String, number: Option<String>) -> Option<Token> {
        match t.as_str() {
            "h" | "hs" | "hour" | "hours" => match number {
                None => None, // TODO: handle the none case.
                Some(v) => Some(Token::Hour(v.parse::<f32>().unwrap_or(-1.0))),
            },
            "m" | "ms" | "minute" | "minutes" => match number {
                None => None, // TODO: handle the none case.
                Some(v) => Some(Token::Minute(v.parse::<f32>().unwrap_or(-1.0))),
            },
            d => {
                let num = number.unwrap_or(String::new());
                if num.is_empty() {
                    Some(Token::Unknown(t))
                } else {
                    Some(Token::Unknown(format!("{d} with {num}")))
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Token::Minute(m) => format!("{m} minute{}", if *m > 1.0 { "s" } else { "" }),
            Token::Hour(h) => format!("{h} hour{}", if *h > 1.0 { "s" } else { "" }),
            Token::Plus => String::from("+"),
            Token::Minuse => String::from("-"),
            Token::Unknown(unknown) => format!("Unkown: {}", *unknown),
        }
    }
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

    // Others
    Unknown,
}
