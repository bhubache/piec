use core::fmt;

use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
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
    /// `"`
    DoubleQuote,

    /// `,`
    Comma,

    /// `````
    BackTick,

    /// `(`
    OpenParen,

    /// `)`
    CloseParen,

    /// `[`
    OpenSquareBracket,

    /// `]`
    CloseSquareBracket,

    /// `{`
    OpenCurlyBrace,

    /// `}`
    CloseCurlyBrace,

    /// `;` starts the comment
    LineComment(String),

    Literal(Literal),
    Atom(String),
    Identifier(String),
    Keyword(Keyword),
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DoubleQuote => write!(f, "\""),
            Self::Comma => write!(f, ","),
            Self::BackTick => write!(f, "`"),
            Self::OpenParen => write!(f, "("),
            Self::CloseParen => write!(f, ")"),
            Self::OpenSquareBracket => write!(f, "["),
            Self::CloseSquareBracket => write!(f, "]"),
            Self::OpenCurlyBrace => write!(f, "{{"),
            Self::CloseCurlyBrace => write!(f, "}}"),
            Self::LineComment(comment) => write!(f, ";{}", comment),
            Self::Literal(lit) => write!(f, "{}", lit),
            Self::Atom(atom) => write!(f, "{}", atom),
            Self::Identifier(ident) => write!(f, "{}", ident),
            Self::Keyword(keyword) => write!(f, "{}", keyword),
        }
    }
}

#[cfg_attr(test, derive(strum_macros::EnumIter))]
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Nat(u32),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nat(nat) => write!(f, "{}", nat),
        }
    }
}

#[cfg_attr(test, derive(strum_macros::EnumIter))]
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

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl From<&Keyword> for String {
    fn from(value: &Keyword) -> Self {
        let string = match value {
            Keyword::GreekCapitalLetterPi => "Π",
            Keyword::NaryProduct => "∏",
            Keyword::RightwardsArrow => "→",
            Keyword::WideRightwardsArrow => "->",
            Keyword::LambdaSymbol => "λ",
            Keyword::SigmaSymbol => "Σ",
            Keyword::ListCons => "::",
            Keyword::Absurd => "Absurd",
            Keyword::Atom => "Atom",
            Keyword::Either => "Either",
            Keyword::Eq => "=",
            Keyword::List => "List",
            Keyword::Nat => "Nat",
            Keyword::Pair => "Pair",
            Keyword::Pi => "Pi",
            Keyword::Sigma => "Sigma",
            Keyword::Todo => "TODO",
            Keyword::Trivial => "Trivial",
            Keyword::Universe => "U",
            Keyword::Vec => "Vec",
            Keyword::Add1 => "add1",
            Keyword::Car => "car",
            Keyword::Cdr => "cdr",
            Keyword::CheckSame => "check-same",
            Keyword::Claim => "claim",
            Keyword::Cong => "cong",
            Keyword::Cons => "cons",
            Keyword::Define => "define",
            Keyword::DefineMultiplePieKw => "define-pie-keywords",
            Keyword::DefinePieKw => "define-pie-keyword",
            Keyword::Head => "head",
            Keyword::IndEquals => "ind-=",
            Keyword::IndAbsurd => "ind-Absurd",
            Keyword::IndEither => "ind-Either",
            Keyword::IndList => "ind-List",
            Keyword::IndNat => "ind-Nat",
            Keyword::IndVec => "ind-Vec",
            Keyword::IterNat => "iter-Nat",
            Keyword::Lambda => "lambda",
            Keyword::Left => "left",
            Keyword::Nil => "nil",
            Keyword::Quote => "quote",
            Keyword::RecList => "rec-List",
            Keyword::RecNat => "rec-Nat",
            Keyword::Replace => "replace",
            Keyword::Right => "right",
            Keyword::Same => "same",
            Keyword::Sole => "sole",
            Keyword::Symm => "symm",
            Keyword::Tail => "tail",
            Keyword::The => "the",
            Keyword::Trans => "trans",
            Keyword::VecCons => "vec::",
            Keyword::VecNil => "vecnil",
            Keyword::WhichNat => "which-Nat",
            Keyword::Zero => "zero",
        };

        String::from(string)
    }
}

impl TryFrom<&str> for Keyword {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let keyword = match value {
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
            _ => return Err(()),
        };

        Ok(keyword)
    }
}
