#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Type {
    INTEGER,
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
}

#[derive(Debug, Copy, Clone)]
pub struct Token {
    pub token_type: Type,
    pub token_value: Option<isize>,
}