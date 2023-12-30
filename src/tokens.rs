#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Integer(i32),
    Decimal(f32),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Pipe,
    LeftParen,
    RightParen,
    LeftBrack,
    RightBrack,
}
