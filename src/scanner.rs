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
    pub error: Option<ScannerError>,
    line_num: u32,
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.at_end() {
            return None;
        }

        match self.error.as_ref() {
            Some(_) => None,
            None => {
                match self.parse_token() {
                    Ok(token) => Some(token),
                    Err(err) => {
                        self.error = Some(err);
                        None
                    }
                }
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
            error: None,
            peek: match peek { Some(c) => c, None => '\0' },
            line_num: line_num,
        }
    }

    pub fn parse_token(&mut self) -> Result<Token, ScannerError> {
        // Skip whitespace characteres
        while self.peek.is_whitespace() {
            let _ = self.advance_cursor();
        }

        let cursor = self.advance_cursor();
        match cursor {
            '\0' => Ok(self.token(TokenType::End)),
            '@' => {
                let identifier = self.grab_while(|c| !c.is_whitespace());
                Ok(self.token(TokenType::Identifier(identifier)))
            },
            '(' => {
                let s = self.grab_while(|c| c != ')' && !c.is_whitespace());
                if self.peek != ')' {
                    return Err(self.scanner_error("Expected label to be terminated by closing )"));
                }
                let _ = self.advance_cursor();
                Ok(self.token(TokenType::Label(s)))
            },
            'A' => Ok(self.token(TokenType::ARegister)),
            'D' => Ok(self.token(TokenType::DRegister)),
            'M' => Ok(self.token(TokenType::Memory)),
            '=' => Ok(self.token(TokenType::Equal)),
            '-' => Ok(self.token(TokenType::Minus)),
            '+' => Ok(self.token(TokenType::Plus)),
            '!' => Ok(self.token(TokenType::Not)),
            ';' => Ok(self.token(TokenType::Semicolon)),
            '/' => {
                if self.peek == '/' {
                    self.cursor = '\0';
                    self.peek = '\0';
                    Ok(self.token(TokenType::End))
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
            s.push(self.advance_cursor());
        }
    }

    /// Advance the cursor, returning the new cursor result
    fn advance_cursor(&mut self) -> char {
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