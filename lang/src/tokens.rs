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
    #[token("Int32")]
    Int32,
    #[token("Int64")]
    Int64,
    #[token("Uint32")]
    Uint32,
    #[token("Uint64")]
    Uint64,
    #[token("Bool")]
    Bool,
    #[token("String")]
    String,

    // Punctuation
    #[token(",")]
    Comma,
    #[token("[")]
    LeftSquareBracket,
    #[token("]")]
    RightSquareBracket,
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
    #[token(":")]
    Colon,

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

    #[token("[0-9]+(.[0-9]+)?", |lex| lex.slice().to_owned())]
    Number(String),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| lex.slice().to_owned())]
    StringLiteral(String),

    #[token("[a-zA-Z][a-zA-Z0-9_]*", |lex| lex.slice().to_owned())]
    Identifier(String),

    #[token("//.*(?:\n|\r\n|\r|$)")]
    Comment,
}
