use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str::Chars;

#[derive(Debug)]
pub enum TokenType {
    Label(String),
    Identifier(String),
    Equal,
    Plus,
    Minus,
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
    EOF,
}

// @CAPITALLETERNAME
// dest=comp;jump
// JGT
// JEQ
// JGE
// JLT
// JNE
// JLE
// JMP

#[derive(Debug)]
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

pub struct ScannerError {
    line_num: u32,
}

impl ScannerError {
    fn new(line_num: u32) -> ScannerError {
        ScannerError{
            line_num: line_num,
        }
    }
}

pub struct Scanner<'a> {
    iter: std::iter::Peekable<Chars<'a>>,
    line_num: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(line: &str, line_num: u32) -> Scanner {
        let mut iter = line.chars().peekable();
        Scanner {
            iter: iter,
            line_num: line_num,
        }
    }

    fn parse_token(&self) -> Result<Option<Token>, ScannerError> {
        match self.advance() {
            '@' => {
                let identifier = self.grab_to_end();
                Ok(Some(Token::new(TokenType::Identifier(identifier), self.line_num)))
            },
            '(' => {
                let mut s = String::new();
                while self.peek() != ')' && !self.is_end() {
                    s.push(self.advance());
                }
                if self.is_end() {
                    // TODO: Raise unterminated label error
                    return Err(self.scanner_error());
                }
                let _ = self.advance();
                Ok(Some(self.token(TokenType::Label(s))))
            },
            'A' => Ok(Some(self.token(TokenType::ARegister))),
            'D' => Ok(Some(self.token(TokenType::DRegister))),
            'M' => Ok(Some(self.token(TokenType::Memory))),
            '=' => Ok(Some(self.token(TokenType::Equal))),
            '-' => Ok(Some(self.token(TokenType::Minus))),
            '+' => Ok(Some(self.token(TokenType::Plus))),
            ';' => Ok(Some(self.token(TokenType::Semicolon))),
            '/' => {
                if self.match('/') {
                    Ok(None)
                } else {
                    // TODO: Raise unexpected slash error
                    Err(self.scanner_error())
                }
            },
            '\t' => Ok(None),
            ' ' => Ok(None),
            'J' => {}, // TODO: grab jump statements
            _ => {
                // TODO: Digits
                return Token::new(TokenType::Number(0), String::new(), self.line_num),
                // TODO: Error if nothing
            }
        }
    }

    fn token(&self, token: TokenType) -> Token {
        Token::new(token, self.line_num)
    }

    fn scanner_error(&self) -> ScannerError {
        ScannerError::new(self.line_num)
    }

    fn peek(&self) -> char {
        self.current_char
    }

    fn advance(&mut self) -> char {
        let old_ch = self.current_char;
        self.current_char = match self.iter.next() {
            Some(c) => c,
            None => '\0',
        };
        old_ch
    }

    fn match(&mut self, expected: char) -> bool {
        if self.is_end() {
            return false
        }

    }

    fn grab_to_end(&mut self) -> String {
        self.grab_while(|c| c != '\0')
    }

    fn grab_while<F>(&mut self, predicate: F) -> String where F: Fn(char) -> bool {
        let mut s = String::new();
        self.take_while_into(&mut s, predicate);
        s
    }

    fn take_while_into<F>(&mut self, s: &mut String, predicate: F) where F: Fn(char) -> bool {
        if !self.is_end() {
            s.push(self.current_char);
        }
        while let Some(c) = self.iter.next() {
            if !predicate(c) {
                self.current_char = c;
                return;
            }
            s.push(c);
        }
        self.current_char = '\0';
    }

    fn is_end(&self) -> bool {
        self.current_char == '\0'
    }
}

pub struct Scanlines<R: Read> {
    reader: io::BufReader<R>,
    line: String,
    line_num: u32,
}

impl <'a, R: Read> Scanlines<R> {
    pub fn new(f: R) -> Scanlines<R> {
        Scanlines {
            reader: io::BufReader::new(f),
            line: String::new(),
            line_num: 0,
        }
    }

    pub fn next(&'a mut self) -> Option<io::Result<Scanner<'a>>> {
        self.line.clear();
        match self.reader.read_line(&mut self.line) {
            Ok(num_bytes) => if num_bytes == 0 {
                return None;
            },
            Err(e) => return Some(Err(e))
        }
        self.line_num += 1;
        Some(Ok(Scanner::new(&self.line, self.line_num)))
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: hackassembler [asm_file]");
    } else {
        let filename = &args[1];
        let file = File::open(filename)?;
        let mut sl = Scanlines::new(file);
        while let Some(line) = sl.next() {

//            println!("Token: {}", token);
        }
    }

    Ok(())
}
