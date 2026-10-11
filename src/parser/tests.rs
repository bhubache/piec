// TODO: Learn how to thoroughly test this
use super::*;
use crate::parser::ast::{self, Expression, ExpressionKind, Program, Type};
use crate::span::Span;

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
            Token::new(TokenKind::OpenParen, Span::new(0, 1)),
            Token::new(TokenKind::Keyword(Keyword::The), Span::new(1, 4)),
            Token::new(TokenKind::Keyword(Keyword::Nat), Span::new(5, 8)),
            Token::new(TokenKind::Literal(Literal::Nat(1)), Span::new(9, 10)),
            Token::new(TokenKind::CloseParen, Span::new(10, 11)),
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
            Token::new(TokenKind::OpenParen, Span::new(0, 1)),
            Token::new(TokenKind::Keyword(Keyword::The), Span::new(1, 4)),
            Token::new(TokenKind::Literal(Literal::Nat(123)), Span::new(5, 8)),
            Token::new(TokenKind::Literal(Literal::Nat(1)), Span::new(9, 10)),
            Token::new(TokenKind::CloseParen, Span::new(10, 11)),
        ],
    );
}
