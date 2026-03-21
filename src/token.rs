#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Keyword(Keyword),
    Ident(String),
    Literal(i32),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semi,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    Int,
    Void,
    Return,
}
