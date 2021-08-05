mod lexer;
mod rule;

pub use crate::lexer::{Lexer, LexerContext};
pub use crate::rule::{KeywordRule, LiteralRule, RegexRule, Rule};

pub mod prelude {
    pub use crate::lexer::{Lexer, LexerContext};
    pub use crate::rule::{KeywordRule, LiteralRule, RegexRule, Rule};
}
