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

        if self.check_first(TokenKind::OpenParen)
            && self.check_second(TokenKind::Keyword(Keyword::Claim))
        {
            self.step().unwrap();
            let claim_token_span = self.step().unwrap().span;
            let ident = self
                .expect_identifier()
                .map_err(|maybe_token| match maybe_token {
                    Some(bad_token) => {
                        self.save_diagnostic(
                            "expected name to be associated with `claim`",
                            Span::new(start_span.start, bad_token.span.end),
                            vec![
                                Annotation::new(claim_token_span, "expected name to follow this"),
                                Annotation::new(
                                    bad_token.span,
                                    format!("instead got {}", bad_token.kind),
                                ),
                            ],
                        );

                        Recoverable::Yes
                    }
                    None => Recoverable::No,
                })?;
            let expr = self.parse_expr()?;
            let last_token = match self.expect(TokenKind::CloseParen) {
                Ok(token) => token,
                Err(maybe_bad_token) => {
                    let msg = "expected `)` to finish `claim`";

                    let (span, annotations) = match maybe_bad_token {
                        Some(bad_token) => (
                            Span::new(start_span.start, bad_token.span.end),
                            vec![Annotation::new(
                                bad_token.span,
                                format!("instead got {}", bad_token.kind),
                            )],
                        ),
                        None => (
                            Span::new(start_span.start, expr.span.end),
                            vec![Annotation::new(expr.span, "instead got EOF")],
                        ),
                    };
                    self.save_diagnostic(msg, span, annotations);

                    return Err(Recoverable::Yes);
                }
            };

            Ok(ast::Declaration::new(
                ast::DeclarationKind::Claim(ident, expr),
                Span::new(start_span.start, last_token.span.end),
            ))
        } else if self.check_first(TokenKind::OpenParen)
            && self.check_second(TokenKind::Keyword(Keyword::Define))
        {
            self.step();
            self.step();
            todo!();
        } else if self.check_first(TokenKind::OpenParen)
            && self.check_second(TokenKind::Keyword(Keyword::CheckSame))
        {
            self.step();
            self.step();
            todo!();
        } else {
            // FIXME: Shouldn't unwrap here
            let expr = self.parse_expr().unwrap();
            let span = expr.span;
            Ok(ast::Declaration::new(
                ast::DeclarationKind::Expr(expr),
                span,
            ))
        }
    }

    fn parse_expr(&mut self) -> Result<ast::Expression, Recoverable> {
        if self.check_first(TokenKind::Keyword(Keyword::Todo)) {
            let tok = &self.tokens[self.curr_index];
            self.curr_index += 1;

            Ok(ast::Expression::new(ast::ExpressionKind::Todo, tok.span))
        } else {
            if self.check_first(TokenKind::Keyword(Keyword::Nat)) {
                let tok = self.step().ok_or(Recoverable::No)?;

                Ok(ast::Expression::new(ast::ExpressionKind::Nat, tok.span))
            } else {
                Err(Recoverable::No)
            }
        }
    }

    fn check_first(&self, token: TokenKind) -> bool {
        token == self.tokens[self.curr_index].kind
    }

    fn check_second(&self, token: TokenKind) -> bool {
        token == self.tokens[self.curr_index + 1].kind
    }

    fn step(&mut self) -> Option<&Token> {
        let ret_token = self.tokens.get(self.curr_index);
        self.curr_index += 1;

        ret_token
    }

    fn expect(&mut self, token_kind: TokenKind) -> Result<&Token, Option<Token>> {
        let token = self.step().ok_or(None)?;
        if token.kind == token_kind {
            Ok(token)
        } else {
            Err(Some(token.clone()))
        }
    }

    fn expect_identifier(&mut self) -> Result<Spanned<String>, Option<Token>> {
        let token = self.step().ok_or(None)?;
        match token.kind {
            TokenKind::Identifier(ref ident) => Ok(Spanned::new(ident.to_string(), token.span)),
            _ => Err(Some(token.clone())),
        }
    }

    fn save_diagnostic(
        &mut self,
        message: impl Into<String>,
        span: Span,
        annotations: Vec<Annotation>,
    ) {
        let diag = Diagnostic::new(self.file, message, span, annotations);
        self.diagnostics.push(diag);
    }
}

#[derive(Debug)]
enum Recoverable {
    Yes,
    No,
}
