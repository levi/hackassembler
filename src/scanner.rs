use token::Token;
use token::TokenKind;

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
            let _ = self.push();
        }

        let cursor = self.push();
        match cursor {
            '\0' => Ok(self.token(TokenKind::NewLine)),
            '@' => {
                let value = self.grab_while(|c| !c.is_whitespace());
                Ok(self.token(match value.parse::<u32>() {
                    Ok(n) => TokenKind::Address(n),
                    Err(_) => TokenKind::Symbol(value),
                }))
            },
            '(' => {
                if self.peek.is_digit(10) {
                    return Err(self.scanner_error("Symbol cannot start with a digit"));
                }
                let s = self.grab_while(|c| c != ')' && !c.is_whitespace());
                if self.peek != ')' {
                    return Err(self.scanner_error("Expected Symbol to be terminated by closing )"));
                }
                let _ = self.push();
                Ok(self.token(TokenKind::Label(s)))
            },
            'A' => Ok(self.token(TokenKind::ARegister)),
            'D' => Ok(self.token(TokenKind::DRegister)),
            'M' => Ok(self.token(TokenKind::Memory)),
            '=' => Ok(self.token(TokenKind::Equal)),
            '-' => Ok(self.token(TokenKind::Minus)),
            '+' => Ok(self.token(TokenKind::Plus)),
            '&' => Ok(self.token(TokenKind::And)),
            '|' => Ok(self.token(TokenKind::Or)),
            '!' => Ok(self.token(TokenKind::Not)),
            ';' => Ok(self.token(TokenKind::Semicolon)),
            '/' => {
                if self.peek == '/' {
                    self.cursor = '\0';
                    self.peek = '\0';
                    Ok(self.token(TokenKind::NewLine))
                } else {
                    Err(self.scanner_error("Unexpected slash character"))
                }
            },
            'J' => {
                let keyword = self.grab_cursor_while(|c| !c.is_whitespace());
                match keyword.as_ref() {
                    "JGT" => Ok(self.token(TokenKind::JumpGreaterThan)),
                    "JEQ" => Ok(self.token(TokenKind::JumpEqual)),
                    "JGE" => Ok(self.token(TokenKind::JumpGreaterThanEqual)),
                    "JLT" => Ok(self.token(TokenKind::JumpLessThan)),
                    "JNE" => Ok(self.token(TokenKind::JumpNotEqual)),
                    "JLE" => Ok(self.token(TokenKind::JumpLessThanEqual)),
                    "JMP" => Ok(self.token(TokenKind::Jump)),
                    _ => {
                        Err(self.scanner_error("Unexpected jump type"))
                    },
                }
            },
            _ => {
                if cursor.is_digit(10) {
                    let buf = self.grab_cursor_while(|c| c.is_digit(10));
                    let num = buf.parse::<u32>().unwrap();
                    return Ok(self.token(TokenKind::Number(num)));
                }

                Err(self.scanner_error("Unexpected character"))
            }
        }
    }

    fn token(&self, kind: TokenKind) -> Token {
        Token::new(kind, self.line_num)
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
    fn push(&mut self) -> char {
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

#[derive(Debug)]
pub struct ScannerError {
    description: String,
    line: u32,
}

impl ScannerError {
    fn new(description: &str, line: u32) -> ScannerError {
        ScannerError{
            description: String::from(description),
            line: line,
        }
    }
}

impl std::fmt::Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Syntax error: [Line {}] {} ", self.line, self.description)
    }
}

impl std::error::Error for ScannerError {}

#[cfg(test)]
mod tests {
    #[test]
    fn register_jump() {
        use token::{Token, TokenKind};
        use scanner::Scanner;
        let s = Scanner::new("D;JNE\n\0", 0);
        let mut tokens = Vec::new();
        for token in s {
            tokens.push(token.unwrap());
        }
        assert_eq!(tokens, vec![
            Token::new(TokenKind::DRegister, 0), 
            Token::new(TokenKind::Semicolon, 0), 
            Token::new(TokenKind::JumpNotEqual, 0),
            Token::new(TokenKind::NewLine, 0)
        ])
    }
}