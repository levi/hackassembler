use std::convert;
use std::io;
use std::io::prelude::*;
use std::fs::File;

use scanlines::Scanlines;
use scanner::{ScannerError};
use token::{Token, TokenType};
use parser::{Parser, ParserError};
use instruction::{Instruction, InstructionError};
use symbol_table::SymbolTable;

type Result<T> = std::result::Result<T, AssemblerError>;

pub struct Assembler {
    symbols: SymbolTable,
    tokens: Vec<Token>,
    instructions: Vec<Instruction>,
    binary_output: String,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            symbols: SymbolTable::new(),
            tokens: Vec::new(),
            instructions: Vec::new(),       
            binary_output: String::new(),
        }
    }

    pub fn assemble_file(&mut self, filepath: &str) -> Result<()> {
        let filename = filepath.split("/").last().unwrap();
        let filename = filename.split(".").next().unwrap();

        let file = File::open(filepath)?;

        self.tokenize(file)?;
        self.parse()?;
        self.scan_addresses()?;
        self.encode_instructions()?;
        self.write_file(filename)?;
        Ok(())
    }

    fn tokenize(&mut self, file: File) -> Result<()> {
        let mut sl = Scanlines::new(file);
        while let Some(line) = sl.next() {
            let mut line_tokens: Vec<Token> = Vec::new();
            let mut line = line?;
            for result in line {
                let result = result?;
                line_tokens.push(result);
            }
            if !line_tokens.is_empty() && line_tokens[0].token != TokenType::NewLine {
                self.tokens.append(&mut line_tokens);
            }
        }

        // Add end of file to let the parser terminate
        let mut last_line: u32 = 0;
        if let Some(t) = self.tokens.last() {
            last_line = t.line;
        }
        self.tokens.push(Token::new(TokenType::EOF, last_line + 1));

        Ok(())
    }

    fn parse(&mut self) -> Result<()> {
        let mut p = Parser::new(self.tokens);
        self.instructions = p.parse()?;
        Ok(())
    }

    fn scan_addresses(&mut self) -> Result<()> {
        let mut rom_address: u16 = 0;
        for i in self.instructions {
            match i.identifier_symbol() {
                Some(s) => self.symbols.add_symbol(s, rom_address),
                None => rom_address += 1,
            };
        }
        Ok(())
    }

    fn encode_instructions(&mut self) -> Result<()> {
        let mut out = String::new();
        for i in self.instructions {
            match i.binary_string() {
                Ok(binary) => match binary {
                    Some(b) => out.push_str(&format!("{}\n", b)),
                    None => {},
                },
                Err(err) => {
                    println!("Instruction error: {:?}", err)
                },
            };
        }
        Ok(())
    }

    fn write_file(&self, filename: &str) -> Result<()> {
        let mut file = File::create(format!("{}.hack", filename))?;
        file.write_all(self.binary_output.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug)]
enum AssemblerError {
    IoError(io::Error),
    ScanError(ScannerError),
    ParseError(ParserError),
    InstructionError(InstructionError),
}

impl std::error::Error for AssemblerError {

}

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