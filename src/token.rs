#[derive(Debug,Clone,PartialEq)]
pub enum TokenType {
    Label(String),
    Identifier(String),
    Equal,
    Plus,
    Minus,
    Not,
    Ampersand,
    Pipe,
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
    pub token: TokenType,
    pub line: u32,
}

impl Token {
    pub fn new(token: TokenType, line: u32) -> Token {
        return Token {
            token: token,
            line: line,
        }
    }
}
