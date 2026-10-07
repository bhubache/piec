// TODO: Learn how to thoroughly test this
use super::*;

use pretty_assertions::assert_eq;

fn lex_helper(source: &str, expected: Vec<Token>) {
    let file = SourceFile::new("test_input".to_string(), source.to_string());
    let lexer = Lexer::new(&file);
    let observed = lexer.lex().unwrap();

    assert_eq!(observed, expected);
}

fn lex_helper_neg(source: &str) {
    let file = SourceFile::new("test_input".to_string(), source.to_string());
    let lexer = Lexer::new(&file);

    assert!(lexer.lex().is_err());
}

#[test]
fn open_paren() {
    lex_helper(
        "(",
        vec![Token::new(
            TokenKind::OpenParen,
            Span::new(Position::new(0, 0), Position::new(0, 1)),
        )],
    );
}

#[test]
fn invalid_token() {
    // TODO: More precise test?
    lex_helper_neg("'");
}

#[test]
fn list_cons() {
    lex_helper(
        "::",
        vec![Token::new(
            TokenKind::Keyword(Keyword::ListCons),
            Span::new(Position::new(0, 0), Position::new(0, 2)),
        )],
    );
}

#[test]
fn multiple_open_paren() {
    lex_helper(
        "(((",
        vec![
            Token::new(
                TokenKind::OpenParen,
                Span::new(Position::new(0, 0), Position::new(0, 1)),
            ),
            Token::new(
                TokenKind::OpenParen,
                Span::new(Position::new(0, 1), Position::new(0, 2)),
            ),
            Token::new(
                TokenKind::OpenParen,
                Span::new(Position::new(0, 2), Position::new(0, 3)),
            ),
        ],
    );
}

#[test]
fn claim_nat() {
    lex_helper(
        "(claim foo\n  (Nat))",
        vec![
            Token::new(
                TokenKind::OpenParen,
                Span::new(Position::new(0, 0), Position::new(0, 1)),
            ),
            Token::new(
                TokenKind::Keyword(Keyword::Claim),
                Span::new(Position::new(0, 1), Position::new(0, 6)),
            ),
            Token::new(
                TokenKind::Identifier(String::from("foo")),
                Span::new(Position::new(0, 7), Position::new(0, 10)),
            ),
            Token::new(
                TokenKind::OpenParen,
                Span::new(Position::new(1, 2), Position::new(1, 3)),
            ),
            Token::new(
                TokenKind::Keyword(Keyword::Nat),
                Span::new(Position::new(1, 3), Position::new(1, 6)),
            ),
            Token::new(
                TokenKind::CloseParen,
                Span::new(Position::new(1, 6), Position::new(1, 7)),
            ),
            Token::new(
                TokenKind::CloseParen,
                Span::new(Position::new(1, 7), Position::new(1, 8)),
            ),
        ],
    );
}

#[test]
fn straightforward_the() {
    lex_helper(
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
                Span::new(Position::new(0, 5), Position::new(0, 8)),
            ),
            Token::new(
                TokenKind::Literal(Literal::Nat(1)),
                Span::new(Position::new(0, 9), Position::new(0, 10)),
            ),
            Token::new(
                TokenKind::CloseParen,
                Span::new(Position::new(0, 10), Position::new(0, 11)),
            ),
        ],
    );
}
