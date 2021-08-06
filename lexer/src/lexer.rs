use common::Token;

use crate::rule::Rule;

#[derive(Clone)]
pub struct LexerContext {
    pub line_number: usize,
}

impl Default for LexerContext {
    fn default() -> Self {
        Self { line_number: 1 }
    }
}

pub struct Lexer {
    rules: Vec<Box<dyn Rule>>,
}

impl Lexer {
    pub fn new(rules: Vec<Box<dyn Rule>>) -> Self {
        Self { rules }
    }

    pub fn lex<'a, 'b>(&'a mut self, input: &'b str) -> Vec<(Token<'b>, LexerContext)> {
        let mut current = input;
        let mut context = LexerContext::default();
        let mut result = vec![];

        while !current.is_empty() {
            let mut current_match: Option<(usize, &mut dyn Rule, Token)> = None;

            for rule in self.rules.iter_mut() {
                match rule.try_match(current) {
                    Some(token) => {
                        if current_match
                            .as_ref()
                            .map(|m| token.length > m.0)
                            .unwrap_or(true)
                        {
                            current_match = Some((token.length, rule.as_mut(), token));
                        }
                    }
                    None => (),
                }
            }

            let mat = current_match.expect("Should have had at least one match");
            current = mat.1.accept(&mat.2, &mut context, current);

            result.push((mat.2, context.clone()));
        }

        result
    }
}
