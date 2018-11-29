use std::vec::Vec;
use token::{Token,TokenType};
use code::Code;

pub struct ParserError {
    token: Token,
    description: String,
}

impl ParserError {
    pub fn new(token: Token, description: String) -> ParserError {
        ParserError{
            token: token,
            description: description,
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser{
            tokens: tokens,
            cursor: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Code>, ParserError> {
        let mut codes = Vec::new();

        while !self.at_end() {
            match self.code() {
                Ok(c) => codes.push(c),
                Err(e) => {
                    return Err(e);
                } 
            }
        }

        Ok(codes)
    }

    fn code(&mut self) -> Result<Code, ParserError> {
        if self.match_one(TokenType::Identifier) {
            return self.a_instruction();
        } else if self.match_one(TokenType::Label) {
            return self.label();
        }

        self.c_instruction()
    }

    fn label(&mut self) -> Result<Code, ParserError> {\

    }

    fn a_instruction(&mut self) -> Result<Code, ParserError> {

    }

    // dest=comp;jump
    // dest=comp
    // comp
    // comp;jump

    fn c_instruction(&mut self) -> Result<Code, ParserError> {
        use token::TokenType::*;
        let mut inst = CInstruction::new();

        // dest= 
        let mut dest: Vec<Token> = Vec::with_capacity(3);
        while self.match_any([ARegister, DRegister, Memory]) {
            // TODO: Prevent more than 3
            // TODO: Prevent duplicates
            dest.push(self.peek());
        } 

        if self.match_one(Equal) {
            if dest.is_empty() {
                return Err(ParserError::new(self.peek(), "Missing destination l-value"));
            }
        } else if dest.len() > 0 {
            for _ in 0..dest.len() {
                self.pop();
            }
        }

        // comp
        // 0
        // 1
        // -1
        // D
        // A
        // !D
        //


        // ;jump
        let mut jump: Option<Token> = None;
        if self.match_one(Semicolon) {
            if self.match_any([
                Jump, 
                JumpEqual, 
                JumpGreaterThan, 
                JumpGreaterThanEqual, 
                JumpLessThan, 
                JumpLessThanEqual, 
                JumpNotEqual
            ]) {
                jump = self.peek();
            } else {
                return Err(ParserError::new(self.peek(), "Invalid jump command"));
            }
        }

        Ok(CInstruction::new(dest, comp, jump))
    }

    fn match_one(&mut self, t: &TokenType) -> bool {
        if self.check(t) {
            self.push();
            return true;
        }

        false
    }

    fn match_any(&mut self, tokens: &[Token::TokenType]) -> bool {
        for t in tokens {
            if self.check(t) {
                self.push();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, t: &TokenType, description: String) -> Result<Token, ParserError> {
        if !self.check(t) {
            return Err(ParserError::new(self.peek(), description));
        }

        Ok(self.push())
    }

    fn push(&mut self) -> Token {
        if !self.at_end() {
            self.cursor += 1;
        }
        self.previous()
    }

    fn pop(&mut self) {
        self.cursor -= 1;
    }

    fn check(&self, t: &TokenType) -> bool {
        if self.at_end() {
            return false
        }
        self.peek().token == *t
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.cursor]
    }

    fn previous(&mut self) -> Token {
        self.tokens[self.cursor - 1]
    }

    fn at_end(&self) -> bool {
        self.peek().token == TokenType::EOF
    }
}