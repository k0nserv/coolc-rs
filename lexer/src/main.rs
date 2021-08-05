use common::prelude::*;
use lexer::prelude::*;

fn re_rule(pattern: &str, token: TokenKind, desc: &str) -> Box<dyn Rule> {
    Box::new(
        RegexRule::new(pattern, token)
            .expect(&format!("Should be able to build regex rule for {}", desc)),
    )
}

fn lit_rule(lit: &'static str, token: TokenKind) -> Box<dyn Rule> {
    Box::new(LiteralRule::new(lit, token))
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
        // True and False get special rules due to their behaviour
        re_rule("t(?i:rue)", TokenKind::Keyword(KeywordKind::True), "true"),
        re_rule(
            "f(?i:alse)",
            TokenKind::Keyword(KeywordKind::False),
            "false",
        ),
        // Whitespace
        re_rule(r"[ \t\n\r\f\v]*", TokenKind::Whitespace, "whitespace"),
        // Type ID
        re_rule(
            r"(SELF_TYPE|[A-Z][A-Za-z0-9_]*)",
            TokenKind::TypeId,
            "Type ID",
        ),
        // Object ID
        re_rule(
            r"(self|[a-z][A-Za-z0-9_]*)",
            TokenKind::ObjectId,
            "Object ID",
        ),
        // Int
        re_rule(r"[0-9]+", TokenKind::Int, "Int"),
        re_rule(r"(self|[a-z][a-z0-9_]*)", TokenKind::ObjectId, "Object ID"),
        // Single characters
        lit_rule("{", TokenKind::OpenBrace),
        lit_rule("}", TokenKind::CloseBrace),
        lit_rule("(", TokenKind::OpenParen),
        lit_rule(")", TokenKind::CloseParen),
        lit_rule(":", TokenKind::Colon),
        lit_rule(";", TokenKind::SemiColon),
        lit_rule("@", TokenKind::At),
        lit_rule(".", TokenKind::Dot),
        lit_rule("=", TokenKind::Equal),
        lit_rule("~", TokenKind::Tilde),
        // Operators
        lit_rule("+", TokenKind::Plus),
        lit_rule("-", TokenKind::Minus),
        lit_rule("*", TokenKind::Star),
        lit_rule("/", TokenKind::Slash),
        lit_rule("<", TokenKind::Lt),
        lit_rule(">", TokenKind::Gt),
        lit_rule("=<", TokenKind::Le),
        lit_rule("=>", TokenKind::DArrow),
        lit_rule("<-", TokenKind::DArrow),
        // Error catch all
        re_rule(r".", TokenKind::Error, "catch-all"),
    ]
}

fn main() {
    let lexer = Lexer::new(rules());
    let input = r#"
        class Main inherits IO {
           main(): SELF_TYPE {
           };
        };
    "#;
    let tokens = lexer.lex(input);
    println!("{:?}", tokens);

    for t in tokens {
        print!("{}", t.as_str());
    }
}
