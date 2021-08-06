use clap::{crate_authors, crate_version, App, Arg};
use regex::Match;

use std::fs::File;
use std::io::Read;

use common::prelude::*;
use lexer::prelude::*;

fn re_rule(pattern: &str, token: TokenKind, desc: &str) -> Box<RegexRule> {
    Box::new(
        RegexRule::new(pattern, token)
            .expect(&format!("Should be able to build regex rule for {}", desc)),
    )
}

fn refined_re_rule<F>(pattern: &str, refinement: F, desc: &str) -> Box<dyn Rule>
where
    F: FnMut(Match) -> Option<TokenKind> + 'static,
{
    Box::new(
        RegexRule::refined(pattern, Box::new(refinement))
            .expect(&format!("Should be able to build regex rule for {}", desc)),
    )
}

fn lit_rule(lit: &'static str, token: TokenKind) -> Box<dyn Rule> {
    Box::new(LiteralRule::new(lit, token))
}

fn refine_type_id(mat: Match) -> Option<TokenKind> {
    Some(TokenKind::TypeId(mat.as_str().into()))
}

fn refine_object_id(mat: Match) -> Option<TokenKind> {
    Some(TokenKind::ObjectId(mat.as_str().into()))
}

fn refine_int(mat: Match) -> Option<TokenKind> {
    Some(TokenKind::Int(mat.as_str().into()))
}

fn refine_error(mat: Match) -> Option<TokenKind> {
    Some(TokenKind::Error(mat.as_str().into()))
}

fn rules() -> Vec<Box<dyn Rule>> {
    // Lexical analysis rules
    // Order matter heres, we ues max munch and when two rules consume the same
    // number of characters the first one wins.
    vec![
        // Keywords
        Box::new(KeywordRule::new(
            vec![
                ("class", KeywordKind::Class),
                ("else", KeywordKind::Else),
                ("fi", KeywordKind::Fi),
                ("if", KeywordKind::If),
                ("in", KeywordKind::In),
                ("inherits", KeywordKind::Inherits),
                ("isvoid", KeywordKind::IsVoid),
                ("let", KeywordKind::Let),
                ("loop", KeywordKind::Loop),
                ("pool", KeywordKind::Pool),
                ("then", KeywordKind::Then),
                ("while", KeywordKind::While),
                ("case", KeywordKind::Case),
                ("esac", KeywordKind::Esac),
                ("new", KeywordKind::New),
                ("of", KeywordKind::Of),
                ("not", KeywordKind::Not),
                ("not", KeywordKind::Not),
            ]
            .into_iter()
            .collect(),
        )),
        lit_rule("<=", TokenKind::Le),
        lit_rule("=>", TokenKind::DArrow),
        lit_rule("<-", TokenKind::Assign),
        // Comments
        Box::new(BlockCommentRule::default()),
        Box::new(
            re_rule(r"--[^\n]*$", TokenKind::LineComment, "Line Comment").with_accepting_fn(
                Box::new(|token, lexer, source| {
                    if token.length >= source.len() {
                        // Reached EOF
                        ""
                    } else {
                        lexer.line_number += 1;
                        // `$` in regex does not consume the newline, eat it manually
                        &source[token.length + 1..]
                    }
                }),
            ),
        ),
        // Strings
        Box::new(StringRule::default()),
        // Single characters
        lit_rule("{", TokenKind::OpenBrace),
        lit_rule("}", TokenKind::CloseBrace),
        lit_rule("(", TokenKind::OpenParen),
        lit_rule(")", TokenKind::CloseParen),
        lit_rule(":", TokenKind::Colon),
        lit_rule(";", TokenKind::SemiColon),
        lit_rule("@", TokenKind::At),
        lit_rule(".", TokenKind::Dot),
        lit_rule(",", TokenKind::Comma),
        lit_rule("=", TokenKind::Equal),
        lit_rule("~", TokenKind::Tilde),
        // Operators
        lit_rule("+", TokenKind::Plus),
        lit_rule("-", TokenKind::Minus),
        lit_rule("*", TokenKind::Star),
        lit_rule("/", TokenKind::Slash),
        lit_rule("<", TokenKind::Lt),
        // True and False get special rules due to their behaviour
        re_rule("t(?i:rue)", TokenKind::Bool(true), "true"),
        re_rule("f(?i:alse)", TokenKind::Bool(false), "false"),
        // Int
        refined_re_rule(r"[0-9]+", refine_int, "Int"),
        // Type ID
        refined_re_rule(r"(SELF_TYPE|[A-Z][A-Za-z0-9_]*)", refine_type_id, "Type ID"),
        // Object ID
        refined_re_rule(r"(self|[a-z][A-Za-z0-9_]*)", refine_object_id, "Object ID"),
        // Newlines, to count line number
        Box::new(
            re_rule(r"\n", TokenKind::Whitespace, "whitespace").with_accepting_fn(Box::new(
                |_, lexer, source| {
                    lexer.line_number += 1;
                    // Eat it
                    &source[1..]
                },
            )),
        ),
        // Whitespace
        re_rule(r"[ \t\r\f\v]+", TokenKind::Whitespace, "whitespace"),
        // Error catch all
        refined_re_rule(r".", refine_error, "catch-all"),
    ]
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let matches = App::new("lexer")
        .version(crate_version!())
        .author(crate_authors!())
        .about("A lexer for the COOL language")
        .arg(
            Arg::with_name("FILES")
                .multiple(true)
                .index(1)
                .required(true),
        )
        .get_matches();

    let mut lexer = Lexer::new(rules());
    let mut buffer = String::default();

    for path in matches.values_of("FILES").unwrap() {
        let mut file = File::open(path)?;
        file.read_to_string(&mut buffer)?;

        println!("#name \"{}\"", path);

        let tokens = lexer.lex(&buffer);

        for (t, context) in tokens {
            let string_token = format!("{}", t);

            // dbg!(&t, &t.as_str());
            if string_token.is_empty() {
                continue;
            }

            println!("#{} {}", context.line_number, string_token);
        }

        buffer.clear()
    }

    Ok(())
}
