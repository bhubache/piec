use crate::diagnostics::Annotation;
use crate::diagnostics::Diagnostic;
use crate::lexer::token::Keyword;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use crate::parser::ast::Spanned;
use crate::source::SourceFile;
use crate::span::Span;

mod ast;

#[cfg(test)]
mod tests;

pub struct Parser<'a> {
    file: &'a SourceFile,
    tokens: Vec<Token>,
    curr_index: usize,
    diagnostics: Vec<Diagnostic<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(file: &'a SourceFile, tokens: Vec<Token>) -> Self {
        Parser {
            file,
            tokens,
            curr_index: 0,
            diagnostics: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Result<ast::Program, Vec<Diagnostic<'a>>> {
        let mut declarations = Vec::new();
        'outer: while self.curr_index < self.tokens.len() {
            match self.parse_declaration() {
                Ok(decl) => declarations.push(decl),
                Err(Recoverable::No) => break,
                Err(Recoverable::Yes) => loop {
                    match self.step() {
                        Some(token) if token.kind == TokenKind::CloseParen => break,
                        Some(_) => continue,
                        None => break 'outer,
                    }
                },
            }
        }

        if !self.diagnostics.is_empty() {
            Err(self.diagnostics)
        } else {
            Ok(ast::Program::new(declarations))
        }
    }

    fn parse_declaration(&mut self) -> Result<ast::Declaration, Recoverable> {
        let start_span = self.tokens[self.curr_index].span;

        if self.check_first(TokenKind::OpenParen) && self.check_second(TokenKind::Keyword(Keyword::Claim)) {
            self.step().unwrap();
            self.step().unwrap();
            let ident = self.expect_identifier()?;
            let expr = self.parse_expr()?;
            let last_token = self.expect(TokenKind::CloseParen)?;

            Ok(ast::Declaration::new(
                ast::DeclarationKind::Claim(ident, expr),
                Span::new(start_span.start, last_token.span.end),
            ))
        } else if self.check_first(TokenKind::OpenParen) && self.check_second(TokenKind::Keyword(Keyword::Define)) {
            self.step();
            self.step();
            todo!();
        } else if self.check_first(TokenKind::OpenParen) && self.check_second(TokenKind::Keyword(Keyword::CheckSame)) {
            self.step();
            self.step();
            todo!();
        } else {
            // FIXME: Shouldn't unwrap here
            let expr = self.parse_expr().unwrap();
            let span = expr.span;
            Ok(ast::Declaration::new(ast::DeclarationKind::Expr(expr), span))
        }
    }

    fn parse_expr(&mut self) -> Result<ast::Expression, Recoverable> {
        if self.check_first(TokenKind::Keyword(Keyword::Todo)) {
            let tok = &self.tokens[self.curr_index];
            self.curr_index += 1;

            Ok(ast::Expression::new(ast::ExpressionKind::Todo, tok.span))
        } else {
            if self.check_first(TokenKind::Keyword(Keyword::Nat)) {
                let tok = self
                    .step()
                    .expect("This should never fail because we checked before stepping");

                Ok(ast::Expression::new(ast::ExpressionKind::Nat, tok.span))
            } else {
                let prev_span = self.peek_prev().span;
                self.save_diagnostic("unexpected EOF", prev_span, vec![]);

                Err(Recoverable::No)
            }
        }
    }

    fn peek_prev(&self) -> &Token {
        self.tokens
            .get(self.curr_index - 1)
            .expect("This should never be called when `self.curr_index` is 0")
    }

    fn check_first(&self, token: TokenKind) -> bool {
        token == self.tokens[self.curr_index].kind
    }

    fn check_second(&self, token: TokenKind) -> bool {
        token == self.tokens[self.curr_index + 1].kind
    }

    fn step(&mut self) -> Option<&Token> {
        let ret_token = self.tokens.get(self.curr_index);
        if ret_token.is_some() {
            self.curr_index += 1;
        }

        ret_token
    }

    fn expect(&mut self, token_kind: TokenKind) -> Result<&Token, Recoverable> {
        match self.step() {
            None => {
                let last_span = self.peek_prev().span;
                self.save_diagnostic(
                    "unexpected EOF",
                    last_span,
                    vec![Annotation::new(
                        Span::new(last_span.end, last_span.end),
                        format!("expected `{}`", token_kind),
                    )],
                );

                Err(Recoverable::No)
            }
            Some(token) if token.kind == token_kind => Ok(token),
            Some(bad_token) => {
                let bad_tok_copy = bad_token.clone();
                self.save_diagnostic(
                    format!("expected `{}`, got `{}`", token_kind, bad_tok_copy.kind),
                    bad_tok_copy.span,
                    vec![Annotation::new(
                        bad_tok_copy.span,
                        format!("instead got `{}`", bad_tok_copy.kind),
                    )],
                );

                Err(Recoverable::Yes)
            }
        }
    }

    fn expect_identifier(&mut self) -> Result<Spanned<String>, Recoverable> {
        let prev_span = self.peek_prev().span;
        match self.step() {
            None => {
                self.save_diagnostic(
                    "unexpected EOF",
                    Span::new(prev_span.end, prev_span.end),
                    vec![Annotation::new(
                        Span::new(prev_span.end, prev_span.end),
                        "missing identifier",
                    )],
                );

                Err(Recoverable::No)
            }
            Some(token) => match token.kind {
                TokenKind::Identifier(ref ident) => Ok(Spanned::new(ident.to_string(), token.span)),
                _ => {
                    let tok_copy = token.clone();
                    self.save_diagnostic(
                        "missing identifier",
                        tok_copy.span,
                        vec![
                            Annotation::new(prev_span, "expected name to follow this"),
                            Annotation::new(tok_copy.span, format!("instead got `{}`", tok_copy.kind)),
                        ],
                    );

                    Err(Recoverable::Yes)
                }
            },
        }
    }

    fn save_diagnostic(&mut self, message: impl Into<String>, span: Span, annotations: Vec<Annotation>) {
        let diag = Diagnostic::new(self.file, message, span, annotations);
        self.diagnostics.push(diag);
    }
}

#[derive(Debug)]
enum Recoverable {
    Yes,
    No,
}
