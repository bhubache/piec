#![cfg_attr(test, feature(stmt_expr_attributes))]

mod compiler;
pub use compiler::Compiler;
mod diagnostics;
mod lexer;
mod parser;
mod source;
mod span;

#[cfg(test)]
mod test_utils;
