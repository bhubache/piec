use crate::diagnostics::Annotation;
use crate::diagnostics::Diagnostic;
use crate::lexer::token::Keyword;
use crate::lexer::token::Literal;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use crate::parser::ast::Expression;
use crate::parser::ast::ExpressionKind;
use crate::parser::ast::Spanned;
use crate::source::SourceFile;
use crate::span::Span;

mod ast;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod property_based_tests;

pub type UnaryExprKindCons = fn(Box<Expression>) -> ExpressionKind;
pub type BinaryExprKindCons = fn(Box<Expression>, Box<Expression>) -> ExpressionKind;
pub type TernaryExprKindCons = fn(Box<Expression>, Box<Expression>, Box<Expression>) -> ExpressionKind;
pub type QuaternaryExprKindCons =
    fn(Box<Expression>, Box<Expression>, Box<Expression>, Box<Expression>) -> ExpressionKind;
pub type QuinaryExprKindCons =
    fn(Box<Expression>, Box<Expression>, Box<Expression>, Box<Expression>, Box<Expression>) -> ExpressionKind;

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
        // TODO: Shouldn't unwrap
        let token = self.step().unwrap();

        let expr_span_start = token.span.start;
        let expr_kind = match token.kind {
            TokenKind::Keyword(Keyword::Universe) => ast::ExpressionKind::Universe,
            TokenKind::Keyword(Keyword::Nat) => ast::ExpressionKind::Nat,
            TokenKind::Keyword(Keyword::Zero) => ast::ExpressionKind::Zero,
            TokenKind::Keyword(Keyword::Atom) => ast::ExpressionKind::Atom,
            TokenKind::Keyword(Keyword::Trivial) => ast::ExpressionKind::Trivial,
            TokenKind::Keyword(Keyword::Sole) => ast::ExpressionKind::Sole,
            TokenKind::Keyword(Keyword::Nil) => ast::ExpressionKind::Nil,
            TokenKind::Keyword(Keyword::Absurd) => ast::ExpressionKind::Absurd,
            TokenKind::Keyword(Keyword::VecNil) => ast::ExpressionKind::Vecnil,
            TokenKind::Keyword(Keyword::Todo) => ast::ExpressionKind::Todo,
            TokenKind::Literal(Literal::Nat(value)) => ast::ExpressionKind::NatLiteral(value),
            TokenKind::Atom(ref value) => ast::ExpressionKind::Quote(value.to_string()),
            TokenKind::Keyword(Keyword::Add1) => self.parse_unary(ExpressionKind::Add1)?,
            TokenKind::Keyword(Keyword::Car) => self.parse_unary(ExpressionKind::Car)?,
            TokenKind::Keyword(Keyword::Cdr) => self.parse_unary(ExpressionKind::Cdr)?,
            TokenKind::Keyword(Keyword::Same) => self.parse_unary(ExpressionKind::Same)?,
            TokenKind::Keyword(Keyword::Symm) => self.parse_unary(ExpressionKind::Symm)?,
            TokenKind::Keyword(Keyword::Left) => self.parse_unary(ExpressionKind::Left)?,
            TokenKind::Keyword(Keyword::Right) => self.parse_unary(ExpressionKind::Right)?,
            TokenKind::Keyword(Keyword::List) => self.parse_unary(ExpressionKind::List)?,
            TokenKind::Keyword(Keyword::Cong) => self.parse_binary(ExpressionKind::Cong)?,
            TokenKind::Keyword(Keyword::Trans) => self.parse_binary(ExpressionKind::Trans)?,
            TokenKind::Keyword(Keyword::IndAbsurd) => self.parse_binary(ExpressionKind::IndAbsurd)?,
            TokenKind::Keyword(Keyword::The) => self.parse_binary(ExpressionKind::The)?,
            TokenKind::Keyword(Keyword::Cons) => self.parse_binary(ExpressionKind::Cons)?,
            TokenKind::Keyword(Keyword::Pair) => self.parse_binary(ExpressionKind::Pair)?,
            TokenKind::Keyword(Keyword::VecCons) => self.parse_binary(ExpressionKind::VecCons)?,
            TokenKind::Keyword(Keyword::ListCons) => self.parse_binary(ExpressionKind::ListCons)?,
            TokenKind::Keyword(Keyword::Either) => self.parse_binary(ExpressionKind::Either)?,
            TokenKind::Keyword(Keyword::Vec) => self.parse_binary(ExpressionKind::Vec)?,
            TokenKind::Keyword(Keyword::Replace) => self.parse_ternary(ExpressionKind::Replace)?,
            TokenKind::Keyword(Keyword::Eq) => self.parse_ternary(ExpressionKind::Eq)?,
            TokenKind::Keyword(Keyword::RecList) => self.parse_ternary(ExpressionKind::RecList)?,
            TokenKind::Keyword(Keyword::RecNat) => self.parse_ternary(ExpressionKind::RecNat)?,
            TokenKind::Keyword(Keyword::IterNat) => self.parse_ternary(ExpressionKind::IterNat)?,
            TokenKind::Keyword(Keyword::WhichNat) => self.parse_ternary(ExpressionKind::WhichNat)?,
            TokenKind::Keyword(Keyword::IndEq) => self.parse_ternary(ExpressionKind::IndEq)?,
            TokenKind::Keyword(Keyword::IndEither) => self.parse_quaternary(ExpressionKind::IndEither)?,
            TokenKind::Keyword(Keyword::IndNat) => self.parse_quaternary(ExpressionKind::IndNat)?,
            TokenKind::Keyword(Keyword::IndList) => self.parse_quaternary(ExpressionKind::IndList)?,
            TokenKind::Keyword(Keyword::IndVec) => self.parse_quinary(ExpressionKind::IndVec)?,
            TokenKind::OpenParen => {
                // TODO: Don't unwrap
                let next_tok = self.step().unwrap();
                match next_tok.kind {
                    TokenKind::Keyword(Keyword::Lambda) | TokenKind::Keyword(Keyword::LambdaSymbol) => {
                        self.parse_lambda()?
                    }
                    _ => todo!("remaining bindings aren't yet supported"),
                }
            }
            TokenKind::Identifier(ref ident) => ExpressionKind::Variable(ident.to_string()),
            _ => todo!(),
        };

        Ok(ast::Expression::new(
            expr_kind,
            Span::new(expr_span_start, self.peek_prev().span.end),
        ))
    }

    fn parse_lambda(&mut self) -> Result<ExpressionKind, Recoverable> {
        self.expect(TokenKind::OpenParen)?;

        let mut variables = Vec::new();
        while !self.check_first(TokenKind::CloseParen) {
            variables.push(self.expect_identifier()?);
        }

        self.expect(TokenKind::CloseParen)?;
        self.expect(TokenKind::OpenParen)?;
        let expr = self.parse_expr()?;

        // TODO: Should it be parse_expr's responsibility to get the closing paren for things like
        // `cons`?
        self.expect(TokenKind::CloseParen)?;

        self.expect(TokenKind::CloseParen)?;

        Ok(ExpressionKind::Lambda(variables, Box::new(expr)))
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

    fn parse_unary(&mut self, cons: UnaryExprKindCons) -> Result<ExpressionKind, Recoverable> {
        Ok(cons(Box::new(self.parse_expr()?)))
    }

    fn parse_binary(&mut self, cons: BinaryExprKindCons) -> Result<ExpressionKind, Recoverable> {
        Ok(cons(Box::new(self.parse_expr()?), Box::new(self.parse_expr()?)))
    }

    fn parse_ternary(&mut self, cons: TernaryExprKindCons) -> Result<ExpressionKind, Recoverable> {
        Ok(cons(
            Box::new(self.parse_expr()?),
            Box::new(self.parse_expr()?),
            Box::new(self.parse_expr()?),
        ))
    }

    fn parse_quaternary(&mut self, cons: QuaternaryExprKindCons) -> Result<ExpressionKind, Recoverable> {
        Ok(cons(
            Box::new(self.parse_expr()?),
            Box::new(self.parse_expr()?),
            Box::new(self.parse_expr()?),
            Box::new(self.parse_expr()?),
        ))
    }

    fn parse_quinary(&mut self, cons: QuinaryExprKindCons) -> Result<ExpressionKind, Recoverable> {
        Ok(cons(
            Box::new(self.parse_expr()?),
            Box::new(self.parse_expr()?),
            Box::new(self.parse_expr()?),
            Box::new(self.parse_expr()?),
            Box::new(self.parse_expr()?),
        ))
    }
}

#[derive(Debug)]
enum Recoverable {
    Yes,
    No,
}
