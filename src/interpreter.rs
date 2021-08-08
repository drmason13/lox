use std::{
    fs::File,
    io::{stdin, Read},
};

use anyhow::Result;

use crate::{lexing::Lexer, LoxError};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn run_file(&mut self, path: String) -> Result<()> {
        let mut file = match File::open(&path) {
            Err(e) => panic!("couldn't open {}: {}", path, e),
            Ok(file) => file,
        };

        let mut src = String::new();
        match file.read_to_string(&mut src) {
            Err(e) => panic!("couldn't read {}: {}", path, e),
            Ok(_) => Ok(self.run(src)?),
        }
    }

    pub fn run_prompt(&mut self) -> Result<()> {
        let stdin = stdin();

        loop {
            let mut line = String::new();
            // check if we received 0 bytes...
            if let 0 = stdin.read_line(&mut line)? {
                // ... received EOF, for example if the user pressed ctrl-D
                break;
            };
            // otherwise we read a line of (possibly invalid) code and should try to run it
            if let Err(e) = self.run(line) {
                // REPL is more forgiving of errors, print the error and keep looping!
                eprintln!("{}", e);
            }
        }

        Ok(())
    }

    pub fn run(&mut self, source: String) -> Result<(), LoxError> {
        let scanner: Lexer = Lexer::new(source);

        // For now, just print the tokens.
        for token in scanner.scan_tokens()? {
            println!("token: `{}`", token);
        }
        Ok(())
    }
}
