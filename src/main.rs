use std::env;
use std::fs::File;
use scanlines::Scanlines;

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
        let mut sl = Scanlines::new(file);
        while let Some(line) = sl.next() {
            let mut line = line?;
            while let Some(token) = line.next() {
                println!("Token: {:?}", token);
            }
            if let Some(err) = line.error {
                println!("Error: {:?}", err);
            }
        }
    }

    Ok(())
}
