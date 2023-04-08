/*
 * TODO: add documentation about file.
 */

#![allow(unused_variables)]

use std::iter::Peekable;
use super::{
    position::Positioned,
    token::{Token, TokenVec},
    ast::Ast,
};

/// A scanner implementation for [Parser] that's build on top of [Peekable].
#[derive(Clone)]
struct TokenScanner {
    pub index: usize,
    it: Peekable<TokenVec>,
}

impl TokenScanner {
    /// Constructs a new [TokenScanner] instance, initializing the `index`
    /// field to `0` and the `it` field to a [Peekable]
    /// iterator over the [TokenVec].
    pub fn new(buf: TokenVec) -> TokenScanner {
        TokenScanner { index: 0, it: buf.peekable() }
    }

    /// Returns the next token in the input string, updating the
    /// `index` field accordingly. Returns `None` if there are no
    /// more characters to read.
    pub fn next(&mut self) -> Option<Positioned<Token>> {
        let next = self.it.next();
        if next.is_some() {
            self.index = self.index + 1
        }

        next
    }

    /// Returns a reference to the next item in the input [TokenVec]
    /// without advancing the iterator. Returns `None` if there are no more
    /// characters to read.
    pub fn peek(&mut self) -> Option<&Positioned<Token>> {
        self.it.peek()
    }

    /// Consumes the next token if it satisfies a given condition `x`,
    /// updating the `index` field accordingly. Returns `true` if
    /// the token was consumed, `false` otherwise.
    pub fn consume_if<F>(&mut self, condition: F) -> bool
    where
        F: Fn(Positioned<Token>) -> bool,
    {
        if let Some(token) = self.peek() {
            if condition(token.clone()) {
                self.next().unwrap();
                return true;
            }
        }

        return false;
    }

    /// Consumes the next token if the following token satisfies a
    /// given condition `x`, updating the `current_position` field
    /// accordingly. Returns `true` if the token was consumed, `false`
    /// otherwise.
    pub fn consume_if_next<F>(&mut self, condition: F) -> bool
    where
        F: Fn(Positioned<Token>) -> bool,
    {
        let mut it = self.it.clone();
        match it.next() {
            None => return false,
            _ => (),
        }

        if let Some(ch) = it.peek() {
            if condition(ch.clone()) {
                self.next().unwrap();
                return true;
            }
        }

        false
    }

    /// Consumes characters from the input string as long as they satisfy a
    /// given condition `x`, updating the `current_position` field
    /// accordingly. Returns a vector containing the consumed characters.
    pub fn consume_while<F>(&mut self, condition: F) -> Vec<Positioned<Token>>
    where
        F: Fn(Positioned<Token>) -> bool,
    {
        let mut chars: Vec<Positioned<Token>> = Vec::new();
        while let Some(token) = self.clone().peek() {
            if !condition(token.clone()) {
                break;
            } else {
                self.next();
                chars.push(token.clone());
            }
        }

        chars
    }
}


/// TODO: docs
pub struct Parser {
    it: TokenScanner
}

impl Parser {
  pub fn new(tokens: Vec<Positioned<Token>>) -> Parser {
      Parser {
          it: TokenScanner::new(TokenVec::new(tokens))
      }
  }

  pub fn parse(tokens: Vec<Positioned<Token>>) -> Vec<Ast> {
      let mut parser = Parser::new(tokens);
      parser.parse_buffer()
  }

  pub fn parse_buffer(&mut self) -> Vec<Ast> {
      let mut asts: Vec<Ast> = Vec::new();

      loop {
          let initial_position = self.it.index;
          let token = match self.it.next() {
              None => break,
              Some(t) => t,
          };

          if let Some(ast) = self.generate_ast(token) {
              asts.push(ast);
          }
      }

      vec![]
  }

  pub fn generate_ast(&mut self, start: Positioned<Token>) -> Option<Ast> {
      let tokens: Vec<Positioned<Token>> = Vec::new();
      // TODO:
      Some(Ast::Group(tokens))
  }

  pub fn read_single_ast(&mut self, start: Positioned<Token>) -> Option<Ast> {
      None // TODO:
  }

  pub fn read_group_ast(&mut self, start: Positioned<Token>) -> Ast {
      // TODO:
      Ast::Group(Vec::from([start]))
  }
}
