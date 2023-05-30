use anyhow::Result;
use parser::scanner::Scanner;
use std::{fs::File, io::Read, path::PathBuf};

mod ast;
mod parser;

pub struct JSPlusPlus {
    had_error: bool,
}

impl JSPlusPlus {
    pub fn new() -> JSPlusPlus {
        JSPlusPlus { had_error: false }
    }

    pub fn error(&self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&self, line: usize, location: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, location, message);
    }

    fn run(&self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{}", token.to_string());
        }
    }

    pub fn run_file(&self, path: PathBuf) -> Result<()> {
        let file = File::open(path);
        let mut contents = String::new();
        file?.read_to_string(&mut contents)?;

        self.run(contents);

        if self.had_error {
            std::process::exit(65);
        }

        Ok(())
    }

    pub fn run_prompt(&mut self) -> Result<()> {
        loop {
            print!("> ");
            let mut line = String::new();
            std::io::stdin().read_line(&mut line)?;

            if "exit" == line.trim() {
                break Ok(());
            }

            self.run(line);
            self.had_error = false;
        }
    }
}
