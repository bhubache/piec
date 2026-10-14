use crate::lexer::token;

use proptest::prelude::*;

pub fn arb_ident() -> impl Strategy<Value = String> {
    r#"((\\.|\|[^|]*\|)|([^",'`()\[\]{};\s\\|#])|(#%))((\\.|\|[^|]*\|)|([^",'`()\[\]{};\s\\|]))*"#
        .prop_filter("must not be a keyword", |ident| {
            token::Keyword::try_from(ident.as_str()).is_err()
        })
        // TODO: Is there a way to derive this from `token::Literal`?
        .prop_filter("must not be a literal", |ident| ident.parse::<usize>().is_err())
}
