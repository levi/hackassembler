use token::{Token, TokenKind};

type Result<T> = std::result::Result<T, InstructionError>;

#[derive(Debug)]
pub enum Instruction {
    Symbol(Token),
    AInstruction(Token),
    CInstruction { dest: Vec<Token>, comp: Expression, jump: Option<Token> }
}

impl Instruction {
    pub fn binary_string(&self) -> Result<Option<String>> {
        let binary = self.binary()?;
        match binary {
            Some(bits) => {
                let mut out = String::new();
                for bit in bits {
                    out.push_str(&bit.to_string());
                }
                Ok(Some(out))
            },
            None => Ok(None),
        }
    }

    pub fn identifier_symbol(&self) -> Option<&str> {
        match self {
            Instruction::Symbol(t) => {
                match t.kind {
                    TokenKind::Symbol(ref i) => Some(i),
                    _ => None,
                }
            },
            _ => None,
        }
    }

    pub fn binary(&self) -> Result<Option<Vec<u8>>> {
        match self {
            Instruction::Symbol(_) => Ok(None),
            Instruction::AInstruction(t) => Ok(Some(self.a_binary(&t)?)),
            Instruction::CInstruction { dest, comp, jump } => {
                let binary = self.c_binary(dest, comp, jump)?;
                Ok(Some(binary))
            },
        }
    }

    fn a_binary(&self, token: &Token) -> Result<Vec<u8>> {
        match token.kind {
            TokenKind::Address(n) => {
                if n > std::u16::MAX as u32 {
                    return Err(self.error("Address value greater than 16-bit address width", token.line))
                }
                Ok(format!("{:b}", n as u16).chars().map(|i| i.to_digit(10).unwrap() as u8).collect())
            },
            _ => Err(self.error("Label a instructions not yet supported", token.line)),
        }
    }

    fn c_binary(&self, dest: &Vec<Token>, comp: &Expression, jump: &Option<Token>) -> Result<Vec<u8>> {
        let mut binary: Vec<u8> = vec![1,1,1,0];
        binary[3] = self.opcode(&comp) as u8;

        let comp = self.comp_bits(&comp)?;
        binary.extend(&comp);
        let dest = self.dest_bits(&dest)?;
        binary.extend(&dest);
        let jump = self.jump_bits(&jump)?;
        binary.extend(&jump);

        Ok(binary)
    }

    fn opcode(&self, comp: &Expression) -> bool {
        match comp {
            Expression::Binary{ left, operator: _, right } => {
                self.memory_code(&left) || self.memory_code(&right)
            },
            Expression::Unary{ operator: _, right } => self.memory_code(&right),
            Expression::Literal(t) => self.memory_code(&t)
        }
    }

    fn memory_code(&self, t: &Token) -> bool {
        match t.kind {
            TokenKind::Memory => true,
            _ => false,
        }
    }

    fn comp_bits(&self, comp: &Expression) -> Result<Vec<u8>> {
        use token::TokenKind::*;
        match comp {
            Expression::Binary{ left, operator, right } => {
                match operator.kind {
                    Plus => match (&left.kind, &right.kind) {
                        (DRegister, Number(1)) => Ok(vec![0,1,1,1,1,1]),
                        (ARegister, Number(1)) | (Memory, Number(1)) => Ok(vec![1,1,0,1,1,1]),
                        (DRegister, ARegister) | (DRegister, Memory) => Ok(vec![0,0,0,0,1,0]),
                        _ => Err(self.error("Invalid + binary expression", left.line)),
                    },
                    Minus => match (&left.kind, &right.kind) {
                        (DRegister, Number(1)) => Ok(vec![0,0,1,1,1,0]),
                        (ARegister, Number(1)) | (Memory, Number(1)) => Ok(vec![1,1,0,0,1,0]),
                        (DRegister, ARegister) | (DRegister, Memory) => Ok(vec![0,1,0,0,1,1]),
                        (ARegister, DRegister) | (Memory, DRegister) => Ok(vec![0,0,0,1,1,1]),
                        _ => Err(self.error("Invalid - binary expression", left.line)),
                    },
                    And => match (&left.kind, &right.kind) {
                        (DRegister, ARegister) | (DRegister, Memory) => Ok(vec![0,0,0,0,0,0]),
                        _ => Err(self.error("Invalid & binary expression", left.line)),
                    },
                    Or => match (&left.kind, &right.kind) {
                        (DRegister, ARegister) | (DRegister, Memory) => Ok(vec![0,1,0,1,0,1]),
                        _ => Err(self.error("Invalid | binary expression", left.line)),
                    },
                    _ => Err(self.error("Invalid binary expression", operator.line)),
                }
            },
            Expression::Unary{ operator, right } => {
                match operator.kind {
                    Minus => match right.kind {
                        Number(1) => Ok(vec![1,1,1,0,1,0]),
                        DRegister => Ok(vec![0,0,1,1,1,1]),
                        ARegister | Memory => Ok(vec![1,1,0,0,1,1]),
                        _ => Err(self.error("Invalid - unary expression", right.line)),
                    },
                    Not => match right.kind {
                        DRegister => Ok(vec![0,0,1,1,0,1]),
                        ARegister | Memory => Ok(vec![1,1,0,0,0,1]),
                        _ => Err(self.error("Invalid ! unary expression", right.line)),
                    },
                    _ => Err(self.error("Invalid unary expression", operator.line)),
                }
            },
            Expression::Literal(t) => {
                match t.kind {
                    Number(n) => {
                        match n {
                            0 => Ok(vec![0,1,0,1,0,1,0]),
                            1 => Ok(vec![0,1,1,1,1,1,1]),
                            _ => Err(self.error("Invalid value in expression. Only 0 or 1 allowed.", t.line)),
                        }
                    },
                    DRegister => Ok(vec![0,0,0,1,1,0,0]),
                    ARegister => Ok(vec![0,1,1,0,0,0,0]),
                    Memory => Ok(vec![1,1,1,0,0,0,0]),
                    _ => Err(self.error("Invalid literal value", t.line)),
                }
            },
        }
    }

    fn dest_bits(&self, dest: &Vec<Token>) -> Result<Vec<u8>> {
        let mut bits = vec![0, 0, 0];
        for d in dest {
            match d.kind {
                TokenKind::ARegister => bits[0] = 1,
                TokenKind::DRegister => bits[1] = 1,
                TokenKind::Memory => bits[2] = 1,
                _ => return Err(self.error("Invalid destination", d.line)),
            };
        }
        Ok(bits)
    }

    fn jump_bits(&self, jump: &Option<Token>) -> Result<Vec<u8>> {
        match jump {
            Some(t) => match t.kind {
                TokenKind::Jump => Ok(vec![1, 1, 1]),
                TokenKind::JumpGreaterThan => Ok(vec![0, 0, 1]),
                TokenKind::JumpEqual => Ok(vec![0, 1, 0]),
                TokenKind::JumpGreaterThanEqual => Ok(vec![0, 1, 1]),
                TokenKind::JumpLessThan => Ok(vec![1, 0, 0]),
                TokenKind::JumpNotEqual => Ok(vec![1, 0, 1]),
                TokenKind::JumpLessThanEqual => Ok(vec![1, 1, 0]),
                _ => Err(self.error("Invalid jump command", t.line)),
            },
            None => Ok(vec![0, 0, 0]),
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