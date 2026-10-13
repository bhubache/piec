use crate::span::Span;

#[derive(Debug, PartialEq)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }
}

pub type Expression = Spanned<ExpressionKind>;

#[derive(Debug, PartialEq)]
pub struct Program {
    decls: Vec<Declaration>,
}

impl Program {
    pub fn new(decls: Vec<Declaration>) -> Self {
        Self { decls }
    }
}

#[derive(Debug, PartialEq)]
pub enum DeclarationKind {
    Claim(Spanned<String>, Expression),
    Define(Spanned<String>, Expression),
    SamenessCheck(Expression, Expression, Expression),
    Expr(Expression),
}

pub type Declaration = Spanned<DeclarationKind>;

#[derive(Debug, PartialEq)]
pub enum ExpressionKind {
    // Zero arguments
    Universe,
    Nat,
    Zero,
    Atom,
    Trivial,
    Sole,
    Nil,
    Absurd,
    Vecnil,
    Todo,

    // Literals
    NatLiteral(u32),
    Quote(String),
    Variable(String),

    // Unary
    Add1(Box<Expression>),
    Car(Box<Expression>),
    Cdr(Box<Expression>),
    Same(Box<Expression>),
    Symm(Box<Expression>),
    Left(Box<Expression>),
    Right(Box<Expression>),
    List(Box<Expression>),

    // Binary
    Cong(Box<Expression>, Box<Expression>),
    Trans(Box<Expression>, Box<Expression>),
    IndAbsurd(Box<Expression>, Box<Expression>),
    The(Box<Expression>, Box<Expression>),
    Cons(Box<Expression>, Box<Expression>),
    Pair(Box<Expression>, Box<Expression>),
    VecCons(Box<Expression>, Box<Expression>),
    ListCons(Box<Expression>, Box<Expression>),
    Either(Box<Expression>, Box<Expression>),
    Vec(Box<Expression>, Box<Expression>),

    // Ternary
    Replace(Box<Expression>, Box<Expression>, Box<Expression>),
    Eq(Box<Expression>, Box<Expression>, Box<Expression>),
    RecList(Box<Expression>, Box<Expression>, Box<Expression>),
    RecNat(Box<Expression>, Box<Expression>, Box<Expression>),
    IterNat(Box<Expression>, Box<Expression>, Box<Expression>),
    WhichNat(Box<Expression>, Box<Expression>, Box<Expression>),
    IndEq(Box<Expression>, Box<Expression>, Box<Expression>),

    // Quaternary
    IndEither(Box<Expression>, Box<Expression>, Box<Expression>, Box<Expression>),
    IndNat(Box<Expression>, Box<Expression>, Box<Expression>, Box<Expression>),
    IndList(Box<Expression>, Box<Expression>, Box<Expression>, Box<Expression>),

    // Quinary
    IndVec(
        Box<Expression>,
        Box<Expression>,
        Box<Expression>,
        Box<Expression>,
        Box<Expression>,
    ),

    // Binding
    Lambda(Vec<Spanned<String>>, Box<Expression>),
    Pi(Vec<(Spanned<String>, Box<Expression>)>, Box<Expression>),
    Sigma(Vec<(Spanned<String>, Box<Expression>)>, Box<Expression>),

    // Application
    Application(Box<Expression>, Box<Expression>, Vec<Expression>),
}
