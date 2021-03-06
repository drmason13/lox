use std::{
    fs::File,
    io::{stdin, Read},
};

use anyhow::Result;

use crate::{
    ast::{ExprStmt, Stmt},
    evaluate::Evaluator,
    lex::Lexer,
    LoxError,
};

pub struct Interpreter {
    evaluator: Evaluator,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            evaluator: Evaluator,
        }
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
            if let Err(e) = self.run(line.trim_end_matches('\n')) {
                // REPL is more forgiving of errors, print the error and keep looping!
                eprintln!("{}", e);
            }
        }

        Ok(())
    }

    pub fn run(&mut self, source: impl Into<String>) -> Result<(), LoxError> {
        let source = source.into();

        let scanner: Lexer = Lexer::new(source);
        if let Some(statement) = scanner.advance_to_parsing().next() {
            if let Stmt::ExprStmt(ExprStmt(expr)) = statement? {
                let result = self.evaluator.evaluate(expr)?;
                println!("{}", &result);
            }
        }

        Ok(())
    }
}
