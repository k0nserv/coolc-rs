mod cursor;
mod lexer;
mod rule;

use crate::cursor::Cursor;
pub use crate::lexer::{Lexer, LexerContext};
pub use crate::rule::{BlockCommentRule, KeywordRule, LiteralRule, RegexRule, Rule, StringRule};

pub mod prelude {
    pub use crate::lexer::{Lexer, LexerContext};
    pub use crate::rule::{
        BlockCommentRule, KeywordRule, LiteralRule, RegexRule, Rule, StringRule,
    };
}
