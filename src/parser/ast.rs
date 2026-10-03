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
    // `(` `the` Type Expression `)`
    The(Type, Value),
    Todo,
    Type(Type),
    Universe(Universe),
    Value(Value),
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Absurd,
    Atom,
    Trivial,
    Nat,
    Pair(Box<Type>, Box<Type>),
    List(Box<Type>),
    Vec(Box<Type>, NatConstructorExpr),
    Either(Box<Type>, Box<Type>),
    Pi, // TODO: What does this take in?
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Sole,
    Quote(String),
    Nat(NatConstructorExpr),
    Cons(Box<Value>, Box<Value>),
    Lambda(Box<Function>),
    List(ListConstructorExpr),
    Vec(VecConstructorExpr),
    Either(Box<EitherConstructorExpr>),
}

#[derive(Debug, PartialEq)]
pub struct Function(FunctionArgument, FunctionBody);

#[derive(Debug, PartialEq)]
pub struct FunctionArgument(String);

#[derive(Debug, PartialEq)]
pub struct FunctionBody(Value);

#[derive(Debug, PartialEq)]
pub enum NatConstructorExpr {
    Zero,
    Add1(u32),
}

#[derive(Debug, PartialEq)]
pub enum ListConstructorExpr {
    Nil,
    // Cons(Value, List<Value>),
}

#[derive(Debug, PartialEq)]
pub enum VecConstructorExpr {
    Nil,
    // Cons(Value, Vec<E, add1 k>),
}

#[derive(Debug, PartialEq)]
pub enum EitherConstructorExpr {
    Left(Value),
    Right(Value),
}

#[derive(Debug, PartialEq)]
pub struct Universe {}

#[derive(Debug, PartialEq)]
pub enum Constructor {
    Sole,

    // TODO: What data should this contain?
    Quote, // Each atom (e.g. 'foo 'bar are themselves constructors)
    Zero,
    Add1(u32),
}

#[derive(Debug, PartialEq)]
pub enum Eliminator {
    // IndAbsurd { target: Type::Absurd, motive: Universe},
    // WhichNat { target, base, step },
}
