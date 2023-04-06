/*
 * TODO: add documentation about file.
 */

use super::position::BytePos;
use std::{iter::Peekable, str::Chars};

/// A wrapper implementation for [std::iter::Peekable].
/// Which makes string manipulations easy to implement.
pub struct ScannerIt<'a> {
    pub position: BytePos,
    it: Peekable<Chars<'a>>,
}

impl<'a> ScannerIt<'a> {
    /// Constructs a new `Scanner` instance, initializing the `position`
    /// field to `BytePos::default()` and the `it` field to a `Peekable`
    /// iterator over the characters in the input string `buf`.
    pub fn new(buf: &str) -> ScannerIt {
        ScannerIt {
            position: BytePos::default(),
            it: buf.chars().peekable(),
        }
    }

    /// Returns the next character in the input string, updating the
    /// `position` field accordingly. Returns `None` if there are no
    /// more characters to read.
    pub fn next(&mut self) -> Option<char> {
        let next = self.it.next();
        if let Some(c) = next {
            self.position = self.position.shift(c);
        }
        next
    }

    /// Returns a reference to the next character in the input string,
    /// without advancing the iterator. Returns `None` if there are no more
    /// characters to read.
    pub fn peek(&mut self) -> Option<&char> {
        self.it.peek()
    }

    /// Consumes the next character if it satisfies a given condition `x`,
    /// updating the `current_position` field accordingly. Returns `true` if
    /// the character was consumed, `false` otherwise.
    pub fn consume_if<F>(&mut self, condition: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        if let Some(&ch) = self.peek() {
            if condition(ch) {
                self.next().unwrap();
                return true;
            }
        }

        return false;
    }

    /// Consumes the next character if the following character satisfies a
    /// given condition `x`, updating the `current_position` field
    /// accordingly. Returns `true` if the character was consumed, `false`
    /// otherwise.
    pub fn consume_if_next<F>(&mut self, condition: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        let mut it = self.it.clone();
        match it.next() {
            None => return false,
            _ => (),
        }

        if let Some(&ch) = it.peek() {
            if condition(ch) {
                self.next().unwrap();
                return true;
            }
        }

        false
    }

    /// Consumes characters from the input string as long as they satisfy a
    /// given condition `x`, updating the `current_position` field
    /// accordingly. Returns a vector containing the consumed characters.
    pub fn consume_while<F>(&mut self, condition: F) -> Vec<char>
    where
        F: Fn(char) -> bool,
    {
        let mut chars: Vec<char> = Vec::new();
        while let Some(&ch) = self.peek() {
            if !condition(ch) {
                break;
            } else {
                self.next().unwrap();
                chars.push(ch);
            }
        }

        chars
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut scanner = ScannerIt::new("hello");
        assert_eq!(scanner.next(), Some('h'));
        assert_eq!(scanner.next(), Some('e'));
        assert_eq!(scanner.next(), Some('l'));
        assert_eq!(scanner.next(), Some('l'));
        assert_eq!(scanner.next(), Some('o'));
        assert_eq!(scanner.next(), None);
    }

    #[test]
    fn test_peek() {
        let mut scanner = ScannerIt::new("hello");
        assert_eq!(scanner.peek(), Some(&'h'));
        assert_eq!(scanner.next(), Some('h'));
        assert_eq!(scanner.peek(), Some(&'e'));
        assert_eq!(scanner.next(), Some('e'));
        assert_eq!(scanner.peek(), Some(&'l'));
        assert_eq!(scanner.next(), Some('l'));
        assert_eq!(scanner.next(), Some('l'));
        assert_eq!(scanner.peek(), Some(&'o'));
        assert_eq!(scanner.next(), Some('o'));
        assert_eq!(scanner.peek(), None);
        assert_eq!(scanner.next(), None);
        assert_eq!(scanner.peek(), None);
    }

    #[test]
    fn test_consume_if() {
        let mut scanner = ScannerIt::new("hello");
        assert_eq!(scanner.consume_if(|c| c == 'h'), true);
        assert_eq!(scanner.consume_if(|c| c == 'h'), false);
        assert_eq!(scanner.consume_if(|c| c == 'e'), true);
        assert_eq!(scanner.consume_if(|c| c == 'e'), false);
        assert_eq!(scanner.consume_if(|c| c == 'l'), true);
        assert_eq!(scanner.consume_if(|c| c == 'l'), true);
        assert_eq!(scanner.consume_if(|c| c == 'l'), false);
        assert_eq!(scanner.consume_if(|c| c == 'o'), true);
        assert_eq!(scanner.consume_if(|c| c == 'o'), false);
    }

    #[test]
    fn test_consume_if_next() {
        let mut scanner = ScannerIt::new("hello");
        assert_eq!(scanner.consume_if_next(|c| c == 'e'), true);
        assert_eq!(scanner.consume_if_next(|c| c == 'l'), true);
        assert_eq!(scanner.consume_if_next(|c| c == 'l'), true);
        assert_eq!(scanner.consume_if_next(|c| c == 'l'), false);
        assert_eq!(scanner.consume_if_next(|c| c == 'o'), true);
        assert_eq!(scanner.consume_if_next(|c| c == 'o'), false);
        assert_eq!(scanner.consume_if_next(|c| c == 'o'), false);
    }

    #[test]
    fn test_consume_while() {
        let mut scanner = ScannerIt::new("hello world");
        let consumed_chars = scanner.consume_while(|c| c.is_alphabetic());
        assert_eq!(consumed_chars, vec!['h', 'e', 'l', 'l', 'o']);
        assert_eq!(scanner.peek(), Some(&' '));
        let consumed_chars = scanner.consume_while(|c| c.is_whitespace());
        assert_eq!(consumed_chars, vec![' ']);
        assert_eq!(scanner.peek(), Some(&'w'));
    }
}
