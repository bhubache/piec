use super::*;
use strum::IntoEnumIterator;

use pretty_assertions::assert_eq;
use proptest::prelude::*;

fn arb_token_kind() -> impl Strategy<Value = token::TokenKind> {
    prop_oneof![
        Just(TokenKind::OpenParen),
        Just(TokenKind::CloseParen),
        Just(TokenKind::Comma),
        r"'[a-zA-Z\-]+".prop_map(TokenKind::Atom),
        r#"((\\.|\|[^|]*\|)|([^",'`()\[\]{};\s\\|#])|(#%))((\\.|\|[^|]*\|)|([^",'`()\[\]{};\s\\|]))*"#
            .prop_map(TokenKind::Identifier)
            .prop_filter("must not be a keyword", |ident| token::Keyword::try_from(
                ident.to_string().as_str()
            )
            .is_err())
            // TODO: Is there a way to derive this from `token::Literal`?
            .prop_filter("must not be a literal", |ident| ident.to_string().parse::<usize>().is_err()),
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
    #![proptest_config(ProptestConfig {
        timeout: 300 * 1000,
        .. ProptestConfig::default()
    })]

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

proptest! {
    #![proptest_config(ProptestConfig {
        timeout: 300 * 1000,
        .. ProptestConfig::default()
    })]

    #[test]
    fn should_not_panic(
        arbitrary_string in prop::collection::vec(any::<char>(), 1..10000)
        .prop_map(|chars| chars.into_iter().collect::<String>())
    ) {
        let file = SourceFile::new("test_input".to_string(), arbitrary_string);
        let lexer = Lexer::new(&file);
        let _ = lexer.lex();
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
fn lone_quote() {
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
fn multiple_diagnostics() {
    lex_helper_neg(
        r"' foo \",
        #[rustfmt::skip]
        "\
Error: lone `'`
   ╭─[ test_input:1:1 ]
   │
 1 │ ' foo \\
   │ ┬  
   │ ╰── expected one or more alphabetic characters and/or hyphens to follow this to produce an Atom
───╯
Error: EOF following `\\`
   ╭─[ test_input:1:7 ]
   │
 1 │ ' foo \\
   │       ┬  
   │       ╰── expected any single character to follow this to complete, or start, an identifier
───╯
",
    );
}

#[test]
fn claim_nat() {
    lex_helper(
        "(claim foo\n  (Nat))",
        vec![
            Token::new(TokenKind::OpenParen, Span::new(0, 1)),
            Token::new(TokenKind::Keyword(Keyword::Claim), Span::new(1, 6)),
            Token::new(TokenKind::Identifier(String::from("foo")), Span::new(7, 10)),
            Token::new(TokenKind::OpenParen, Span::new(13, 14)),
            Token::new(TokenKind::Keyword(Keyword::Nat), Span::new(14, 17)),
            Token::new(TokenKind::CloseParen, Span::new(17, 18)),
            Token::new(TokenKind::CloseParen, Span::new(18, 19)),
        ],
    );
}

#[test]
fn straightforward_the() {
    lex_helper(
        "(the Nat 1)",
        vec![
            Token::new(TokenKind::OpenParen, Span::new(0, 1)),
            Token::new(TokenKind::Keyword(Keyword::The), Span::new(1, 4)),
            Token::new(TokenKind::Keyword(Keyword::Nat), Span::new(5, 8)),
            Token::new(TokenKind::Literal(Literal::Nat(1)), Span::new(9, 10)),
            Token::new(TokenKind::CloseParen, Span::new(10, 11)),
        ],
    );
}

#[test]
fn ident_starting_with_backslash() {
    lex_helper(
        r"\'d",
        vec![Token::new(
            TokenKind::Identifier(String::from(r"\'d")),
            Span::new(0, 3),
        )],
    );
}

#[test]
fn ident_starting_with_pipe() {
    lex_helper(
        r"|foo|",
        vec![Token::new(
            TokenKind::Identifier(String::from(r"|foo|")),
            Span::new(0, 5),
        )],
    );
}

#[test]
fn ident_starting_pound_percent() {
    lex_helper(
        r"#%foo",
        vec![Token::new(
            TokenKind::Identifier(String::from(r"#%foo")),
            Span::new(0, 5),
        )],
    );
}

#[test]
fn backslash_ident_incomplete() {
    lex_helper_neg(
        r"foo\",
        #[rustfmt::skip]
        "\
Error: EOF following `\\`
   ╭─[ test_input:1:1 ]
   │
 1 │ foo\\
   │ ──┬─  
   │   ╰─── expected any single character to follow this to complete, or start, an identifier
───╯
",
    );
}

#[test]
fn pipe_ident_incomplete() {
    lex_helper_neg(
        r"|foo",
        #[rustfmt::skip]
        "\
Error: EOF encountered when searching for enclosing `|`
   ╭─[ test_input:1:1 ]
   │
 1 │ |foo
   │ ──┬─  
   │   ╰─── expected a `|` at the end to complete the identifier
───╯
",
    );
}

#[test]
fn missing_character_after_pound() {
    lex_helper_neg(
        r"#",
        #[rustfmt::skip]
        "\
Error: EOF following `#`
   ╭─[ test_input:1:1 ]
   │
 1 │ #
   │ ┬  
   │ ╰── expected a `%` to follow this to complete, or start, an identifier
───╯
",
    );
}

#[test]
fn incorrect_character_following_pound() {
    lex_helper_neg(
        r"#5",
        #[rustfmt::skip]
        "\
Error: Unexpected character `5` following `#`
   ╭─[ test_input:1:1 ]
   │
 1 │ #5
   │  ┬  
   │  ╰── expected this character to be `%`, indicating an identifier or the start of an identifier
───╯
",
    );
}

#[test]
fn empty_string() {
    lex_helper("", vec![]);
}

#[test]
fn multi_line_pipe_ident_incomplete() {
    lex_helper_neg(
        r"|foo
bar||",
        #[rustfmt::skip]

        "\
Error: EOF encountered when searching for enclosing `|`
   ╭─[ test_input:1:1 ]
   │
 1 │ ╭─▶ |foo
 2 │ ├─▶ bar||
   │ │          
   │ ╰────────── expected a `|` at the end to complete the identifier
───╯
",
    );
}
