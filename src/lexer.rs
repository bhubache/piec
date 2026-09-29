use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Eof,
    Unknown,
}

pub struct Lexer<'a> {
    input: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars(),
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            match self.next_token() {
                Token::Eof => break,
                tok => tokens.push(tok),
            }
        }

        tokens
    }

    fn next_token(&mut self) -> Token {
        let Some(first_char) = self.input.next() else {
            return Token::Eof;
        };

        match first_char {
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            _ => Token::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_paren() {
        let mut lex = Lexer::new("(");
        let tokens = lex.lex();

        assert_eq!(tokens, vec![Token::OpenParen])
    }

    #[test]
    fn multiple_open_paren() {
        let mut lex = Lexer::new("(((");
        let tokens = lex.lex();

        assert_eq!(
            tokens,
            vec![Token::OpenParen, Token::OpenParen, Token::OpenParen]
        )
    }
}
