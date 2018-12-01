use std::convert;
use std::io;
use std::io::prelude::*;
use std::fs::File;

use scanlines::Scanlines;
use scanner::{ScannerError};
use token::{Token, TokenKind};
use parser::{Parser, ParserError};
use instruction::{Instruction, InstructionError};
use symbol_table::SymbolTable;

type Result<T> = std::result::Result<T, AssemblerError>;

pub struct Assembler {
    symbols: SymbolTable,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            symbols: SymbolTable::new(),
        }
    }

    pub fn assemble_file(&mut self, filepath: &str) -> Result<()> {
        let filename = filepath.split("/").last().unwrap();
        let filename = filename.split(".").next().unwrap();

        let file = File::open(filepath)?;

        let tokens = self.tokenize(file)?;
        let instructions = self.parse(tokens)?;
        let output_string = self.encode_binary(&instructions)?;
        self.write_file(filename, output_string)?;

        Ok(())
    }

    fn tokenize(&mut self, file: File) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut sl = Scanlines::new(file);
        while let Some(line) = sl.next() {
            let mut line_tokens = Vec::new();
            let mut line = line?;
            for result in line {
                let result = result?;
                line_tokens.push(result);
            }
            if !line_tokens.is_empty() && line_tokens[0].kind != TokenKind::NewLine {
                tokens.append(&mut line_tokens);
            }
        }

        // Add end of file to let the parser terminate
        let mut last_line: u32 = 0;
        if let Some(t) = tokens.last() {
            last_line = t.line;
        }
        tokens.push(Token::new(TokenKind::EOF, last_line + 1));
        Ok(tokens)
    }

    fn parse(&mut self, tokens: Vec<Token>) -> Result<Vec<Instruction>> {
        let mut p = Parser::new(tokens);
        Ok(p.parse()?)
    }

    fn encode_binary(&mut self, instructions: &Vec<Instruction>) -> Result<String> {
        let mut out = String::new();

        let mut rom_address: u16 = 0;
        for i in instructions {
            match i.symbol_string() {
                Some(s) => self.symbols.add_symbol(s, rom_address),
                None => rom_address += 1,
            };
        }

        for i in instructions {
            if let Some(b) = i.binary_string(&mut self.symbols)? {
                out.push_str(&format!("{}\n", b));
            }
        }

        Ok(out)
    }

    fn write_file(&self, filename: &str, output_string: String) -> Result<()> {
        let mut file = File::create(format!("{}.hack", filename))?;
        file.write_all(output_string.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum AssemblerError {
    IoError(io::Error),
    ScanError(ScannerError),
    ParseError(ParserError),
    InstructionError(InstructionError),
}

impl std::fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AssemblerError::IoError(err) => write!(f, "{}", err),
            AssemblerError::ScanError(err) => write!(f, "{}", err),
            AssemblerError::ParseError(err) => write!(f, "{}", err),
            AssemblerError::InstructionError(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for AssemblerError {}

impl convert::From<io::Error> for AssemblerError {
    fn from(error: io::Error) -> Self {
        AssemblerError::IoError(error)
    }
}

impl convert::From<ScannerError> for AssemblerError {
    fn from(error: ScannerError) -> Self {
        AssemblerError::ScanError(error)
    }
}

impl convert::From<ParserError> for AssemblerError {
    fn from(error: ParserError) -> Self {
        AssemblerError::ParseError(error)
    }
}

impl convert::From<InstructionError> for AssemblerError {
    fn from(error: InstructionError) -> Self {
        AssemblerError::InstructionError(error)
    }
}