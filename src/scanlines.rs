use std::io;
use std::io::prelude::*;
use scanner::Scanner;

pub struct Scanlines<R: Read> {
    reader: io::BufReader<R>,
    line: String,
    line_num: u32,
}

impl <'a, R: Read> Scanlines<R> {
    pub fn new(f: R) -> Scanlines<R> {
        Scanlines {
            reader: io::BufReader::new(f),
            line: String::new(),
            line_num: 0,
        }
    }

    pub fn next(&'a mut self) -> Option<io::Result<Scanner<'a>>> {
        self.line.clear();
        match self.reader.read_line(&mut self.line) {
            Ok(num_bytes) => if num_bytes == 0 {
                return None;
            },
            Err(e) => return Some(Err(e))
        }
        self.line_num += 1;
        Some(Ok(Scanner::new(&self.line, self.line_num)))
    }
}
