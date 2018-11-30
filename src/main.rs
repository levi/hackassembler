use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::time::{SystemTime};

use scanlines::Scanlines;
use token::{Token, TokenType};
use parser::Parser;

mod code;
mod parser;
mod token;
mod scanner;
mod scanlines;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: hackassembler [asm_file]");
    } else {
        let now = SystemTime::now();

        let filepath = &args[1];
        let filename = filepath.split("/").last().unwrap();
        let filename = filename.split(".").next().unwrap();

        let file = File::open(filepath)?;

        let mut tokens: Vec<Token> = Vec::new();

        let mut sl = Scanlines::new(file);
        while let Some(line) = sl.next() {
            let mut line_tokens: Vec<Token> = Vec::new();
            let mut line = line?;
            for result in line {
                match result {
                    Ok(token) => line_tokens.push(token),
                    Err(err) => {
                        println!("Error: {:?}", err);
                    }
                }
            }
            if !line_tokens.is_empty() && line_tokens[0].token != TokenType::NewLine {
                tokens.append(&mut line_tokens);
            }
        }

        // Add end of file to let the parser terminate
        let mut last_line: u32 = 0;
        if let Some(t) = tokens.last() {
            last_line = t.line;
        }
        tokens.push(Token::new(TokenType::EOF, last_line + 1));
        
        let mut did_error = false;

        let mut p = Parser::new(tokens);
        match p.parse() {
            Ok(codes) => {
                let mut out = String::new();
                for code in codes {
                    match code.binary_string() {
                        Ok(binary) => match binary {
                            Some(b) => out.push_str(&format!("{}\n", b)),
                            None => {},
                        },
                        Err(err) => {
                            did_error = true;
                            println!("Code error: {:?}", err)
                        },
                    };
                }
                if !did_error {
                    let mut out_file = File::create(format!("{}.hack", filename))?;
                    out_file.write_all(out.as_bytes())?;
                    if let Ok(elapsed) = now.elapsed() {
                        println!("Compilation successful. Done in {} seconds!", elapsed.subsec_nanos() as f64 / 1000_000_000_f64);
                    }
                }
            },
            Err(err) => println!("Parser error: {:?}", err),
        };
    }

    Ok(())
}
