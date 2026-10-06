use std::str::Chars;

use crate::diagnostics::Annotation;
use crate::diagnostics::Diagnostic;
use crate::lexer::token::Keyword;
use crate::lexer::token::Literal;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use crate::span::Position;
use crate::span::Span;

pub mod token;

pub struct Lexer<'a> {
    file_path: &'a str,
    source: &'a str,
    input: Chars<'a>,
    line: usize,
    column: usize,
    curr_token_to_be: String,
    diagnostics: Vec<Diagnostic<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, file_path: &'a str) -> Self {
        Lexer {
            file_path,
            source: input,
            input: input.chars(),
            line: 0,
            column: 0,
            curr_token_to_be: String::new(),
            diagnostics: Vec::new(),
        }
    }

    pub fn lex(mut self) -> Result<Vec<Token>, Vec<Diagnostic<'a>>> {
        let mut tokens = Vec::new();

        loop {
            match self.next_token() {
                Token {
                    kind: TokenKind::Eof,
                    ..
                } => break,
                Token {
                    kind: TokenKind::Unknown,
                    ..
                } => continue,
                tok => tokens.push(tok),
            }
        }

        if !self.diagnostics.is_empty() {
            Err(self.diagnostics)
        } else {
            Ok(tokens)
        }
    }

    fn next_token(&mut self) -> Token {
        self.eat_whitespace();

        let start_line = self.line;
        let start_column = self.column;

        let Some(first_char) = self.step() else {
            return Token::new(TokenKind::Eof, Span::empty());
        };

        let token_kind = match first_char {
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            ',' => TokenKind::Comma,
            '\'' => match self.eat_while_true(|c| c.is_alphabetic() || c == '-') {
                true => TokenKind::Atom(self.curr_token_to_be.clone()),
                false => {
                    self.save_diagnostic(
                        "lone `'`",
                        self.curr_token_span(start_line, start_column),
                        vec![
                            Annotation::new(self.curr_token_span(start_line, start_column), "expected one or more alphabetic characters and/or hyphens to follow this"),
                        ],
                    );
                    TokenKind::Unknown
                }
            },
            '0'..='9' => {
                self.eat_while_true(|c| c.is_ascii_digit());
                TokenKind::Literal(Literal::Nat(self.curr_token_to_be.parse().unwrap()))
            }
            character if self.is_valid_ident_start(character) => self.eat_identifier(),
            _ => TokenKind::Unknown,
        };

        self.curr_token_to_be.clear();

        Token::new(token_kind, self.curr_token_span(start_line, start_column))
    }

    fn save_diagnostic(&mut self, message: &str, span: Span, annotations: Vec<Annotation>) {
        let diag = Diagnostic::new(
            self.file_path,
            self.source,
            message.to_string(),
            span,
            annotations,
        );
        self.diagnostics.push(diag);
    }

    fn curr_token_span(&self, start_line: usize, start_column: usize) -> Span {
        Span::new(
            Position::new(start_line, start_column),
            Position::new(self.line, self.column),
        )
    }

    fn step(&mut self) -> Option<char> {
        let next = self.input.next()?;
        self.curr_token_to_be.push(next);

        if next == (0xA as char) {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }

        Some(next)
    }

    fn eat_while_true(&mut self, predicate: fn(char) -> bool) -> bool {
        let mut consumed_at_least_on_char = false;
        while self.peek_first().is_some() && predicate(self.peek_first().unwrap()) {
            consumed_at_least_on_char = true;
            self.step();
        }

        consumed_at_least_on_char
    }

    fn eat_whitespace(&mut self) {
        self.eat_while_true(char::is_whitespace);
        self.curr_token_to_be.clear();
    }

    fn eat_identifier(&mut self) -> TokenKind {
        self.eat_while_true(is_valid_ident_continued);
        match Keyword::new(&self.curr_token_to_be) {
            Some(keyword) => TokenKind::Keyword(keyword),
            None => TokenKind::Identifier(self.curr_token_to_be.clone()),
        }
    }

    fn peek_first(&self) -> Option<char> {
        self.input.clone().next()
    }

    fn is_valid_ident_start(&self, character: char) -> bool {
        // TODO: The official pie implementation supports some interesting identifiers.
        // Below is how pie officially defines identifiers.
        //
        //     [identifier-delims (:or (char-set "\",'`()[]{};") pie-whitespace)]
        //     [identifier-chars (:~ identifier-delims "\\" "|")]
        //     [identifier-escapes (:or (:: "\\" any-char)
        //                              (:: "|" (:* (:~ "|")) "|"))]
        //     [identifier-start (:or identifier-escapes
        //                            (:~ identifier-delims "\\" "|" "#")
        //                            "#%")]
        //     [identifier (:: identifier-start
        //                     (:* identifier-escapes identifier-chars))]
        //
        // Identifiers of the following form are not yet supported:
        //     - Those defined by identifier-escapes
        //     - Those starting with `#%`
        !is_ident_delimiter(character) && !matches!(character, '\\' | '|' | '#')
    }
}

fn is_valid_ident_continued(character: char) -> bool {
    !is_ident_delimiter(character) && !matches!(character, '\\' | '|')
}

fn is_ident_delimiter(character: char) -> bool {
    matches!(
        character,
        '"' | ',' | '\'' | '`' | '(' | ')' | '[' | ']' | '{' | '}' | ';'
    ) || character.is_whitespace()
}

// TODO: Learn how to thoroughly test this
#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn open_paren() {
        let lex = Lexer::new("(", "stdin");
        let tokens = lex.lex().unwrap();

        assert_eq!(
            tokens,
            vec![Token::new(
                TokenKind::OpenParen,
                Span::new(Position::new(0, 0), Position::new(0, 1)),
            )]
        );
    }

    #[test]
    fn invalid_token() {
        let lex = Lexer::new("'", "stdin");

        // TODO: More precise test?
        assert!(lex.lex().is_err())
    }

    #[test]
    fn list_cons() {
        let lex = Lexer::new("::", "stdin");
        let tokens = lex.lex().unwrap();

        assert_eq!(
            tokens,
            vec![Token::new(
                TokenKind::Keyword(Keyword::ListCons),
                Span::new(Position::new(0, 0), Position::new(0, 2)),
            )]
        );
    }

    #[test]
    fn multiple_open_paren() {
        let lex = Lexer::new("(((", "stdin");
        let tokens = lex.lex().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::new(
                    TokenKind::OpenParen,
                    Span::new(Position::new(0, 0), Position::new(0, 1)),
                ),
                Token::new(
                    TokenKind::OpenParen,
                    Span::new(Position::new(0, 1), Position::new(0, 2)),
                ),
                Token::new(
                    TokenKind::OpenParen,
                    Span::new(Position::new(0, 2), Position::new(0, 3)),
                )
            ]
        );
    }

    #[test]
    fn claim_nat() {
        let lex = Lexer::new("(claim foo\n  (Nat))", "stdin");
        let tokens = lex.lex().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::new(
                    TokenKind::OpenParen,
                    Span::new(Position::new(0, 0), Position::new(0, 1)),
                ),
                Token::new(
                    TokenKind::Keyword(Keyword::Claim),
                    Span::new(Position::new(0, 1), Position::new(0, 6)),
                ),
                Token::new(
                    TokenKind::Identifier(String::from("foo")),
                    Span::new(Position::new(0, 7), Position::new(0, 10)),
                ),
                Token::new(
                    TokenKind::OpenParen,
                    Span::new(Position::new(1, 2), Position::new(1, 3)),
                ),
                Token::new(
                    TokenKind::Keyword(Keyword::Nat),
                    Span::new(Position::new(1, 3), Position::new(1, 6)),
                ),
                Token::new(
                    TokenKind::CloseParen,
                    Span::new(Position::new(1, 6), Position::new(1, 7)),
                ),
                Token::new(
                    TokenKind::CloseParen,
                    Span::new(Position::new(1, 7), Position::new(1, 8)),
                ),
            ]
        );
    }

    #[test]
    fn straightforward_the() {
        let lex = Lexer::new("(the Nat 1)", "stdin");
        let tokens = lex.lex().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::new(
                    TokenKind::OpenParen,
                    Span::new(Position::new(0, 0), Position::new(0, 1)),
                ),
                Token::new(
                    TokenKind::Keyword(Keyword::The),
                    Span::new(Position::new(0, 1), Position::new(0, 4)),
                ),
                Token::new(
                    TokenKind::Keyword(Keyword::Nat),
                    Span::new(Position::new(0, 5), Position::new(0, 8)),
                ),
                Token::new(
                    TokenKind::Literal(Literal::Nat(1)),
                    Span::new(Position::new(0, 9), Position::new(0, 10)),
                ),
                Token::new(
                    TokenKind::CloseParen,
                    Span::new(Position::new(0, 10), Position::new(0, 11)),
                ),
            ]
        );
    }
}
