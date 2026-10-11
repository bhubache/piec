#[derive(Debug, PartialEq)]
pub struct Program {
    exprs: Vec<Expression>,
}

impl Program {
    pub fn new(exprs: Vec<Expression>) -> Self {
        Self { exprs }
    }
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    kind: ExpressionKind,
}

impl Expression {
    pub fn new(kind: ExpressionKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug, PartialEq)]
pub enum ExpressionKind {
    Todo,
}
