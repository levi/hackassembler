mod token;
mod scanner;

use std::env;
use scanner::Scanner;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: hackassembler [asm_file]");
    } else {
        let filename = &args[1];
        let scn = Scanner::with_source_path(filename)?;
        scn.scan_tokens();
        for token in scn.tokens {
            println!("Token: {}", token);
        }
    }

    Ok(())
}
