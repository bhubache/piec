use crate::lexer;
use crate::lexer::Keyword;
use crate::lexer::Token;
use crate::lexer::TokenKind;

mod ast;

// TODO: Figure out how exactly errors should be represented/handled
#[derive(Debug)]
pub struct ParseError;

pub struct Parser {
    tokens: Vec<Token>,
    curr_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            curr_index: 0,
        }
    }

    pub fn parse(&mut self) -> Result<ast::Program, ParseError> {
        let mut expressions = Vec::new();
        while self.curr_index < self.tokens.len() {
            // TODO: Attempt to parse as much as possible and collect errors for comprehensive reporting
            let expr = self.parse_expr()?;

            expressions.push(expr);
        }

        Ok(ast::Program::new(expressions))
    }

    fn parse_expr(&mut self) -> Result<ast::Expression, ParseError> {
        self.parse_todo().or_else(|_| self.parse_the())
    }

    fn parse_the(&mut self) -> Result<ast::Expression, ParseError> {
        self.expect(TokenKind::OpenParen)?;
        self.expect(TokenKind::Keyword(Keyword::The))?;
        let expr = ast::Expression::new(ast::ExpressionKind::The(
            self.parse_type()?,
            self.parse_value()?,
        ));
        self.expect(TokenKind::CloseParen)?;

        Ok(expr)
    }

    fn parse_todo(&mut self) -> Result<ast::Expression, ParseError> {
        self.expect(TokenKind::Keyword(Keyword::Todo))?;

        Ok(ast::Expression::new(ast::ExpressionKind::Todo))
    }

    fn parse_type(&mut self) -> Result<ast::Type, ParseError> {
        let type_ = if self.eat_term(TokenKind::Keyword(Keyword::Absurd)) {
            ast::Type::Absurd
        } else if self.eat_term(TokenKind::Keyword(Keyword::Trivial)) {
            ast::Type::Trivial
        } else if self.eat_term(TokenKind::Keyword(Keyword::Nat)) {
            ast::Type::Nat
        } else {
            return Err(ParseError);
        };

        Ok(type_)
    }

    // 1. Literal nat value e.g. `7`
    // 2. Expression with a constructor at the top
    // 3. Expression that can be evaluated to number 2
    fn parse_value(&mut self) -> Result<ast::Value, ParseError> {
        let value = match self.tokens[self.curr_index].kind {
            TokenKind::Literal(lexer::Literal::Nat(nat)) => {
                self.step();
                let nat_cons_expr = if nat == 0 {
                    ast::NatConstructorExpr::Zero
                } else {
                    ast::NatConstructorExpr::Add1(nat - 1)
                };
                ast::Value::Nat(nat_cons_expr)
            }
            _ => return Err(ParseError),
        };

        Ok(value)
    }

    fn expect(&mut self, token: TokenKind) -> Result<(), ParseError> {
        match self.eat_term(token.clone()) {
            true => Ok(()),
            false => {
                dbg!(
                    "{observed} != {expected}",
                    &self.tokens[self.curr_index].kind,
                    token
                );
                Err(ParseError)
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
}

// TODO: Learn how to thoroughly test this
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::span::{Position, Span};
    use crate::lexer::{Keyword, Literal, Token, TokenKind};
    use crate::parser::ast::{self, Expression, ExpressionKind, Program, Type};

    use pretty_assertions::assert_eq;

    #[test]
    fn straightforward_the() {
        // (the Nat 1)
        let mut parser = Parser::new(vec![
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
        ]);
        let parse_tree = parser.parse().unwrap();

        assert_eq!(
            parse_tree,
            Program::new(vec![Expression::new(ExpressionKind::The(
                Type::Nat,
                ast::Value::Nat(ast::NatConstructorExpr::Add1(0)),
            )),]),
        );
    }
}
