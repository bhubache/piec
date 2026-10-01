#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    start: Position,
    end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub fn empty() -> Self {
        Span {
            start: Position::new(0, 0),
            end: Position::new(0, 0),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    line: usize,
    column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}
