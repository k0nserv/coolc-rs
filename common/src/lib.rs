#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum KeywordKind {
    Class,
    Else,
    False,
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
    True,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TokenKind {
    // Any sequence of space(ascii 32), \n(10), \f(12), \r(13), \t(9), \v(11)
    Whitespace,
    ObjectId,
    TypeId,
    Int,

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
    /// Greater Than ">"
    Gt,
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

    Error,
}

#[derive(Debug)]
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
