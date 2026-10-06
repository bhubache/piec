use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Token { kind, span }
    }
}

// TODO:
// - error handling
// - comments
// - Look through pie lexer for rest
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// `(`
    OpenParen,
    /// `)`
    CloseParen,
    /// `,`
    Comma,
    Literal(Literal),
    Atom(String),
    Identifier(String),
    Keyword(Keyword),
    Eof,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Nat(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    /// `Π` - 0x3A0
    /// Function types
    GreekCapitalLetterPi,

    /// `∏` - 0x220F
    /// Alias for GreekCapitalLetterPi symbol that's easier to type on some keyboards
    NaryProduct,

    /// `→` - 0x2192
    RightwardsArrow,

    /// Alias for RightwardsArrow symbol
    WideRightwardsArrow,

    /// `λ` - 0x3BB
    LambdaSymbol,

    /// `Σ` - 0x3A3
    SigmaSymbol,

    /// `::`
    ListCons,
    Absurd,
    Atom,
    Either,
    Eq,
    List,
    Nat,
    Pair,

    /// Alias for `GreekCapitalLetterPi`
    Pi,

    /// Alias for `SigmaSymbol`
    Sigma,
    Todo,
    Trivial,
    Universe,
    Vec,
    Add1,
    Car,
    Cdr,
    CheckSame,
    Claim,
    Cong,
    Cons,
    Define,
    DefinePieKw,
    DefineMultiplePieKw,
    Head,
    IndEquals,
    IndAbsurd,
    IndEither,
    IndList,
    IndNat,
    IndVec,
    IterNat,

    /// Alias for `LambdaSymbol`
    Lambda,
    Left,
    Nil,
    Quote,
    RecList,
    RecNat,
    Replace,
    Right,
    Same,
    Sole,
    Symm,
    Tail,
    The,
    Trans,
    VecCons,
    VecNil,
    WhichNat,
    Zero,
}

impl Keyword {
    pub fn new(string: &str) -> Option<Self> {
        let keyword = match string {
            "Π" => Self::GreekCapitalLetterPi,
            "∏" => Self::NaryProduct,
            "→" => Self::RightwardsArrow,
            "->" => Self::WideRightwardsArrow,
            "λ" => Self::LambdaSymbol,
            "Σ" => Self::SigmaSymbol,
            "::" => Self::ListCons,
            "Absurd" => Self::Absurd,
            "Atom" => Self::Atom,
            "Either" => Self::Either,
            "=" => Self::Eq,
            "List" => Self::List,
            "Nat" => Self::Nat,
            "Pair" => Self::Pair,
            "Pi" => Self::Pi,
            "Sigma" => Self::Sigma,
            "TODO" => Self::Todo,
            "Trivial" => Self::Trivial,
            "U" => Self::Universe,
            "Vec" => Self::Vec,
            "add1" => Self::Add1,
            "car" => Self::Car,
            "cdr" => Self::Cdr,
            "check-same" => Self::CheckSame,
            "claim" => Self::Claim,
            "cong" => Self::Cong,
            "cons" => Self::Cons,
            "define" => Self::Define,
            "define-pie-keywords" => Self::DefineMultiplePieKw,
            "define-pie-keyword" => Self::DefinePieKw,
            "head" => Self::Head,
            "ind-=" => Self::IndEquals,
            "ind-Absurd" => Self::IndAbsurd,
            "ind-Either" => Self::IndEither,
            "ind-List" => Self::IndList,
            "ind-Nat" => Self::IndNat,
            "ind-Vec" => Self::IndVec,
            "iter-Nat" => Self::IterNat,
            "lambda" => Self::Lambda,
            "left" => Self::Left,
            "nil" => Self::Nil,
            "quote" => Self::Quote,
            "rec-List" => Self::RecList,
            "rec-Nat" => Self::RecNat,
            "replace" => Self::Replace,
            "right" => Self::Right,
            "same" => Self::Same,
            "sole" => Self::Sole,
            "symm" => Self::Symm,
            "tail" => Self::Tail,
            "the" => Self::The,
            "trans" => Self::Trans,
            "vec::" => Self::VecCons,
            "vecnil" => Self::VecNil,
            "which-Nat" => Self::WhichNat,
            "zero" => Self::Zero,
            _ => return None,
        };

        Some(keyword)
    }
}
