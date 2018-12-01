use std::vec::Vec;
use token::*;
use token::TokenKind::*;
use instruction::{Instruction, Expression};

type Result<T> = std::result::Result<T, ParserError>;

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

    pub fn parse(&mut self) -> Result<Vec<Instruction>> { 
        let mut codes: Vec<Instruction> = Vec::new();

        while !self.at_end() {
            match self.statement() {
                Ok(c) => codes.push(c),
                Err(e) => {
                    return Err(e);
                } 
            }
        }

        Ok(codes)
    }

    fn statement(&mut self) -> Result<Instruction> {
        let statement = match self.peek().kind {
            Label(_) => self.symbol()?,
            Address(_) | Symbol(_) => self.a_instruction()?,
            _ => self.c_instruction()?, 
        };

        let _ = self.try_push(NewLine, "A statement must end with a new line.");

        Ok(statement)
    }

    fn symbol(&mut self) -> Result<Instruction> {
        Ok(Instruction::Label(self.push()))
    }

    fn a_instruction(&mut self) -> Result<Instruction> {
        Ok(Instruction::AInstruction(self.push()))
    }

    fn c_instruction(&mut self) -> Result<Instruction> {
        Ok(Instruction::CInstruction{
            dest: self.dest()?,
            comp: self.comp()?,
            jump: self.jump()?,
        })
    }

    fn dest(&mut self) -> Result<Vec<Token>> {
        let mut dest: Vec<Token> = Vec::with_capacity(3);
        while self.match_any(&[ARegister, DRegister, Memory]) {
            let token = self.previous();
            if dest.contains(&token) {
                return Err(self.error("Duplicate destination"))
            }
            dest.push(token);
        } 

        if self.match_one(Equal) {
            if dest.is_empty() {
                return Err(self.error("Missing destination l-value"));
            }
        } else if dest.len() > 0 {
            for _ in 0..dest.len() {
                self.pop();
            }
        }

        Ok(dest)
    }

    fn comp(&mut self) -> Result<Expression> {
        if self.match_any(&[Minus, Not]) {
            let operator = self.previous();
            let right = self.literal()?;
            return Ok(Expression::Unary{
                operator: operator,
                right: right,
            })
        }

        let left = self.literal()?;

        if self.match_any(&[Plus, Minus, And, Or]) {
            let operator = self.previous();
            let right = self.literal()?;
            return Ok(Expression::Binary{
                left: left,
                operator: operator,
                right: right,
            });
        }

        Ok(Expression::Literal(left))
    }

    fn literal(&mut self) -> Result<Token> {
        if self.match_any(&[ARegister, DRegister, Memory, Number(0), Number(1)]) {
            return Ok(self.previous())
        }

        Err(self.error("Unexpected expression"))
    }

    fn jump(&mut self) -> Result<Option<Token>> {
        let mut jump: Option<Token> = None;
        if self.match_one(Semicolon) {
            if self.match_any(&[
                Jump, 
                JumpEqual, 
                JumpGreaterThan, 
                JumpGreaterThanEqual, 
                JumpLessThan, 
                JumpLessThanEqual, 
                JumpNotEqual
            ]) {
                jump = Some(self.previous());
            } else {
                return Err(self.error("Invalid jump code"));
            }
        }
        Ok(jump)
    }

    fn error(&self, description: &str) -> ParserError {
        ParserError::new(self.peek(), description)
    }

    /// Pushes the cursor if the current matches the provided token
    fn match_one(&mut self, t: TokenKind) -> bool {
        if self.check(&t) {
            self.push();
            return true;
        }
        false
    }

    /// Pushes the cursor if the current matches any of the provided tokens
    fn match_any(&mut self, tokens: &[TokenKind]) -> bool {
        for t in tokens {
            if self.check(t) {
                self.push();
                return true;
            }
        }
        false
    }

    /// Pushes the cursor, erroring when the current doesn't match the provided token
    fn try_push(&mut self, t: TokenKind, description: &str) -> Result<Token> {
        if !self.check(&t) {
            return Err(self.error(description))
        }
        Ok(self.push())
    }

    /// Advance the cursor to the next token, returning the previous token
    fn push(&mut self) -> Token {
        if !self.at_end() {
            self.cursor += 1;
        }
        self.previous()
    }

    /// Backtrack the cursor to the previous token
    fn pop(&mut self) {
        self.cursor -= 1;
    }

    /// Check if the current token matches the provided type
    fn check(&self, t: &TokenKind) -> bool {
        if self.at_end() {
            return false
        }
        self.peek().kind == *t
    }

    /// Look at the token at the current cursor position
    fn peek(&self) -> Token {
        self.tokens[self.cursor].clone()
    }

    /// Look at the token at the previous cursor position
    fn previous(&self) -> Token {
        self.tokens[self.cursor - 1].clone()
    }

    /// Determine if the parser is at the end of the file
    fn at_end(&self) -> bool {
        self.peek().kind == TokenKind::EOF
    }
}

#[derive(Debug)]
pub struct ParserError {
    token: Token,
    description: String,
}

impl ParserError {
    pub fn new(token: Token, description: &str) -> ParserError {
        ParserError{
            token: token,
            description: String::from(description),
        }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Syntax error: [Line {}] {} ", self.token.line, self.description)
    }
}

impl std::error::Error for ParserError {}