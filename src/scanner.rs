use token::Token;
use token::TokenType;

#[derive(Debug)]
pub struct ScannerError {
    reason: String,
    line_num: u32,
}

impl ScannerError {
    fn new(reason: &str, line_num: u32) -> ScannerError {
        ScannerError{
            reason: String::from(reason),
            line_num: line_num,
        }
    }
}

pub struct Scanner<'a> {
    iter: std::str::Chars<'a>,
    cursor: char,
    peek: char,
    did_error: bool,
    line_num: u32,
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Result<Token, ScannerError>;

    fn next(&mut self) -> Option<Result<Token, ScannerError>> {
        if self.at_end() {
            return None;
        }

        if self.did_error {
            return None
        }

        match self.parse_token() {
            Ok(t) => Some(Ok(t)),
            Err(err) => {
                self.did_error = true;
                Some(Err(err))
            }
        }
    }
}

impl<'a> Scanner<'a> {
    pub fn new(line: &str, line_num: u32) -> Scanner {
        let mut iter = line.chars();
        let peek = iter.next();
        Scanner {
            iter: iter,
            cursor: ' ',
            did_error: false,
            peek: match peek { Some(c) => c, None => '\0' },
            line_num: line_num,
        }
    }

    pub fn parse_token(&mut self) -> Result<Token, ScannerError> {
        // Skip whitespace characteres
        while self.peek.is_whitespace() {
            let _ = self.advance();
        }

        let cursor = self.advance();
        match cursor {
            '\0' => Ok(self.token(TokenType::NewLine)),
            '@' => {
                let value = self.grab_while(|c| !c.is_whitespace());
                Ok(self.token(match value.parse::<u32>() {
                    Some(n) => TokenType::Address(n),
                    None => TokenType::Identifier(value),
                }))
            },
            '(' => {
                if self.peek().is_digit(10) {
                    return Err(self.scanner_error("Symbol cannot start with a digit"));
                }
                let s = self.grab_while(|c| c != ')' && !c.is_whitespace());
                if self.peek != ')' {
                    return Err(self.scanner_error("Expected Symbol to be terminated by closing )"));
                }
                let _ = self.advance();
                Ok(self.token(TokenType::Symbol(s)))
            },
            'A' => Ok(self.token(TokenType::ARegister)),
            'D' => Ok(self.token(TokenType::DRegister)),
            'M' => Ok(self.token(TokenType::Memory)),
            '=' => Ok(self.token(TokenType::Equal)),
            '-' => Ok(self.token(TokenType::Minus)),
            '+' => Ok(self.token(TokenType::Plus)),
            '&' => Ok(self.token(TokenType::And)),
            '|' => Ok(self.token(TokenType::Or)),
            '!' => Ok(self.token(TokenType::Not)),
            ';' => Ok(self.token(TokenType::Semicolon)),
            '/' => {
                if self.peek == '/' {
                    self.cursor = '\0';
                    self.peek = '\0';
                    Ok(self.token(TokenType::NewLine))
                } else {
                    Err(self.scanner_error("Unexpected slash character"))
                }
            },
            'J' => {
                let keyword = self.grab_cursor_while(|c| !c.is_whitespace());
                match keyword.as_ref() {
                    "JGT" => Ok(self.token(TokenType::JumpGreaterThan)),
                    "JEQ" => Ok(self.token(TokenType::JumpEqual)),
                    "JGE" => Ok(self.token(TokenType::JumpGreaterThanEqual)),
                    "JLT" => Ok(self.token(TokenType::JumpLessThan)),
                    "JNE" => Ok(self.token(TokenType::JumpNotEqual)),
                    "JLE" => Ok(self.token(TokenType::JumpLessThanEqual)),
                    "JMP" => Ok(self.token(TokenType::Jump)),
                    _ => {
                        Err(self.scanner_error("Unexpected jump type"))
                    },
                }
            },
            _ => {
                if cursor.is_digit(10) {
                    let buf = self.grab_cursor_while(|c| c.is_digit(10));
                    let num = buf.parse::<u32>().unwrap();
                    return Ok(self.token(TokenType::Number(num)));
                }

                Err(self.scanner_error("Unexpected character"))
            }
        }
    }

    fn token(&self, token: TokenType) -> Token {
        Token::new(token, self.line_num)
    }

    fn scanner_error(&self, reason: &str) -> ScannerError {
        ScannerError::new(reason, self.line_num)
    }

    /// Returns a string of all future characters until the predicate is false
    fn grab_while<F>(&mut self, predicate: F) -> String where F: Fn(char) -> bool {
        let mut s = String::new();
        self.take_while(&mut s, predicate);
        s
    }

    /// Returns a string of all the characters from the cursor until the predicate is false
    fn grab_cursor_while<F>(&mut self, predicate: F) -> String where F: Fn(char) -> bool {
        let mut s = String::new();
        if !self.at_end() {
            s.push(self.cursor);
        }
        self.take_while(&mut s, predicate);
        s
    }

    fn take_while<F>(&mut self, s: &mut String, predicate: F) where F: Fn(char) -> bool {
        while predicate(self.peek) {
            s.push(self.push());
        }
    }

    /// Advance the cursor, returning the new cursor result
    fn advance(&mut self) -> char {
        self.cursor = self.peek;
        self.peek = match self.iter.next() {
            Some(c) => c,
            None => '\0',
        };
        self.cursor
    }

    /// Determines if the scanner is at the end of the line
    fn at_end(&mut self) -> bool {
        self.cursor == '\0'
    }
}