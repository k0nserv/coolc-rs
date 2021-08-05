use common::Token;

use crate::rule::Rule;

pub struct LexerContext {}

pub struct Lexer {
    rules: Vec<Box<dyn Rule>>,
}

impl Lexer {
    pub fn new(rules: Vec<Box<dyn Rule>>) -> Self {
        Self { rules }
    }

    pub fn lex<'a, 'b>(&'a self, input: &'b str) -> Vec<Token<'b>> {
        let mut current = input;
        let mut context = LexerContext {};
        let mut result = vec![];

        while !current.is_empty() {
            let mut current_match: Option<(usize, &dyn Rule, Token)> = None;

            // Loop backwards to enforce presedence
            for rule in self.rules.iter() {
                match rule.try_match(current) {
                    Some(token) => {
                        if current_match
                            .as_ref()
                            .map(|m| token.length > m.0)
                            .unwrap_or(true)
                        {
                            current_match = Some((token.length, rule.as_ref(), token));
                        }
                    }
                    None => (),
                }
            }

            let mat = current_match.expect("Should have had at least one match");
            mat.1.accept(&mat.2, &mut context);

            current = &current[mat.0..];
            // dbg!(&mat.2, current);
            result.push(mat.2);
        }

        result
    }
}
