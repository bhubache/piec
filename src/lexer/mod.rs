use std::str::Chars;

use crate::diagnostics::Annotation;
use crate::diagnostics::Diagnostic;
use crate::lexer::token::Keyword;
use crate::lexer::token::Literal;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use crate::source::SourceFile;
use crate::span::Span;

pub mod token;

#[cfg(test)]
mod tests;

pub struct Lexer<'a> {
    file: &'a SourceFile,
    input: Chars<'a>,
    start_byte_offset: usize,
    curr_byte_offset: usize,
    curr_lexeme: String,
    diagnostics: Vec<Diagnostic<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(file: &'a SourceFile) -> Self {
        Lexer {
            file,
            input: file.source.chars(),
            start_byte_offset: 0,
            curr_byte_offset: 0,
            curr_lexeme: String::new(),
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

        self.start_byte_offset = self.curr_byte_offset;

        let first_char = self.step()?;

        let token_kind = match first_char {
            '"' => TokenKind::DoubleQuote,
            ',' => TokenKind::Comma,
            '\'' => match self.eat_while_true(|c| c.is_alphabetic() || c == '-') {
                true => TokenKind::Atom(self.curr_lexeme.clone()),
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

                TokenKind::LineComment(self.curr_lexeme.clone())
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
                                    span.start += 1;
                                    span.end += 1;
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

                if self.curr_lexeme.chars().all(|ch| ch.is_ascii_digit()) {
                    TokenKind::Literal(Literal::Nat(self.curr_lexeme.parse().unwrap()))
                } else {
                    match Keyword::try_from(self.curr_lexeme.as_str()) {
                        Ok(keyword) => TokenKind::Keyword(keyword),
                        Err(_) => TokenKind::Identifier(self.curr_lexeme.clone()),
                    }
                }
            }
        };

        self.curr_lexeme.clear();

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
        Span::new(self.start_byte_offset, self.curr_byte_offset)
    }

    fn step(&mut self) -> Option<char> {
        let next = self.input.next()?;
        self.curr_lexeme.push(next);

        self.curr_byte_offset += next.len_utf8();

        Some(next)
    }

    fn eat_while_true(&mut self, predicate: fn(char) -> bool) -> bool {
        let mut consumed_at_least_one_char = false;
        while self.peek_first().is_some() && predicate(self.peek_first().unwrap()) {
            consumed_at_least_one_char = true;
            self.step();
        }

        consumed_at_least_one_char
    }

    fn eat_whitespace(&mut self) {
        self.eat_while_true(char::is_whitespace);
        self.curr_lexeme.clear();
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
