use token::{Token, TokenType};

#[derive(Debug)]
pub struct InstructionError {
    description: String,
}

impl InstructionError {
    fn new(description: &str) -> InstructionError {
        InstructionError {
            description: String::from(description),
        }
    }
}

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

    pub fn identifier_symbol(&self) -> Option<String> {
        match self {
            Instruction::Symbol(t) => {
                match t.token {
                    TokenType::Symbol(i) => Some(i),
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
        match token.token {
            TokenType::Address(n) => {
                if n > std::u16::MAX as u32 {
                    return Err(self.error("Address value greater than 16-bit address width"))
                }
                Ok(format!("{:b}", n as u16).chars().map(|i| i.to_digit(10).unwrap() as u8).collect())
            },
            _ => Err(self.error("Label a instructions not yet supported")),
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
        match t.token {
            TokenType::Memory => true,
            _ => false,
        }
    }

    fn comp_bits(&self, comp: &Expression) -> Result<Vec<u8>> {
        use token::TokenType::*;
        match comp {
            Expression::Binary{ left, operator, right } => {
                match operator.token {
                    Plus => match (&left.token, &right.token) {
                        (DRegister, Number(1)) => Ok(vec![0,1,1,1,1,1]),
                        (ARegister, Number(1)) | (Memory, Number(1)) => Ok(vec![1,1,0,1,1,1]),
                        (DRegister, ARegister) | (DRegister, Memory) => Ok(vec![0,0,0,0,1,0]),
                        _ => Err(self.error("Invalid + binary expression")),
                    },
                    Minus => match (&left.token, &right.token) {
                        (DRegister, Number(1)) => Ok(vec![0,0,1,1,1,0]),
                        (ARegister, Number(1)) | (Memory, Number(1)) => Ok(vec![1,1,0,0,1,0]),
                        (DRegister, ARegister) | (DRegister, Memory) => Ok(vec![0,1,0,0,1,1]),
                        (ARegister, DRegister) | (Memory, DRegister) => Ok(vec![0,0,0,1,1,1]),
                        _ => Err(self.error("Invalid - binary expression")),
                    },
                    And => match (&left.token, &right.token) {
                        (DRegister, ARegister) | (DRegister, Memory) => Ok(vec![0,0,0,0,0,0]),
                        _ => Err(self.error("Invalid & binary expression")),
                    },
                    Or => match (&left.token, &right.token) {
                        (DRegister, ARegister) | (DRegister, Memory) => Ok(vec![0,1,0,1,0,1]),
                        _ => Err(self.error("Invalid | binary expression")),
                    },
                    _ => Err(self.error("Invalid binary expression")),
                }
            },
            Expression::Unary{ operator, right } => {
                match operator.token {
                    Minus => match right.token {
                        Number(1) => Ok(vec![1,1,1,0,1,0]),
                        DRegister => Ok(vec![0,0,1,1,1,1]),
                        ARegister | Memory => Ok(vec![1,1,0,0,1,1]),
                        _ => Err(self.error("Invalid - unary expression")),
                    },
                    Not => match right.token {
                        DRegister => Ok(vec![0,0,1,1,0,1]),
                        ARegister | Memory => Ok(vec![1,1,0,0,0,1]),
                        _ => Err(self.error("Invalid ! unary expression")),
                    },
                    _ => Err(self.error("Invalid unary expression")),
                }
            },
            Expression::Literal(t) => {
                match t.token {
                    Number(n) => {
                        match n {
                            0 => Ok(vec![0,1,0,1,0,1,0]),
                            1 => Ok(vec![0,1,1,1,1,1,1]),
                            _ => Err(self.error("Invalid value in expression. Only 0 or 1 allowed.")),
                        }
                    },
                    DRegister => Ok(vec![0,0,0,1,1,0,0]),
                    ARegister => Ok(vec![0,1,1,0,0,0,0]),
                    Memory => Ok(vec![1,1,1,0,0,0,0]),
                    _ => Err(self.error("Invalid literal value")),
                }
            },
        }
    }

    fn dest_bits(&self, dest: &Vec<Token>) -> Result<Vec<u8>> {
        let mut bits = vec![0, 0, 0];
        for d in dest {
            match d.token {
                TokenType::ARegister => bits[0] = 1,
                TokenType::DRegister => bits[1] = 1,
                TokenType::Memory => bits[2] = 1,
                _ => return Err(self.error("Invalid destination")),
            };
        }
        Ok(bits)
    }

    fn jump_bits(&self, jump: &Option<Token>) -> Result<Vec<u8>> {
        match jump {
            Some(t) => match t.token {
                TokenType::Jump => Ok(vec![1, 1, 1]),
                TokenType::JumpGreaterThan => Ok(vec![0, 0, 1]),
                TokenType::JumpEqual => Ok(vec![0, 1, 0]),
                TokenType::JumpGreaterThanEqual => Ok(vec![0, 1, 1]),
                TokenType::JumpLessThan => Ok(vec![1, 0, 0]),
                TokenType::JumpNotEqual => Ok(vec![1, 0, 1]),
                TokenType::JumpLessThanEqual => Ok(vec![1, 1, 0]),
                _ => Err(self.error("Invalid jump command")),
            },
            None => Ok(vec![0, 0, 0]),
        }
    }

    fn error(&self, description: &str) -> InstructionError {
        InstructionError::new(description)
    }
}

#[derive(Debug)]
pub enum Expression {
    Binary { left: Token, operator: Token, right: Token },
    Unary { operator: Token, right: Token },
    Literal(Token),
}