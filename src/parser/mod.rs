use crate::diagnostics::Annotation;
use crate::diagnostics::Diagnostic;
use crate::lexer::token::Keyword;
use crate::lexer::token::Literal;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use crate::source::SourceFile;

mod ast;

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
        let mut expressions = Vec::new();
        while self.curr_index < self.tokens.len() {
            // TODO: Attempt to parse as much as possible and collect errors for comprehensive reporting
            match self.parse_expr() {
                Ok(expr) => {
                    expressions.push(expr);
                }
                Err(_) => continue,
            }
        }

        if !self.diagnostics.is_empty() {
            Err(self.diagnostics)
        } else {
            Ok(ast::Program::new(expressions))
        }
    }

    fn parse_expr(&mut self) -> Result<ast::Expression, ()> {
        if self.check(TokenKind::Keyword(Keyword::Todo)) {
            self.parse_todo()
        } else {
            self.parse_the()
        }
    }

    fn parse_the(&mut self) -> Result<ast::Expression, ()> {
        self.expect(TokenKind::OpenParen)?;
        self.expect(TokenKind::Keyword(Keyword::The))?;
        let expr = ast::Expression::new(ast::ExpressionKind::The(
            self.parse_type(),
            self.parse_value()?,
        ));
        self.expect(TokenKind::CloseParen)?;

        Ok(expr)
    }

    fn parse_todo(&mut self) -> Result<ast::Expression, ()> {
        self.expect(TokenKind::Keyword(Keyword::Todo))?;

        Ok(ast::Expression::new(ast::ExpressionKind::Todo))
    }

    fn parse_type(&mut self) -> ast::Type {
        if self.eat_term(TokenKind::Keyword(Keyword::Absurd)) {
            ast::Type::Absurd
        } else if self.eat_term(TokenKind::Keyword(Keyword::Trivial)) {
            ast::Type::Trivial
        } else if self.eat_term(TokenKind::Keyword(Keyword::Nat)) {
            ast::Type::Nat
        } else {
            self.save_diagnostic("expected type", vec![]);
            self.step();
            ast::Type::Error
        }
    }

    // 1. Literal nat value e.g. `7`
    // 2. Expression with a constructor at the top
    // 3. Expression that can be evaluated to number 2
    fn parse_value(&mut self) -> Result<ast::Value, ()> {
        let value = match self.tokens[self.curr_index].kind {
            TokenKind::Literal(Literal::Nat(nat)) => {
                self.step();
                let nat_cons_expr = if nat == 0 {
                    ast::NatConstructorExpr::Zero
                } else {
                    ast::NatConstructorExpr::Add1(nat - 1)
                };
                ast::Value::Nat(nat_cons_expr)
            }
            _ => {
                self.save_diagnostic("failed to parse value", vec![]);
                return Err(());
            }
        };

        Ok(value)
    }

    fn expect(&mut self, token: TokenKind) -> Result<(), ()> {
        match self.eat_term(token.clone()) {
            true => Ok(()),
            false => {
                dbg!(
                    "{observed} != {expected}",
                    &self.tokens[self.curr_index].kind,
                    token
                );
                // NOTE: Saving a diagnostic here requires that the parser accurately predict the
                // token without the need to try alternatives. That is, if we were to later find an
                // alternative worked just fine, we'd want to somehow remove this diagnostic.
                self.save_diagnostic("failed during expect", vec![]);

                Err(())
            }
        }
    }

    fn check(&self, token: TokenKind) -> bool {
        token == self.tokens[self.curr_index].kind
    }

    fn step(&mut self) {
        self.curr_index += 1;
    }

    fn eat_term(&mut self, token: TokenKind) -> bool {
        let matches = self.check(token);
        if matches {
            self.step();
        }

        matches
    }

    fn save_diagnostic(&mut self, message: &'a str, annotations: Vec<Annotation>) {
        let diag = Diagnostic::new(
            self.file,
            message,
            self.tokens[self.curr_index].span,
            annotations,
        );

        self.diagnostics.push(diag);
    }
}

// TODO: Learn how to thoroughly test this
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{self, Expression, ExpressionKind, Program, Type};
    use crate::span::{Position, Span};

    use pretty_assertions::assert_eq;

    fn run_parse(source: &str, tokens: Vec<Token>, expected: Program) {
        let file = SourceFile::new("test_input".to_string(), source.to_string());
        let parser = Parser::new(&file, tokens);

        match parser.parse() {
            Ok(observed) => assert_eq!(observed, expected),
            Err(diagnostics) => {
                for diag in diagnostics.iter() {
                    diag.report();
                }

                panic!("Errors occurred during parsing");
            }
        }
    }

    fn run_parse_neg(source: &str, tokens: Vec<Token>) {
        let file = SourceFile::new("test_input".to_string(), source.to_string());
        let parser = Parser::new(&file, tokens);

        assert!(parser.parse().is_err());
    }

    #[test]
    fn straightforward_the() {
        run_parse(
            "(the Nat 1)",
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
                    Span::new(Position::new(0, 4), Position::new(0, 7)),
                ),
                Token::new(
                    TokenKind::Literal(Literal::Nat(1)),
                    Span::new(Position::new(0, 7), Position::new(0, 8)),
                ),
                Token::new(
                    TokenKind::CloseParen,
                    Span::new(Position::new(0, 8), Position::new(0, 9)),
                ),
            ],
            Program::new(vec![Expression::new(ExpressionKind::The(
                Type::Nat,
                ast::Value::Nat(ast::NatConstructorExpr::Add1(0)),
            ))]),
        )
    }

    #[test]
    fn invalid_type_in_the_expr() {
        run_parse_neg(
            "(the 123 1)",
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
                    TokenKind::Literal(Literal::Nat(123)),
                    Span::new(Position::new(0, 4), Position::new(0, 7)),
                ),
                Token::new(
                    TokenKind::Literal(Literal::Nat(1)),
                    Span::new(Position::new(0, 7), Position::new(0, 8)),
                ),
                Token::new(
                    TokenKind::CloseParen,
                    Span::new(Position::new(0, 8), Position::new(0, 9)),
                ),
            ],
        );
    }
}
