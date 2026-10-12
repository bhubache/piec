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
        let mut declarations = Vec::new();
        while self.curr_index < self.tokens.len() {
            match self.parse_declaration() {
                Ok(decl) => declarations.push(decl),
                Err(Recoverable::Yes) => continue,
                Err(Recoverable::No) => break,
            }
        }

        if !self.diagnostics.is_empty() {
            Err(self.diagnostics)
        } else {
            Ok(ast::Program::new(declarations))
        }
    }

    fn parse_declaration(&mut self) -> Result<ast::Declaration, Recoverable> {
        if self.check_first(TokenKind::OpenParen)
            && self.check_second(TokenKind::Keyword(Keyword::Claim))
        {
            self.step();
            self.step();
            todo!();
        } else if self.check_first(TokenKind::OpenParen)
            && self.check_second(TokenKind::Keyword(Keyword::Define))
        {
            self.step();
            self.step();
            todo!();
        } else if self.check_first(TokenKind::OpenParen)
            && self.check_second(TokenKind::Keyword(Keyword::CheckSame))
        {
            self.step();
            self.step();
            todo!();
        } else {
            // FIXME: Shouldn't unwrap here
            let expr = self.parse_expr().unwrap();
            let span = expr.span;
            Ok(ast::Declaration::new(
                ast::DeclarationKind::Expr(expr),
                span,
            ))
        }
    }

    fn parse_expr(&mut self) -> Result<ast::Expression, Recoverable> {
        if self.check_first(TokenKind::Keyword(Keyword::Todo)) {
            let tok = &self.tokens[self.curr_index];
            self.curr_index += 1;

            Ok(ast::Expression::new(ast::ExpressionKind::Todo, tok.span))
        } else {
            Err(Recoverable::No)
        }
    }

    fn check_first(&self, token: TokenKind) -> bool {
        token == self.tokens[self.curr_index].kind
    }

    fn check_second(&self, token: TokenKind) -> bool {
        token == self.tokens[self.curr_index + 1].kind
    }

    fn step(&mut self) {
        self.curr_index += 1;
    }

    fn eat_token(&mut self, token: TokenKind) -> Option<Token> {
        let matches = self.check_first(token);
        let tok_ret = self.tokens[self.curr_index].clone();
        if matches {
            self.step();
        }

        if matches { Some(tok_ret) } else { None }
    }
}

#[derive(Debug)]
enum Recoverable {
    Yes,
    No,
}
