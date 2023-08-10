use std::collections::HashMap;

use super::{
    keyword::KEYWORDS,
    token::{Token, TokenType},
};

pub(crate) struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn add_token_with_literal(
        &mut self,
        token_type: TokenType,
        literal: Option<HashMap<String, String>>,
    ) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            println!("Unterminated string.");
            return;
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        let mut map = HashMap::new();
        map.insert("value".to_string(), value);
        self.add_token_with_literal(TokenType::String, Some(map));
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current].to_string();
        let mut map = HashMap::new();
        map.insert("value".to_string(), value);
        self.add_token_with_literal(TokenType::Number, Some(map));
    }

    fn get_identifier_type(&self) -> TokenType {
        let text = self.source[self.start..self.current].to_string();

        let keywords = KEYWORDS.lock().unwrap();
        match keywords.get(&text.as_str()) {
            Some(token_type) => *token_type,
            None => TokenType::Identifier,
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        self.add_token(self.get_identifier_type());
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),

            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }

            '/' => {
                if self.match_char('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,

            '"' => self.string(),

            'o' => {
                if self.match_char('r') {
                    self.add_token(TokenType::Or);
                }
            }

            _ => {
                if c.is_digit(10) {
                    return self.number();
                } else if c.is_alphanumeric() {
                    return self.identifier();
                }

                println!("Unexpected character: {}", c);
            }
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            String::from(""),
            Some(HashMap::new()),
            self.line,
        ));
        &self.tokens
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{scanner::Scanner, token::TokenType};

    #[test]
    fn test_scanner_variable() {
        let expected_tokens = [
            TokenType::Let,
            TokenType::Equal,
            TokenType::Number,
            TokenType::Semicolon,
            TokenType::EOF,
        ];

        let source = String::from("let = 1;");
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.token_type, expected_tokens[i]);
        }

        assert_eq!(tokens.len(), expected_tokens.len());
    }

    #[test]
    fn test_scanner_variable_with_string() {
        let expected_tokens = [
            TokenType::Let,
            TokenType::Equal,
            TokenType::String,
            TokenType::Semicolon,
            TokenType::EOF,
        ];

        let source = String::from("let = \"hello\";");
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.token_type, expected_tokens[i]);
        }

        assert_eq!(tokens.len(), expected_tokens.len());
    }

    #[test]
    fn test_scanner_function() {
        let expected_tokens = [
            TokenType::Function,
            TokenType::Identifier,
            TokenType::LeftParen,
            TokenType::Identifier,
            TokenType::Comma,
            TokenType::Identifier,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::Return,
            TokenType::Identifier,
            TokenType::Plus,
            TokenType::Identifier,
            TokenType::Semicolon,
            TokenType::RightBrace,
            TokenType::EOF,
        ];

        let source = String::from("function add(a, b) { return a + b; }");
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.token_type, expected_tokens[i]);
        }

        assert_eq!(tokens.len(), expected_tokens.len());
    }

    #[test]
    fn test_scanner_complex() {
        let expected_tokens = [
            TokenType::Let,
            TokenType::Identifier,
            TokenType::Equal,
            TokenType::Number,
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Identifier,
            TokenType::Equal,
            TokenType::Number,
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Identifier,
            TokenType::Equal,
            TokenType::Identifier,
            TokenType::Plus,
            TokenType::Identifier,
            TokenType::Semicolon,
            TokenType::Function,
            TokenType::Identifier,
            TokenType::LeftParen,
            TokenType::Identifier,
            TokenType::Comma,
            TokenType::Identifier,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::Return,
            TokenType::Identifier,
            TokenType::Plus,
            TokenType::Identifier,
            TokenType::Semicolon,
            TokenType::RightBrace,
            TokenType::Let,
            TokenType::Identifier,
            TokenType::Equal,
            TokenType::Identifier,
            TokenType::LeftParen,
            TokenType::Identifier,
            TokenType::Comma,
            TokenType::Identifier,
            TokenType::RightParen,
            TokenType::Semicolon,
            TokenType::EOF,
        ];

        let source = String::from(
            "
            let a = 1;
            let b = 2;
            let c = a + b;
            function add(a, b) {
                return a + b;
            }
            let d = add(a, b);
            ",
        );
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.token_type, expected_tokens[i]);
        }

        assert_eq!(tokens.len(), expected_tokens.len());
    }
}
