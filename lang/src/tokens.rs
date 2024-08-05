use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token {
    // Keywords
    #[token("store")]
    Store,
    #[token("query")]
    Query,
    #[token("modify")]
    Modify,
    #[token("delete")]
    Delete,
    #[token("true")]
    True,
    #[token("false")]
    False,

    // Punctuation
    #[token(",")]
    Comma,
    #[token("{")]
    LeftCurlyBrace,
    #[token("}")]
    RightCurlyBrace,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token(".")]
    Dot,
    #[token(";")]
    SemiColon,

    // Operators
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("<=")]
    LessThanEq,
    #[token(">=")]
    GreaterThanEq,
    #[token("=")]
    Eq,
    #[token("!")]
    Not,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Times,
    #[token("/")]
    Divide,

    #[token("[0-9]+(.[0-9]+)?")]
    Number,

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| lex.slice().to_owned())]
    String(String),

    #[token("[a-zA-Z][a-zA-Z0-9_]*")]
    Identifier,

    #[token("//.*(?:\n|\r\n|\r|$)")]
    Comment,
}
