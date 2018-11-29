use std::vec::Vec;
use token::*;
use token::TokenType::*;
use code::{Code, Expression};

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

    pub fn parse(&mut self) -> Result<Vec<Code>> { 
        let mut codes: Vec<Code> = Vec::new();

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

    fn statement(&mut self) -> Result<Code> {
        let statement = match self.peek().token {
            Identifier(_) => self.a_instruction()?,
            Label(_) => self.label()?,
            _ => self.c_instruction()?, 
        };

        let _ = self.try_push(NewLine, "A statement must end with a new line.");

        Ok(statement)
    }

    fn label(&mut self) -> Result<Code> {
        Ok(Code::Label(self.push()))
    }

    fn a_instruction(&mut self) -> Result<Code> {
        Ok(Code::AInstruction(self.push()))
    }

    fn c_instruction(&mut self) -> Result<Code> {
        Ok(Code::CInstruction{
            dest: self.dest()?,
            comp: self.comp()?,
            jump: self.jump()?,
        })
    }

    fn dest(&mut self) -> Result<Vec<Token>> {
        let mut dest: Vec<Token> = Vec::with_capacity(3);
        while self.match_any(&[ARegister, DRegister, Memory]) {
            if dest.len() > 3 {
                return Err(self.error("Too many destinations given"))
            }
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
        let left = self.unary()?;

        if self.match_any(&[Plus, Minus, Ampersand, Pipe]) {
            let operator = self.previous();
            let right = self.primary()?;
            return Ok(Expression::Binary{
                left: Box::new(left),
                operator: operator,
                right: Box::new(right),
            });
        }

        Ok(left)
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

    fn unary(&mut self) -> Result<Expression> {
        if self.match_any(&[Minus, Not]) {
            let operator = self.previous();
            let right = self.primary()?;
            return Ok(Expression::Unary{
                operator: operator,
                right: Box::new(right),
            })
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expression> {
        if self.match_any(&[ARegister, DRegister, Memory, Number(0), Number(1)]) {
            return Ok(Expression::Literal(self.previous()))
        }

        Err(self.error("Unexpected expression"))
    }

    fn error(&self, description: &str) -> ParserError {
        ParserError::new(self.peek(), description)
    }

    /// Pushes the cursor if the current matches the provided token
    fn match_one(&mut self, t: TokenType) -> bool {
        if self.check(&t) {
            self.push();
            return true;
        }
        false
    }

    /// Pushes the cursor if the current matches any of the provided tokens
    fn match_any(&mut self, tokens: &[TokenType]) -> bool {
        for t in tokens {
            if self.check(t) {
                self.push();
                return true;
            }
        }
        false
    }

    /// Pushes the cursor, erroring when the current doesn't match the provided token
    fn try_push(&mut self, t: TokenType, description: &str) -> Result<Token> {
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
    fn check(&self, t: &TokenType) -> bool {
        if self.at_end() {
            return false
        }
        self.peek().token == *t
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
        self.peek().token == TokenType::EOF
    }
}