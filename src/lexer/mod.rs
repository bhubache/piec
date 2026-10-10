use std::str::Chars;

use crate::diagnostics::Annotation;
use crate::diagnostics::Diagnostic;
use crate::lexer::token::Keyword;
use crate::lexer::token::Literal;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use crate::source::SourceFile;
use crate::span::Position;
use crate::span::Span;

pub mod token;

#[cfg(test)]
mod tests;

pub struct Lexer<'a> {
    file: &'a SourceFile,
    input: Chars<'a>,
    start_line: usize,
    start_column: usize,
    line: usize,
    column: usize,
    curr_token_to_be: String,
    diagnostics: Vec<Diagnostic<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(file: &'a SourceFile) -> Self {
        Lexer {
            file,
            input: file.source.chars(),
            start_line: 0,
            start_column: 0,
            line: 0,
            column: 0,
            curr_token_to_be: String::new(),
            diagnostics: Vec::new(),
        }
    }

    // Possible next_token return scenarios other than normal token:
    // - first character is EOF
    // - reach EOF while lexing for ident that must be more than one character
    pub fn lex(mut self) -> Result<Vec<Token>, Vec<Diagnostic<'a>>> {
        let mut tokens = Vec::new();

        while let Some(tok) = self.next_token() {
            tokens.push(tok);
        }

        if !self.diagnostics.is_empty() {
            Err(self.diagnostics)
        } else {
            Ok(tokens)
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.eat_whitespace();

        self.start_line = self.line;
        self.start_column = self.column;

        let first_char = self.step()?;

        let token_kind = match first_char {
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            ',' => TokenKind::Comma,
            '\'' => match self.eat_while_true(|c| c.is_alphabetic() || c == '-') {
                true => TokenKind::Atom(self.curr_token_to_be.clone()),
                false => {
                    self.save_diagnostic(
                        "lone `'`",
                        vec![
                            Annotation::new(self.curr_token_span(), "expected one or more alphabetic characters and/or hyphens to follow this to produce an Atom"),
                        ],
                    );
                    return None;
                }
            },
            '0'..='9' => {
                self.eat_while_true(|c| c.is_ascii_digit());
                TokenKind::Literal(Literal::Nat(self.curr_token_to_be.parse().unwrap()))
            }
            character if self.is_valid_ident_start(character) => self.eat_identifier(),
            character => {
                self.save_diagnostic(
                    format!("unknown start of token encountered: `{}`", character),
                    vec![],
                );
                return None;
            }
        };

        self.curr_token_to_be.clear();

        Some(Token::new(token_kind, self.curr_token_span()))
    }

    fn save_diagnostic(&mut self, message: impl Into<String>, annotations: Vec<Annotation>) {
        let diag = Diagnostic::new(self.file, message, self.curr_token_span(), annotations);
        self.diagnostics.push(diag);
    }

    fn curr_token_span(&self) -> Span {
        Span::new(
            Position::new(self.start_line, self.start_column),
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

        // FIXME: At this point the token should be an ident delimiter

        match Keyword::try_from(self.curr_token_to_be.clone()) {
            Ok(keyword) => TokenKind::Keyword(keyword),
            Err(_) => TokenKind::Identifier(self.curr_token_to_be.clone()),
        }
    }

    fn peek_first(&self) -> Option<char> {
        self.input.clone().next()
    }

    fn at_eof(&self) -> bool {
        self.input.as_str().is_empty()
    }

    fn is_valid_ident_start(&mut self, character: char) -> bool {
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
        if self.is_ident_escape(character) {
            true
        } else {
            !is_ident_delimiter(character) && !matches!(character, '\\' | '|' | '#')
        }
    }

    fn is_ident_escape(&mut self, character: char) -> bool {
        if character == '\\' {
            if self.at_eof() {
                false
            } else {
                self.step();
                true
            }
        } else if character == '|' {
            todo!()
        } else if character == '#' {
            match self.peek_first() {
                Some(ch) => ch == '%',
                None => false,
            }
        } else {
            false
        }
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
