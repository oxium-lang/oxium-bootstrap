#[derive(Debug, PartialEq)]
pub enum Token {
    DataType(usize, usize, String),
    Keyword(usize, usize, String),
    Operator(usize, usize, String),
    Identifier(usize, usize, String),
    IntLit(usize, usize, String),
    FlLit(usize, usize, String),
    StringLit(usize, usize, String),
    CharLit(usize, usize, char),
    Seperator(usize, usize, char),
    Eof,
}
