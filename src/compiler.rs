use std::fmt;
use std::fs;

use crate::lexer::Lexer;

#[derive(Debug)]
pub enum CompilerError {
    IoError(std::io::Error),
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::IoError(error) => error.fmt(f),
        }
    }
}

impl From<std::io::Error> for CompilerError {
    fn from(value: std::io::Error) -> Self {
        CompilerError::IoError(value)
    }
}

pub struct Compiler {
    text: String,
}

impl Compiler {
    pub fn from_file(path: &str) -> Result<Self, CompilerError> {
        let text = fs::read_to_string(path)?;

        Ok(Compiler { text })
    }

    pub fn compile(&self) -> Result<(), CompilerError> {
        let mut lexer = Lexer::new(&self.text);
        let _tokens = lexer.lex();

        Ok(())
    }
}
