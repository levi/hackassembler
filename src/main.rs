use std::env;
use std::fs::File;
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
        let filename = &args[1];
        let file = File::open(filename)?;

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

        if let Some(t) = tokens.last() {
            tokens.push(Token::new(TokenType::EOF, t.line + 1));
        }

        let p = Parser::new(tokens);
        let codes = p.parse();

        println!("{:?}", codes);
    }

    Ok(())
}
