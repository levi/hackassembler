use std::env;
use std::time::{SystemTime};

use assembler::Assembler;

mod instruction;
mod parser;
mod token;
mod scanner;
mod scanlines;
mod symbol_table;
mod assembler;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: hackassembler [asm_file]");
    } else {
        let now = SystemTime::now();

        let filepath = &args[1];
        println!("Assembling: {}", filepath);

        let mut a = Assembler::new();
        match a.assemble_file(filepath) {
            Ok(_) => {
                if let Ok(elapsed) = now.elapsed() {
                    println!("Compilation successful. Done in {} seconds!", elapsed.subsec_nanos() as f64 / 1000_000_000_f64);
                }
            },
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
}
