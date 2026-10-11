// TODO: Learn how to thoroughly test this
use super::*;
use crate::lexer::token;
use crate::parser::ast::{Expression, ExpressionKind, Program};
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
fn just_todo() {
    run_parse(
        "TODO",
        vec![Token::new(
            TokenKind::Keyword(token::Keyword::Todo),
            Span::new(0, 4),
        )],
        Program::new(vec![Expression::new(ExpressionKind::Todo)]),
    )
}

#[test]
fn two_todos() {
    run_parse(
        "TODO TODO",
        vec![
            Token::new(TokenKind::Keyword(token::Keyword::Todo), Span::new(0, 4)),
            Token::new(TokenKind::Keyword(token::Keyword::Todo), Span::new(5, 9)),
        ],
        Program::new(vec![
            Expression::new(ExpressionKind::Todo),
            Expression::new(ExpressionKind::Todo),
        ]),
    )
}
