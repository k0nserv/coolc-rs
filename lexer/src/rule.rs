use regex::{Regex, RegexBuilder};

use std::collections::HashMap;

use common::{KeywordKind, Token, TokenKind};

use crate::LexerContext;

#[derive(Debug)]
pub enum RuleError {
    RegexError(regex::Error),
}

impl From<regex::Error> for RuleError {
    fn from(err: regex::Error) -> Self {
        Self::RegexError(err)
    }
}

pub trait Rule {
    fn try_match<'a, 'b>(&'a self, source: &'b str) -> Option<Token<'b>>;
    fn accept(&self, token: &Token, context: &mut LexerContext);
}

pub struct RegexRule {
    regex: Regex,
    token_kind: TokenKind,
}

impl RegexRule {
    pub fn new(pattern: &str, token_kind: TokenKind) -> Result<Self, RuleError> {
        let regex = RegexBuilder::new(&format!("\\A(?:{})", &pattern))
            .multi_line(true)
            .dot_matches_new_line(true)
            .build()?;

        Ok(Self { regex, token_kind })
    }
}

impl Rule for RegexRule {
    fn try_match<'a, 'b>(&'a self, source: &'b str) -> Option<Token<'b>> {
        self.regex
            .find(source)
            .map(|mat| Token::new(self.token_kind, mat.end() - mat.start(), source))
    }

    fn accept(&self, token: &Token, context: &mut LexerContext) {}
}

pub struct KeywordRule {
    mapping: HashMap<&'static str, KeywordKind>,
}

impl KeywordRule {
    pub fn new(mapping: HashMap<&'static str, KeywordKind>) -> Self {
        Self { mapping }
    }
}

impl Rule for KeywordRule {
    fn try_match<'a, 'b>(&'a self, source: &'b str) -> Option<Token<'b>> {
        let mat = {
            let mut longest_match_length = 0;
            self.mapping
                .iter()
                .filter(|(&key, _)| {
                    if longest_match_length > key.len() {
                        false
                    } else {
                        if source.len() >= key.len()
                            && source[..key.len()].to_lowercase() == key.to_lowercase()
                        {
                            longest_match_length = key.len();
                            true
                        } else {
                            false
                        }
                    }
                })
                .last()
        };

        mat.map(|(k, &kind)| Token::new(TokenKind::Keyword(kind), k.len(), source))
    }

    fn accept(&self, token: &Token, context: &mut LexerContext) {}
}

pub struct LiteralRule {
    lit: &'static str,
    token_kind: TokenKind,
}

impl LiteralRule {
    pub fn new(lit: &'static str, token_kind: TokenKind) -> Self {
        Self { lit, token_kind }
    }
}

impl Rule for LiteralRule {
    fn try_match<'a, 'b>(&'a self, source: &'b str) -> Option<Token<'b>> {
        if source.len() >= self.lit.len() {
            (&source[..self.lit.len()] == self.lit)
                .then(|| Token::new(self.token_kind, self.lit.len(), source))
        } else {
            None
        }
    }

    fn accept(&self, token: &Token, context: &mut LexerContext) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    fn int_rule() -> impl Rule {
        RegexRule::new("[0-9]+", TokenKind::Int).unwrap()
    }

    #[test]
    fn test_int_rule() {
        let rule = RegexRule::new("[0-9]+", TokenKind::Int);

        assert!(rule.is_ok());
    }

    #[test]
    fn test_try_match_matches() {
        let rule = int_rule();

        let token = rule.try_match("12313\n\tlet a <- 10");

        assert!(token.is_some());
        let token = token.unwrap();

        assert_eq!(token.as_str(), "12313");
        assert_eq!(token.length, 5);
    }

    #[test]
    fn test_keyword_rule() {
        let keyword_rule = KeywordRule::new(
            vec![("InheRits", KeywordKind::Inherits), ("in", KeywordKind::In)]
                .into_iter()
                .collect(),
        );

        let token = keyword_rule.try_match("inherits A");

        assert!(token.is_some());
        let token = token.unwrap();

        assert_eq!(token.as_str(), "inherits");
        assert_eq!(token.length, 8);
    }
}
