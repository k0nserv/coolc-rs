use std::str::Chars;

//Inspired by https://doc.rust-lang.org/nightly/nightly-rustc/src/rustc_lexer/cursor.rs.html

pub struct Cursor<'s> {
    initial_len: usize,
    chars: Chars<'s>,
}

impl<'s> Cursor<'s> {
    pub fn bump(&mut self) -> Option<char> {
        let c = self.chars.next();

        c
    }

    pub fn peek(&mut self) -> Option<char> {
        self.chars().nth(0)
    }

    pub fn second(&mut self) -> Option<char> {
        self.chars().nth(1)
    }

    pub fn peek_many(&mut self, n: usize) -> &str {
        let s = &self.chars().as_str();

        if s.len() >= n {
            &s[0..n]
        } else {
            &s[0..s.len()]
        }
    }

    pub fn next_is_null(&mut self) -> bool {
        self.peek().map(|c| c == '\0').unwrap_or(false)
    }

    pub fn next_is_newline(&mut self) -> bool {
        self.peek().map(|c| c == '\n').unwrap_or(false)
    }

    pub fn is_eof(&self) -> bool {
        self.chars().as_str().is_empty()
    }

    pub fn consumed_len(&self) -> usize {
        self.initial_len - self.chars.as_str().len()
    }

    pub fn length_including(&self, chars: &[char]) -> usize {
        self.chars()
            .position(|c| chars.contains(&c))
            .map(|p| p + 1)
            .unwrap_or(self.chars().count())
    }

    pub fn chars(&self) -> Chars<'s> {
        self.chars.clone()
    }
}

impl<'s> From<&'s str> for Cursor<'s> {
    fn from(s: &'s str) -> Self {
        Self {
            initial_len: s.len(),
            chars: s.chars(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_bump() {
        let mut cursor: Cursor = "\"Hello World\" class".into();

        assert_eq!(cursor.bump(), Some('\"'));
        assert_eq!(cursor.bump(), Some('H'));
        assert_eq!(cursor.bump(), Some('e'));
        assert_eq!(cursor.bump(), Some('l'));
        assert_eq!(cursor.bump(), Some('l'));
    }
}
