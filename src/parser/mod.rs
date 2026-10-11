use crate::diagnostics::Diagnostic;
use crate::lexer::token::Keyword;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use crate::source::SourceFile;

mod ast;

#[cfg(test)]
mod tests;

pub struct Parser<'a> {
    file: &'a SourceFile,
    tokens: Vec<Token>,
    curr_index: usize,
    diagnostics: Vec<Diagnostic<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(file: &'a SourceFile, tokens: Vec<Token>) -> Self {
        Parser {
            file,
            tokens,
            curr_index: 0,
            diagnostics: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Result<ast::Program, Vec<Diagnostic<'a>>> {
        let mut expressions = Vec::new();
        while self.curr_index < self.tokens.len() {
            match self.parse_expr() {
                Ok(expr) => expressions.push(expr),
                Err(Recoverable::Yes) => continue,
                Err(Recoverable::No) => break,
            }
        }

        if !self.diagnostics.is_empty() {
            Err(self.diagnostics)
        } else {
            Ok(ast::Program::new(expressions))
        }
    }

    fn parse_expr(&mut self) -> Result<ast::Expression, Recoverable> {
        if self.eat_token(TokenKind::Keyword(Keyword::Todo)) {
            Ok(ast::Expression::new(ast::ExpressionKind::Todo))
        } else {
            Err(Recoverable::No)
        }
    }

    fn check(&self, token: TokenKind) -> bool {
        token == self.tokens[self.curr_index].kind
    }

    fn step(&mut self) {
        self.curr_index += 1;
    }

    fn eat_token(&mut self, token: TokenKind) -> bool {
        let matches = self.check(token);
        if matches {
            self.step();
        }

        matches
    }
}

enum Recoverable {
    Yes,
    No,
}
