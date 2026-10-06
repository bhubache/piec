use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl From<Span> for Range<usize> {
    fn from(value: Span) -> Self {
        Self {
            start: value.start.column,
            end: value.end.column,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    line: usize,
    column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}
