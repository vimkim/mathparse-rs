#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
}
