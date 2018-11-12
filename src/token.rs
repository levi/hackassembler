#[derive(Debug)]
pub enum TokenType {
    Label(String),
    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub token: TokenType,
    pub lexeme: String,
    pub line: i32,
}

impl Token {
    pub fn new(token: TokenType, lexeme: String, line: i32) -> Token {
        return Token {
            token: token,
            lexeme: lexeme,
            line: line,
        }
    }
}
