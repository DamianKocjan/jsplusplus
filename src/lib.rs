use anyhow::Result;
use parser::{
    scanner::Scanner,
    token::{Token, TokenType},
    Parser,
};
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

    pub fn error(line: usize, message: &str) {
        JSPlusPlus::report(line, "", message);
    }

    pub fn error_token(token: &Token, message: &str) {
        if token.token_type == TokenType::EOF {
            JSPlusPlus::report(token.line, " at end", message);
        } else {
            JSPlusPlus::report(token.line, &format!(" at '{}'", token.lexeme), message);
        }
    }

    fn report(line: usize, location: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, location, message);
    }

    fn run(&self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{}", token.to_string());
        }

        let mut parser = Parser::new(tokens.clone());
        let stmts = parser.parse();

        for stmt in stmts {
            println!("{:?}", stmt);
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
