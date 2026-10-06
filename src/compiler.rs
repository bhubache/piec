use std::fmt;
use std::fs;

use crate::lexer::Lexer;
use crate::source::SourceFile;

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
    file: SourceFile,
}

impl Compiler {
    pub fn from_file(path: &str) -> Result<Self, CompilerError> {
        let source = fs::read_to_string(path)?;

        Ok(Self {
            file: SourceFile::new(path.to_string(), source),
        })
    }

    pub fn compile(&self) -> Result<(), CompilerError> {
        let lexer = Lexer::new(&self.file);
        let _tokens = match lexer.lex() {
            Ok(tokens) => tokens,
            Err(diagnostics) => {
                for diag in diagnostics.iter() {
                    diag.report();
                }

                panic!("unable to lex due to {} above error(s)", diagnostics.len());
            }
        };

        Ok(())
    }
}
