use std::io::Read;

use super::*;
use gag::BufferRedirect;
use strum::IntoEnumIterator;

use pretty_assertions::assert_eq;
use proptest::prelude::*;

fn arb_token_kind() -> impl Strategy<Value = token::TokenKind> {
    prop_oneof![
        Just(TokenKind::OpenParen),
        Just(TokenKind::CloseParen),
        Just(TokenKind::Comma),
        r"'[a-zA-Z\-]+".prop_map(TokenKind::Atom),
        // TODO: Accept more
        r"[a-zA-Z][a-zA-Z0-9]*"
            .prop_map(TokenKind::Identifier)
            .prop_filter("must not be a keyword", |ident| token::Keyword::try_from(
                ident.to_string()
            )
            .is_err()),
        arb_keyword().prop_map(TokenKind::Keyword),
        arb_literal().prop_map(TokenKind::Literal),
    ]
}

fn arb_keyword() -> impl Strategy<Value = token::Keyword> {
    let keywords: Vec<token::Keyword> = token::Keyword::iter().collect();
    proptest::sample::select(keywords)
}

fn arb_literal() -> impl Strategy<Value = token::Literal> {
    let literals: Vec<token::Literal> = token::Literal::iter().collect();
    proptest::sample::select(literals)
}

proptest! {
    #[test]
    fn property_based(token_kinds in prop::collection::vec(arb_token_kind(), 1..1000)) {
        let token_strings: Vec<String> = token_kinds.iter().map(|tok| format!("{}", tok)).collect();
        let file = SourceFile::new("test_input".to_string(), token_strings.join(" "));
        let lexer = Lexer::new(&file);
        let observed = lexer.lex().unwrap();

        let observed_kinds: Vec<TokenKind> = observed.into_iter().map(|tok| tok.kind).collect();

        assert_eq!(observed_kinds, token_kinds);
    }
}

fn lex_helper(source: &str, expected: Vec<Token>) {
    let file = SourceFile::new("test_input".to_string(), source.to_string());
    let lexer = Lexer::new(&file);
    let observed = lexer.lex().unwrap();

    assert_eq!(observed, expected);
}

fn lex_helper_neg(source: &str, expected_diag_msgs: &str) {
    let file = SourceFile::new("test_input".to_string(), source.to_string());
    let lexer = Lexer::new(&file);
    let maybe_tokens = lexer.lex();

    match maybe_tokens {
        Ok(tokens) => panic!("String lexed when it should not have: {:?}", tokens),
        Err(diagnostics) => {
            let mut buf = BufferRedirect::stderr().unwrap();
            for diag in diagnostics.iter() {
                diag.report();
            }

            let mut output = String::new();
            buf.read_to_string(&mut output).unwrap();

            assert_eq!(
                strip_ansi_escapes::strip_str(&output[..]),
                expected_diag_msgs
            );
        }
    }
}

#[test]
fn invalid_token() {
    lex_helper_neg(
        "'",
        #[rustfmt::skip]
        "\
Error: lone `'`
   ╭─[ test_input:1:1 ]
   │
 1 │ '
   │ ┬  
   │ ╰── expected one or more alphabetic characters and/or hyphens to follow this to produce an Atom
───╯
",
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

#[test]
fn ident_starting_with_backslash() {
    lex_helper(
        r"\'d",
        vec![Token::new(
            TokenKind::Identifier(String::from(r"\'d")),
            Span::new(Position::new(0, 0), Position::new(0, 3)),
        )],
    );
}

#[test]
fn ident_starting_pound_percent() {
    lex_helper(
        r"#%foo",
        vec![Token::new(
            TokenKind::Identifier(String::from(r"#%foo")),
            Span::new(Position::new(0, 0), Position::new(0, 5)),
        )],
    );
}
