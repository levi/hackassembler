use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::vec::Vec;
use std::error::Error;

use token::Token;
use token::TokenType;

pub struct Scanner {
    pub tokens: Vec<Token>,
    // errors: Vec<Error>,
    reader: BufReader<File>,
    buf: String,
    start: i32,
    cursor: i32,
    line: i32,
}

impl Scanner {
    pub fn with_source_path(path: &String) -> std::io::Result<Scanner> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let scanner = Scanner {
            tokens: Vec::new(),
            reader: reader,
            buf: String::new(),
            start: 0,
            cursor: 0,
            line: 1,
        };
        Ok(scanner)
    }

    pub fn scan_tokens(&self) -> std::io::Result<Vec<Token>> {
        let mut num_bytes = self.reader.read_line(&mut self.buf)?;
        while num_bytes != 0 {
            self.scan_token();
            self.line += 1;
            self.buf = String::new();
            num_bytes = self.reader.read_line(&mut self.buf)?;
        }
        let eof = Token::new(TokenType::EOF, String::new(), self.line);
        self.tokens.push(eof);
        Ok(self.tokens)
    }

    fn scan_token(&self) {
        self.cursor += 1;
        match
    }

    fn advance(&self) {

    }
}
