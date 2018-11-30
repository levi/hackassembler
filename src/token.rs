#[derive(Debug,Clone,PartialEq)]
pub enum TokenKind {
    Symbol(String),
    Address(u32),
    Identifier(String),
    Equal,
    Plus,
    Minus,
    Not,
    And,
    Or,
    Memory,
    DRegister,
    ARegister,
    Semicolon,
    Number(u32),
    Jump,
    JumpGreaterThan,
    JumpEqual,
    JumpGreaterThanEqual,
    JumpLessThan,
    JumpNotEqual,
    JumpLessThanEqual,
    NewLine,
    EOF
}

#[derive(Debug,Clone,PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: u32,
}

impl Token {
    pub fn new(kind: TokenKind, line: u32) -> Token {
        return Token {
            kind: kind,
            line: line,
        }
    }
}
