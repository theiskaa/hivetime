/*
 * TODO: add documentation about file.
 */

use super::{
    position::{Position, Positioned},
    scanner::ScannerIt,
    token::Token,
};

pub struct Lexer<'a> {
    it: ScannerIt<'a>,
}

impl<'a> Lexer<'a> {
    /// Creates a new instance of the [Lexer] struct with the given buffer.
    pub fn new(buffer: &str) -> Lexer {
        Lexer {
            it: ScannerIt::new(buffer),
        }
    }

    /// Takes a buffer string, creates a [Lexer] instance, and calls
    /// `lex_buffer` to parse it and return a list of `Positioned<Token>`s.
    pub fn lex(buffer: &str) -> Vec<Positioned<Token>> {
        let mut lexer = Lexer::new(buffer);
        lexer.lex_buffer()
    }

    /// The main method that parses the buffer and returns a list of `Positioned<Token>s`.
    pub fn lex_buffer(&mut self) -> Vec<Positioned<Token>> {
        let mut tokens: Vec<Positioned<Token>> = Vec::new();
        loop {
            let initial_position = self.it.position;
            let ch = match self.it.next() {
                None => break,
                Some(c) => c,
            };
            if let Some(token) = self.generate_token(ch.clone()) {
                tokens.push(Positioned::new(
                    token,
                    Position {
                        start: initial_position,
                        end: self.it.position,
                    },
                ));
            }
        }
        tokens
    }

    /// Returns one of two tokens depending on whether the next character in the buffer matches the given character.
    pub fn either(&mut self, matches: char, a: Token, b: Token) -> Option<Token> {
        if self.it.consume_if(|ch| ch == matches) {
            Some(a)
        } else {
            Some(b)
        }
    }

    ///
    pub fn generate_token(&mut self, ch: char) -> Option<Token> {
        match ch {
            ' ' | '\t' | '\r' => None,
            '\n' => {
                // TODO: increment line cursor.
                None
            },
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minuse),
            num if num.is_numeric() => self.read_time(num),
            unkown => Some(Token::Unknown(String::from(unkown))),
        }
    }

    ///
    pub fn read_time(&mut self, start_with: char) -> Option<Token> {
        let mut number: String = String::from(start_with);
        let mut time_type: String = String::new();

        let start: String = self
            .it
            .consume_while(|a| a.is_numeric())
            .into_iter()
            .collect();
        number.push_str(start.as_str());

        if self.it.peek() == Some(&'.') && self.it.consume_if_next(|ch| ch.is_numeric()) {
            let end: String = self
                .it
                .consume_while(|a| a.is_numeric())
                .into_iter()
                .collect();
            number.push('.');
            number.push_str(end.as_str());
        }

        self.it.consume_while(|ch| ch == ' ' || ch == '\r' || ch == '\t' || ch == '\n');
        if self.it.peek().unwrap_or(&' ').is_ascii_alphabetic() {
            let time: String = self
                .it
                .consume_while(|a| a.is_ascii_alphabetic())
                .into_iter()
                .collect();

            time_type.push_str(time.as_str());
        }

        Token::time(time_type, Some(number))
    }
}
