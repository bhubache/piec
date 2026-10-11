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
            '"' => TokenKind::DoubleQuote,
            ',' => TokenKind::Comma,
            '\'' => match self.eat_while_true(|c| c.is_alphabetic() || c == '-') {
                true => TokenKind::Atom(self.curr_token_to_be.clone()),
                false => {
                    // TODO: Shouldn't we attempt to continue lexing to uncover as many errors as
                    // possible?
                    self.save_diagnostic(
                        "lone `'`",
                        vec![
                            Annotation::new(
                                self.curr_token_span(),
                                "expected one or more alphabetic characters and/or hyphens to follow this to produce an Atom",
                            ),
                        ],
                    );
                    return None;
                }
            },
            '`' => TokenKind::BackTick,
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '[' => TokenKind::OpenSquareBracket,
            ']' => TokenKind::CloseSquareBracket,
            '{' => TokenKind::OpenCurlyBrace,
            '}' => TokenKind::CloseCurlyBrace,
            ';' => {
                self.eat_while_true(|ch| ch != '\n');

                TokenKind::LineComment(self.curr_token_to_be.clone())
            }
            character => {
                match character {
                    '\\' => {
                        self.eat_rest_of_backlash_ident()?;
                    }
                    '|' => {
                        self.eat_rest_of_pipe_ident()?;
                    }
                    '#' => {
                        match self.peek_first() {
                            Some(next_char) => {
                                if next_char == '%' {
                                    self.step();
                                } else {
                                    let mut span = self.curr_token_span();
                                    span.start.column += 1;
                                    span.end.column += 1;
                                    self.save_diagnostic(
                                        format!("Unexpected character `{}` following `#`", next_char),
                                        vec![Annotation::new(span, "expected this character to be `%`, indicating an identifier or the start of an identifier")],
                                    );
                                }
                            }
                            None => {
                                self.save_diagnostic(
                                    "EOF following `#`",
                                    vec![Annotation::new(self.curr_token_span(), "expected a `%` to follow this to complete, or start, an identifier")],
                                )
                            }
                        }
                    }
                    _ => {
                        if is_ident_delimiter(character) || matches!(character, '\\' | '|' | '#') {
                            self.save_diagnostic(
                                "invalid start of identifier",
                                vec![Annotation::new(self.curr_token_span(), "invalid start of identifier")],
                            );
                        }
                    }
                }

                let mut keep_eating = true;
                while !self.at_eof() && keep_eating {
                    match self.peek_first().unwrap() {
                        '\\' => {
                            self.step();
                            self.eat_rest_of_backlash_ident()?;
                        }
                        '|' => {
                            self.step();
                            self.eat_rest_of_pipe_ident()?;
                        }
                        character => {
                            if is_valid_ident_continued(character) {
                                self.step();
                            } else {
                                keep_eating = false;
                            }
                        }
                    }
                }

                if self.curr_token_to_be.chars().all(|ch| ch.is_ascii_digit()) {
                    TokenKind::Literal(Literal::Nat(self.curr_token_to_be.parse().unwrap()))
                } else {
                    match Keyword::try_from(self.curr_token_to_be.as_str()) {
                        Ok(keyword) => TokenKind::Keyword(keyword),
                        Err(_) => TokenKind::Identifier(self.curr_token_to_be.clone()),
                    }
                }
            }
        };

        self.curr_token_to_be.clear();

        Some(Token::new(token_kind, self.curr_token_span()))
    }

    fn eat_rest_of_backlash_ident(&mut self) -> Option<char> {
        self.step().or_else(|| {
            self.save_diagnostic(
                r"EOF following `\`",
                vec![Annotation::new(self.curr_token_span(), "expected any single character to follow this to complete, or start, an identifier")],
            );

            None
        })
    }

    fn eat_rest_of_pipe_ident(&mut self) -> Option<char> {
        self.eat_while_true(|ch| ch != '|');
        self.step().or_else(|| {
            self.save_diagnostic(
                "EOF encountered when searching for enclosing `|`",
                vec![Annotation::new(
                    self.curr_token_span(),
                    "expected a `|` at the end to complete the identifier",
                )],
            );

            None
        })
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

    fn peek_first(&self) -> Option<char> {
        self.input.clone().next()
    }

    fn at_eof(&self) -> bool {
        self.input.as_str().is_empty()
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
