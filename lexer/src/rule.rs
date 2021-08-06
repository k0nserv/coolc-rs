use either::Either;
use regex::{Match, Regex, RegexBuilder};

use std::collections::HashMap;

use common::{KeywordKind, Token, TokenKind};

use crate::{Cursor, LexerContext};

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
    fn try_match<'a, 'b>(&'a mut self, source: &'b str) -> Option<Token<'b>>;
    fn accept<'s>(
        &'_ mut self,
        token: &Token<'s>,
        context: &mut LexerContext,
        source: &'s str,
    ) -> &'s str;
}

type RefinementFn = Box<dyn FnMut(Match) -> Option<TokenKind>>;
type AcceptingFn = Box<dyn for<'s> FnMut(&Token, &mut LexerContext, &'s str) -> &'s str>;
pub struct RegexRule {
    regex: Regex,
    token_kind: Either<TokenKind, RefinementFn>,
    accepting_fn: Option<AcceptingFn>,
}

impl RegexRule {
    pub fn new(pattern: &str, token_kind: TokenKind) -> Result<Self, RuleError> {
        let regex = RegexBuilder::new(&format!("\\A(?:{})", &pattern))
            .multi_line(true)
            .dot_matches_new_line(true)
            .build()?;

        Ok(Self {
            regex,
            token_kind: Either::Left(token_kind),
            accepting_fn: None,
        })
    }

    pub fn with_regex(regex: Regex, token_kind: TokenKind) -> Self {
        Self {
            regex,
            token_kind: Either::Left(token_kind),
            accepting_fn: None,
        }
    }

    pub fn refined(
        pattern: &str,
        refinement: Box<dyn FnMut(Match) -> Option<TokenKind>>,
    ) -> Result<Self, RuleError> {
        let regex = RegexBuilder::new(&format!("\\A(?:{})", &pattern))
            .multi_line(true)
            .dot_matches_new_line(true)
            .build()?;

        Ok(Self {
            regex,
            token_kind: Either::Right(refinement),
            accepting_fn: None,
        })
    }

    pub fn with_accepting_fn(self, accepting_fn: AcceptingFn) -> Self {
        Self {
            regex: self.regex,
            token_kind: self.token_kind,
            accepting_fn: Some(accepting_fn),
        }
    }
}

impl Rule for RegexRule {
    fn try_match<'a, 'b>(&'a mut self, source: &'b str) -> Option<Token<'b>> {
        self.regex
            .find(source)
            .and_then(|mat| match self.token_kind.as_mut() {
                Either::Left(token_kind) => Some(Token::new(
                    token_kind.clone(),
                    mat.end() - mat.start(),
                    source,
                )),
                Either::Right(refinement) => refinement(mat)
                    .map(|token_kind| Token::new(token_kind, mat.end() - mat.start(), source)),
            })
    }

    fn accept<'s>(
        &mut self,
        token: &Token<'s>,
        context: &mut LexerContext,
        source: &'s str,
    ) -> &'s str {
        match &mut self.accepting_fn {
            Some(afn) => afn(token, context, source),
            _ => (&source[token.length..]),
        }
    }
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
    fn try_match<'a, 'b>(&'a mut self, source: &'b str) -> Option<Token<'b>> {
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

    fn accept<'s>(
        &mut self,
        token: &Token<'s>,
        _context: &mut LexerContext,
        source: &'s str,
    ) -> &'s str {
        &source[token.length..]
    }
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
    fn try_match<'a, 'b>(&'a mut self, source: &'b str) -> Option<Token<'b>> {
        if source.len() >= self.lit.len() {
            (&source[..self.lit.len()] == self.lit)
                .then(|| Token::new(self.token_kind.clone(), self.lit.len(), source))
        } else {
            None
        }
    }

    fn accept<'s>(
        &mut self,
        token: &Token<'s>,
        _context: &mut LexerContext,
        source: &'s str,
    ) -> &'s str {
        &source[token.length..]
    }
}

pub struct StringRule {
    buffer: String,
    number_of_lines: usize,
    recovery_consume: Option<usize>,
}

impl Default for StringRule {
    fn default() -> Self {
        Self {
            buffer: String::with_capacity(1024),
            number_of_lines: 0,
            recovery_consume: None,
        }
    }
}

impl StringRule {
    fn reset(&mut self) {
        self.number_of_lines = 0;
        self.buffer.clear();
        self.recovery_consume = None;
    }

    fn consume_string(&mut self, mut cursor: Cursor) -> Result<(usize, String), (usize, String)> {
        self.reset();
        let result: &mut String = &mut self.buffer;

        loop {
            if cursor.is_eof() {
                // EOF in string
                return Err((cursor.consumed_len(), "EOF in string constant.".into()));
            } else if cursor.next_is_null() {
                // Null in string

                // Consume until a stable state
                self.recovery_consume = Some(cursor.length_including(&['\n', '\"']));

                return Err((
                    cursor.consumed_len(),
                    "String contains null character.".into(),
                ));
            } else if cursor.next_is_newline() {
                // Unescaped newline
                self.number_of_lines += 1;

                // Eat newline
                let _ = cursor.bump();
                return Err((
                    cursor.consumed_len(),
                    "Unterminated string constant.".into(),
                ));
            } else if cursor.peek().map(|c| c == '\\').unwrap_or(false) && cursor.second().is_some()
            {
                let _ = cursor.bump();

                match cursor.bump().unwrap() {
                    'b' => result.push('\x08'),
                    't' => result.push('\t'),
                    'n' => result.push('\n'),
                    'f' => result.push('\x0C'),
                    '\n' => {
                        result.push('\n');
                        self.number_of_lines += 1;
                    }
                    '\0' => {
                        // Consume until a stable state
                        self.recovery_consume = Some(cursor.length_including(&['\n', '\"']));
                        return Err((
                            cursor.consumed_len(),
                            "String contains escaped null character.".into(),
                        ));
                    }
                    other => result.push(other),
                }
            } else if cursor.peek().map(|c| c == '\"').unwrap_or(false) {
                let _ = cursor.bump();
                return Ok((cursor.consumed_len(), result.clone()));
            } else {
                match cursor.bump() {
                    Some(c) => result.push(c),
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl Rule for StringRule {
    fn try_match<'a, 'b>(&'a mut self, source: &'b str) -> Option<Token<'b>> {
        let mut cursor: Cursor = source.into();
        if cursor.bump().map(|c| c != '\"').unwrap_or(true) {
            return None;
        }

        let consumed_string = self.consume_string(cursor);

        match consumed_string {
            Ok((consumed_length, s)) => {
                Some(Token::new(TokenKind::String(s), consumed_length, source))
            }
            Err((consumed_length, reason)) => Some(Token::new(
                TokenKind::Error(reason),
                consumed_length,
                source,
            )),
        }
    }

    fn accept<'s>(
        &mut self,
        token: &Token<'s>,
        context: &mut LexerContext,
        source: &'s str,
    ) -> &'s str {
        context.line_number += self.number_of_lines;

        match self.recovery_consume {
            Some(recovery) => &source[token.length + recovery..],
            None => &source[token.length..],
        }
    }
}

#[derive(Default)]
pub struct BlockCommentRule {
    depth: i64,
    number_of_lines: usize,
}

impl BlockCommentRule {
    fn consume_comment(&mut self, mut cursor: Cursor) -> Result<usize, (usize, String)> {
        self.depth = 0;
        self.number_of_lines = 0;

        loop {
            match cursor.bump() {
                // New comment
                Some(c) if c == '(' && cursor.peek() == Some('*') => {
                    cursor.bump();
                    self.depth += 1;
                }
                Some(c) if c == '*' && cursor.peek() == Some(')') => {
                    cursor.bump();
                    self.depth -= 1;

                    if self.depth == 0 {
                        return Ok(cursor.consumed_len());
                    } else if self.depth < 0 {
                        return Err((cursor.consumed_len(), "Unmatched *)".into()));
                    }
                }
                Some(c) if c == '\n' => {
                    self.number_of_lines += 1;
                }
                Some(_) => (),
                None => {
                    return Err((cursor.consumed_len(), "EOF in comment".into()));
                }
            }
        }
    }
}

impl Rule for BlockCommentRule {
    fn try_match<'a, 'b>(&'a mut self, source: &'b str) -> Option<Token<'b>> {
        let mut cursor: Cursor = source.into();
        let first_two = cursor.peek_many(2);

        if first_two == "*)" {
            return Some(Token::new(
                TokenKind::Error("Unmatched *)".into()),
                2,
                source,
            ));
        }

        if cursor.peek_many(2) != "(*" {
            return None;
        }

        match self.consume_comment(cursor) {
            Ok(consumed_length) => {
                Some(Token::new(TokenKind::BlockComment, consumed_length, source))
            }
            Err((consumed_length, reason)) => Some(Token::new(
                TokenKind::Error(reason),
                consumed_length,
                source,
            )),
        }
    }

    fn accept<'s>(
        &mut self,
        token: &Token<'s>,
        context: &mut LexerContext,
        source: &'s str,
    ) -> &'s str {
        context.line_number += self.number_of_lines;
        &source[token.length..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn int_rule() -> impl Rule {
        RegexRule::new("[0-9]+", TokenKind::Int("0".into())).unwrap()
    }

    fn string_rule() -> impl Rule {
        StringRule::default()
    }

    fn line_comment_rule() -> impl Rule {
        RegexRule::with_regex(
            RegexBuilder::new(r"\A(:?--[^\n]*$)")
                .multi_line(true)
                .build()
                .unwrap(),
            TokenKind::LineComment,
        )
    }

    #[test]
    fn test_int_rule() {
        let rule = RegexRule::new("[0-9]+", TokenKind::Int("0".into()));

        assert!(rule.is_ok());
    }

    #[test]
    fn test_try_match_matches() {
        let mut rule = int_rule();

        let token = rule.try_match("12313\n\tlet a <- 10");

        assert!(token.is_some());
        let token = token.unwrap();

        assert_eq!(token.as_str(), "12313");
        assert_eq!(token.length, 5);
    }

    #[test]
    fn test_keyword_rule() {
        let mut keyword_rule = KeywordRule::new(
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

    #[test]
    fn test_eof_in_line_comment() {
        let mut rule = line_comment_rule();

        let token = rule.try_match("-- can they handle EOF in \"--\" state?");

        assert!(token.is_some());
    }

    #[test]
    fn test_string_rule() {
        let mut rule = string_rule();

        let token = rule.try_match("\"Hello World\" class");

        assert!(token.is_some());
        let token = token.unwrap();

        dbg!(&token);
        assert_eq!(token.as_str(), "\"Hello World\"");
        assert_eq!(token.length, 13);
    }

    #[test]
    fn test_string_rule_2() {
        let mut rule = string_rule();

        let token = rule.try_match("\"\\n\\tTo add a number to \");");

        assert!(token.is_some());
        let token = token.unwrap();

        match &token.kind {
            TokenKind::String(s) => assert_eq!(s, "\n\tTo add a number to "),
            _ => assert!(false, "Token kind should be String"),
        };

        assert_eq!(token.as_str(), "\"\\n\\tTo add a number to \"");
        assert_eq!(token.length, 25);
    }
}
