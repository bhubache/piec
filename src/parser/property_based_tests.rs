use super::*;
use crate::parser::ast::{Declaration, DeclarationKind, Expression, ExpressionKind, Program};
use crate::span::Span;
use crate::test_utils;
use std::fmt;

use pretty_assertions::assert_eq;
use proptest::prelude::*;

fn dummy_span() -> Span {
    Span::new(0, 0)
}

fn arb_expr() -> impl Strategy<Value = Expression> {
    let leaf = prop_oneof![
        Just(ExpressionKind::Universe),
        Just(ExpressionKind::Nat),
        Just(ExpressionKind::Zero),
        Just(ExpressionKind::Atom),
        Just(ExpressionKind::Trivial),
        Just(ExpressionKind::Sole),
        Just(ExpressionKind::Nil),
        Just(ExpressionKind::Absurd),
        Just(ExpressionKind::Vecnil),
        Just(ExpressionKind::Todo),
        any::<u32>().prop_map(ExpressionKind::NatLiteral),
        test_utils::arb_ident().prop_map(ExpressionKind::Variable),
    ]
    .prop_map(|kind| Expression::new(kind, Span::new(0, 0)));

    leaf.prop_recursive(3, 256, 10, |inner| {
        prop_oneof![
            inner
                .clone()
                .prop_map(|expr| Expression::new(ExpressionKind::Add1(Box::new(expr)), dummy_span())),
            inner
                .clone()
                .prop_map(|expr| Expression::new(ExpressionKind::Car(Box::new(expr)), dummy_span())),
            inner
                .clone()
                .prop_map(|expr| Expression::new(ExpressionKind::Cdr(Box::new(expr)), dummy_span())),
            inner
                .clone()
                .prop_map(|expr| Expression::new(ExpressionKind::Same(Box::new(expr)), dummy_span())),
            inner
                .clone()
                .prop_map(|expr| Expression::new(ExpressionKind::Symm(Box::new(expr)), dummy_span())),
            inner
                .clone()
                .prop_map(|expr| Expression::new(ExpressionKind::Left(Box::new(expr)), dummy_span())),
            inner
                .clone()
                .prop_map(|expr| Expression::new(ExpressionKind::Right(Box::new(expr)), dummy_span())),
            inner
                .clone()
                .prop_map(|expr| Expression::new(ExpressionKind::List(Box::new(expr)), dummy_span())),
            inner
                .clone()
                .prop_map(|expr| Expression::new(ExpressionKind::List(Box::new(expr)), dummy_span())),
            (inner.clone(), inner.clone())
                .prop_map(|(e1, e2)| Expression::new(ExpressionKind::Cong(Box::new(e1), Box::new(e2)), dummy_span())),
            (inner.clone(), inner.clone())
                .prop_map(|(e1, e2)| Expression::new(ExpressionKind::Trans(Box::new(e1), Box::new(e2)), dummy_span())),
            (inner.clone(), inner.clone()).prop_map(|(e1, e2)| Expression::new(
                ExpressionKind::IndAbsurd(Box::new(e1), Box::new(e2)),
                dummy_span()
            )),
            (inner.clone(), inner.clone())
                .prop_map(|(e1, e2)| Expression::new(ExpressionKind::The(Box::new(e1), Box::new(e2)), dummy_span())),
            (inner.clone(), inner.clone())
                .prop_map(|(e1, e2)| Expression::new(ExpressionKind::Cons(Box::new(e1), Box::new(e2)), dummy_span())),
            (inner.clone(), inner.clone())
                .prop_map(|(e1, e2)| Expression::new(ExpressionKind::Pair(Box::new(e1), Box::new(e2)), dummy_span())),
            (inner.clone(), inner.clone()).prop_map(|(e1, e2)| Expression::new(
                ExpressionKind::VecCons(Box::new(e1), Box::new(e2)),
                dummy_span()
            )),
            (inner.clone(), inner.clone()).prop_map(|(e1, e2)| Expression::new(
                ExpressionKind::ListCons(Box::new(e1), Box::new(e2)),
                dummy_span()
            )),
            (inner.clone(), inner.clone())
                .prop_map(|(e1, e2)| Expression::new(ExpressionKind::Either(Box::new(e1), Box::new(e2)), dummy_span())),
            (inner.clone(), inner.clone())
                .prop_map(|(e1, e2)| Expression::new(ExpressionKind::Vec(Box::new(e1), Box::new(e2)), dummy_span())),
            (inner.clone(), inner.clone(), inner.clone()).prop_map(|(e1, e2, e3)| Expression::new(
                ExpressionKind::Replace(Box::new(e1), Box::new(e2), Box::new(e3)),
                dummy_span()
            )),
            (inner.clone(), inner.clone(), inner.clone()).prop_map(|(e1, e2, e3)| Expression::new(
                ExpressionKind::Eq(Box::new(e1), Box::new(e2), Box::new(e3)),
                dummy_span()
            )),
            (inner.clone(), inner.clone(), inner.clone()).prop_map(|(e1, e2, e3)| Expression::new(
                ExpressionKind::RecList(Box::new(e1), Box::new(e2), Box::new(e3)),
                dummy_span()
            )),
            (inner.clone(), inner.clone(), inner.clone()).prop_map(|(e1, e2, e3)| Expression::new(
                ExpressionKind::RecNat(Box::new(e1), Box::new(e2), Box::new(e3)),
                dummy_span()
            )),
            (inner.clone(), inner.clone(), inner.clone()).prop_map(|(e1, e2, e3)| Expression::new(
                ExpressionKind::IterNat(Box::new(e1), Box::new(e2), Box::new(e3)),
                dummy_span()
            )),
            (inner.clone(), inner.clone(), inner.clone()).prop_map(|(e1, e2, e3)| Expression::new(
                ExpressionKind::WhichNat(Box::new(e1), Box::new(e2), Box::new(e3)),
                dummy_span()
            )),
            (inner.clone(), inner.clone(), inner.clone()).prop_map(|(e1, e2, e3)| Expression::new(
                ExpressionKind::IndEq(Box::new(e1), Box::new(e2), Box::new(e3)),
                dummy_span()
            )),
            (inner.clone(), inner.clone(), inner.clone(), inner.clone()).prop_map(|(e1, e2, e3, e4)| Expression::new(
                ExpressionKind::IndEither(Box::new(e1), Box::new(e2), Box::new(e3), Box::new(e4)),
                dummy_span()
            )),
            (inner.clone(), inner.clone(), inner.clone(), inner.clone()).prop_map(|(e1, e2, e3, e4)| Expression::new(
                ExpressionKind::IndNat(Box::new(e1), Box::new(e2), Box::new(e3), Box::new(e4)),
                dummy_span()
            )),
            (inner.clone(), inner.clone(), inner.clone(), inner.clone()).prop_map(|(e1, e2, e3, e4)| Expression::new(
                ExpressionKind::IndList(Box::new(e1), Box::new(e2), Box::new(e3), Box::new(e4)),
                dummy_span()
            )),
            (
                inner.clone(),
                inner.clone(),
                inner.clone(),
                inner.clone(),
                inner.clone()
            )
                .prop_map(|(e1, e2, e3, e4, e5)| Expression::new(
                    ExpressionKind::IndVec(Box::new(e1), Box::new(e2), Box::new(e3), Box::new(e4), Box::new(e5)),
                    dummy_span()
                )),
            (prop::collection::vec(test_utils::arb_ident(), 1..4), inner.clone()).prop_map(|(args, e2)| {
                Expression::new(
                    ExpressionKind::Lambda(
                        args.into_iter().map(|arg| Spanned::new(arg, dummy_span())).collect(),
                        Box::new(e2),
                    ),
                    dummy_span(),
                )
            }),
            (
                prop::collection::vec((test_utils::arb_ident(), inner.clone()), 1..4),
                inner.clone()
            )
                .prop_map(|(args, e1)| Expression::new(
                    ExpressionKind::Pi(
                        args.into_iter()
                            .map(|(val, ty)| (Spanned::new(val, dummy_span()), Box::new(ty)))
                            .collect(),
                        Box::new(e1)
                    ),
                    dummy_span()
                ),),
            (
                prop::collection::vec((test_utils::arb_ident(), inner.clone()), 1..4),
                inner.clone()
            )
                .prop_map(|(args, e1)| Expression::new(
                    ExpressionKind::Sigma(
                        args.into_iter()
                            .map(|(val, ty)| (Spanned::new(val, dummy_span()), Box::new(ty)))
                            .collect(),
                        Box::new(e1)
                    ),
                    dummy_span()
                ),),
        ]
    })
}

fn arb_decl_kind() -> impl Strategy<Value = DeclarationKind> {
    prop_oneof![
        (test_utils::arb_ident(), arb_expr())
            .prop_map(|(name, expr)| DeclarationKind::Claim(Spanned::new(name, Span::new(0, 0)), expr)),
        (test_utils::arb_ident(), arb_expr())
            .prop_map(|(name, expr)| DeclarationKind::Define(Spanned::new(name, Span::new(0, 0)), expr)),
        (arb_expr(), arb_expr(), arb_expr()).prop_map(|(e1, e2, e3)| DeclarationKind::SamenessCheck(e1, e2, e3)),
        arb_expr().prop_map(DeclarationKind::Expr),
    ]
}

fn arb_decl() -> impl Strategy<Value = Declaration> {
    arb_decl_kind().prop_map(|decl_kind| Declaration::new(decl_kind, Span::new(0, 0)))
}

proptest! {
    #![proptest_config(ProptestConfig {
        timeout: 300 * 1000,
        .. ProptestConfig::default()
    })]

    #[test]
    fn round_trip(expected_decl in arb_decl()) {
        let source_code = expected_decl.to_string();
        let file = SourceFile::new("test_input".to_string(), source_code);
        let lexer = crate::lexer::Lexer::new(&file);
        let tokens = lexer.lex().unwrap();
        let parser = Parser::new(&file, tokens);

        let maybe_ast = parser.parse();

        match maybe_ast {
            Ok(mut observed) => {
                observed.clear_spans();
                assert_eq!(observed.decls.len(), 1);
                assert_eq!(observed.decls[0], expected_decl);
            }
            Err(diagnostics) => {
                for diag in diagnostics.iter() {
                    diag.report();
                }

                panic!("Errors occurred during parsing");
            }
        }
    }
}

impl Program {
    fn clear_spans(&mut self) {
        for decl in self.decls.iter_mut() {
            decl.clear_spans();
        }
    }
}

impl Declaration {
    fn clear_spans(&mut self) {
        self.span = dummy_span();
        self.node.clear_spans();
    }
}

impl DeclarationKind {
    fn clear_spans(&mut self) {
        match self {
            Self::Claim(spanned_str, expr) => {
                spanned_str.span = dummy_span();
                expr.clear_spans();
            }
            Self::Define(spanned_str, expr) => {
                spanned_str.span = dummy_span();
                expr.clear_spans();
            }
            Self::SamenessCheck(e1, e2, e3) => {
                e1.clear_spans();
                e2.clear_spans();
                e3.clear_spans();
            }
            Self::Expr(expr) => expr.clear_spans(),
        }
    }
}

impl Expression {
    fn clear_spans(&mut self) {
        self.span = dummy_span();
        self.node.clear_spans();
    }
}

impl ExpressionKind {
    fn clear_spans(&mut self) {
        match self {
            Self::Add1(e1) => e1.clear_spans(),
            Self::Car(e1) => e1.clear_spans(),
            Self::Cdr(e1) => e1.clear_spans(),
            Self::Same(e1) => e1.clear_spans(),
            Self::Symm(e1) => e1.clear_spans(),
            Self::Left(e1) => e1.clear_spans(),
            Self::Right(e1) => e1.clear_spans(),
            Self::List(e1) => e1.clear_spans(),
            Self::Cong(e1, e2) => {
                e1.clear_spans();
                e2.clear_spans();
            }
            Self::Trans(e1, e2) => {
                e1.clear_spans();
                e2.clear_spans();
            }
            Self::IndAbsurd(e1, e2) => {
                e1.clear_spans();
                e2.clear_spans();
            }
            Self::The(e1, e2) => {
                e1.clear_spans();
                e2.clear_spans();
            }
            Self::Cons(e1, e2) => {
                e1.clear_spans();
                e2.clear_spans();
            }
            Self::Pair(e1, e2) => {
                e1.clear_spans();
                e2.clear_spans();
            }
            Self::VecCons(e1, e2) => {
                e1.clear_spans();
                e2.clear_spans();
            }
            Self::ListCons(e1, e2) => {
                e1.clear_spans();
                e2.clear_spans();
            }
            Self::Either(e1, e2) => {
                e1.clear_spans();
                e2.clear_spans();
            }
            Self::Vec(e1, e2) => {
                e1.clear_spans();
                e2.clear_spans();
            }
            Self::Replace(e1, e2, e3) => {
                e1.clear_spans();
                e2.clear_spans();
                e3.clear_spans();
            }
            Self::Eq(e1, e2, e3) => {
                e1.clear_spans();
                e2.clear_spans();
                e3.clear_spans();
            }
            Self::RecList(e1, e2, e3) => {
                e1.clear_spans();
                e2.clear_spans();
                e3.clear_spans();
            }
            Self::RecNat(e1, e2, e3) => {
                e1.clear_spans();
                e2.clear_spans();
                e3.clear_spans();
            }
            Self::IterNat(e1, e2, e3) => {
                e1.clear_spans();
                e2.clear_spans();
                e3.clear_spans();
            }
            Self::WhichNat(e1, e2, e3) => {
                e1.clear_spans();
                e2.clear_spans();
                e3.clear_spans();
            }
            Self::IndEq(e1, e2, e3) => {
                e1.clear_spans();
                e2.clear_spans();
                e3.clear_spans();
            }
            Self::IndEither(e1, e2, e3, e4) => {
                e1.clear_spans();
                e2.clear_spans();
                e3.clear_spans();
                e4.clear_spans();
            }
            Self::IndNat(e1, e2, e3, e4) => {
                e1.clear_spans();
                e2.clear_spans();
                e3.clear_spans();
                e4.clear_spans();
            }
            Self::IndList(e1, e2, e3, e4) => {
                e1.clear_spans();
                e2.clear_spans();
                e3.clear_spans();
                e4.clear_spans();
            }
            Self::IndVec(e1, e2, e3, e4, e5) => {
                e1.clear_spans();
                e2.clear_spans();
                e3.clear_spans();
                e4.clear_spans();
                e5.clear_spans();
            }
            Self::Lambda(args, expr) => {
                for spanned_str in args.iter_mut() {
                    spanned_str.span = dummy_span();
                }
                expr.clear_spans();
            }
            Self::Pi(args, expr) | Self::Sigma(args, expr) => {
                for (spanned_str, arg_ty) in args.iter_mut() {
                    spanned_str.span = dummy_span();
                    arg_ty.clear_spans();
                }
                expr.clear_spans();
            }
            _ => (),
        }
    }
}

impl<T: fmt::Display> fmt::Display for Spanned<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.node)
    }
}

impl fmt::Display for DeclarationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Claim(name, expr) => write!(f, "(claim {} {})", name, expr),
            Self::Define(name, expr) => write!(f, "(define {} {})", name, expr),
            Self::SamenessCheck(expr1, expr2, expr3) => write!(f, "(check-same {} {} {})", expr1, expr2, expr3),
            Self::Expr(expr) => write!(f, "{}", expr),
        }
    }
}

impl fmt::Display for ExpressionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Universe => write!(f, "U"),
            Self::Nat => write!(f, "Nat"),
            Self::Zero => write!(f, "zero"),
            Self::Atom => write!(f, "Atom"),
            Self::Trivial => write!(f, "Trivial"),
            Self::Sole => write!(f, "sole"),
            Self::Nil => write!(f, "nil"),
            Self::Absurd => write!(f, "Absurd"),
            Self::Vecnil => write!(f, "vecnil"),
            Self::Todo => write!(f, "TODO"),
            Self::NatLiteral(value) => write!(f, "{}", value),
            Self::Quote(value) => write!(f, "{}", value),
            Self::Variable(value) => write!(f, "{}", value),

            Self::Add1(value) => write!(f, "(add1 {})", value),
            Self::Car(value) => write!(f, "(car {})", value),
            Self::Cdr(value) => write!(f, "(cdr {})", value),
            Self::Same(value) => write!(f, "(same {})", value),
            Self::Symm(value) => write!(f, "(symm {})", value),
            Self::Left(value) => write!(f, "(left {})", value),
            Self::Right(value) => write!(f, "(right {})", value),
            Self::List(value) => write!(f, "(List {})", value),

            Self::Cong(e1, e2) => write!(f, "(cong {} {})", e1, e2),
            Self::Trans(e1, e2) => write!(f, "(trans {} {})", e1, e2),
            Self::IndAbsurd(e1, e2) => write!(f, "(ind-Absurd {} {})", e1, e2),
            Self::The(e1, e2) => write!(f, "(the {} {})", e1, e2),
            Self::Cons(e1, e2) => write!(f, "(cons {} {})", e1, e2),
            Self::Pair(e1, e2) => write!(f, "(Pair {} {})", e1, e2),
            Self::VecCons(e1, e2) => write!(f, "(vec:: {} {})", e1, e2),
            Self::ListCons(e1, e2) => write!(f, "(:: {} {})", e1, e2),
            Self::Either(e1, e2) => write!(f, "(Either {} {})", e1, e2),
            Self::Vec(e1, e2) => write!(f, "(Vec {} {})", e1, e2),

            Self::Replace(e1, e2, e3) => write!(f, "(replace {} {} {})", e1, e2, e3),
            Self::Eq(e1, e2, e3) => write!(f, "(= {} {} {})", e1, e2, e3),
            Self::RecList(e1, e2, e3) => write!(f, "(rec-List {} {} {})", e1, e2, e3),
            Self::RecNat(e1, e2, e3) => write!(f, "(rec-Nat {} {} {})", e1, e2, e3),
            Self::IterNat(e1, e2, e3) => write!(f, "(iter-Nat {} {} {})", e1, e2, e3),
            Self::WhichNat(e1, e2, e3) => write!(f, "(which-Nat {} {} {})", e1, e2, e3),
            Self::IndEq(e1, e2, e3) => write!(f, "(ind-= {} {} {})", e1, e2, e3),

            Self::IndEither(e1, e2, e3, e4) => write!(f, "(ind-Either {} {} {} {})", e1, e2, e3, e4),
            Self::IndNat(e1, e2, e3, e4) => write!(f, "(ind-Nat {} {} {} {})", e1, e2, e3, e4),
            Self::IndList(e1, e2, e3, e4) => write!(f, "(ind-List {} {} {} {})", e1, e2, e3, e4),

            Self::IndVec(e1, e2, e3, e4, e5) => write!(f, "(ind-Vec {} {} {} {} {})", e1, e2, e3, e4, e5),

            Self::Lambda(args, body) => {
                write!(
                    f,
                    "(λ ({}) {})",
                    args.iter()
                        .map(|spanned_str| spanned_str.node.clone())
                        .collect::<Vec<String>>()
                        .join(" "),
                    body,
                )
            }
            Self::Pi(args, body) => {
                write!(
                    f,
                    "(Π ({}) {})",
                    args.iter()
                        .map(|(spanned_str, arg_ty)| format!("({} {})", spanned_str.node.clone(), arg_ty))
                        .collect::<Vec<String>>()
                        .join(" "),
                    body,
                )
            }
            Self::Sigma(args, body) => {
                write!(
                    f,
                    "(Σ ({}) {})",
                    args.iter()
                        .map(|(spanned_str, arg_ty)| format!("({} {})", spanned_str.node.clone(), arg_ty))
                        .collect::<Vec<String>>()
                        .join(" "),
                    body,
                )
            }
            _ => todo!(),
        }
    }
}
