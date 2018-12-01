use token::{Token, TokenKind};
use symbol_table::SymbolTable;

type Result<T> = std::result::Result<T, InstructionError>;

#[derive(Debug)]
pub enum Instruction {
    Label(Token),
    AInstruction(Token),
    CInstruction { dest: Vec<Token>, comp: Expression, jump: Option<Token> }
}

impl Instruction {
    pub fn symbol_string(&self) -> Option<&str> {
        match self {
            Instruction::Label(t) => match t.kind {
                TokenKind::Label(ref i) => Some(i),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn binary_string(&self, symbols: &mut SymbolTable) -> Result<Option<String>> {
        if let Some(b) = self.binary(symbols)? {
            return Ok(Some(format!("{:016b}", b)))
        }
        Ok(None)
    }

    pub fn binary(&self, symbols: &mut SymbolTable) -> Result<Option<u16>> {
        use instruction::Instruction::*;
        match self {
            Label(_) => Ok(None),
            AInstruction(t) => Ok(Some(self.a_binary(&t, symbols)?)),
            CInstruction { dest, comp, jump } => Ok(Some(self.c_binary(dest, comp, jump)?)),
        }
    }

    fn a_binary(&self, token: &Token, symbols: &mut SymbolTable) -> Result<u16> {
        match token.kind {
            TokenKind::Address(n) => {
                if n > std::u16::MAX as u32 {
                    return Err(self.error("Address value greater than 16-bit address width", token.line))
                }
                Ok(n as u16)
            },
            TokenKind::Symbol(ref s) => Ok(symbols.address_for(&s).clone()),
            _ => Err(self.error("Token cannot be encoded as a instruction", token.line))
        }
    }

    fn c_binary(&self, dest: &Vec<Token>, comp: &Expression, jump: &Option<Token>) -> Result<u16> {
        let mut code: u16 = 0xE000;
        code |= self.opcode(&comp);
        code |= self.comp_bits(&comp)?;
        code |= self.dest_bits(&dest)?;
        code |= self.jump_bits(&jump)?;
        Ok(code)
    }

    fn opcode(&self, comp: &Expression) -> u16 {
        match comp {
            Expression::Binary{ left, operator: _, right } => {
                self.memory_code(&left) | self.memory_code(&right)
            },
            Expression::Unary{ operator: _, right } => self.memory_code(&right),
            Expression::Literal(t) => self.memory_code(&t)
        }
    }

    fn memory_code(&self, t: &Token) -> u16 {
        match t.kind {
            TokenKind::Memory => 0x1000,
            _ => 0x0,
        }
    }

    fn comp_bits(&self, comp: &Expression) -> Result<u16> {
        use token::TokenKind::*;
        match comp {
            Expression::Binary{ left, operator, right } => {
                match operator.kind {
                    Plus => match (&left.kind, &right.kind) {
                        (DRegister, Number(1)) => Ok(0x7C0),
                        (ARegister, Number(1)) | (Memory, Number(1)) => Ok(0xDC0),
                        (DRegister, ARegister) | (DRegister, Memory) => Ok(0x80),
                        _ => Err(self.error("Invalid + binary expression", left.line)),
                    },
                    Minus => match (&left.kind, &right.kind) {
                        (DRegister, Number(1)) => Ok(0x380),
                        (ARegister, Number(1)) | (Memory, Number(1)) => Ok(0xC80),
                        (DRegister, ARegister) | (DRegister, Memory) => Ok(0x4C0),
                        (ARegister, DRegister) | (Memory, DRegister) => Ok(0x1C0),
                        _ => Err(self.error("Invalid - binary expression", left.line)),
                    },
                    And => match (&left.kind, &right.kind) {
                        (DRegister, ARegister) | (DRegister, Memory) => Ok(0x0),
                        _ => Err(self.error("Invalid & binary expression", left.line)),
                    },
                    Or => match (&left.kind, &right.kind) {
                        (DRegister, ARegister) | (DRegister, Memory) => Ok(0x540),
                        _ => Err(self.error("Invalid | binary expression", left.line)),
                    },
                    _ => Err(self.error("Invalid binary expression", operator.line)),
                }
            },
            Expression::Unary{ operator, right } => {
                match operator.kind {
                    Minus => match right.kind {
                        Number(1) => Ok(0xE80),
                        DRegister => Ok(0x3C0),
                        ARegister | Memory => Ok(0xCC0),
                        _ => Err(self.error("Invalid - unary expression", right.line)),
                    },
                    Not => match right.kind {
                        DRegister => Ok(0x340),
                        ARegister | Memory => Ok(0xC40),
                        _ => Err(self.error("Invalid ! unary expression", right.line)),
                    },
                    _ => Err(self.error("Invalid unary expression", operator.line)),
                }
            },
            Expression::Literal(t) => {
                match t.kind {
                    Number(n) => {
                        match n {
                            0 => Ok(0xA80),
                            1 => Ok(0xFC0),
                            _ => Err(self.error("Invalid value in expression. Only 0 or 1 allowed.", t.line)),
                        }
                    },
                    DRegister => Ok(0x300),
                    ARegister | Memory => Ok(0xC00),
                    _ => Err(self.error("Invalid literal value", t.line)),
                }
            },
        }
    }

    fn dest_bits(&self, dest: &Vec<Token>) -> Result<u16> {
        let mut out = 0x0;
        for d in dest {
            match d.kind {
                TokenKind::ARegister => out |= 0x20,
                TokenKind::DRegister => out |= 0x10,
                TokenKind::Memory => out |= 0x8,
                _ => return Err(self.error("Invalid destination", d.line)),
            };
        }
        Ok(out)
    }

    fn jump_bits(&self, jump: &Option<Token>) -> Result<u16> {
        match jump {
            Some(t) => match t.kind {
                TokenKind::Jump => Ok(0x7),
                TokenKind::JumpGreaterThan => Ok(0x1),
                TokenKind::JumpEqual => Ok(0x2),
                TokenKind::JumpGreaterThanEqual => Ok(0x3),
                TokenKind::JumpLessThan => Ok(0x4),
                TokenKind::JumpNotEqual => Ok(0x5),
                TokenKind::JumpLessThanEqual => Ok(0x6),
                _ => Err(self.error("Invalid jump command", t.line)),
            },
            None => Ok(0x0),
        }
    }

    fn error(&self, description: &str, line: u32) -> InstructionError {
        InstructionError::new(description, line)
    }
}

#[derive(Debug)]
pub struct InstructionError {
    description: String,
    line: u32,
}

impl InstructionError {
    fn new(description: &str, line: u32) -> InstructionError {
        InstructionError {
            description: String::from(description),
            line: line,
        }
    }
}

impl std::fmt::Display for InstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Instruction error: [Line {}] {} ", self.line, self.description)
    }
}

impl std::error::Error for InstructionError {}

#[derive(Debug)]
pub enum Expression {
    Binary { left: Token, operator: Token, right: Token },
    Unary { operator: Token, right: Token },
    Literal(Token),
}