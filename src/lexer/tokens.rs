#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    Eof,

    Int(i64),

    LParen,
    RParen,

    Plus,  // +
    Minus, // -
    Star,  // *
    Dash,  // /
}

#[derive(Debug, Clone)]
pub struct Lexeme {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub typ: TokenType,
    pub lexeme: Lexeme,
}
