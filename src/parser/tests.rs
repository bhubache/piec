// TODO: Learn how to thoroughly test this
use super::*;
use crate::lexer::token;
use crate::parser::ast::{Declaration, DeclarationKind, Expression, ExpressionKind, Program};
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

fn run_parse_neg(source: &str, tokens: Vec<Token>, expected_diag_msgs: &str) {
    let file = SourceFile::new("test_input".to_string(), source.to_string());
    let parser = Parser::new(&file, tokens);

    let maybe_ast = parser.parse();

    match maybe_ast {
        Ok(ast) => panic!("Tokens parsed when it should not have: {:?}", ast),
        Err(diagnostics) => {
            let mut vec = Vec::new();
            for diag in diagnostics.iter() {
                diag.write_to(&mut vec);
            }

            let output = String::from_utf8(vec).unwrap();

            println!("{}", output);

            assert_eq!(
                strip_ansi_escapes::strip_str(&output[..]),
                expected_diag_msgs
            );
        }
    }
}

#[test]
fn no_tokens() {
    run_parse("", vec![], Program::new(vec![]));
}

#[test]
fn just_todo() {
    run_parse(
        "TODO",
        vec![Token::new(
            TokenKind::Keyword(token::Keyword::Todo),
            Span::new(0, 4),
        )],
        Program::new(vec![Declaration::new(
            DeclarationKind::Expr(Expression::new(ExpressionKind::Todo, Span::new(0, 4))),
            Span::new(0, 4),
        )]),
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
            Declaration::new(
                DeclarationKind::Expr(Expression::new(ExpressionKind::Todo, Span::new(0, 4))),
                Span::new(0, 4),
            ),
            Declaration::new(
                DeclarationKind::Expr(Expression::new(ExpressionKind::Todo, Span::new(5, 9))),
                Span::new(5, 9),
            ),
        ]),
    )
}

#[test]
fn simple_claim() {
    run_parse(
        "\
(claim foo
  Nat)",
        vec![
            Token::new(TokenKind::OpenParen, Span::new(0, 1)),
            Token::new(TokenKind::Keyword(token::Keyword::Claim), Span::new(1, 6)),
            Token::new(TokenKind::Identifier(String::from("foo")), Span::new(7, 10)),
            Token::new(TokenKind::Keyword(token::Keyword::Nat), Span::new(14, 17)),
            Token::new(TokenKind::CloseParen, Span::new(17, 18)),
        ],
        Program::new(vec![Declaration::new(
            DeclarationKind::Claim(
                Spanned::new(String::from("foo"), Span::new(7, 10)),
                Expression::new(ExpressionKind::Nat, Span::new(14, 17)),
            ),
            Span::new(0, 18),
        )]),
    );
}

// TODO: Figure out how to best include additional context such as that the missing token is part
// of a `claim` declaration
#[test]
fn claim_missing_trailing_closing_paren() {
    run_parse_neg(
        "\
(claim foo
  Nat",
        vec![
            Token::new(TokenKind::OpenParen, Span::new(0, 1)),
            Token::new(TokenKind::Keyword(token::Keyword::Claim), Span::new(1, 6)),
            Token::new(TokenKind::Identifier(String::from("foo")), Span::new(7, 10)),
            Token::new(TokenKind::Keyword(token::Keyword::Nat), Span::new(13, 16)),
        ],
        "\
Error: unexpected EOF
   ╭─[ test_input:2:3 ]
   │
 2 │   Nat
   │      │ 
   │      ╰─ expected `)`
───╯
",
    );
}

#[test]
fn claim_missing_name() {
    run_parse_neg(
        "\
(claim
  Nat)",
        vec![
            Token::new(TokenKind::OpenParen, Span::new(0, 1)),
            Token::new(TokenKind::Keyword(token::Keyword::Claim), Span::new(1, 6)),
            Token::new(TokenKind::Keyword(token::Keyword::Nat), Span::new(9, 12)),
            Token::new(TokenKind::CloseParen, Span::new(12, 13)),
        ],
        "\
Error: missing identifier
   ╭─[ test_input:2:3 ]
   │
 1 │ (claim
   │  ──┬──  
   │    ╰──── expected name to follow this
 2 │   Nat)
   │   ─┬─  
   │    ╰─── instead got `Nat`
───╯
",
    );
}
