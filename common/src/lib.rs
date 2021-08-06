use std::fmt;

fn escaped_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());

    for c in s.chars() {
        match c {
            '\\' => result.push_str("\\\\"),
            '\"' => result.push_str("\\\""),
            '\n' => result.push_str("\\n"),
            '\t' => result.push_str("\\t"),
            '\x08' => result.push_str("\\b"),
            '\x0C' => result.push_str("\\f"),
            c if c.is_ascii() && !c.is_control() => result.push(c),
            c => result.push_str(&format!("\\{:03o}", c as u32)),
        }
    }

    result
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum KeywordKind {
    Class,
    Else,
    Fi,
    If,
    In,
    Inherits,
    IsVoid,
    Let,
    Loop,
    Pool,
    Then,
    While,
    Case,
    Esac,
    New,
    Of,
    Not,
}

impl fmt::Display for KeywordKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Class => write!(f, "CLASS"),
            Self::Else => write!(f, "ELSE"),
            Self::Fi => write!(f, "FI"),
            Self::If => write!(f, "IF"),
            Self::In => write!(f, "IN"),
            Self::Inherits => write!(f, "INHERITS"),
            Self::IsVoid => write!(f, "ISVOID"),
            Self::Let => write!(f, "LET"),
            Self::Loop => write!(f, "LOOP"),
            Self::Pool => write!(f, "POOL"),
            Self::Then => write!(f, "THEN"),
            Self::While => write!(f, "WHILE"),
            Self::Case => write!(f, "CASE"),
            Self::Esac => write!(f, "ESAC"),
            Self::New => write!(f, "NEW"),
            Self::Of => write!(f, "OF"),
            Self::Not => write!(f, "NOT"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TokenKind {
    // Any sequence of space(ascii 32), \n(10), \f(12), \r(13), \t(9), \v(11)
    Whitespace,
    ObjectId(String),
    TypeId(String),
    Int(String),
    String(String),
    Bool(bool),

    // Comments
    LineComment,
    BlockComment,

    // Keywords
    Keyword(KeywordKind),

    // Operator
    Plus,
    Minus,
    Star,
    Slash,
    Tilde,
    /// Less Than "<"
    Lt,
    /// Less "<="
    Le,
    /// "=>"
    DArrow,

    /// "<-"
    Assign,

    // Single Characters
    //
    /// ":"
    Colon,

    /// "‚"
    Comma,

    /// "."
    Dot,

    /// "="
    Equal,

    /// "("
    OpenParen,

    /// ")"
    CloseParen,

    /// "{"
    OpenBrace,

    /// "{"
    CloseBrace,

    /// "@"
    At,

    /// ";"
    SemiColon,

    Error(String),
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Whitespace => write!(f, ""),
            Self::ObjectId(s) => write!(f, "OBJECTID {}", s),
            Self::TypeId(s) => write!(f, "TYPEID {}", s),
            Self::Int(v) => write!(f, "INT_CONST {}", v),
            Self::String(s) => write!(f, "STR_CONST \"{}\"", escaped_string(s)),
            Self::Bool(b) => write!(f, "BOOL_CONST {}", b),

            Self::LineComment => write!(f, ""),
            Self::BlockComment => write!(f, ""),

            Self::Keyword(k) => write!(f, "{}", k),

            Self::Plus => write!(f, "'+'"),
            Self::Minus => write!(f, "'-'"),
            Self::Star => write!(f, "'*'"),
            Self::Slash => write!(f, "'/'"),
            Self::Tilde => write!(f, "'~'"),
            Self::Lt => write!(f, "'<'"),
            Self::Le => write!(f, "LE"),
            Self::DArrow => write!(f, "DARROW"),
            Self::Assign => write!(f, "ASSIGN"),
            Self::Colon => write!(f, "':'"),
            Self::Comma => write!(f, "','"),
            Self::Dot => write!(f, "'.'"),
            Self::Equal => write!(f, "'='"),
            Self::OpenParen => write!(f, "'('"),
            Self::CloseParen => write!(f, "')'"),
            Self::OpenBrace => write!(f, "'{{'"),
            Self::CloseBrace => write!(f, "'}}'"),
            Self::At => write!(f, "'@'"),
            Self::SemiColon => write!(f, "';'"),
            Self::Error(reason) => {
                if &reason[0..1] == "\0" {
                    write!(f, "ERROR \"\\000\"")
                } else {
                    write!(f, "ERROR \"{}\"", escaped_string(reason))
                }
            }
        }
    }
}

pub struct Token<'s> {
    pub kind: TokenKind,
    pub length: usize,
    source: &'s str,
}

impl<'s> Token<'s> {
    pub fn new(kind: TokenKind, length: usize, source: &'s str) -> Self {
        Self {
            kind,
            length,
            source,
        }
    }

    pub fn as_str(&self) -> &str {
        &self.source[..self.length]
    }
}

impl<'s> fmt::Display for Token<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl<'s> fmt::Debug for Token<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Token")
            .field("kind", &self.kind)
            .field("length", &self.length)
            .finish()
    }
}
pub mod prelude {
    pub use super::{KeywordKind, Token, TokenKind};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
